import logging
import pytest
from calliphlox import Trigger
import calliphlox


@pytest.fixture(scope="module")
def runtime():
    runtime = calliphlox.Runtime()
    yield runtime
    del runtime


def test_set():
    t = Trigger()
    assert t.enable == False
    t.enable = True
    assert t.enable == True


def test_set_camera_identifier(caplog, runtime):
    caplog.set_level(logging.DEBUG)

    dm = runtime.device_manager()
    devices = dm.devices()

    # FIXME: (nclack) This is a terribly awkward pattern. getattr is returning
    #         a clone of the underlying attribute, not a reference, so
    #         dot-expressions don't really work as lvalues here - dots don't 
    #         compose

    p = runtime.get_configuration()
    camera = p.camera
    assert camera.identifier is None
    camera.identifier = dm.select(
        calliphlox.DeviceKind.Camera, "simulated: radial sin"
    )
    assert camera.identifier != None
    p.camera = camera
    assert camera.identifier != None


