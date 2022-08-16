from typing import Any, ClassVar, Dict, List, Optional, Tuple, final, overload

@final
class DeviceKind:
    Camera: ClassVar[DeviceKind] = ...
    NONE: ClassVar[DeviceKind] = ...
    Signals: ClassVar[DeviceKind] = ...
    StageAxis: ClassVar[DeviceKind] = ...
    Storage: ClassVar[DeviceKind] = ...

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

@final
class DeviceIdentifier:
    id: Tuple[int, int]
    kind: DeviceKind
    name: str
    def dict(self) -> Dict[str, Any]: ...
    def __eq__(self, other) -> bool: ...
    def __ne__(self, other) -> bool: ...

@final
class DeviceManager:
    def devices(self) -> List[DeviceIdentifier]: ...
    def select(
        self, kind: DeviceKind, name: Optional[str]
    ) -> None | DeviceIdentifier: ...

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
    gain_db: float
    exposure_time_us: float
    binning: float
    pixel_type: SampleType
    offset: Tuple[int, int]
    shape: Tuple[int, int]
    triggers: List[Trigger]
    def dict(self) -> Dict[str, Any]: ...

@final
class Camera:
    identifier: DeviceIdentifier
    settings: CameraProperties
    def dict(self) -> Dict[str, Any]: ...

@final
class StorageProperties:
    filename: None | str
    first_frame_id: int
    def dict(self) -> Dict[str, Any]: ...

@final
class Storage:
    identifier: DeviceIdentifier
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
    identifier: DeviceIdentifier
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
    identifier: DeviceIdentifier
    settings: SignalProperties
    def dict(self) -> Dict[str, Any]: ...

@final
class Properties:
    camera: Camera
    storage: Storage
    stages: Tuple[StageAxis, StageAxis, StageAxis]
    signals: Signals
    max_frame_count: int
    frame_average_count: int
    def dict(self) -> Dict[str, Any]: ...

@final
class Runtime:
    def start(self) -> None: ...
    def stop(self) -> None: ...
    def device_manager(self) -> DeviceManager: ...
    def set_configuration(self, properties: Properties) -> Properties: ...
    def get_configuration(self) -> Properties: ...
    def get_available_data(
        self,
    ) -> Any: ...  # FIXME(nclack): type AvailableData

def core_api_version() -> str: ...
