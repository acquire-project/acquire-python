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
    """The `AvailableDataContext` class is the context manager for available
    data for the given VideoStream ID.

    """

    def __enter__(self) -> AvailableData:
        """Get the available data from the runtime and return it."""
        ...
    def __exit__(self, exc_type: Any, exc_value: Any, traceback: Any) -> None:
        """Clean up any references to the available data returned when entering
        this context.
        """
        ...

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
        """Initializes a Camera object with optional arguments."""
        ...
    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the Camera object's attributes."""
        ...

@final
class CameraCapabilities:
    """The `CameraCapabilities` class is used to describe the camera's
    supported properties.

    Attributes:
        exposure_time_us:
            An instance of the `Property` class that captures the range and
            type of supported values in microseconds for the camera's exposure
            time, which is how long in microseconds the camera collects light
            from the sample for a single frame.
        line_interval_us:
            An instance of the `Property` class that captures the range and
            type of supported values in microseconds for a rolling shutter
            camera to scan one line.
        readout_direction:
            An instance of the `Property` class that specifies whether the data
            is read out of the camera forwards or backwards and if that
            direction can be chosen by the user.
        binning:
            An instance of the `Property` class that captures the range and
            type of support values for binning, which is combining adjacent
            pixels by averaging in each direction, and whether the binning
            factor can be chosen by the user.
        offset:
            An instance of the `OffsetShapeCapabilities` class that represents
            the horizontal and vertical offset for the region of interest on
            the camera chip.
        shape:
            An instance of the `OffsetShapeCapabilities` class that represents
            the width and height of the region of interest on the camera chip.
        supported_pixel_types:
            A list containing instances of the `SampleType` class representing
            each of the supported pixel types, such as 8-bit unsigned integer
            (uint8).
        digital_lines:
            An instance of the `DigitalLineCapabilities` class which indicates
            the number and names of the available lines. Up to 8 lines are
            supported with the last line typically being the camera software trigger.
        triggers:
            An instance of the `TriggerCapabilities` class which indicate what
            kinds of triggers (start acquisition, start exposure, or start a
            frame) are supported.

    """

    exposure_time_us: Property
    line_interval_us: Property
    readout_direction: Property
    binning: Property
    offset: OffsetCapabilities
    shape: ShapeCapabilities
    supported_pixel_types: List[SampleType]
    digital_lines: DigitalLineCapabilities
    triggers: TriggerCapabilities

    def dict(self) -> Dict[str, Any]:
        """Creates a dictionary of a `CameraCapabilities` object's attributes."""
        ...

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
        """Initializes a CameraProperties object with optional arguments."""
        ...
    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `CameraProperties` object's attributes."""
        ...

@final
class Capabilities:
    """The `Capabilities` class contains representations of each of the 2
    supported VideoStream objects.

    Attributes:
        video:
            A tuple containing two `VideoStreamCapabilities` instances since
            `acquire` supports simultaneous streaming from 2 video sources.
    """

    video: Tuple[VideoStreamCapabilities, VideoStreamCapabilities]

    def __init__(self, *args: None, **kwargs: Any) -> None:
        """Initializes a Capabilities object with optional arguments."""
        ...
    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `Capabilities` object's attributes."""
        ...

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
        """Initializes a DeviceIdentifier object with optional arguments."""
        ...
    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `DeviceIdentifier` object's attributes."""
        ...
    @staticmethod
    def none() -> DeviceIdentifier:
        """Returns a "None" type DeviceIdentifier.

        Useful when a DeviceIdentifier is not needed.
        """
        ...
    def __eq__(self, other: object) -> bool:
        """Checks if two DeviceIdentifier objects are equal."""
        ...
    def __ge__(self, other: object) -> bool:
        """Checks if this DeviceIdentifier is greater than or equal to another."""
        ...
    def __gt__(self, other: object) -> bool:
        """Checks if this DeviceIdentifier is greater than another."""
        ...
    def __le__(self, other: object) -> bool:
        """Checks if this DeviceIdentifier is less than or equal to another."""
        ...
    def __lt__(self, other: object) -> bool:
        """Checks if this DeviceIdentifier is less than another."""
        ...
    def __ne__(self, other: object) -> bool:
        """Checks if two DeviceIdentifier objects are not equal."""
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
        """Initializes the DeviceKind class."""
        ...
    def __eq__(self, other: object) -> bool:
        """Checks if two DeviceKind objects are equal."""
        ...
    def __ge__(self, other: object) -> bool:
        """Checks if this DeviceKind is greater than or equal to another."""
        ...
    def __gt__(self, other: object) -> bool:
        """Checks if this DeviceKind is greater than another."""
        ...
    def __int__(self) -> int:
        """Converts the DeviceKind to an integer."""
        ...
    def __le__(self, other: object) -> bool:
        """Checks if this DeviceKind is less than or equal to another."""
        ...
    def __lt__(self, other: object) -> bool:
        """Checks if this DeviceKind is less than another."""
        ...
    def __ne__(self, other: object) -> bool:
        """Checks if two DeviceKind objects are not equal."""
        ...

@final
class DeviceManager:
    """The `DeviceManager` class manages selection of available devices in the
    system.

    Regular expressions are accepted for the name argument.
    """

    def devices(self) -> List[DeviceIdentifier]:
        """Returns a list of all available device identifiers."""
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
                The name of the device to select. Regular expressions supported.

        Returns:
            The selected device identifier, or None if the specified device is
            not available.
        """
    def select_one_of(
        self, kind: DeviceKind, names: List[str]
    ) -> Optional[DeviceIdentifier]:
        """Selects the first device in the list of devices that is of one of
        the specified kinds.

        Parameters:
            kind:
                The type of device to select.
            names:
                A list of device names to choose from. Regular expressions
                supported.

        Returns:
            Optional[DeviceIdentifier]: The selected device identifier, or None
            if none of the specified devices are available.
        """

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
        """Checks if two DeviceState objects are equal."""
        ...
    def __ge__(self, other: object) -> bool:
        """Checks if this DeviceState is greater than or equal to another."""
        ...
    def __gt__(self, other: object) -> bool:
        """Checks if this DeviceState is greater than another."""
        ...
    def __int__(self) -> int:
        """Converts the DeviceState to an integer."""
        ...
    def __le__(self, other: object) -> bool:
        """Checks if this DeviceState is less than or equal to another."""
        ...
    def __lt__(self, other: object) -> bool:
        """Checks if this DeviceState is less than another."""
        ...
    def __ne__(self, other: object) -> bool:
        """Checks if two DeviceState objects are not equal."""
        ...

@final
class DigitalLineCapabilities:
    """The `DigitalLineCapabilities` class represents the digital lines
    supported by the device.

    Attributes:
        line_count:
            Integer number representing the number of digital lines supported.
        names:
            Tuple of strings to name each of the digital lines, typically the
            last one is the camera software trigger.
    """

    line_count: int
    names: Tuple[str, ...]

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `DigitalLineCapabilities` object's
        attributes.
        """
        ...

@final
class DimensionType:
    """Used to specify the physical meaning of a dimension, such as space or
    time dimension.

    When downsampling, Space and Time dimensions are downsampled by the same
    factor. Channel and Other dimensions are not downsampled.

    This value is also reflected in the dimension metadata of an OME-Zarr
    dataset.

    Attributes:
        Space:
            Enum-type class variable of `DimensionType` that indicates a spatial
            dimension.
        Channel:
            Enum-type class variable of `DimensionType` that indicates a color
            channel dimension.
        Time:
            Enum-type class variable of `DimensionType` that indicates a time
            dimension.
        Other:
            Enum-type class variable of `DimensionType` that indicates the
            dimension is not a space, channel, or time.
    """

    Space: ClassVar[DimensionType]
    Channel: ClassVar[DimensionType]
    Time: ClassVar[DimensionType]
    Other: ClassVar[DimensionType]

    def __init__(self, *args: None, **kwargs: Any) -> None: ...
    def __eq__(self, other: object) -> bool:
        """Checks if two DimensionType objects are equal."""
        ...
    def __ge__(self, other: object) -> bool:
        """Checks if this DimensionType is greater than or equal to another."""
        ...
    def __gt__(self, other: object) -> bool:
        """Checks if this DimensionType is greater than another."""
        ...
    def __int__(self) -> int:
        """Converts the DimensionType to an integer."""
        ...
    def __le__(self, other: object) -> bool:
        """Checks if this DimensionType is less than or equal to another."""
        ...
    def __lt__(self, other: object) -> bool:
        """Checks if this DimensionType is less than another."""
        ...
    def __ne__(self, other: object) -> bool:
        """Checks if two DimensionType objects are not equal."""
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
        """Checks if two Direction objects are equal."""
        ...
    def __ge__(self, other: object) -> bool:
        """Checks if this Direction is greater than or equal to another."""
        ...
    def __gt__(self, other: object) -> bool:
        """Checks if this Direction is greater than another."""
        ...
    def __int__(self) -> int:
        """Converts the Direction to an integer."""
        ...
    def __le__(self, other: object) -> bool:
        """Checks if this Direction is less than or equal to another."""
        ...
    def __lt__(self, other: object) -> bool:
        """Checks if this Direction is less than another."""
        ...
    def __ne__(self, other: object) -> bool:
        """Checks if two Direction objects are not equal."""
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
        """Returns a dictionary of a `InputTriggers` object's attributes."""
        ...

@final
class OffsetCapabilities:
    """Represents the size of the offset of the region of interest
    on the camera.

    The sum of the offset and shape is the size of the full camera chip.

    Attributes:
        x:
            An instance of the `Property` class which represents the horizontal offset of
            the region of interest on the camera chip.
        y:
            An instance of the `Property` class which represents the vertical offset of the
            region of interest on the camera chip.
    """
    
    x: Property
    y: Property

    def dict(self) -> Dict[str, Any]: 
        """Returns a dictionary of a `OffsetCapabilities` object's attributes."""
        ...

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
        """Returns a dictionary of the `OutputTriggers` object's attributes."""

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
        """Initializes a PID object with optional arguments."""
        ...
    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the PID attributes."""
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
        """Initializes a `Properties` object with optional arguments."""
        ...
    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `Properties` attributes."""
        ...

@final
class Property:
    """Indicates the type of and whether the property can be overwritten.

    For numerical values, it also captures the accepted range of values.

    Attributes:
        writable:
            A boolean indicating whether the property can be written.
        low:
            Floating point number for the lower bound of the property, if
            applicable.
        high:
            Floating point number for the upper bound of the property, if
            applicable.
        kind:
            An instance of the `PropertyType` class which indicates the type of
            the property (fixed precision, floating-point, enum, or string).
    """

    writable: bool
    low: float
    high: float
    kind: PropertyType

    def __init__(self, *args: None, **kwargs: Any) -> None:
        """Initializes a Property object with optional arguments."""
        ...
    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `Property` object's attributes."""
        ...

@final
class PropertyType:
    """The `PropertyType` class indicates the type of the property (fixed
    precision, floating-point, enum, or string).

    Attributes:
        FixedPrecision:
            Enum-type class variable of `PropertyType` that indicates fixed
            precision or integer values.
        FloatingPrecision:
            Enum-type class variable of `PropertyType` that indicates floating
            point precision values.
        Enum:
            Enum-type class variable of `PropertyType` that indicates enum-type
            values.
        String:
            Enum-type class variable of `PropertyType` that indicates string
            values.
    """

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
        """Initializes the Runtime object with optional arguments."""
        ...
    def device_manager(self) -> DeviceManager:
        """Returns the DeviceManager instance associated with this Runtime.

        Call `device_manager()` to return the `DeviceManager` object associated
        with this `Runtime` instance.
        """
        ...
    def get_available_data(self, stream_id: int) -> AvailableDataContext:
        """Returns the AvailableDataContext instance for the given stream ID.

        Call `get_available_data` with a specific `stream_id`, 0 or 1, to
        return the context manager, `AvailableDataContext`, associated with the
        1st or 2nd video source, respectively.

        Parameters:
            stream_id:
                The ID of the stream for which available data is requested.

        Returns:
            AvailableDataContext:
                Context manager for available data for the given VideoStream ID.
        """
        ...
    def get_configuration(self) -> Properties:
        """Returns the current configuration properties of the runtime.

        Call `get_configuration()` to return the `Properties` object associated
        with this `Runtime` instance.
        """
        ...
    def get_capabilities(self) -> Capabilities:
        """Returns the current capabilites of the runtime as an instance of
        Capabilities.

        Call `get_capabilities()` to return the `Capabilities` object associated
        with this `Runtime` instance.
        """
        ...
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
    def execute_trigger(self, stream_id: int) -> None:
        """Executes a trigger for the given stream ID.

        Call `execute_trigger` with a specific `stream_id`, 0 or 1, to execute
        a trigger for that video source.
        """
        ...
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
        """Initializes a SampleRateHz object with optional arguments."""
        ...
    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `SampleRateHz` object's attributes."""
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
        """Checks if two SampleType objects are equal."""
        ...
    def __ge__(self, other: object) -> bool:
        """Checks if this SampleType is greater than or equal to another."""
        ...
    def __gt__(self, other: object) -> bool:
        """Checks if this SampleType is greater than another."""
        ...
    def __int__(self) -> int:
        """Converts the SampleType to an integer."""
        ...
    def __le__(self, other: object) -> bool:
        """Checks if this SampleType is less than or equal to another."""
        ...
    def __lt__(self, other: object) -> bool:
        """Checks if this SampleType is less than another."""
        ...
    def __ne__(self, other: object) -> bool:
        """Checks if two SampleType objects are not equal."""
        ...

@final
class ShapeCapabilities:
    """Represents the shape of the region of interest
    on the camera.

    The sum of the offset and shape is the size of the full camera chip.

    Attributes:
        x:
            An instance of the `Property` class which represents the width of
            the region of interest on the camera.
        y:
            An instance of the `Property` class which represents the height of
            the region of interest on the camera.
    """

    x: Property
    y: Property

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of a `ShapeCapabilities` object's
        attributes.
        """
        ...

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
        """Checks if two SignalIOKind objects are equal."""
        ...
    def __ge__(self, other: object) -> bool:
        """Checks if this SignalIOKind is greater than or equal to another."""
        ...
    def __gt__(self, other: object) -> bool:
        """Checks if this SignalIOKind is greater than another."""
        ...
    def __int__(self) -> int:
        """Converts the SignalIOKind to an integer."""
        ...
    def __le__(self, other: object) -> bool:
        """Checks if this SignalIOKind is less than or equal to another."""
        ...
    def __lt__(self, other: object) -> bool:
        """Checks if this SignalIOKind is less than another."""
        ...
    def __ne__(self, other: object) -> bool:
        """Checks if two SignalIOKind objects are not equal."""
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
        """Checks if two SignalType objects are equal."""
        ...
    def __ge__(self, other: object) -> bool:
        """Checks if this SignalType is greater than or equal to another."""
        ...
    def __gt__(self, other: object) -> bool:
        """Checks if this SignalType is greater than another."""
        ...
    def __int__(self) -> int:
        """Converts the SignalType to an integer."""
        ...
    def __le__(self, other: object) -> bool:
        """Checks if this SignalType is less than or equal to another."""
        ...
    def __lt__(self, other: object) -> bool:
        """Checks if this SignalType is less than another."""
        ...
    def __ne__(self, other: object) -> bool:
        """Checks if two SignalType objects are not equal."""
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
        """Returns a dictionary of the `Storage` object's attributes."""
        ...

@final
class StorageCapabilities:
    """The `StorageCapabilities` class represents what types of data handling
    is supported by the storage device.

    Attributes:
        chunking_is_supported:
            A boolean indicating whether chunking is supported for this storage
            device.
        sharding_is_supported:
            A boolean indicating whether sharding is supported for this storage
            device.
        multiscale_is_supported:
            A boolean indicating whether multiscale storage is supported.

    """

    chunking_is_supported: bool
    sharding_is_supported: bool
    multiscale_is_supported: bool
    s3_is_supported: bool

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of a `StorageCapabilities` object's attributes."""
        ...

@final
class StorageDimension:
    """Represents the type and size of the dimension for storage.

    Attributes:
        name:
            A string representing the name or label of the storage dimension.
        kind:
            An instance of the `DimensionType` specifying if the storage
            dimension is space, channel, time, or a different physical
            dimension
        array_size_px:
            The size of the output array along this dimension, in pixels. The
            final (i.e., append) dimension must have size 0.
        chunk_size_px:
            The size of a chunk along this dimension, in pixels.
        shard_size_chunks:
            Integer number of chunks per shard. Shards enable aggregating
            multiple chunks into a single file. This value is ignored if
            sharding is not supported by the storage device.
    """

    name: str
    kind: DimensionType
    array_size_px: int
    chunk_size_px: int
    shard_size_chunks: int

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `StorageDimensions` object's attributes."""
        ...

@final
class StorageProperties:
    """The `StorageProperties` class represents properties for data storage.

    Attributes:
        uri:
            The URI where the image data will be stored.
        external_metadata_json:
            Optional JSON-formatted metadata for the acquisition.
        s3_access_key_id:
            The access key ID for the S3 bucket. This value is only applicable
            for Zarr storage devices and S3 URIs.
        s3_secret_access_key:
            The secret access key for the S3 bucket. This value is only applicable
            for Zarr storage devices and S3 URIs.
        first_frame_id:
            The ID of the first frame.
        pixel_scale_um:
            A tuple of two floats representing the pixel size of the camera in
            micrometers.
        acquisition_dimensions:
            A list of instances of the `StorageDimension` class, one for each
            acquisition dimension. The fastest changing dimension should be
            first in the list and the append dimension should be last. This
            value is only applicable for Zarr storage devices.
        enable_multiscale:
            A boolean indicating whether multiscale storage is enabled.
    """

    uri: Optional[str]
    external_metadata_json: Optional[str]
    s3_access_key_id: Optional[str]
    s3_secret_access_key: Optional[str]
    first_frame_id: int
    pixel_scale_um: Tuple[float, float]
    acquisition_dimensions: List[StorageDimension]
    enable_multiscale: bool

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `StorageProperties` object's attributes."""
        ...

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
        """Initializes a Trigger object with optional arguments."""
        ...
    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of the `Trigger` object's attributes."""
        ...

@final
class TriggerCapabilities:
    """Specifies what types of events the trigger can initiate.

    Attributes:
        acquisition_start:
            An instance of the `TriggerInputOutputCapabilities` class indicating
            which lines, either input or output, are supported for starting
            acquisition.
        exposure:
            An instance of the `TriggerInputOutputCapabilities` class indicating
            which lines, either input or output, are supported for starting
            exposure.
        frame_start:
            An instance of the `TriggerInputOutputCapabilities` class indicating
            which lines, either input or output, are supported for starting a
            frame.
    """

    acquisition_start: TriggerInputOutputCapabilities
    exposure: TriggerInputOutputCapabilities
    frame_start: TriggerInputOutputCapabilities

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of a `TriggerCapabilities` object's attributes."""
        ...

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
            Enum-type class variable of `TriggerEdge` that defines any edge of
            the trigger.
        LevelLow:
            Enum-type class variable of `TriggerEdge` that defines the low
            level of the trigger.
        LevelHigh:
            Enum-type class variable of `TriggerEdge` that defines the high
            level of the trigger.
    """

    Falling: ClassVar[TriggerEdge]
    NotApplicable: ClassVar[TriggerEdge]
    Rising: ClassVar[TriggerEdge]
    AnyEdge: ClassVar[TriggerEdge]
    LevelLow: ClassVar[TriggerEdge]
    LevelHigh: ClassVar[TriggerEdge]

    def __eq__(self, other: object) -> bool:
        """Checks if two TriggerEdge objects are equal."""
        ...
    def __ge__(self, other: object) -> bool:
        """Checks if this TriggerEdge is greater than or equal to another."""
        ...
    def __gt__(self, other: object) -> bool:
        """Checks if this TriggerEdge is greater than another."""
        ...
    def __int__(self) -> int:
        """Converts the TriggerEdge to an integer."""
        ...
    def __le__(self, other: object) -> bool:
        """Checks if this TriggerEdge is less than or equal to another."""
        ...
    def __lt__(self, other: object) -> bool:
        """Checks if this TriggerEdge is less than another."""
        ...
    def __ne__(self, other: object) -> bool:
        """Checks if two TriggerEdge objects are not equal."""
        ...

@final
class TriggerInputOutputCapabilities:
    """Specifies which of the up to 8 supported digital lines can be used for
    either input or output triggering.

    The 2 attributes, input and output, each are read-only values and 8-bit
    integers from the conversion of the 8 binary digit representation of the
    digital lines to a decimal integer.

    Attributes:
        input:
            8-bit integer representing which digital lines can be used for
            input triggering. For example, if lines 0 and 2 were available for
            input triggers, the 8 binary digit representation of the lines is
            00000101, which is 5 in the decimal system.
        output:
            8-bit integer representing which digital lines can be used for
            output triggering. For example, if lines 3 and 5 were available for
            output triggers, the 8 binary digit representation of the lines is
            00101000, which is 40 in the decimal system.

    Examples:
        If lines 0 and 2 were available for input triggers, the 8 binary
        digit representation would be 0b00000101, since the 8 available
        lines are zero indexed. 00000101 binary is 5 in the decimal system,
        so the input attribute would have a value of 5.
    """

    input: int
    output: int

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of a `TriggerInputOutputCapabilities` object's
        attributes.
        """
        ...

@final
class VideoFrame:
    """The `VideoFrame` class represents data from acquisition of a frame."""

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
        """Returns a dictionary of the `VideoFrameMetadata` object's attributes."""

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
        """Returns a dictionary of a `VideoFrameTimestamps` object's
        attributes.
        """
        ...

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
        """Returns a dictionary of the `VideoStream` object's attributes."""

@final
class VideoStreamCapabilities:
    """The `VideoStreamCapabilities` class captures the capabilities for a
    video stream.

    Attributes:
        camera:
            An instance of the CameraCapabilities class which represents the
            capabilities for the camera in this video stream.
        storage:
            An instance of the StorageCapabilities class which represents the
            capabilities for the storage device in this video stream.
        max_frame_count:
            An instance of the Property class.
        frame_average_count:
            An instance of the Property class.
    """

    camera: CameraCapabilities
    storage: StorageCapabilities
    max_frame_count: Property
    frame_average_count: Property

    def dict(self) -> Dict[str, Any]:
        """Returns a dictionary of a `VideoStreamCapabilities` object's
        attributes.
        """
        ...

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
        """Initializes a VoltageRange object."""
        ...
    @overload
    def __init__(self, mn: float, mx: float) -> None:
        """Initializes a VoltageObject object with mn and mx provided."""
        ...
    def dict(self) -> Dict[str, float]:
        """Returns a dictionary of the `VoltageRange` object's attributes."""
        ...

def core_api_version() -> str:
    """Returns the version string for the core API."""
    ...
