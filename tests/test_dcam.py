import acquire
import pytest
from acquire import DeviceKind, SampleType


@pytest.fixture(scope="module")
def _runtime():
    runtime = acquire.Runtime()
    yield runtime


@pytest.fixture(scope="function")
def runtime(_runtime: acquire.Runtime):
    yield _runtime
    _runtime.set_configuration(acquire.Properties())


def test_set_exposure_output_trigger(runtime: acquire.Runtime) -> None:
    dm = runtime.device_manager()
    p = runtime.get_configuration()

    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "Hamamatsu C15440-20UP.*"
    )
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Trash")
    p.video[0].camera.settings.binning = 1
    p.video[0].camera.settings.shape = (2304, 2304)
    p.video[0].camera.settings.pixel_type = SampleType.U16
    p.video[0].max_frame_count = 10

    # line=1 corresponds with "Timing 1"
    p.video[0].camera.settings.output_triggers.exposure = acquire.Trigger(
        enable=True, line=1, edge="Rising"
    )
    # Apply configuration and read back the configuration from devices
    p = runtime.set_configuration(p)
    # Check that it was accepted
    assert p.video[0].camera.settings.output_triggers.exposure.enable is True


def test_ext_triggering(runtime: acquire.Runtime):
    dm = runtime.device_manager()
    p = runtime.get_configuration()

    p.video[0].camera.identifier = dm.select(
        DeviceKind.Camera, "Hamamatsu C15440-20UP.*"
    )
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Trash")
    p.video[0].camera.settings.binning = 1
    p.video[0].camera.settings.shape = (2304, 2304)
    p.video[0].camera.settings.pixel_type = SampleType.U16
    p.video[0].max_frame_count = 10

    # line=0 corresponds with "Ext.Trig"
    p.video[0].camera.settings.input_triggers.frame_start = acquire.Trigger(
        enable=True, line=0, edge="Rising"
    )

    # Apply configuration and read back the configuration from devices
    p = runtime.set_configuration(p)
    # Check that it was accepted
    assert p.video[0].camera.settings.input_triggers.frame_start.enable is True


def test_a_theory(runtime: acquire.Runtime):
    dm = runtime.device_manager()
    p = runtime.get_configuration()
    print(dm.devices())
    cameras = [
        d.name
        for d in dm.devices()
        if (d.kind == DeviceKind.Camera)
        and ("Hamamatsu C15440-20UP" in d.name)
    ]

    p.video[0].camera.identifier = dm.select_one_of(DeviceKind.Camera, cameras)
    runtime.set_configuration(p)

    frame_shape = [2304, 2304]

    p.video[0].camera.settings.binning = 1
    p.video[0].camera.settings.shape = (frame_shape[0], frame_shape[1])
    p.video[0].max_frame_count = 5000
    p.video[0].frame_average_count = 0  # disables
    p.video[0].camera.settings.exposure_time_us = 1e4
    p.video[0].camera.settings.line_interval_us = 1e4 / 2304
    p.video[0].camera.settings.pixel_type = SampleType.U16

    zarr_types = [
        "ZarrBlosc1Lz4ByteShuffle",
        # "Zarr",
        # "ZarrBlosc1ZstdByteShuffle",
    ]
    # chunking = [144, 288, 576]
    chunking = [144]
    multiscale = [False]

    for t in zarr_types:
        for chunk in chunking:
            for boolean in multiscale:
                frame_size = frame_shape[0] * frame_shape[1]
                p.video[0].storage.settings.chunk_dims_px.width = chunk
                p.video[0].storage.settings.chunk_dims_px.height = chunk
                p.video[0].storage.settings.chunk_dims_px.planes = chunk

                p.video[0].storage.settings.shard_dims_chunks.width = (
                    frame_size // chunk
                )
                p.video[0].storage.settings.shard_dims_chunks.height = (
                    frame_size // chunk
                )
                p.video[0].storage.settings.shard_dims_chunks.planes = 128

                p.video[0].storage.settings.enable_multiscale = boolean

                filepath = rf"{t}_chunk_{chunk}_multicscale_{boolean}.zarr"
                p.video[0].storage.settings.filename = filepath
                p.video[0].storage.identifier = dm.select(
                    DeviceKind.Storage, t
                )

                runtime.set_configuration(p)
                runtime.start()
                frames_collected = 0
                counts = 0
                while frames_collected < 5000:
                    counts += 1
                    if a := runtime.get_available_data(0):
                        packet = a.get_frame_count()
                        f = next(a.frames())
                        print(f.data().shape)
                        del f
                        del a

                        frames_collected += packet
                    print(f"frames collected: {frames_collected}")

                print("out of loop")
                runtime.abort()

    runtime.abort()
    print("Finished")
