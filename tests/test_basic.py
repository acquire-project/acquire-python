import json
import logging
import time
from datetime import timedelta
from time import sleep
from typing import Any, Dict, List, Optional

import acquire
from acquire import DeviceKind, DeviceState, Runtime, Trigger, PropertyType
import dask.array as da
import numcodecs.blosc as blosc
import pytest
import tifffile
import zarr
from ome_zarr.io import parse_url
from ome_zarr.reader import Reader
from skimage.transform import downscale_local_mean
import numpy as np


@pytest.fixture(scope="module")
def _runtime():
    runtime = acquire.Runtime()
    yield runtime


@pytest.fixture(scope="function")
def runtime(_runtime: Runtime):
    yield _runtime
    _runtime.set_configuration(acquire.Properties())


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
        and p.video[0].camera.identifier.kind == acquire.DeviceKind.NONE
    )
    p.video[0].camera.identifier = dm.select(
        acquire.DeviceKind.Camera, "simulated: radial sin"
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


def test_select_empty_string(runtime: Runtime):
    assert runtime.device_manager().select(DeviceKind.Storage, "")


def test_zero_conf_start(runtime: Runtime):
    with pytest.raises(RuntimeError):
        runtime.start()


def test_repeat_acq(runtime: Runtime):
    p = acquire.setup(runtime, "simulated: radial sin", "Trash")
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
            a = None
            break
    runtime.stop()
    assert runtime.get_available_data(0) is None
    # TODO: (nclack) assert 1 acquired frame. stop should block
    runtime.start()
    while True:
        if a := runtime.get_available_data(0):
            logging.info(f"Got {a.get_frame_count()}")
            a = None
            break
    runtime.stop()
    assert runtime.get_available_data(0) is None
    # TODO: (nclack) assert 1 more acquired frame. stop cancels and waits.


def test_repeat_with_no_stop(runtime: Runtime):
    """Stop is required between starts. This tests that an exception is
    raised."""
    p = acquire.setup(runtime, "simulated: radial sin", "Trash")
    assert p.video[0].camera.identifier is not None
    assert p.video[0].storage.identifier is not None
    p.video[0].camera.settings.shape = (192, 108)
    p.video[0].camera.settings.exposure_time_us = 1e4
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
    assert p.video[0].storage.identifier.kind == acquire.DeviceKind.NONE
    p.video[0].storage.identifier = dm.select(
        acquire.DeviceKind.Storage, "Tiff"
    )
    assert p.video[0].storage.identifier is not None

    p.video[0].storage.settings.filename = "out.tif"
    assert p.video[0].storage.settings.filename == "out.tif"


def test_setup(runtime: Runtime):
    p = acquire.setup(runtime, "simulated.*empty", "Trash")
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
    p.video[0].storage.settings.pixel_scale_um = (0.5, 4)
    p.video[0].storage.settings.chunk_dims_px.width = 33
    p.video[0].storage.settings.chunk_dims_px.height = 47
    p.video[0].storage.settings.chunk_dims_px.planes = 4

    p = runtime.set_configuration(p)

    nframes = 0
    runtime.start()
    while nframes < p.video[0].max_frame_count:
        if packet := runtime.get_available_data(0):
            nframes += packet.get_frame_count()
            packet = None
    runtime.stop()

    assert p.video[0].storage.settings.filename
    store = parse_url(p.video[0].storage.settings.filename)
    assert store
    reader = Reader(store)
    nodes = list(reader())

    # ome-ngff supports multiple images, in separate directories but we only
    # wrote one.
    multi_scale_image_node = nodes[0]

    # ome-ngff always stores multi-scale images, but we only have a single
    # scale/level.
    image_data = multi_scale_image_node.data[0]
    assert image_data.shape == (
        p.video[0].max_frame_count,
        1,
        p.video[0].camera.settings.shape[1],
        p.video[0].camera.settings.shape[0],
    )

    multi_scale_image_metadata = multi_scale_image_node.metadata

    axes = multi_scale_image_metadata["axes"]
    axis_names = tuple(a["name"] for a in axes)
    assert axis_names == ("t", "c", "y", "x")

    axis_types = tuple(a["type"] for a in axes)
    assert axis_types == ("time", "channel", "space", "space")

    axis_units = tuple(a.get("unit") for a in axes)
    assert axis_units == (None, None, "micrometer", "micrometer")

    # We only have one multi-scale level and one transform.
    transform = multi_scale_image_metadata["coordinateTransformations"][0][0]
    pixel_scale_um = tuple(
        transform["scale"][axis_names.index(axis)] for axis in ("x", "y")
    )
    assert pixel_scale_um == p.video[0].storage.settings.pixel_scale_um

    # ome-zarr only reads attributes it recognizes, so use a plain zarr reader
    # to read external metadata instead.
    group = zarr.open(p.video[0].storage.settings.filename)
    assert group["0"].attrs.asdict() == metadata


@pytest.mark.parametrize(
    ("compressor_name",),
    [
        ("zstd",),
        ("lz4",),
    ],
)
def test_write_compressed_zarr(
    runtime: Runtime, request: pytest.FixtureRequest, compressor_name
):
    filename = f"{request.node.name}.zarr"
    filename = filename.replace("[", "_").replace("]", "_")

    dm = runtime.device_manager()
    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*empty.*"
    )
    p.video[0].camera.settings.shape = (64, 48)
    p.video[0].camera.settings.exposure_time_us = 1e4
    p.video[0].storage.identifier = dm.select(
        DeviceKind.Storage,
        f"ZarrBlosc1{compressor_name.capitalize()}ByteShuffle",
    )
    p.video[0].max_frame_count = 70
    p.video[0].storage.settings.filename = filename
    metadata = {"foo": "bar"}
    p.video[0].storage.settings.external_metadata_json = json.dumps(metadata)
    runtime.set_configuration(p)

    runtime.start()
    runtime.stop()

    # load from Zarr
    group = zarr.open(p.video[0].storage.settings.filename)
    data = group["0"]

    assert data.compressor.cname == compressor_name
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
    data = da.from_zarr(p.video[0].storage.settings.filename, component="0")
    assert data.shape == (
        p.video[0].max_frame_count,
        1,
        p.video[0].camera.settings.shape[1],
        p.video[0].camera.settings.shape[0],
    )


@pytest.mark.parametrize(
    ("number_of_frames", "expected_number_of_chunks", "compression"),
    [
        (64, 4, None),
        (64, 4, {"codec": "zstd", "clevel": 1, "shuffle": 1}),
        (65, 8, None),  # rollover
        (65, 8, {"codec": "blosclz", "clevel": 2, "shuffle": 2}),  # rollover
    ],
)
def test_write_zarr_with_chunking(
    runtime: acquire.Runtime,
    request: pytest.FixtureRequest,
    number_of_frames: int,
    expected_number_of_chunks: int,
    compression: Optional[dict],
):
    dm = runtime.device_manager()

    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*empty.*"
    )
    p.video[0].camera.settings.shape = (1920, 1080)
    p.video[0].camera.settings.exposure_time_us = 1e4
    p.video[0].camera.settings.pixel_type = acquire.SampleType.U8
    p.video[0].storage.identifier = dm.select(
        DeviceKind.Storage,
        "Zarr",
    )
    p.video[0].storage.settings.filename = f"{request.node.name}.zarr"
    p.video[0].max_frame_count = number_of_frames

    p.video[0].storage.settings.chunk_dims_px.width = 1920 // 2
    p.video[0].storage.settings.chunk_dims_px.height = 1080 // 2
    p.video[0].storage.settings.chunk_dims_px.planes = 64

    runtime.set_configuration(p)

    runtime.start()
    runtime.stop()

    group = zarr.open(p.video[0].storage.settings.filename)
    data = group["0"]

    assert data.chunks == (64, 1, 1080 // 2, 1920 // 2)

    assert data.shape == (
        number_of_frames,
        1,
        p.video[0].camera.settings.shape[1],
        p.video[0].camera.settings.shape[0],
    )
    assert data.nchunks == expected_number_of_chunks


def test_write_zarr_multiscale(
    runtime: acquire.Runtime,
    request: pytest.FixtureRequest,
):
    filename = f"{request.node.name}.zarr"
    filename = filename.replace("[", "_").replace("]", "_")

    dm = runtime.device_manager()

    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*empty.*"
    )
    p.video[0].camera.settings.shape = (1920, 1080)
    p.video[0].camera.settings.exposure_time_us = 1e4
    p.video[0].camera.settings.pixel_type = acquire.SampleType.U8
    p.video[0].storage.identifier = dm.select(
        DeviceKind.Storage,
        "Zarr",
    )
    p.video[0].storage.settings.filename = filename
    p.video[0].storage.settings.pixel_scale_um = (1, 1)
    p.video[0].max_frame_count = 100

    p.video[0].storage.settings.chunk_dims_px.width = (
        p.video[0].camera.settings.shape[0] // 3
    )
    p.video[0].storage.settings.chunk_dims_px.height = (
        p.video[0].camera.settings.shape[1] // 3
    )
    p.video[0].storage.settings.chunk_dims_px.planes = 64

    p.video[0].storage.settings.enable_multiscale = True

    runtime.set_configuration(p)

    runtime.start()
    runtime.stop()

    reader = Reader(parse_url(filename))
    zgroup = list(reader())[0]
    # loads each layer as a dask array from the Zarr dataset
    data = [
        da.from_zarr(filename, component=str(i))
        for i in range(len(zgroup.data))
    ]
    assert len(data) == 3

    image = data[0][0, 0, :, :].compute()  # convert dask array to numpy array

    for d in data:
        assert (
            np.linalg.norm(image - d[0, 0, :, :].compute()) == 0
        )  # validate against the same method from scikit-image
        image = downscale_local_mean(image, (2, 2)).astype(np.uint8)


@pytest.mark.parametrize(
    ("number_of_frames", "expected_number_of_chunks", "codec"),
    [
        (64, 4, None),
        (64, 4, "zstd"),
        (65, 8, None),  # rollover
        (65, 8, "lz4"),  # rollover
    ],
)
def test_write_zarr_v3(
    runtime: acquire.Runtime,
    request: pytest.FixtureRequest,
    number_of_frames: int,
    expected_number_of_chunks: int,
    codec: Optional[str],
):
    dm = runtime.device_manager()

    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*empty.*"
    )

    p.video[0].camera.settings.shape = (1920, 1080)
    p.video[0].camera.settings.exposure_time_us = 1e4
    p.video[0].camera.settings.pixel_type = acquire.SampleType.U8
    p.video[0].storage.identifier = dm.select(
        DeviceKind.Storage,
        f"ZarrV3Blosc1{codec.capitalize()}ByteShuffle" if codec else "ZarrV3",
    )
    p.video[0].storage.settings.filename = f"{request.node.name}.zarr"
    p.video[0].max_frame_count = number_of_frames

    p.video[0].storage.settings.chunk_dims_px.width = 1920 // 2
    p.video[0].storage.settings.chunk_dims_px.height = 1080 // 2
    p.video[0].storage.settings.chunk_dims_px.planes = 64

    runtime.set_configuration(p)

    runtime.start()
    runtime.stop()

    store = zarr.DirectoryStoreV3(p.video[0].storage.settings.filename)
    group = zarr.open(store=store, mode="r")
    data = group["0"]

    assert data.chunks == (64, 1, 1080 // 2, 1920 // 2)

    assert data.shape == (
        number_of_frames,
        1,
        p.video[0].camera.settings.shape[1],
        p.video[0].camera.settings.shape[0],
    )
    assert data.nchunks == expected_number_of_chunks


@pytest.mark.skip(
    reason="Runs into memory limitations on github ci."
    + " See https://github.com/acquire-project/cpx/issues/147"
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
    p.video[0].camera.settings.pixel_type = acquire.SampleType.U8
    p.video[0].max_frame_count = 90
    p.video[0].frame_average_count = 0  # disables

    p.video[1].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*empty.*"
    )
    p.video[1].storage.identifier = dm.select(DeviceKind.Storage, "Trash")
    p.video[1].camera.settings.binning = 1
    p.video[1].camera.settings.shape = (64, 64)
    p.video[1].camera.settings.pixel_type = acquire.SampleType.U8
    p.video[1].max_frame_count = 71
    p.video[1].frame_average_count = 0  # disables

    p = runtime.set_configuration(p)

    nframes = [0, 0]

    def is_not_done() -> bool:
        return runtime.get_state() == DeviceState.Running and (
            (nframes[0] < p.video[0].max_frame_count)
            or (nframes[1] < p.video[1].max_frame_count)
        )

    runtime.start()

    stream_id = 0
    while is_not_done():
        if nframes[stream_id] < p.video[stream_id].max_frame_count:
            if packet := runtime.get_available_data(stream_id):
                n = packet.get_frame_count()
                for i, frame in enumerate(packet.frames()):
                    expected_frame_id = nframes[stream_id] + i
                    assert frame.metadata().frame_id == expected_frame_id, (
                        "frame id's didn't match "
                        + f"({frame.metadata().frame_id}!={expected_frame_id})"
                        + f" [stream {stream_id} nframes {nframes}]"
                    )
                    del frame
                del packet
                nframes[stream_id] += n
                logging.debug(f"NFRAMES {nframes}")

        stream_id = (stream_id + 1) % 2
    logging.info("Stopping")
    runtime.stop()
    assert nframes[0] == p.video[0].max_frame_count
    assert nframes[1] == p.video[1].max_frame_count


def test_abort(runtime: Runtime):
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

    logging.debug(
        f"Frames expected: {p.video[0].max_frame_count}, actual: {nframes}"
    )
    assert nframes < p.video[0].max_frame_count


def wait_for_data(
    runtime: Runtime, stream_id: int = 0, timeout: Optional[timedelta] = None
) -> acquire.AvailableData:
    # None is used as a missing sentinel value, not to indicate no timeout.
    if timeout is None:
        timeout = timedelta(seconds=5)
    sleep_duration = timedelta(microseconds=10000)
    elapsed = timedelta()
    while elapsed < timeout:
        if packet := runtime.get_available_data(stream_id):
            return packet
        sleep(sleep_duration.total_seconds())
        elapsed += sleep_duration
    raise RuntimeError(
        f"Timed out waiting for condition after {elapsed.total_seconds()}"
        " seconds."
    )


def test_execute_trigger(runtime: Runtime):
    dm = runtime.device_manager()
    p = runtime.get_configuration()

    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "simulated.*empty.*"
    )
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Trash")
    p.video[0].camera.settings.binning = 1
    p.video[0].camera.settings.shape = (64, 48)
    p.video[0].camera.settings.exposure_time_us = 1e3
    p.video[0].camera.settings.pixel_type = acquire.SampleType.U8
    p.video[0].camera.settings.input_triggers.frame_start = Trigger(
        enable=True, line=0, edge="Rising"
    )
    p.video[0].max_frame_count = 10

    p = runtime.set_configuration(p)

    assert p.video[0].camera.settings.input_triggers.frame_start.enable

    runtime.start()

    # No triggers yet, so no data.
    assert runtime.get_available_data(0) is None

    # Snap a few individual frames
    for i in range(p.video[0].max_frame_count):
        runtime.execute_trigger(0)
        packet = wait_for_data(runtime, 0)
        frames = tuple(packet.frames())
        assert packet.get_frame_count() == 1
        assert frames[0].metadata().frame_id == i
        del frames
        del packet

    runtime.stop()


@pytest.mark.parametrize(
    ("descriptor",),
    [
        ("simulated.*empty",),
        ("simulated.*random",),
        ("simulated.*sin",),
    ],
)
def test_simulated_camera_capabilities(
    runtime: Runtime,
    descriptor: str,
):
    dm = runtime.device_manager()
    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(DeviceKind.Camera, descriptor)
    # to ensure consistent offset.{x,y}.high values across testing scenarios
    p.video[0].camera.settings.shape = (1, 1)
    runtime.set_configuration(p)

    c = runtime.get_capabilities()
    camera = c.video[0].camera
    assert camera.shape.x.writable is True
    assert camera.shape.x.low == 1.0
    assert camera.shape.x.high == 8192.0
    assert camera.shape.x.kind == PropertyType.FixedPrecision

    assert camera.shape.y.writable is True
    assert camera.shape.y.low == 1.0
    assert camera.shape.y.high == 8192.0
    assert camera.shape.y.kind == PropertyType.FixedPrecision

    assert camera.offset.x.writable is True
    assert camera.offset.x.low == 0.0
    assert camera.offset.x.high == 8190.0
    assert camera.offset.x.kind == PropertyType.FixedPrecision

    assert camera.offset.y.writable is True
    assert camera.offset.y.low == 0.0
    assert camera.offset.y.high == 8190.0
    assert camera.offset.y.kind == PropertyType.FixedPrecision

    assert camera.binning.writable is True
    assert camera.binning.low == 1.0
    assert camera.binning.high == 8.0
    assert camera.binning.kind == PropertyType.FixedPrecision

    assert camera.exposure_time_us.writable is True
    assert camera.exposure_time_us.low == 0.0
    assert camera.exposure_time_us.high == 1e6
    assert camera.exposure_time_us.kind == PropertyType.FixedPrecision

    assert camera.line_interval_us.writable is False
    assert camera.line_interval_us.low == camera.line_interval_us.high == 0.0
    assert camera.line_interval_us.kind == PropertyType.FixedPrecision

    assert camera.readout_direction.writable is False
    assert camera.readout_direction.low == camera.readout_direction.high == 0.0
    assert camera.readout_direction.kind == PropertyType.FixedPrecision

    assert len(camera.supported_pixel_types) == 5
    assert acquire.SampleType.U8 in camera.supported_pixel_types
    assert acquire.SampleType.U16 in camera.supported_pixel_types
    assert acquire.SampleType.I8 in camera.supported_pixel_types
    assert acquire.SampleType.I16 in camera.supported_pixel_types
    assert acquire.SampleType.F32 in camera.supported_pixel_types

    assert camera.digital_lines.line_count == 1
    assert camera.digital_lines.names[0] == "software"
    assert camera.digital_lines.names[1:] == [""] * 7

    assert camera.triggers.acquisition_start == (0, 0)
    assert camera.triggers.exposure == (0, 0)
    assert camera.triggers.frame_start == (1, 0)


@pytest.mark.parametrize(
    ("descriptor", "chunking", "sharding", "multiscale"),
    [
        ("raw", None, None, False),
        ("trash", None, None, False),
        ("tiff", None, None, False),
        ("tiff-json", None, None, False),
        (
            "zarr",
            {
                "width": {"low": 32, "high": 65535},
                "height": {"low": 32, "high": 65535},
                "planes": {"low": 32, "high": 65535},
            },
            None,
            True,
        ),
    ],
)
def test_storage_capabilities(
    runtime: Runtime,
    request: pytest.FixtureRequest,
    descriptor: str,
    chunking: Optional[Dict[str, Any]],
    sharding: Optional[Dict[str, Any]],
    multiscale: bool,
):
    dm = runtime.device_manager()
    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(DeviceKind.Camera, ".*empty")
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, descriptor)

    # FIXME (aliddell): hack to get the storage capabilities to be populated
    p.video[0].storage.settings.filename = f"{request.node.name}.out"

    p.video[0].storage.settings.external_metadata_json = json.dumps(
        {"hello": "world"}
    )  # for tiff-json
    p.video[0].max_frame_count = 1000
    runtime.set_configuration(p)

    # FIXME (aliddell): hack to get the storage capabilities to be populated
    runtime.start()
    c = runtime.get_capabilities()
    # FIXME (aliddell): hack to get the storage capabilities to be populated
    runtime.abort()
    storage = c.video[0].storage

    chunk_dims_px = storage.chunk_dims_px

    assert chunk_dims_px.width.kind == PropertyType.FixedPrecision
    assert chunk_dims_px.height.kind == PropertyType.FixedPrecision
    assert chunk_dims_px.planes.kind == PropertyType.FixedPrecision

    if chunking is None:
        assert chunk_dims_px.is_supported is False
        assert chunk_dims_px.width.low == chunk_dims_px.width.high == 0.0
        assert chunk_dims_px.height.low == chunk_dims_px.height.high == 0.0
        assert chunk_dims_px.planes.low == chunk_dims_px.planes.high == 0.0
    else:
        assert chunk_dims_px.is_supported is True
        assert chunk_dims_px.width.low == chunking["width"]["low"]
        assert chunk_dims_px.width.high == chunking["width"]["high"]
        assert chunk_dims_px.height.low == chunking["height"]["low"]
        assert chunk_dims_px.height.high == chunking["height"]["high"]
        assert chunk_dims_px.planes.low == chunking["planes"]["low"]
        assert chunk_dims_px.planes.high == chunking["planes"]["high"]

    shard_dims_chunks = storage.shard_dims_chunks

    assert shard_dims_chunks.width.kind == PropertyType.FixedPrecision
    assert shard_dims_chunks.height.kind == PropertyType.FixedPrecision
    assert shard_dims_chunks.planes.kind == PropertyType.FixedPrecision

    if sharding is None:
        assert shard_dims_chunks.is_supported is False
        assert shard_dims_chunks.width.low == 0.0
        assert shard_dims_chunks.width.high == 0.0

        assert shard_dims_chunks.height.low == 0.0
        assert shard_dims_chunks.height.high == 0.0

        assert shard_dims_chunks.planes.low == 0.0
        assert shard_dims_chunks.planes.high == 0.0
    else:
        assert shard_dims_chunks.is_supported is True
        assert shard_dims_chunks.width.low == chunking["width"]["low"]
        assert shard_dims_chunks.width.high == chunking["width"]["high"]
        assert shard_dims_chunks.height.low == chunking["height"]["low"]
        assert shard_dims_chunks.height.high == chunking["height"]["high"]
        assert shard_dims_chunks.planes.low == chunking["planes"]["low"]
        assert shard_dims_chunks.planes.high == chunking["planes"]["high"]

    assert storage.multiscale.is_supported == multiscale


# FIXME: (nclack) awkwardness around references  (available frames, f)

# NOTES:
#
# With pytest, use `--log-cli-level=0` to see the lowest level logs.
