import json
import logging
import time
from time import sleep
from typing import Any, Dict, List

import calliphlox
import pytest
import tifffile
import zarr
import numcodecs.blosc as blosc
import dask.array as da
from calliphlox.calliphlox import DeviceKind, Runtime, Trigger


@pytest.fixture(scope="module")
def runtime():
    runtime = calliphlox.Runtime()
    yield runtime


def test_set():
    t = Trigger()
    assert not t.enable
    t.enable = True
    assert t.enable


def test_list_devices(runtime: Runtime):
    dm = runtime.device_manager()
    for d in dm.devices():
        print(d.dict())


def test_set_camera_identifier(runtime: Runtime):
    dm = runtime.device_manager()

    p = runtime.get_configuration()
    assert (
        p.video[0].camera.identifier is not None
        and p.video[0].camera.identifier.kind == calliphlox.DeviceKind.NONE
    )
    p.video[0].camera.identifier = dm.select(
        calliphlox.DeviceKind.Camera, "simulated: radial sin"
    )
    assert p.video[0].camera.identifier is not None


@pytest.mark.parametrize(
    "input,expected",
    [
        (["does not exist 1", "does not exist 2", "does not exist 3"], None),
        (
            [
                "does not exist 1",
                "simulated: radial sin",
                "simulated: uniform random",
            ],
            "simulated: radial sin",
        ),
        (["simulated: radial sin"], "simulated: radial sin"),
        (["simulated.*sin"], "simulated: radial sin"),
        ([".*radial.*"], "simulated: radial sin"),
        ([], None),
    ],
)
def test_select_one_of(
    runtime: Runtime,
    input: List[str],
    expected: str,
):
    h = runtime.device_manager().select_one_of(DeviceKind.Camera, input)
    result = None if h is None else h.name
    assert result == expected


def test_zero_conf_start(runtime: Runtime):
    with pytest.raises(RuntimeError):
        runtime.start()


def test_repeat_acq(runtime: Runtime):
    p = calliphlox.setup(runtime, "simulated: radial sin", "Trash")
    assert p.video[0].camera.identifier is not None
    assert p.video[0].storage.identifier is not None
    assert p.video[0].storage.settings.filename == "out.tif"
    p.video[0].camera.settings.shape = (192, 108)
    p.video[0].max_frame_count = 10
    p = runtime.set_configuration(p)
    runtime.start()
    while True:
        if a := runtime.get_available_data(0):
            logging.info(f"Got {a.get_frame_count()}")
            break
    runtime.stop()
    assert runtime.get_available_data(0) is None
    # TODO: (nclack) assert 1 acquired frame. stop should block
    runtime.start()
    while True:
        if a := runtime.get_available_data(0):
            logging.info(f"Got {a.get_frame_count()}")
            break
    runtime.stop()
    assert runtime.get_available_data(0) is None
    # TODO: (nclack) assert 1 more acquired frame. stop cancels and waits.


def test_repeat_with_no_stop(runtime: Runtime):
    """Stop is required between starts. This tests that an exception is
    raised."""
    p = calliphlox.setup(runtime, "simulated: radial sin", "Trash")
    assert p.video[0].camera.identifier is not None
    assert p.video[0].storage.identifier is not None
    p.video[0].camera.settings.shape = (192, 108)
    p.video[0].max_frame_count = 11
    p = runtime.set_configuration(p)
    runtime.start()
    # wait for 1 frame
    while True:
        if a := runtime.get_available_data(0):
            logging.info(f"Got {a.get_frame_count()}")
            a = None
            break
    # acq is still on going here
    with pytest.raises(RuntimeError):
        logging.info("Next start should fail gracefully")
        runtime.start()
    runtime.stop()


def test_set_storage(runtime: Runtime):
    dm = runtime.device_manager()

    p = runtime.get_configuration()
    p.video[0].storage.identifier = None
    p = runtime.set_configuration(p)
    assert p.video[0].storage.identifier is not None
    assert p.video[0].storage.identifier.kind == calliphlox.DeviceKind.NONE
    p.video[0].storage.identifier = dm.select(
        calliphlox.DeviceKind.Storage, "Tiff"
    )
    assert p.video[0].storage.identifier is not None

    p.video[0].storage.settings.filename = "out.tif"
    assert p.video[0].storage.settings.filename == "out.tif"


def test_setup(runtime: Runtime):
    p = calliphlox.setup(runtime, "simulated: radial sin", "Trash")
    assert p.video[0].camera.identifier is not None
    assert p.video[0].storage.identifier is not None
    assert p.video[0].storage.settings.filename == "out.tif"
    assert p.video[0].max_frame_count == 100
    p.video[0].camera.settings.shape = (192, 108)
    p = runtime.set_configuration(p)

    logging.info(f"max_frame_count: {p.video[0].max_frame_count}")

    runtime.start()

    nframes = 0
    t0 = time.time()

    def took_too_long():
        # Time limit the test
        return time.time() - t0 > 20.0

    while nframes < p.video[0].max_frame_count and not took_too_long():
        clock = time.time()
        if a := runtime.get_available_data(0):
            packet = a.get_frame_count()
            for f in a.frames():
                logging.info(
                    f"{f.data().shape} {f.data()[0][0][0][0]} {f.metadata()}"
                )
                del f  # <-- fails to get the last frames if this is held?
            del a  # <-- fails to get the last frames if this is held?
            nframes += packet
            logging.info(
                f"frame count: {nframes} - frames in packet: {packet}"
            )

        elapsed = time.time() - clock
        sleep(max(0, 0.1 - elapsed))
    logging.info("stopping")

    runtime.stop()
    if took_too_long():
        raise RuntimeError("Took too long")


def test_selection_is_consistent(runtime: Runtime):
    hcam1 = runtime.device_manager().select(DeviceKind.Camera)
    assert hcam1 is not None
    hcam2 = runtime.device_manager().select(DeviceKind.Camera, hcam1.name)
    assert hcam1 == hcam2


def test_change_filename(runtime: Runtime):
    dm = runtime.device_manager()
    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(DeviceKind.Camera, "simulated.*")
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Tiff")
    p.video[0].max_frame_count = 1

    names = [
        "out1.tif",
        "quite a bit longer.tif",
        "s.tif",
        "another long one ok it is really long this time.tif",
    ]
    for name in names:
        p.video[0].storage.settings.filename = name
        p = runtime.set_configuration(p)
        assert p.video[0].storage.settings.filename == name

        nframes = 0
        runtime.start()
        while nframes < p.video[0].max_frame_count:
            if packet := runtime.get_available_data(0):
                nframes += packet.get_frame_count()
                packet = None
        logging.info("Stopping")
        runtime.stop()


def test_write_external_metadata_to_tiff(
    runtime: Runtime, request: pytest.FixtureRequest
):
    dm = runtime.device_manager()
    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*sin"
    )
    p.video[0].camera.settings.shape = (33, 47)
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Tiff")
    p.video[0].max_frame_count = 3
    p.video[0].storage.settings.filename = f"{request.node.name}.tif"
    metadata = {"hello": "world"}
    p.video[0].storage.settings.external_metadata_json = json.dumps(metadata)
    runtime.set_configuration(p)

    nframes = 0
    runtime.start()
    while nframes < p.video[0].max_frame_count:
        if packet := runtime.get_available_data(0):
            nframes += packet.get_frame_count()
            packet = None
    runtime.stop()

    # Check that the written tif has the expected structure
    with tifffile.TiffFile(p.video[0].storage.settings.filename) as f:

        def meta(iframe: int) -> Dict[Any, Any]:
            return json.loads(f.pages[iframe].tags["ImageDescription"].value)

        # first frame should have metadata
        assert meta(0)["metadata"] == metadata
        assert meta(0)["frame_id"] == 0

        # remaining frames should not, but should have e.g. frame id
        for i in range(1, p.video[0].max_frame_count):
            assert "metadata" not in meta(i).keys()
            assert meta(i)["frame_id"] == i


def test_write_external_metadata_to_zarr(
    runtime: Runtime, request: pytest.FixtureRequest
):
    dm = runtime.device_manager()
    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*sin.*"
    )
    p.video[0].camera.settings.shape = (33, 47)
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Zarr")
    p.video[0].max_frame_count = 4
    p.video[0].storage.settings.filename = f"{request.node.name}.zarr"
    metadata = {"hello": "world"}
    p.video[0].storage.settings.external_metadata_json = json.dumps(metadata)
    runtime.set_configuration(p)

    nframes = 0
    runtime.start()
    while nframes < p.video[0].max_frame_count:
        if packet := runtime.get_available_data(0):
            nframes += packet.get_frame_count()
            packet = None
    runtime.stop()

    data = zarr.open(p.video[0].storage.settings.filename)
    assert data.shape == (
        p.video[0].max_frame_count,
        1,
        p.video[0].camera.settings.shape[1],
        p.video[0].camera.settings.shape[0],
    )
    assert data.attrs.asdict() == metadata


def test_write_compressed_zarr(
        runtime: Runtime, request: pytest.FixtureRequest
):
    dm = runtime.device_manager()
    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*sin.*"
    )
    p.video[0].camera.settings.shape = (64, 48)
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "ZarrBlosc1ZstdByteShuffle")
    p.video[0].max_frame_count = 70
    p.video[0].storage.settings.filename = f"{request.node.name}.zarr"
    metadata = {"foo": "bar"}
    p.video[0].storage.settings.external_metadata_json = json.dumps(metadata)
    runtime.set_configuration(p)

    runtime.start()
    runtime.stop()

    # load from Zarr
    data = zarr.open(p.video[0].storage.settings.filename)

    assert data.compressor.cname == "zstd"
    assert data.compressor.clevel == 1
    assert data.compressor.shuffle == blosc.SHUFFLE

    assert data.shape == (
        p.video[0].max_frame_count,
        1,
        p.video[0].camera.settings.shape[1],
        p.video[0].camera.settings.shape[0],
    )
    assert data.attrs.asdict() == metadata

    # load from Dask
    data = da.from_zarr(p.video[0].storage.settings.filename)
    assert data.shape == (
        p.video[0].max_frame_count,
        1,
        p.video[0].camera.settings.shape[1],
        p.video[0].camera.settings.shape[0],
    )


def test_two_video_streams(runtime: Runtime):
    dm = runtime.device_manager()
    p = runtime.get_configuration()

    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*random.*"
    )
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Trash")
    p.video[0].camera.settings.binning = 1
    p.video[0].camera.settings.shape = (64, 64)
    p.video[0].camera.settings.pixel_type = calliphlox.SampleType.U8
    p.video[0].max_frame_count = 90
    p.video[0].frame_average_count = 0  # disables

    p.video[1].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*sin.*"
    )
    p.video[1].storage.identifier = dm.select(DeviceKind.Storage, "Trash")
    p.video[1].camera.settings.binning = 1
    p.video[1].camera.settings.shape = (64, 64)
    p.video[1].camera.settings.pixel_type = calliphlox.SampleType.U8
    p.video[1].max_frame_count = 71
    p.video[1].frame_average_count = 0  # disables

    p = runtime.set_configuration(p)

    nframes = [0, 0]

    def is_not_done() -> bool:
        return (nframes[0] < p.video[0].max_frame_count) or (
            nframes[1] < p.video[1].max_frame_count
        )

    runtime.start()

    stream_id = 0
    while is_not_done():
        if nframes[stream_id] < p.video[stream_id].max_frame_count:
            if packet := runtime.get_available_data(stream_id):
                n = packet.get_frame_count()
                for (i, frame) in enumerate(packet.frames()):
                    expected_frame_id = nframes[stream_id] + i
                    assert frame.metadata().frame_id == expected_frame_id, (
                        "frame id's didn't match "
                        + f"({frame.metadata().frame_id}!={expected_frame_id})"
                        + f" [stream {stream_id} nframes {nframes}]"
                    )
                    frame = None
                packet = None
                nframes[stream_id] += n
                logging.debug(f"NFRAMES {nframes}")

        stream_id = (stream_id + 1) % 2
    logging.info("Stopping")
    runtime.stop()
    assert nframes[0] == p.video[0].max_frame_count
    assert nframes[1] == p.video[1].max_frame_count


def test_abort(runtime):
    dm = runtime.device_manager()
    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*sin"
    )
    p.video[0].camera.settings.shape = (24, 93)
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Trash")
    p.video[0].max_frame_count = 2**30
    runtime.set_configuration(p)

    nframes = 0
    runtime.start()
    sleep(0.05)
    logging.info("Aborting")
    runtime.abort()

    while packet := runtime.get_available_data(0):
        nframes += packet.get_frame_count()

    del packet

    logging.debug(f"Frame count expected: {p.video[0].max_frame_count}, actual: {nframes}")
    assert nframes < p.video[0].max_frame_count


# FIXME: (nclack) awkwardness around references  (available frames, f)

# NOTES:
#
# With pytest, use `--log-cli-level=0` to see the lowest level logs.
