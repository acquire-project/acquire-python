import doctest

from pathlib import Path


def test_readme():
    readme_path = Path(__file__).parent.parent / "README.md"
    assert (
        doctest.testfile(str(readme_path), module_relative=False).failed == 0
    )


def test_modules():
    assert doctest.testmod().failed == 0


if __name__ == "__main__":
    test_readme()
    test_modules()
    print("Done.")
