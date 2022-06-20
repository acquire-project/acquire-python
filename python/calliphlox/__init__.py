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
    p.storage.identifier=dm.select(DeviceKind.Storage, storage)
    p.storage.settings.filename=output_filename
    p.max_frame_count=100
    p.frame_average_count=0 # disables

    return p