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
    """The AvailableData class represents the collection of frames that have
    been captured since the last call to `runtime.get_available_data()`.

    `AvailableData` objects should be set to have a short lifetime, since these
    objects reserve space on the video queue and will eventually block camera
    acquisition to ensure no data is overwritten before it can be processed.

    """
    def frames(self) -> Iterator[VideoFrame]:
        """Returns an iterator over the video frames in the available data.

        Returns:
            An iterator over the video frames in the available data.

        """
        ...

    def get_frame_count(self) -> int:
        """Returns the total number of video frames in the available data.

        Call `get_frame_count()` to query the number of frames in a
        `AvailableData` object.

        Returns:
            The total number of video frames in the AvailableData object.

        """
        ...

    def __iter__(self) -> Iterator[VideoFrame]: ...


@final
class AvailableDataContext:
    def __enter__(self) -> AvailableData: ...
    def __exit__(
        self, exc_type: Any, exc_value: Any, traceback: Any
    ) -> None: ...


@final
class Camera:
    """The `Camera` class is used to describe cameras or other video sources.

    Attributes:
        identifier:
            An optional attribute which contains an instance of the
            `DeviceIdentifier` class. `DeviceIdentifier` has `id` and `kind`
            attributes assigned by `acquire` if the device is natively
            supported. Otherwise, it is of type `None`.
        settings:
            An instance of the `CameraProperties` class which contains the
            settings for the camera.

    """
    identifier: Optional[DeviceIdentifier]
    settings: CameraProperties

    def __init__(self, *args: None, **kwargs: Any) -> None:
        """Initializes a Camera object with optional arguments.
        """
        ...

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the Camera object's attributes.
        """
        ...


@final
class CameraCapabilities:
    exposure_time_us: Property
    line_interval_us: Property
    readout_direction: Property
    binning: Property
    offset: OffsetCapabilities
    shape: ShapeCapabilities
    supported_pixel_types: List[SampleType]
    digital_lines: DigitalLineCapabilities
    triggers: TriggerCapabilities

    def dict(self) -> Dict[str, Any]: ...


@final
class CameraProperties:
    """
    The `CameraProperties` class is used to set the desired camera properties
    for acquisition.

    Attributes:
        exposure_time_us:
            How long in microseconds your camera should collect light from the
            sample. However, for simulated cameras, this is just a waiting
            period before generating the next frame.
        line_interval_us:
            The time to scan one line in microseconds in a rolling shutter
            camera.
        binning:
            How many adjacent pixels in each direction to combine by averaging.
            For example, if `binning` is set to 2, a 2x2 square of pixels will
            be combined by averaging. If `binning` is set to 1, no pixels will
            be combined.
        pixel_type:
            An instance of the `SampleType` class which specifies the numerical
            data type, for example Uint16, a 16-bit unsigned integer type.
        readout_direction:
            An instance of the `Direction` class which specifies whether the
            data is readout forwards or backwards.
        offset:
            A tuple of two integers representing the (x, y) offset in pixels of
            the image region of interest on the camera.
        shape:
            A tuple of two integers representing the (x, y)size in pixels of
            the image region of interest on the camera.
        input_triggers:
            An instance of the `InputTriggers` class, which describes the
            trigger signals for starting acquisition, camera exposure, and
            acquiring a frame.
        output_triggers:
            An instance of the `OutputTriggers` class, which describes the
            trigger signals for the camera exposure, acquiring a frame, as well
            as any wait times for sending the trigger signal.
    """
    exposure_time_us: float
    line_interval_us: float
    binning: float
    pixel_type: SampleType
    readout_direction: Direction
    offset: Tuple[int, int]
    shape: Tuple[int, int]
    input_triggers: InputTriggers
    output_triggers: OutputTriggers

    def __init__(self, *args: None, **kwargs: Any) -> None:
        """Initializes a CameraProperties object with optional arguments.
        """
        ...

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `CameraProperties` object's attributes.
        """
        ...


@final
class Capabilities:
    video: Tuple[VideoStreamCapabilities, VideoStreamCapabilities]

    def __init__(self, *args: None, **kwargs: Any) -> None: ...
    def dict(self) -> Dict[str, Any]: ...


@final
class DeviceIdentifier:
    """Represents an identifier for a supported device, including its unique id
    and type, such as a camera or storage.

    Attributes:
        id:
            A tuple of `(driver_id, device_id)` containing two Uint8 integers
            that serve to identify each driver and device uniquely for a given
            run.
        kind:
            An instance of the `DeviceKind` class that represents the type or
            kind of the device.
        name:
            A string representing the name or label of the device.
    """
    id: Tuple[int, int]
    kind: DeviceKind
    name: str

    def __init__(self, *args: None, **kwargs: Any) -> None:
        """Initializes a DeviceIdentifier object with optional arguments.
        """
        ...

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `DeviceIdentifier` object's attributes.
        """
        ...

    @staticmethod
    def none() -> DeviceIdentifier:
        """Returns a "None" type DeviceIdentifier.

        Useful when a DeviceIdentifier is not needed.
        """
        ...

    def __eq__(self, other: object) -> bool:
        """Checks if two DeviceIdentifier objects are equal.
        """
        ...

    def __ge__(self, other: object) -> bool:
        """Checks if this DeviceIdentifier is greater than or equal to another.
        """
        ...

    def __gt__(self, other: object) -> bool:
        """Checks if this DeviceIdentifier is greater than another.
        """
        ...

    def __le__(self, other: object) -> bool:
        """Checks if this DeviceIdentifier is less than or equal to another.
        """
        ...

    def __lt__(self, other: object) -> bool:
        """Checks if this DeviceIdentifier is less than another.
        """
        ...

    def __ne__(self, other: object) -> bool:
        """Checks if two DeviceIdentifier objects are not equal.
        """
        ...


@final
class DeviceKind:
    """This class represents the types of devices in a given system.

    Attributes:
        Camera:
            Enum-type class variable of `DeviceKind` that specifies a device is
            a camera.
        NONE:
            Enum-type class variable of `DeviceKind` for if a device's kind is
            unavailable.
        Signals:
            Enum-type class variable of `DeviceKind` that specifies a device is
            a signal.
        StageAxis:
            Enum-type class variable of `DeviceKind` that specifies a device is
            a stage.
        Storage:
            Enum-type class variable of `DeviceKind` that specifies a device is
            for storage.
    """
    Camera: ClassVar[DeviceKind]
    NONE: ClassVar[DeviceKind]
    Signals: ClassVar[DeviceKind]
    StageAxis: ClassVar[DeviceKind]
    Storage: ClassVar[DeviceKind]

    def __init__(self, *args: None, **kwargs: Any) -> None:
        """Initializes the DeviceKind class.
        """
        ...

    def __eq__(self, other: object) -> bool:
        """Checks if two DeviceKind objects are equal.
        """
        ...

    def __ge__(self, other: object) -> bool:
        """Checks if this DeviceKind is greater than or equal to another.
        """
        ...

    def __gt__(self, other: object) -> bool:
        """Checks if this DeviceKind is greater than another.
        """
        ...

    def __int__(self) -> int:
        """Converts the DeviceKind to an integer.
        """
        ...

    def __le__(self, other: object) -> bool:
        """Checks if this DeviceKind is less than or equal to another.
        """
        ...

    def __lt__(self, other: object) -> bool:
        """Checks if this DeviceKind is less than another.
        """
        ...

    def __ne__(self, other: object) -> bool:
        """Checks if two DeviceKind objects are not equal.
        """
        ...


@final
class DeviceManager:
    """The `DeviceManager` class manages selection of available devices in the
    system.

    Regular expressions are accepted for the name argument.
    """

    def devices(self) -> List[DeviceIdentifier]:
        """Returns a list of all available device identifiers.
        """
        ...

    def select(
        self, kind: DeviceKind, name: Optional[str] = None
    ) -> Optional[DeviceIdentifier]:
        """Selects a specified device.

        Call this method to choose the first available device of a given type
        or to select a specific device by name.

        Parameters:
            kind:
                The type of device to select.
            name:
                A list of device names to choose from. Regular expressions
                supported.

        Returns:
            The selected device identifier, or None if none of the specified \
            devices are available.
        """

    def select_one_of(
        self, kind: DeviceKind, names: List[str]
    ) -> Optional[DeviceIdentifier]:
        """Choose one device from a list of acceptable devices of a given kind.
        """
        ...


@final
class DeviceState:
    """The `DeviceState` class represents the acquisition status of a device.

    Attributes:
        Closed:
            Enum-type class variable of `DeviceState` that specifies when a
            device is not ready for configuration.
        AwaitingConfiguration:
            Enum-type class variable of `DeviceState` that specifies when a
            device is ready for configuration.
        Armed:
            Enum-type class variable of `DeviceState` that specifies when a
            device is ready to stream data.
        Running:
            Enum-type class variable of `DeviceState` that specifies when a
            device is streaming data.
    """
    Closed: ClassVar[DeviceState]
    AwaitingConfiguration: ClassVar[DeviceState]
    Armed: ClassVar[DeviceState]
    Running: ClassVar[DeviceState]

    def __eq__(self, other: object) -> bool:
        """Checks if two DeviceState objects are equal.
        """
        ...

    def __ge__(self, other: object) -> bool:
        """Checks if this DeviceState is greater than or equal to another.
        """
        ...

    def __gt__(self, other: object) -> bool:
        """Checks if this DeviceState is greater than another.
        """
        ...

    def __int__(self) -> int:
        """Converts the DeviceState to an integer.
        """
        ...

    def __le__(self, other: object) -> bool:
        """Checks if this DeviceState is less than or equal to another.
        """
        ...

    def __lt__(self, other: object) -> bool:
        """Checks if this DeviceState is less than another.
        """
        ...

    def __ne__(self, other: object) -> bool:
        """Checks if two DeviceState objects are not equal.
        """
        ...


@final
class DigitalLineCapabilities:
    line_count: int
    names: Tuple[str, str, str, str, str, str, str, str]

    def dict(self) -> Dict[str, Any]: ...


@final
class DimensionType:
    """The storage dimension type.

    When downsampling, Space and Time dimensions are downsampled by the same
    factor. Channel and Other dimensions are not downsampled.

    This value is also reflected in the dimension metadata of an OME-Zarr
    dataset.

    Attributes:
        Space:
            Spatial dimension.
        Channel:
            Color channel dimension.
        Time:
            Time dimension.
        Other:
            Other dimension.
    """

    Space: ClassVar[DimensionType]
    Channel: ClassVar[DimensionType]
    Time: ClassVar[DimensionType]
    Other: ClassVar[DimensionType]

    def __init__(self, *args: None, **kwargs: Any) -> None: ...

    def __eq__(self, other: object) -> bool:
        """Checks if two DimensionType objects are equal.
        """
        ...

    def __ge__(self, other: object) -> bool:
        """Checks if this DimensionType is greater than or equal to another.
        """
        ...

    def __gt__(self, other: object) -> bool:
        """Checks if this DimensionType is greater than another.
        """
        ...

    def __int__(self) -> int:
        """Converts the DimensionType to an integer.
        """
        ...

    def __le__(self, other: object) -> bool:
        """Checks if this DimensionType is less than or equal to another.
        """
        ...

    def __lt__(self, other: object) -> bool:
        """Checks if this DimensionType is less than another.
        """
        ...

    def __ne__(self, other: object) -> bool:
        """Checks if two DimensionType objects are not equal.
        """
        ...


@final
class Direction:
    """The direction that data is read for streaming.

    Attributes:
        Backward:
            Enum-type class variable of `Direction` that specifies when data
            is streamed backward.
        Forward:
            Enum-type class variable of `Direction` that specifies when data
            is streamed forward.
    """
    Backward: ClassVar[Direction]
    Forward: ClassVar[Direction]

    def __eq__(self, other: object) -> bool:
        """Checks if two Direction objects are equal.
        """
        ...

    def __ge__(self, other: object) -> bool:
        """Checks if this Direction is greater than or equal to another.
        """
        ...

    def __gt__(self, other: object) -> bool:
        """Checks if this Direction is greater than another.
        """
        ...

    def __int__(self) -> int:
        """Converts the Direction to an integer.
        """
        ...

    def __le__(self, other: object) -> bool:
        """Checks if this Direction is less than or equal to another.
        """
        ...

    def __lt__(self, other: object) -> bool:
        """Checks if this Direction is less than another.
        """
        ...

    def __ne__(self, other: object) -> bool:
        """Checks if two Direction objects are not equal.
        """
        ...


@final
class InputTriggers:
    """The `InputTriggers` class represents input triggers for a camera device.

    Attributes:
        acquisition_start:
            An instance of the `Trigger` class representing the trigger for
            starting acquisition.
        exposure:
            An instance of the `Trigger` class representing the trigger for
            exposure.
        frame_start:
            An instance of the `Trigger` class representing the trigger for
            starting a frame.
    """

    acquisition_start: Trigger
    exposure: Trigger
    frame_start: Trigger

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of a `InputTriggers` object's attributes.
        """
        ...


@final
class OffsetCapabilities:
    x: Property
    y: Property

    def dict(self) -> Dict[str, Any]: ...


@final
class OutputTriggers:
    """The `OutputTriggers` class represents output triggers for a camera
    device.

    Attributes:
        exposure:
            An instance of the `Trigger` class representing the trigger for
            exposure.
        frame_start:
            An instance of the `Trigger` class representing the trigger for
            starting a frame.
        trigger_wait:
            An instance of the `Trigger` class representing the trigger for
            waiting before continuing acquisition.
    """
    exposure: Trigger
    frame_start: Trigger
    trigger_wait: Trigger

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `OutputTriggers` object's attributes.
        """


@final
class PID:
    """The `PID` class represents proportional-integral-derivative (PID) values.

    Attributes:
        derivative:
            The derivative value for the PID.
        integral:
            The integral value for the PID.
        proportional:
            The proportional value for the PID.

    """
    derivative: float
    integral: float
    proportional: float

    def __init__(self, *args: None, **kwargs: Any) -> None:
        """Initializes a PID object with optional arguments.
        """
        ...

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the PID attributes.
        """
        ...


@final
class Properties:
    """The `Properties` class represents properties related to video streams.

    Attributes:
        video:
            A tuple containing two `VideoStream` instances since `acquire`
            supports simultaneous streaming from 2 video sources. `VideoStream`
            objects have 2 attributes `camera` and `storage` to set the source
            and sink for the stream.

    """
    video: Tuple[VideoStream, VideoStream]

    def __init__(self, *args: None, **kwargs: Any) -> None:
        """Initializes a `Properties` object with optional arguments.
        """
        ...

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `Properties` attributes.
        """
        ...


@final
class Property:
    writable: bool
    low: float
    high: float
    kind: PropertyType

    def __init__(self, *args: None, **kwargs: Any) -> None: ...
    def dict(self) -> Dict[str, Any]: ...


@final
class PropertyType:
    FixedPrecision: ClassVar[PropertyType]
    FloatingPrecision: ClassVar[PropertyType]
    Enum: ClassVar[PropertyType]
    String: ClassVar[PropertyType]

    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: object) -> bool: ...
    def __gt__(self, other: object) -> bool: ...
    def __int__(self) -> int: ...
    def __le__(self, other: object) -> bool: ...
    def __lt__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...


@final
class Runtime:
    """Coordinates runtime.

    The `Runtime` class coordinates the devices with the storage disc
    including selecting the devices, setting their properties, and starting and
    stopping acquisition.

    """
    def __init__(self, *args: None, **kwargs: Any) -> None:
        """Initializes the Runtime object with optional arguments.
        """
        ...

    def device_manager(self) -> DeviceManager:
        """Returns the DeviceManager instance associated with this Runtime.

        Call `device_manager()` to return the `DeviceManager` object associated
        with this `Runtime` instance.
        """
        ...

    def get_available_data(self, stream_id: int) -> AvailableDataContext:
        """Returns the AvailableData instance for the given stream ID.

        Call `get_available_data` with a specific `stream_id`, 0 or 1, to
        return the `AvailableData` associated with the 1st or 2nd video source,
        respectively.

        Parameters:
            stream_id:
                The ID of the stream for which available data is requested.

        Returns:
            AvailableData:
                The AvailableData instance for the given VideoStream ID.
        """
        ...

    def get_configuration(self) -> Properties:
        """Returns the current configuration properties of the runtime.

        Call `get_configuration()` to return the `Properties` object associated
        with this `Runtime` instance.
        """
        ...
        
    def get_capabilities(self) -> Capabilities: ...

    def get_state(self) -> DeviceState:
        """Returns the current state of the device.

        Call `get_state()` to return the `DeviceState` object associated with
        this `Runtime` instance.
        """
        ...

    def set_configuration(self, properties: Properties) -> Properties:
        """Applies the provided configuration properties to the runtime.

        Call `set_configuration` with a `Properties` object to change the
        properties of this `Runtime` instance.

        Parameters:
            properties:
                The properties to be set.

        Returns:
            The updated configuration properties.
        """
        ...

    def start(self) -> None:
        """Starts the runtime, allowing it to collect data.

        Call `start()` to begin data acquisition.
        """
        ...

    def execute_trigger(self, stream_id: int) -> None: ...

    def stop(self) -> None:
        """Stops the runtime, ending data collection after the max number of
        frames is collected.

        Call `stop()` to end data acquisition once the max number of frames
        specified in `acquire.VideoStream.max_frame_count` is collected. All
        objects are deleted to free up disk space upon shutdown of `Runtime`.
        """
        ...

    def abort(self) -> None:
        """Aborts the runtime, terminating it immediately.

        Call `abort()` to immediately end data acqusition. All objects are
        deleted to free up disk space upon shutdown of `Runtime`.
        """
        ...


@final
class SampleRateHz:
    """The `SampleRateHz` class represents the sampling rate in hertz.

    Attributes:
        numerator:
            The numerator part of the sampling rate fraction.
        denominator:
            The denominator part of the sampling rate fraction.
    """

    numerator: int
    denominator: int

    def __init__(self, *args: None, **kwargs: Any) -> None:
        """Initializes a SampleRateHz object with optional arguments.
        """
        ...

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `SampleRateHz` object's attributes.
        """
        ...


@final
class SampleType:
    """The `SampleType` class defines the type of the values in the streamed
    data.

    Attributes:
        F32:
            Enum-type class variable of `SampleType` that specifies values of
            32-bit floating point type.
        I16:
            Enum-type class variable of `SampleType` that specifies values of
            16-bit signed integer type.
        I8:
            Enum-type class variable of `SampleType` that specifies values of
            8-bit signed integer type.
        U16:
            Enum-type class variable of `SampleType` that specifies values of
            16-bit unsigned integer type.
        U8:
            Enum-type class variable of `SampleType` that specifies values of
            8-bit unsigned integer type.
        U10:
            Enum-type class variable of `SampleType` that specifies values of
            10-bit unsigned integer type.
        U12:
            Enum-type class variable of `SampleType` that specifies values of
            12-bit unsigned integer type.
        U14:
            Enum-type class variable of `SampleType` that specifies values of
            14-bit unsigned integer type.
    """
    F32: ClassVar[SampleType]
    I16: ClassVar[SampleType]
    I8: ClassVar[SampleType]
    U16: ClassVar[SampleType]
    U8: ClassVar[SampleType]
    U10: ClassVar[SampleType]
    U12: ClassVar[SampleType]
    U14: ClassVar[SampleType]

    def __eq__(self, other: object) -> bool:
        """Checks if two SampleType objects are equal.
        """
        ...

    def __ge__(self, other: object) -> bool:
        """Checks if this SampleType is greater than or equal to another.
        """
        ...

    def __gt__(self, other: object) -> bool:
        """Checks if this SampleType is greater than another.
        """
        ...

    def __int__(self) -> int:
        """Converts the SampleType to an integer.
        """
        ...

    def __le__(self, other: object) -> bool:
        """Checks if this SampleType is less than or equal to another.
        """
        ...

    def __lt__(self, other: object) -> bool:
        """Checks if this SampleType is less than another.
        """
        ...

    def __ne__(self, other: object) -> bool:
        """Checks if two SampleType objects are not equal.
        """
        ...


@final
class ShapeCapabilities:
    x: Property
    y: Property

    def dict(self) -> Dict[str, Any]: ...


@final
class SignalIOKind:
    """The `SignalIOKind` class defines the signal type, input or output, for a
    trigger.

    Attributes:
        Input:
            Enum-type class variable of `SignalIOKind` that specifies signal
            coming in to the device.
        Output:
            Enum-type class variable of `SignalIOKind` that specifies signal
            sent out of the device.
    """
    Input: ClassVar[SignalIOKind]
    Output: ClassVar[SignalIOKind]

    def __eq__(self, other: object) -> bool:
        """Checks if two SignalIOKind objects are equal.
        """
        ...

    def __ge__(self, other: object) -> bool:
        """Checks if this SignalIOKind is greater than or equal to another.
        """
        ...

    def __gt__(self, other: object) -> bool:
        """Checks if this SignalIOKind is greater than another.
        """
        ...

    def __int__(self) -> int:
        """Converts the SignalIOKind to an integer.
        """
        ...

    def __le__(self, other: object) -> bool:
        """Checks if this SignalIOKind is less than or equal to another.
        """
        ...

    def __lt__(self, other: object) -> bool:
        """Checks if this SignalIOKind is less than another.
        """
        ...

    def __ne__(self, other: object) -> bool:
        """Checks if two SignalIOKind objects are not equal.
        """
        ...


@final
class SignalType:
    """The `SignalType` class specifies whether a signal is analog or digital.

    Attributes:
        Analog:
            Enum-type class variable of `SignalType` that specifies a signal is
            analog.
        Digital:
            Enum-type class variable of `SignalType` that specifies a signal is
            digital.
    """
    Analog: ClassVar[SignalType]
    Digital: ClassVar[SignalType]

    def __eq__(self, other: object) -> bool:
        """Checks if two SignalType objects are equal.
        """
        ...

    def __ge__(self, other: object) -> bool:
        """Checks if this SignalType is greater than or equal to another.
        """
        ...

    def __gt__(self, other: object) -> bool:
        """Checks if this SignalType is greater than another.
        """
        ...

    def __int__(self) -> int:
        """Converts the SignalType to an integer.
        """
        ...

    def __le__(self, other: object) -> bool:
        """Checks if this SignalType is less than or equal to another.
        """
        ...

    def __lt__(self, other: object) -> bool:
        """Checks if this SignalType is less than another.
        """
        ...

    def __ne__(self, other: object) -> bool:
        """Checks if two SignalType objects are not equal.
        """
        ...


@final
class Storage:
    """The `Storage` class represents storage devices and their settings.

    Attributes:
        identifier:
            An optional attribute which contains an instance of the
            `DeviceIdentifier` class that describes the storage device if that
            device is natively supported. Otherwise, it is of type `None`.
        settings:
            An instance of the `StorageProperties` class which contains the
            settings for the data storage.
    """
    identifier: Optional[DeviceIdentifier]
    settings: StorageProperties

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `Storage` object's attributes.
        """
        ...


@final
class StorageCapabilities:
    chunking_is_supported: bool
    sharding_is_supported: bool
    multiscale_is_supported: bool

    def dict(self) -> Dict[str, Any]: ...


@final
class StorageDimension:
    name: str
    kind: DimensionType
    array_size_px: int
    chunk_size_px: int
    shard_size_chunks: int

    def dict(self) -> Dict[str, Any]: ...


@final
class StorageProperties:
    """The `StorageProperties` class represents properties for data storage.

    Attributes:
        external_metadata_json:
            An optional attribute of the metadata JSON filename as a string.
        filename:
            An optional attribute representing the filename for storing the
            image data.
        first_frame_id:
            An integer representing the ID of the first frame for a given
            acquisition.
        pixel_scale_um:
            A tuple of two floats representing the pixel size of the camera in
            micrometers.
        acquisition_dimensions:
            A list of `StorageDimension` objects representing the dimensions
            of the acquisition.
        enable_multiscale:
            A boolean indicating whether multiscale storage is enabled.
    """

    external_metadata_json: Optional[str]
    filename: Optional[str]
    first_frame_id: int
    pixel_scale_um: Tuple[float, float]
    acquisition_dimensions: List[StorageDimension]
    enable_multiscale: bool

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `StorageProperties` object's attributes.
        """


@final
class Trigger:
    """The `Trigger` class represents a trigger signal.

    Attributes:
        edge:
            An instance of the `TriggerEdge` class specifying if the trigger is
            on the rising or falling edge trigger signal.
        enable:
            A boolean indicating whether the trigger is enabled.
        line:
            An integer representing the max value of the trigger signal.
        kind:
            An instance of the `SignalIOKind` class specifying if the signal is
            input or output.
    """
    edge: TriggerEdge
    enable: bool
    line: int
    kind: SignalIOKind

    def __init__(self, *args: None, **kwargs: Any) -> None:
        """Initializes a Trigger object with optional arguments.
        """
        ...

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `Trigger` object's attributes.
        """
        ...


@final
class TriggerCapabilities:
    acquisition_start: TriggerInputOutputCapabilities
    exposure: TriggerInputOutputCapabilities
    frame_start: TriggerInputOutputCapabilities

    def dict(self) -> Dict[str, Any]: ...


@final
class TriggerEdge:
    """The `TriggerEdge` class represents what edge of the trigger function
    initiates the trigger.

    Attributes:
        Falling:
            Enum-type class variable of `TriggerEdge` that defines the falling
            edge of the trigger.
        NotApplicable:
            Enum-type class variable of `TriggerEdge` that defines if a trigger
            does not have a rising or falling edge.
        Rising:
            Enum-type class variable of `TriggerEdge` that defines the rising
            edge of the trigger.
        AnyEdge:
            Enum-type class variable
        LevelLow:
            Enum-type class variable
        LevelHigh:
            Enum-type class variable
    """
    Falling: ClassVar[TriggerEdge]
    NotApplicable: ClassVar[TriggerEdge]
    Rising: ClassVar[TriggerEdge]
    AnyEdge: ClassVar[TriggerEdge]
    LevelLow: ClassVar[TriggerEdge]
    LevelHigh: ClassVar[TriggerEdge]

    def __eq__(self, other: object) -> bool:
        """Checks if two TriggerEdge objects are equal.
        """
        ...

    def __ge__(self, other: object) -> bool:
        """Checks if this TriggerEdge is greater than or equal to another.
        """
        ...

    def __gt__(self, other: object) -> bool:
        """Checks if this TriggerEdge is greater than another.
        """
        ...

    def __int__(self) -> int:
        """Converts the TriggerEdge to an integer.
        """
        ...

    def __le__(self, other: object) -> bool:
        """Checks if this TriggerEdge is less than or equal to another.
        """
        ...

    def __lt__(self, other: object) -> bool:
        """Checks if this TriggerEdge is less than another.
        """
        ...

    def __ne__(self, other: object) -> bool:
        """Checks if two TriggerEdge objects are not equal.
        """
        ...


@final
class TriggerInputOutputCapabilities:
    input: int
    output: int

    def dict(self) -> Dict[str, Any]: ...


@final
class VideoFrame:
    """The `VideoFrame` class represents data from acquisition of a frame.

    """
    def data(self) -> NDArray[Any]:
        """Returns the data of the video frame as an NDArray.

        Call `data()` to create an NDArray of the `VideoFrame` data.
        """
        ...

    def metadata(self) -> VideoFrameMetadata:
        """Returns the metadata associated with the video frame.

        Call `metadata()` to create a `VideoFrameMetadata` object containing
        the metadata of `VideoFrame`.
        """
        ...


@final
class VideoFrameMetadata:
    """The `VideoFrameMetadata` class represents metadata related to a video
    frame.

    Attributes:
        frame_id:
            An integer representing the ID of the video frame.
        timestamps:
            An instance of the `VideoFrameTimestamps` class specifying the
            video timestamps based on the hardware clock and the acquisition
            clock.
    """

    frame_id: int
    timestamps: VideoFrameTimestamps

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `VideoFrameMetadata` object's attributes.
        """


@final
class VideoFrameTimestamps:
    """The `VideoFrameTimestamps` class represents timestamps related to a
    video frame.

    Attributes:
        hardware:
            An integer representing hardware timestamps.
        acq_thread:
            An integer representing timestamps from the acquisition thread.
    """
    hardware: int
    acq_thread: int

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of a `VideoFrameTimestamps` object's attributes.
        """


@final
class VideoStream:
    """The `VideoStream` class represents a video stream.

    Attributes:
        camera:
            An instance of the `Camera` class representing the camera device
            for the video stream.
        storage:
            An instance of the `Storage` class representing the storage device
            for the video stream.
        max_frame_count:
            An integer representing the maximum number of frames to acquire.
        frame_average_count:
            An integer representing the number of frames to average, if any,
            before streaming. The default value is 0, which disables this
            feature. Setting this to 1 will also prevent averaging.
    """
    camera: Camera
    storage: Storage
    max_frame_count: int
    frame_average_count: int

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `VideoStream` object's attributes.
        """


@final
class VideoStreamCapabilities:
    camera: CameraCapabilities
    storage: StorageCapabilities
    max_frame_count: Property
    frame_average_count: Property

    def dict(self) -> Dict[str, Any]: ...


@final
class VoltageRange:
    """The `VoltageRange` class represents a range of voltage values.

    Attributes:
        mn:
            A float representing the minimum voltage value.
        mx:
            A float representing the maximum voltage value.
    """
    mn: float
    mx: float

    @overload
    def __init__(self) -> None:
        """Initializes a VoltageRange object.
        """
        ...

    @overload
    def __init__(self, mn: float, mx: float) -> None:
        """Initializes a VoltageObject object with mn and mx provided.
        """
        ...

    def dict(self) -> Dict[str, float]:
        """Returns a dictionary of the `VoltageRange` object's attributes.
        """
        ...


def core_api_version() -> str: ...
