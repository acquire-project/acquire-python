from typing import Any, ClassVar, Dict, List, Optional, Tuple, final, overload

@final
class DeviceKind:
    Camera: ClassVar[DeviceKind] = ...
    NONE: ClassVar[DeviceKind] = ...
    Signals: ClassVar[DeviceKind] = ...
    StageAxis: ClassVar[DeviceKind] = ...
    Storage: ClassVar[DeviceKind] = ...

@final
class DeviceState:
    Closed: ClassVar[DeviceState] = ...
    AwaitingConfiguration: ClassVar[DeviceState] = ...
    Armed: ClassVar[DeviceState] = ...
    Running: ClassVar[DeviceState] = ...

@final
class Direction:
    Backward: ClassVar[Direction] = ...
    Forward: ClassVar[Direction] = ...

@final
class SampleType:
    F32: ClassVar[SampleType] = ...
    I16: ClassVar[SampleType] = ...
    I8: ClassVar[SampleType] = ...
    U16: ClassVar[SampleType] = ...
    U8: ClassVar[SampleType] = ...

@final
class SignalIOKind:
    Input: ClassVar[SignalIOKind] = ...
    Output: ClassVar[SignalIOKind] = ...

@final
class SignalType:
    Analog: ClassVar[SignalType] = ...
    Digital: ClassVar[SignalType] = ...

@final
class TriggerEdge:
    Falling: ClassVar[TriggerEdge] = ...
    NotApplicable: ClassVar[TriggerEdge] = ...
    Rising: ClassVar[TriggerEdge] = ...

@final
class TriggerEvent:
    AcquisitionStart: ClassVar[TriggerEvent] = ...
    Exposure: ClassVar[TriggerEvent] = ...
    FrameStart: ClassVar[TriggerEvent] = ...
    FrameTriggerWait: ClassVar[TriggerEvent] = ...
    Unknown: ClassVar[TriggerEvent] = ...

@final
class DeviceIdentifier:
    id: Tuple[int, int]
    kind: DeviceKind
    name: str
    def dict(self) -> Dict[str, Any]: ...
    @staticmethod
    def none() -> DeviceIdentifier: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

@final
class DeviceManager:
    def devices(self) -> List[DeviceIdentifier]: ...
    def select(
        self, kind: DeviceKind, name: Optional[str]
    ) -> Optional[DeviceIdentifier]: ...
    def select_one_of(
        self, kind: DeviceKind, names: List[str]
    ) -> Optional[DeviceIdentifier]: ...

@final
class Trigger:
    enable: bool
    line: int
    event: TriggerEvent
    kind: SignalIOKind
    edge: TriggerEdge
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
    triggers: List[Trigger]
    def dict(self) -> Dict[str, Any]: ...

@final
class Camera:
    identifier: Optional[DeviceIdentifier]
    settings: CameraProperties
    def dict(self) -> Dict[str, Any]: ...

@final
class StorageProperties:
    filename: None | str
    first_frame_id: int
    def dict(self) -> Dict[str, Any]: ...

@final
class Storage:
    identifier: Optional[DeviceIdentifier]
    settings: StorageProperties
    def dict(self) -> Dict[str, Any]: ...

@final
class PID:
    derivative: float
    integral: float
    proportional: float
    def dict(self) -> Dict[str, Any]: ...

@final
class StageAxisState:
    position: float
    velocity: float
    def dict(self) -> Dict[str, Any]: ...

@final
class StageAxisProperties:
    feedback: PID
    immediate: StageAxisState
    target: StageAxisState
    def dict(self) -> Dict[str, Any]: ...

@final
class StageAxis:
    identifier: Optional[DeviceIdentifier]
    settings: StageAxisProperties
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

@final
class Channel:
    sample_type: SampleType
    signal_type: SignalType
    signal_io_kind: SignalIOKind
    voltage_range: VoltageRange
    line: int
    display_name: str
    def dict(self) -> Dict[str, Any]: ...

@final
class SampleRateHz:
    numerator: int
    denominator: int
    def dict(self) -> Dict[str, Any]: ...

@final
class Timing:
    terminal: int
    edge: TriggerEdge
    samples_per_second: SampleRateHz
    def dict(self) -> Dict[str, Any]: ...

@final
class SignalProperties:
    channels: List[Channel]
    timing: Timing
    Trigger: List[Trigger]
    def dict(self) -> Dict[str, Any]: ...

@final
class Signals:
    identifier: Optional[DeviceIdentifier]
    settings: SignalProperties
    def dict(self) -> Dict[str, Any]: ...

@final
class VideoStream:
    camera: Camera
    storage: Storage
    max_frame_count: int
    frame_average_count: int
    def dict(self) -> Dict[str, Any]: ...

@final
class Properties:
    video: Tuple[VideoStream, VideoStream]
    stages: Tuple[StageAxis, StageAxis, StageAxis]
    signals: Signals
    def dict(self) -> Dict[str, Any]: ...

@final
class Runtime:
    def start(self) -> None: ...
    def stop(self) -> None: ...
    def device_manager(self) -> DeviceManager: ...
    def set_configuration(self, properties: Properties) -> Properties: ...
    def get_configuration(self) -> Properties: ...
    def get_available_data(
        self, stream_id: int
    ) -> Any: ...  # FIXME(nclack): type AvailableData
    def get_state(self) -> DeviceState: ...

def core_api_version() -> str: ...
