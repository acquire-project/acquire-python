from typing import (
    Any,
    ClassVar,
    Dict,
    Iterator,
    List,
    Optional,
    Tuple,
    final,
    overload,
)

from numpy.typing import NDArray

@final
class AvailableData:
    def frames(self) -> Iterator[VideoFrame]: ...
    def get_frame_count(self) -> int: ...
    def __iter__(self) -> Iterator[VideoFrame]: ...

@final
class Camera:
    identifier: Optional[DeviceIdentifier]
    settings: CameraProperties
    def __init__(self, *args: None, **kwargs: Any) -> None: ...
    def dict(self) -> Dict[str, Any]: ...

@final
class CameraProperties:
    exposure_time_us: float
    line_interval_us: float
    binning: float
    pixel_type: SampleType
    readout_direction: Direction
    offset: Tuple[int, int]
    shape: Tuple[int, int]
    input_triggers: InputTriggers
    output_triggers: OutputTriggers
    def __init__(self, *args: None, **kwargs: Any) -> None: ...
    def dict(self) -> Dict[str, Any]: ...

@final
class DeviceIdentifier:
    id: Tuple[int, int]
    kind: DeviceKind
    name: str
    def __init__(self, *args: None, **kwargs: Any) -> None: ...
    def dict(self) -> Dict[str, Any]: ...
    @staticmethod
    def none() -> DeviceIdentifier: ...
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: object) -> bool: ...
    def __gt__(self, other: object) -> bool: ...
    def __le__(self, other: object) -> bool: ...
    def __lt__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

@final
class DeviceKind:
    Camera: ClassVar[DeviceKind] = DeviceKind.Camera
    NONE: ClassVar[DeviceKind] = DeviceKind.NONE
    Signals: ClassVar[DeviceKind] = DeviceKind.Signals
    StageAxis: ClassVar[DeviceKind] = DeviceKind.StageAxis
    Storage: ClassVar[DeviceKind] = DeviceKind.Storage
    def __init__(self, *args: None, **kwargs: Any) -> None: ...
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: object) -> bool: ...
    def __gt__(self, other: object) -> bool: ...
    def __int__(self) -> int: ...
    def __le__(self, other: object) -> bool: ...
    def __lt__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

@final
class DeviceManager:
    def devices(self) -> List[DeviceIdentifier]: ...
    @overload
    def select(self, kind: DeviceKind) -> Optional[DeviceIdentifier]: ...
    @overload
    def select(
        self, kind: DeviceKind, name: Optional[str]
    ) -> Optional[DeviceIdentifier]: ...
    def select_one_of(
        self, kind: DeviceKind, names: List[str]
    ) -> Optional[DeviceIdentifier]: ...

@final
class DeviceState:
    Closed: ClassVar[DeviceState] = DeviceState.Closed
    AwaitingConfiguration: ClassVar[
        DeviceState
    ] = DeviceState.AwaitingConfiguration
    Armed: ClassVar[DeviceState] = DeviceState.Armed
    Running: ClassVar[DeviceState] = DeviceState.Running
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: object) -> bool: ...
    def __gt__(self, other: object) -> bool: ...
    def __int__(self) -> int: ...
    def __le__(self, other: object) -> bool: ...
    def __lt__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

@final
class Direction:
    Backward: ClassVar[Direction] = Direction.Backward
    Forward: ClassVar[Direction] = Direction.Forward
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: object) -> bool: ...
    def __gt__(self, other: object) -> bool: ...
    def __int__(self) -> int: ...
    def __le__(self, other: object) -> bool: ...
    def __lt__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

@final
class InputTriggers:
    acquisition_start: Trigger
    exposure: Trigger
    frame_start: Trigger
    def dict(self) -> Dict[str, Any]: ...

@final
class OutputTriggers:
    exposure: Trigger
    frame_start: Trigger
    trigger_wait: Trigger
    def dict(self) -> Dict[str, Any]: ...

@final
class PID:
    derivative: float
    integral: float
    proportional: float
    def __init__(self, *args: None, **kwargs: Any) -> None: ...
    def dict(self) -> Dict[str, Any]: ...

@final
class Properties:
    video: Tuple[VideoStream, VideoStream]
    def __init__(self, *args: None, **kwargs: Any) -> None: ...
    def dict(self) -> Dict[str, Any]: ...

@final
class Runtime:
    def __init__(self, *args: None, **kwargs: Any) -> None: ...
    def device_manager(self) -> DeviceManager: ...
    def get_available_data(self, stream_id: int) -> AvailableData: ...
    def get_configuration(self) -> Properties: ...
    def get_state(self) -> DeviceState: ...
    def set_configuration(self, properties: Properties) -> Properties: ...
    def start(self) -> None: ...
    def stop(self) -> None: ...
    def abort(self) -> None: ...

@final
class SampleRateHz:
    numerator: int
    denominator: int
    def __init__(self, *args: None, **kwargs: Any) -> None: ...
    def dict(self) -> Dict[str, Any]: ...

@final
class SampleType:
    F32: ClassVar[SampleType] = SampleType.F32
    I16: ClassVar[SampleType] = SampleType.I16
    I8: ClassVar[SampleType] = SampleType.I8
    U16: ClassVar[SampleType] = SampleType.U16
    U8: ClassVar[SampleType] = SampleType.U8
    U10: ClassVar[SampleType] = SampleType.U10
    U12: ClassVar[SampleType] = SampleType.U12
    U14: ClassVar[SampleType] = SampleType.U14
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: object) -> bool: ...
    def __gt__(self, other: object) -> bool: ...
    def __int__(self) -> int: ...
    def __le__(self, other: object) -> bool: ...
    def __lt__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

@final
class SignalIOKind:
    Input: ClassVar[SignalIOKind] = SignalIOKind.Input
    Output: ClassVar[SignalIOKind] = SignalIOKind.Output
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: object) -> bool: ...
    def __gt__(self, other: object) -> bool: ...
    def __int__(self) -> int: ...
    def __le__(self, other: object) -> bool: ...
    def __lt__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

@final
class SignalType:
    Analog: ClassVar[SignalType] = SignalType.Analog
    Digital: ClassVar[SignalType] = SignalType.Digital
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: object) -> bool: ...
    def __gt__(self, other: object) -> bool: ...
    def __int__(self) -> int: ...
    def __le__(self, other: object) -> bool: ...
    def __lt__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

@final
class Storage:
    identifier: Optional[DeviceIdentifier]
    settings: StorageProperties
    def dict(self) -> Dict[str, Any]: ...

@final
class TileShape:
    width: int
    height: int
    planes: int
    def dict(self) -> Dict[str, Any]: ...

@final
class ChunkingProperties:
    max_bytes_per_chunk: int
    tile: TileShape
    def dict(self) -> Dict[str, Any]: ...

@final
class StorageProperties:
    external_metadata_json: Optional[str]
    filename: Optional[str]
    first_frame_id: int
    pixel_scale_um: Tuple[float, float]
    chunking: ChunkingProperties
    enable_multiscale: bool
    def dict(self) -> Dict[str, Any]: ...

@final
class Trigger:
    edge: TriggerEdge
    enable: bool
    line: int
    kind: SignalIOKind
    def __init__(self, *args: None, **kwargs: Any) -> None: ...
    def dict(self) -> Dict[str, Any]: ...

@final
class TriggerEdge:
    Falling: ClassVar[TriggerEdge] = TriggerEdge.Falling
    NotApplicable: ClassVar[TriggerEdge] = TriggerEdge.NotApplicable
    Rising: ClassVar[TriggerEdge] = TriggerEdge.Rising
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: object) -> bool: ...
    def __gt__(self, other: object) -> bool: ...
    def __int__(self) -> int: ...
    def __le__(self, other: object) -> bool: ...
    def __lt__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

@final
class VideoFrame:
    def data(self) -> NDArray[Any]: ...
    def metadata(self) -> VideoFrameMetadata: ...

@final
class VideoFrameMetadata:
    frame_id: int
    timestamps: VideoFrameTimestamps
    def dict(self) -> Dict[str, Any]: ...

@final
class VideoFrameTimestamps:
    hardware: int
    acq_thread: int
    def dict(self) -> Dict[str, Any]: ...

@final
class VideoStream:
    camera: Camera
    storage: Storage
    max_frame_count: int
    frame_average_count: int
    def dict(self) -> Dict[str, Any]: ...

@final
class VoltageRange:
    mn: float
    mx: float
    @overload
    def __init__(self) -> None: ...
    @overload
    def __init__(self, mn: float, mx: float) -> None: ...
    def dict(self) -> Dict[str, float]: ...

def core_api_version() -> str: ...
