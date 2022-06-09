from typing import Optional
from .calliphlox import *

__doc__ = calliphlox.__doc__
if hasattr(calliphlox, "__all__"):
    __all__ = calliphlox.__all__

import logging
FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.DEBUG)

runtime=calliphlox.Runtime()

def dbg(v):
    logging.debug(v)
    return v


def setup(camera: str, storage: str, output_filename: Optional[str]) -> Properties:
    dm=runtime.device_manager()
    p=runtime.get_configuration()
    p.camera.identifier=dbg(dm.select(DeviceKind.Camera, camera))
    dbg(p.camera.identifier)
    p.storage.identifier=dbg(dm.select(DeviceKind.Storage, storage))
    return p    