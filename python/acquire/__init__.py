import time
from typing import Any, List, Optional, Tuple, Union

from . import acquire
from .acquire import *

__doc__ = acquire.__doc__

import logging

FORMAT = (
    "%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s"
)
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)


def setup(
    runtime: Runtime,
    camera: Union[str, List[str]] = "simulated: radial sin",
    storage: Union[str, List[str]] = "Tiff",
    output_filename: Optional[str] = "out.tif",
) -> Properties:
    def normalize_fallback_arg(arg: Union[str, List[str]]) -> List[str]:
        if isinstance(arg, str):
            return [arg]
        return arg

    camera = normalize_fallback_arg(camera)
    storage = normalize_fallback_arg(storage)

    dm = runtime.device_manager()
    p = runtime.get_configuration()

    p.video[0].camera.identifier = dm.select_one_of(DeviceKind.Camera, camera)
    p.video[0].camera.settings.binning = 1
    p.video[0].camera.settings.shape = (640, 480)
    p.video[0].storage.identifier = dm.select_one_of(
        DeviceKind.Storage, storage
    )
    p.video[0].storage.settings.filename = output_filename
    p.video[0].max_frame_count = 100
    p.video[0].frame_average_count = 0  # disables

    return p


def setup_one_streams(runtime: Runtime, frame_count: int) -> Properties:
    dm = runtime.device_manager()
    p = runtime.get_configuration()

    cameras = [
        d.name
        for d in dm.devices()
        if (d.kind == DeviceKind.Camera) and ("C15440" in d.name)
    ]
    logging.warning(f"Cameras {cameras}")

    p.video[0].camera.identifier = dm.select(DeviceKind.Camera, cameras[0])
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Trash")
    # p.video[0].storage.settings.filename = output_filename
    p.video[0].camera.settings.binning = 1
    p.video[0].camera.settings.shape = (2304, 2304)
    p.video[0].camera.settings.pixel_type = SampleType.U16
    p.video[0].max_frame_count = frame_count
    p.video[0].frame_average_count = 0  # disables

    return p


def setup_two_streams(runtime: Runtime, frame_count: int) -> Properties:
    dm = runtime.device_manager()
    p = runtime.get_configuration()

    cameras = [
        d.name
        for d in dm.devices()
        if (d.kind == DeviceKind.Camera) and ("C15440" in d.name)
    ]
    if len(cameras) < 2:
        cameras = ["simulated.*random.*", "simulated.*sin.*"]
    logging.warning(f"Cameras {cameras}")

    p.video[0].camera.identifier = dm.select(DeviceKind.Camera, cameras[0])
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Trash")
    # p.video[0].storage.settings.filename = output_filename
    p.video[0].camera.settings.binning = 1
    p.video[0].camera.settings.shape = (2304, 2304)
    p.video[0].camera.settings.pixel_type = SampleType.U16
    p.video[0].max_frame_count = frame_count
    p.video[0].frame_average_count = 0  # disables

    p.video[1].camera.identifier = dm.select(DeviceKind.Camera, cameras[1])
    p.video[1].storage.identifier = dm.select(DeviceKind.Storage, "Trash")
    # p.video[1].storage.settings.filename = output_filename
    p.video[1].camera.settings.binning = 1
    p.video[1].camera.settings.shape = (64, 64)
    p.video[1].camera.settings.pixel_type = SampleType.U16
    p.video[1].max_frame_count = frame_count
    p.video[1].frame_average_count = 0  # disables

    return p


g_runtime = None


def gui(
    viewer: "napari.Viewer",
    frame_count: int = 100,
):
    """Napari dock-widget plugin entry-point

    This instances a magicgui dock widget that streams video to a layer.
    """
    import numpy.typing as npt
    from napari.qt.threading import thread_worker
    from numpy import cumsum, histogram, where

    update_times: List[float] = []

    def get_runtime():
        global g_runtime
        if g_runtime is None:
            logging.info("INITING RUNTIME")
            g_runtime = acquire.Runtime()
        else:
            logging.info("REUSING RUNTIME")
        return g_runtime

    def update_layer(args: Tuple[npt.NDArray[Any], int]):
        (new_image, stream_id) = args
        layer_key = f"Video {stream_id}"
        try:
            clock = time.time()

            layer = viewer.layers[layer_key]
            layer._slice.image._view = new_image
            layer.events.set_data()
            # layer.data=new_image # public api adds another 1-2 ms/frame

            elapsed = time.time() - clock
            update_times.append(elapsed)
            logging.info(f"UPDATED LAYER {layer_key} in {elapsed} s")
        except KeyError:
            # (nclack) This takes ~ 60ms for 630x480 the one time I measured it
            viewer.add_image(new_image, name=layer_key)

    @thread_worker(connect={"yielded": update_layer})
    def do_acquisition():
        logging.basicConfig(level=logging.DEBUG)
        logging.getLogger().setLevel(logging.DEBUG)

        runtime = get_runtime()
        # p = setup_two_streams(runtime,frame_count)
        p = setup_two_streams(runtime, frame_count)

        p = runtime.set_configuration(p)

        runtime.start()

        nframes = [0, 0]
        stream_id = 0

        def is_not_done() -> bool:
            return (nframes[0] < p.video[0].max_frame_count) or (
                nframes[1] < p.video[1].max_frame_count
            )

        while is_not_done():  # runtime.get_state()==DeviceState.Running:
            clock = time.time()

            if nframes[stream_id] < p.video[stream_id].max_frame_count:

                if packet := runtime.get_available_data(stream_id):
                    n = packet.get_frame_count()
                    f = next(packet.frames())
                    im = f.data().squeeze().copy()
                    logging.debug(
                        f"stream {stream_id} frame {f.metadata().frame_id}"
                    )

                    # TODO: (nclack) fix this awkwardness.
                    f = None
                    packet = None

                    yield (im, stream_id)

                    nframes[stream_id] += n
                    logging.info(
                        f"[stream {stream_id}] frame count: {nframes}"
                    )
            stream_id = (stream_id + 1) % 2

            elapsed = time.time() - clock
            time.sleep(max(0, 0.03 - elapsed))
        logging.info("stopping")

        counts, bins = histogram(update_times)
        p50 = bins[where(cumsum(counts) >= 0.5 * len(update_times))[0][0]]
        p90 = bins[where(cumsum(counts) >= 0.9 * len(update_times))[0][0]]
        logging.info(f"Update times - median: {p50*1e3} ms  90%<{p90*1e3} ms")

        runtime.stop()
        logging.info("STOPPED")

    do_acquisition()


# TODO: (nclack) add context manager around runtime and start/stop
