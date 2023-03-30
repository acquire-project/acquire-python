import logging
import pprint

import calliphlox
import pytest
from calliphlox import DeviceKind, SampleType
from calliphlox.calliphlox import TriggerEdge


@pytest.fixture(scope="module")
def runtime():
    runtime = calliphlox.Runtime()
    yield runtime


def test_vieworks_camera_is_preset(runtime: calliphlox.Runtime):
    dm = runtime.device_manager()
    assert dm.select(DeviceKind.Camera, "VIEWORKS.*")


def test_vieworks_stream(
    runtime: calliphlox.Runtime, request: pytest.FixtureRequest
):
    dm = runtime.device_manager()
    p = runtime.get_configuration()

    p.video[0].camera.identifier = dm.select(DeviceKind.Camera, "VIEWORKS.*")
    assert p.video[0].camera.identifier

    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Zarr")
    assert p.video[0].storage.identifier
    p.video[0].storage.settings.filename = request.node.name + ".zarr"
    p.video[0].storage.settings.bytes_per_chunk = 1 << 30  # 1 GB chunks

    # Set the camera here so we can query it's triggering capabilities.
    # This comes in the form of the returned properties.
    p = runtime.set_configuration(p)

    p.video[0].camera.settings.binning = 1
    p.video[0].camera.settings.shape = (14192, 10640)
    p.video[0].camera.settings.pixel_type = SampleType.U12
    p.video[0].camera.settings.exposure_time_us = 1e3

    p.video[0].storage.settings.pixel_scale_um = (0.2, 0.2)
    p.video[0].max_frame_count = 10

    p = runtime.set_configuration(p)

    logging.info(pprint.pformat(p.dict()))

    runtime.start()
    runtime.stop()


def test_vieworks_configure_triggering(runtime: calliphlox.Runtime):
    dm = runtime.device_manager()
    p = runtime.get_configuration()

    p.video[0].camera.identifier = dm.select(DeviceKind.Camera, "VIEWORKS.*")
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Trash")
    assert p.video[0].camera.identifier

    p = runtime.set_configuration(p)

    # There are two defined lines: Line0, and Software
    assert len(p.video[0].camera.settings.triggers) == 2

    # When the camera is first selected, triggers should be disabled
    assert not p.video[0].camera.settings.triggers[0].enable
    assert not p.video[0].camera.settings.triggers[1].enable

    #
    # Enable Line0:
    #
    # There's really own two things to set. On the VP-151MX, there's only
    # one kind of event that can be triggered - the frame exposure start.
    p.video[0].camera.settings.triggers[0].enable = True
    p.video[0].camera.settings.triggers[0].edge = TriggerEdge.Rising

    p = runtime.set_configuration(p)
    assert p.video[0].camera.settings.triggers[0].enable
    assert not p.video[0].camera.settings.triggers[1].enable

    #
    # Enable Software triggering:
    #
    # There's really own two things to set. On the VP-151MX, there's only
    # one kind of event that can be triggered - the frame exposure start.
    p.video[0].camera.settings.triggers[1].enable = True
    p.video[0].camera.settings.triggers[1].edge = TriggerEdge.Rising

    p = runtime.set_configuration(p)
    assert not p.video[0].camera.settings.triggers[0].enable
    assert p.video[0].camera.settings.triggers[1].enable

    #
    # Disable triggering:
    #
    p.video[0].camera.settings.triggers[0].enable = False
    p.video[0].camera.settings.triggers[1].enable = False
    p = runtime.set_configuration(p)
