import logging
import time
from time import sleep

import calliphlox
import pytest
from calliphlox import DeviceKind, Trigger


@pytest.fixture(scope="module")
def runtime():
    runtime = calliphlox.Runtime()
    yield runtime
    del runtime


def test_set():
    t = Trigger()
    assert not t.enable
    t.enable = True
    assert t.enable


def test_list_devices(caplog, runtime):
    caplog.set_level(logging.DEBUG)
    dm = runtime.device_manager()
    for d in dm.devices():
        print(d.dict())


def test_set_camera_identifier(caplog, runtime):
    caplog.set_level(logging.DEBUG)

    dm = runtime.device_manager()

    p = runtime.get_configuration()
    assert p.camera.identifier.kind == calliphlox.DeviceKind.NONE
    p.camera.identifier = dm.select(
        calliphlox.DeviceKind.Camera, "simulated: radial sin"
    )
    assert p.camera.identifier is not None


@pytest.mark.parametrize(
    "input,expected",
    [
        (["does not exist 1", "does not exist 2", "does not exist 3"], None),
        (
            [
                "does not exist 1",
                "simulated: radial sin",
                "simulated: uniform random",
            ],
            "simulated: radial sin",
        ),
        (["simulated: radial sin"], "simulated: radial sin"),
        ([], None),
    ],
)
def test_select_one_of(caplog, runtime, input, expected):
    h = runtime.device_manager().select_one_of(DeviceKind.Camera, input)
    result = None if h is None else h.name
    assert result == expected


def test_zero_conf_start(caplog, runtime):
    caplog.set_level(logging.DEBUG)
    with pytest.raises(RuntimeError):
        runtime.start()


def test_set_storage(caplog, runtime):
    caplog.set_level(logging.DEBUG)

    dm = runtime.device_manager()

    p = runtime.get_configuration()
    assert p.storage.identifier.kind == calliphlox.DeviceKind.NONE
    p.storage.identifier = dm.select(calliphlox.DeviceKind.Storage, "Tiff")
    assert p.storage.identifier is not None

    assert p.storage.settings.filename is None
    p.storage.settings.filename = "out.tif"
    assert p.storage.settings.filename == "out.tif"


def test_setup(caplog, runtime):
    caplog.set_level(logging.DEBUG)
    p = calliphlox.setup(runtime, "simulated: radial sin", "Trash")
    assert p.camera.identifier is not None
    assert p.storage.identifier is not None
    assert p.storage.settings.filename == "out.tif"
    p.camera.settings.shape = (1920, 1080)
    p = runtime.set_configuration(p)
    runtime.start()

    nframes = 0
    while nframes < p.max_frame_count:
        clock = time.time()
        if a := runtime.get_available_data():
            packet = a.get_frame_count()
            for f in a.frames():
                logging.info(
                    f"{f.data().shape} {f.data()[0][0][0][0]} {f.metadata()}"
                )
                f = None  # <-- fails to get the last frames if this is held?
            a = None  # <-- fails to get the last frames if this is held?
            nframes += packet
            logging.info(
                f"frame count: {nframes} - frames in packet: {packet}"
            )

        elapsed = time.time() - clock
        sleep(max(0, 0.1 - elapsed))
    logging.info("stopping")

    runtime.stop()


def test_selection_is_consistent(caplog, runtime):
    caplog.set_level(logging.DEBUG)
    hcam1 = runtime.device_manager().select(DeviceKind.Camera)
    hcam2 = runtime.device_manager().select(DeviceKind.Camera, hcam1.name)
    assert hcam1 == hcam2


# FIXME: (nclack) awkwardness around references  (available frames, f)
# TODO: (nclack) control log level from pytest invocation? use caplog default.
