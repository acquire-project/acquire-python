import time
from typing import Optional
from . import calliphlox

from .calliphlox import (
    Runtime,
    Properties,
    DeviceKind,
    SampleType,
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
from numpy import cumsum, histogram, ones, where

def gui(viewer: 'napari.Viewer', frame_count: int=1000):
    update_times=[]
    
    def update_layer(new_image):
        try:
            clock=time.time()

            layer=viewer.layers['Video']
            layer._slice.image._view = new_image
            layer.events.set_data() 
            # layer.data=new_image # public api adds another 1-2 ms/frame

            elapsed=time.time()-clock
            update_times.append(elapsed)
            logging.info(f"UPDATED LAYER in {elapsed} s")
        except KeyError:
            # (nclack) This takes ~ 60ms for 630x480 the one time I measured it
            viewer.add_image(
                new_image, name='Video'
            )
        


    @thread_worker(connect={'yielded': update_layer})
    def do_acquisition():
        runtime = get_runtime()
        p=setup(runtime,"simulated: radial sin","Tiff")
        p=setup(runtime,"C15440-20UP","Tiff")
        p.camera.settings.shape=(2304,2304)
        p.camera.settings.pixel_type=SampleType.U16
        p.max_frame_count=frame_count
        p=runtime.set_configuration(p)

        runtime.start()
        
        nframes=0
        while nframes<p.max_frame_count:
            clock=time.time()
            if a:=runtime.get_available_data():
                packet=a.get_frame_count()                
                f = next(a.frames())
                im=f.data().squeeze()

                yield im

                f=None # <-- will fail to get the last frames if this is held?
                a=None # <-- will fail to get the last frames if this is held?

                nframes+=packet            
                logging.info(f'frame count: {nframes} - frames in packet: {packet}')

            elapsed=time.time()-clock
            time.sleep(max(0,0.03-elapsed))
        logging.info("stopping")

        counts,bins=histogram(update_times)
        p50=bins[where(cumsum(counts)>=0.5*len(update_times))[0][0]]
        p90=bins[where(cumsum(counts)>=0.9*len(update_times))[0][0]]
        logging.info(f"Update times - median: {p50*1e3} ms  90%<{p90*1e3} ms")

        runtime.stop()

    do_acquisition()

# FIXME: (nclack) would rather not have this depend on napari
# FIXME: (nclack) crashes on seconds button press
# FIXME: (nclack) napari view doesn't update right on