import json
import logging
import os
import time
from datetime import timedelta
from time import sleep
from typing import Any, Dict, List, Optional

import acquire
from acquire import DeviceKind, DeviceState, Runtime, Trigger, PropertyType
import pytest
import tifffile


# FIXME (aliddell): this should be module scoped, but the runtime is leaky
@pytest.fixture(scope="function")
def runtime():
    yield acquire.Runtime()


def test_version():
    assert isinstance(acquire.__version__, str)
    # this will fail if pip install -e . has not been run
    # so feel free to remove this line if it's not what you want to test
    assert acquire.__version__ != "uninstalled"


def test_set():
    t = Trigger()
    assert not t.enable
    t.enable = True
    assert t.enable


def test_storage_properties_pixel_scale_defaults_to_1():
    storage = acquire.StorageProperties()
    assert storage.pixel_scale_um == (1.0, 1.0)


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
    assert (
        p.video[0].camera.identifier is not None
    ), "Expected a camera identifier"
    assert (
        p.video[0].storage.identifier is not None
    ), "Expected a storage identifier"
    assert p.video[0].storage.settings.uri == "out.tif"
    p.video[0].camera.settings.shape = (192, 108)
    p.video[0].max_frame_count = 10
    p = runtime.set_configuration(p)
    runtime.start()
    while True:
        with runtime.get_available_data(0) as a:
            logging.info(f"Got {a.get_frame_count()}")
            break
        if a:
            assert a.get_frame_count() == 0
            assert next(a.frames()) is None
    runtime.stop()
    # TODO: (nclack) assert 1 acquired frame. stop should block
    runtime.start()
    while True:
        with runtime.get_available_data(0) as a:
            logging.info(f"Got {a.get_frame_count()}")
            break

        assert a.get_frame_count() == 0
        assert next(a.frames()) is None
    runtime.stop()
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
        with runtime.get_available_data(0) as a:
            if a.get_frame_count() > 0:
                logging.info(f"Got {a.get_frame_count()} frame")
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

    p.video[0].storage.settings.uri = "out.tif"
    assert p.video[0].storage.settings.uri == "out.tif"


def test_setup(runtime: Runtime):
    p = acquire.setup(runtime, "simulated.*empty", "Trash")
    assert p.video[0].camera.identifier is not None
    assert p.video[0].storage.identifier is not None
    assert p.video[0].storage.settings.uri == "out.tif"
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
        with runtime.get_available_data(0) as a:
            packet = a.get_frame_count()
            for f in a.frames():
                logging.info(
                    f"{f.data().shape} {f.data()[0][0][0][0]} "
                    + f"{f.metadata()}"
                )
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


def test_change_uri(runtime: Runtime):
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
        p.video[0].storage.settings.uri = name
        p = runtime.set_configuration(p)
        assert p.video[0].storage.settings.uri == name

        nframes = 0
        runtime.start()
        while nframes < p.video[0].max_frame_count:
            with runtime.get_available_data(0) as packet:
                nframes += packet.get_frame_count()
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
    p.video[0].storage.settings.uri = f"{request.node.name}.tif"
    metadata = {"hello": "world"}
    p.video[0].storage.settings.external_metadata_json = json.dumps(metadata)
    runtime.set_configuration(p)

    nframes = 0
    runtime.start()
    while nframes < p.video[0].max_frame_count:
        with runtime.get_available_data(0) as packet:
            nframes += packet.get_frame_count()
    runtime.stop()

    # Check that the written tif has the expected structure
    with tifffile.TiffFile(p.video[0].storage.settings.uri) as f:

        def meta(iframe: int) -> Dict[Any, Any]:
            return json.loads(f.pages[iframe].tags["ImageDescription"].value)

        # first frame should have metadata
        assert meta(0)["metadata"] == metadata
        assert meta(0)["frame_id"] == 0

        # remaining frames should not, but should have e.g. frame id
        for i in range(1, p.video[0].max_frame_count):
            assert "metadata" not in meta(i).keys()
            assert meta(i)["frame_id"] == i


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
            with runtime.get_available_data(stream_id) as packet:
                n = packet.get_frame_count()
                for i, frame in enumerate(packet.frames()):
                    expected_frame_id = nframes[stream_id] + i
                    assert frame.metadata().frame_id == expected_frame_id, (
                        "frame id's didn't match "
                        + f"({frame.metadata().frame_id}"
                        + f"!={expected_frame_id})"
                        + f" [stream {stream_id} nframes {nframes}]"
                    )
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

    while True:
        with runtime.get_available_data(0) as packet:
            frame_count = packet.get_frame_count()
            nframes += frame_count
            if frame_count == 0:
                break

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
        with runtime.get_available_data(stream_id) as packet:
            if packet.get_frame_count() > 0:
                frames = list(packet.frames())
                return (len(frames), frames[0].metadata().frame_id)
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
    with runtime.get_available_data(0) as packet:
        assert packet.get_frame_count() == 0

    # Snap a few individual frames
    for i in range(p.video[0].max_frame_count):
        runtime.execute_trigger(0)
        count, frame_id = wait_for_data(runtime, 0)
        assert count == 1
        assert frame_id == i

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

    assert camera.triggers.acquisition_start.input == 0
    assert camera.triggers.acquisition_start.output == 0

    assert camera.triggers.exposure.input == 0
    assert camera.triggers.exposure.output == 0

    assert camera.triggers.frame_start.input == 1
    assert camera.triggers.frame_start.output == 0


@pytest.mark.parametrize(
    ("descriptor", "chunking", "sharding", "multiscale"),
    [
        ("raw", False, False, False),
        ("trash", False, False, False),
        ("tiff", False, False, False),
        ("tiff-json", False, False, False),
        ("zarr", True, False, True),
        ("zarrv3", True, True, False),
    ],
)
def test_storage_capabilities(
    runtime: Runtime,
    descriptor: str,
    chunking: bool,
    sharding: bool,
    multiscale: bool,
):
    dm = runtime.device_manager()
    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(DeviceKind.Camera, ".*empty")
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, descriptor)

    p.video[0].storage.settings.external_metadata_json = json.dumps(
        {"hello": "world"}
    )  # for tiff-json
    p.video[0].max_frame_count = 1
    runtime.set_configuration(p)

    c = runtime.get_capabilities()
    storage = c.video[0].storage

    assert storage.chunking_is_supported == chunking
    assert storage.sharding_is_supported == sharding
    assert storage.multiscale_is_supported == multiscale


def test_invalidated_frame(runtime: Runtime):
    dm = runtime.device_manager()
    p = runtime.get_configuration()
    p.video[0].camera.identifier = dm.select(DeviceKind.Camera, ".*empty")
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "trash")
    p.video[0].max_frame_count = 1
    runtime.set_configuration(p)

    frame = None
    runtime.start()
    while frame is None:
        with runtime.get_available_data(0) as packet:
            if packet.get_frame_count() > 0:
                frame = next(packet.frames())
                frame.data()
    with pytest.raises(RuntimeError):
        frame.metadata()
    with pytest.raises(RuntimeError):
        frame.data()

    runtime.stop()


def test_switch_device_identifier(
    runtime: Runtime, request: pytest.FixtureRequest
):
    p = acquire.setup(runtime, "simulated.*empty", "trash")
    assert p.video[0].storage.identifier.name == "trash"
    p = runtime.set_configuration(p)

    dm = runtime.device_manager()
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "tiff")
    p.video[0].storage.settings.uri = f"{request.node.name}.tif"
    p = runtime.set_configuration(p)
    assert p.video[0].storage.identifier.name == "tiff"

    runtime.start()
    runtime.stop()

    # will raise an exception if the file doesn't exist or is invalid
    with tifffile.TiffFile(p.video[0].storage.settings.uri):
        pass

    # cleanup
    os.remove(p.video[0].storage.settings.uri)


def test_acquire_unaligned(runtime: Runtime):
    dm = runtime.device_manager()
    props = runtime.get_configuration()
    props.video[0].camera.identifier = dm.select(
        acquire.DeviceKind.Camera, ".*empty.*"
    )

    # sizeof(VideoFrame) + 33 * 47 is not divisible by 8
    props.video[0].camera.settings.shape = (33, 47)
    props.video[0].storage.identifier = dm.select(
        acquire.DeviceKind.Storage, "trash"
    )

    props.video[0].max_frame_count = 3
    runtime.set_configuration(props)

    nframes = 0
    runtime.start()
    while nframes < props.video[0].max_frame_count:
        with runtime.get_available_data(0) as packet:
            for i in range(packet.get_frame_count()):
                _ = next(packet.frames())
                nframes += 1
    runtime.stop()
    assert nframes == props.video[0].max_frame_count


# NOTES:
#
# With pytest, use `--log-cli-level=0` to see the lowest level logs.
