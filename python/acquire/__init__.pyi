from typing import List, Optional, Union

import napari  # type: ignore

from .acquire import Runtime, Properties

def setup(
    runtime: Runtime,
    camera: Union[str, List[str]] = ...,
    storage: Union[str, List[str]] = ...,
    output_filename: Optional[str] = ...,
) -> Properties: ...
def setup_one_streams(runtime: Runtime, frame_count: int) -> Properties: ...
def setup_two_streams(runtime: Runtime, frame_count: int) -> Properties: ...

g_runtime: Optional[Runtime]

def gui(
    viewer: "napari.Viewer", frame_count: int = ..., stream_count: int = ...
) -> None: ...
