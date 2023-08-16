import logging
import pprint

import acquire
import pytest
from acquire import DeviceKind, SampleType
from acquire.acquire import Trigger


@pytest.fixture(scope="module")
def _runtime():
    runtime = acquire.Runtime()
    yield runtime


@pytest.fixture(scope="function")
def runtime(_runtime: acquire.Runtime):
    yield _runtime
    _runtime.set_configuration(acquire.Properties())


def test_vieworks_camera_is_preset(runtime: acquire.Runtime):
    dm = runtime.device_manager()
    assert dm.select(DeviceKind.Camera, "VIEWORKS.*")


def test_vieworks_stream(
    runtime: acquire.Runtime, request: pytest.FixtureRequest
):
    dm = runtime.device_manager()
    p = runtime.get_configuration()

    p.video[0].camera.identifier = dm.select(DeviceKind.Camera, "VIEWORKS.*")
    assert p.video[0].camera.identifier

    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Zarr")
    assert p.video[0].storage.identifier
    p.video[0].storage.settings.filename = request.node.name + ".zarr"
    p.video[0].storage.settings.chunking.max_bytes_per_chunk = (
        1 << 30
    )  # 1 GB chunks

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


def test_vieworks_configure_triggering(runtime: acquire.Runtime):
    dm = runtime.device_manager()
    p = runtime.get_configuration()

    p.video[0].camera.identifier = dm.select(DeviceKind.Camera, "VIEWORKS.*")
    p.video[0].storage.identifier = dm.select(DeviceKind.Storage, "Trash")
    assert p.video[0].camera.identifier

    p = runtime.set_configuration(p)

    # When the camera is first selected, triggers should be disabled
    assert not p.video[0].camera.settings.input_triggers.frame_start.enable

    #
    # Enable Line0:
    #
    # There's really own two things to set. On the VP-151MX, there's only
    # one kind of event that can be triggered - the frame exposure start.
    p.video[0].camera.settings.input_triggers.frame_start = Trigger(
        enable=True, line=0, edge="Rising"
    )
    assert p.video[0].camera.settings.input_triggers.frame_start.enable

    p = runtime.set_configuration(p)
    assert p.video[0].camera.settings.input_triggers.frame_start.enable
    assert p.video[0].camera.settings.input_triggers.frame_start.line == 0

    #
    # Enable Software triggering:
    #
    # There's really own two things to set. On the VP-151MX, there's only
    # one kind of event that can be triggered - the frame exposure start.
    p.video[0].camera.settings.input_triggers.frame_start = Trigger(
        enable=True, line=1, edge="Rising"
    )

    p = runtime.set_configuration(p)

    assert p.video[0].camera.settings.input_triggers.frame_start.enable
    assert p.video[0].camera.settings.input_triggers.frame_start.line == 1

    #
    # Disable triggering:
    #
    p.video[0].camera.settings.input_triggers.frame_start.enable = False
    p = runtime.set_configuration(p)
