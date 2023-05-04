import logging

import acquire
import pytest
from acquire import DeviceKind, SampleType, Trigger


@pytest.fixture(scope="module")
def runtime():
    runtime = acquire.Runtime()
    yield runtime


def test_ext_triggering(runtime: acquire.Runtime):
    dm = runtime.device_manager()
    p = runtime.get_configuration()

    cameras = [
        d.name
        for d in dm.devices()
        if (d.kind == DeviceKind.Camera) and ("C15440" in d.name)
    ]
    assert len(cameras) > 0, "No C15440 cameras found"
    logging.warning(f"Cameras {cameras}")

    p.video[0].camera.identifier = dm.select(DeviceKind.Camera, cameras[0])
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Trash")
    p.video[0].camera.settings.binning = 1
    p.video[0].camera.settings.shape = (2304, 2304)
    p.video[0].camera.settings.pixel_type = SampleType.U16
    p.video[0].max_frame_count = 1
    p.video[0].frame_average_count = 0  # disables

    # Set the camera here so we can query it's triggering capabilities.
    # This comes in the form of the returned properties.
    p = runtime.set_configuration(p)

    p.video[0].camera.settings.input_triggers.frame_start = Trigger(enable=True, line=1)
    # Call set_configuration() again to apply the trigger properties
    p = runtime.set_configuration(p)

    assert p.video[0].camera.settings.input_triggers.frame_start.enable is True
    assert p.video[0].camera.settings.input_triggers.frame_start.line == 1
