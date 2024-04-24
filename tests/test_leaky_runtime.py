import acquire


def test_leaky_runtime():
    exposure_time_ms = 12
    nframes = 500

    for _ in range(3):
        runtime = acquire.Runtime()

        dm = runtime.device_manager()
        props = runtime.get_configuration()

        # configure camera
        props.video[0].camera.identifier = dm.select(
            acquire.DeviceKind.Camera, ".*empty.*"
        )
        props.video[0].camera.settings.exposure_time_us = (
            exposure_time_ms * 1000
        )
        props.video[0].camera.settings.offset = (0, 0)
        props.video[0].camera.settings.shape = (2304, 2304)
        props.video[0].camera.settings.pixel_type = acquire.SampleType.U16
        props.video[0].camera.settings.binning = 1

        # configure acquisition
        props.video[0].max_frame_count = nframes
        props.video[0].frame_average_count = 0

        # configure storage
        props.video[0].storage.identifier = dm.select(
            acquire.DeviceKind.Storage, "trash"
        )

        props = runtime.set_configuration(props)
        runtime.start()
        runtime.stop()

        del runtime
