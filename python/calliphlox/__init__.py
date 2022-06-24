import time
from typing import Optional
from . import calliphlox

from .calliphlox import (
    Runtime,
    Properties,
    DeviceKind,
)  # To make PyLance happy, I seem to have to this <--
from .calliphlox import *


__doc__ = calliphlox.__doc__
if hasattr(calliphlox, "__all__"):
    __all__ = calliphlox.__all__

import logging

FORMAT = (
    "%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s"
)
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)


def dbg(v):
    logging.debug(v)
    return v


def setup(
    runtime: Runtime, camera: str = "simulated: radial sin", storage: str = "Tiff", output_filename: Optional[str] = "out.tif"
) -> Properties:

    dm = runtime.device_manager()
    p = runtime.get_configuration()

    p.camera.identifier=dm.select(DeviceKind.Camera, camera)
    p.camera.settings.binning=1
    p.camera.settings.shape=(640,480)
    p.storage.identifier=dm.select(DeviceKind.Storage, storage)
    p.storage.settings.filename=output_filename
    p.max_frame_count=100
    p.frame_average_count=0 # disables

    return p


RUNTIME = None

def get_runtime():
    global RUNTIME
    if RUNTIME is None:
        RUNTIME = calliphlox.Runtime()
    return RUNTIME

from napari.qt.threading import thread_worker;

from numpy import ones
def gui(viewer: 'napari.Viewer', frame_count: int=1000):
    # layer=viewer.add_image(ones((480,640),dtype="uint8"))
    # layer.refresh()
    # viewer.reset_view()
    # viewer.show()
    
    def update_layer(new_image):
        try:
            logging.info("UPDATE LAYER")
            layer=viewer.layers['result']
            layer._slice.image._view = new_image
            layer.events.set_data()
        except KeyError:
            viewer.add_image(
                new_image, name='result'
            )

    @thread_worker(connect={'yielded': update_layer})
    def large_random_images():
        runtime = get_runtime()
        p=setup(runtime,"simulated: radial sin","Trash")
        p.camera.settings.shape=(640,480)
        p.max_frame_count=frame_count
        p=runtime.set_configuration(p)

        runtime.start()
        
        nframes=0
        while nframes<p.max_frame_count:
            clock=time.time()
            if a:=runtime.get_available_data():
                packet=a.get_frame_count()
                f = next(a.frames())
                im=f.data().squeeze().copy()
                
                f=None # <-- will fail to get the last frames if this is held?
                a=None # <-- will fail to get the last frames if this is held?

                yield im

                nframes+=packet            
                logging.info(f'frame count: {nframes} - frames in packet: {packet}')

            elapsed=time.time()-clock
            time.sleep(max(0,0.1-elapsed))
        logging.info("stopping")

        runtime.stop()
    large_random_images()