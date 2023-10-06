import doctest
import pytest

from pathlib import Path


@pytest.fixture(autouse=True)
def base_dir():
    return Path(__file__).parent.parent


def test_readme(base_dir):
    readme_path = base_dir / "README.md"
    assert (
        doctest.testfile(str(readme_path), module_relative=False).failed == 0
    )


def test_modules(base_dir):
    for module in base_dir.glob("python/**/*.py"):
        assert doctest.testfile(str(module), module_relative=False).failed == 0


def test_rust_sources(base_dir):
    for f in base_dir.glob("src/**/*.rs"):
        assert doctest.testfile(str(f), module_relative=False).failed == 0


if __name__ == "__main__":
    base = Path(__file__).parent.parent
    test_readme(base)
    test_modules(base)
    print("Done.")
