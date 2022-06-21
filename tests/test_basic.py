import logging
from time import sleep
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

    p = runtime.get_configuration()
    assert p.camera.identifier.kind==calliphlox.DeviceKind.NONE
    p.camera.identifier = dm.select(
        calliphlox.DeviceKind.Camera, "simulated: radial sin"
    )
    assert p.camera.identifier != None


def test_set_storage(caplog, runtime):
    caplog.set_level(logging.DEBUG)

    dm = runtime.device_manager()
    devices = dm.devices()

    p = runtime.get_configuration()
    assert p.storage.identifier.kind == calliphlox.DeviceKind.NONE
    p.storage.identifier = dm.select(
        calliphlox.DeviceKind.Storage, "Tiff"
    )
    assert p.storage.identifier != None

    assert p.storage.settings.filename is None
    p.storage.settings.filename="out.tif"
    assert p.storage.settings.filename == "out.tif"

def test_setup(caplog,runtime):
    caplog.set_level(logging.DEBUG)
    p=calliphlox.setup(runtime,"simulated: radial sin","Tiff")
    assert p.camera.identifier!=None
    assert p.storage.identifier!=None
    assert p.storage.settings.filename == "out.tif"
    p=runtime.set_configuration(p)
    from pprint import pprint
    pprint(p.dict())
    runtime.start()
    sleep(1)
    runtime.stop()