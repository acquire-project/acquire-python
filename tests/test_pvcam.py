import acquire
import pytest
from acquire import DeviceKind


@pytest.fixture(scope="module")
def _runtime():
    runtime = acquire.Runtime()
    yield runtime


@pytest.fixture(scope="function")
def runtime(_runtime: acquire.Runtime):
    yield _runtime
    _runtime.set_configuration(acquire.Properties())


def test_prime_bsi_camera_is_present(runtime: acquire.Runtime):
    dm = runtime.device_manager()
    assert dm.select(DeviceKind.Camera, ".*BSI.*")
