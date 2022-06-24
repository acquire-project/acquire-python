import logging
from time import sleep
from pprint import pprint
import time
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

def test_list_devices(caplog,runtime):    
    caplog.set_level(logging.DEBUG)
    dm = runtime.device_manager()
    for d in dm.devices():
        print(d.dict())

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
    p=calliphlox.setup(runtime,"simulated: radial sin","Trash")
    assert p.camera.identifier!=None
    assert p.storage.identifier!=None
    assert p.storage.settings.filename == "out.tif"
    p.camera.settings.shape=(1920,1080)
    p=runtime.set_configuration(p)
    pprint(p.dict())
    runtime.start()
    
    nframes=0
    while nframes<p.max_frame_count:
        clock=time.time()
        if a:=runtime.get_available_data():
            packet=a.get_frame_count()
            for f in a.frames():
                logging.info(f"{f.data().shape} {f.data()[0][0][0][0]} {f.metadata()}")
                f=None # <-- will fail to get the last frames if this is held?
            a=None # <-- will fail to get the last frames if this is held?
            nframes+=packet            
            logging.info(f'frame count: {nframes} - frames in packet: {packet}')

        elapsed=time.time()-clock
        sleep(max(0,0.1-elapsed))
    logging.info("stopping")

    runtime.stop()

# FIXME: (nclack) awkwardness around references  (available frames, f)
