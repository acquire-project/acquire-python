import acquire
import pytest
from acquire import DeviceKind, SampleType


@pytest.fixture(scope="module")
def runtime():
    runtime = acquire.Runtime()
    yield runtime


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
