[![Build](https://github.com/acquire-project/acquire-python/actions/workflows/build.yml/badge.svg)](https://github.com/acquire-project/acquire-python/actions/workflows/build.yml)
[![Test](https://github.com/acquire-project/acquire-python/actions/workflows/test_pr.yml/badge.svg)](https://github.com/acquire-project/acquire-python/actions/workflows/test_pr.yml)

# acquire-imaging

This python package provides a multi-camera video streaming library focusing
on performant microscopy.

> **Note** This is an early stage project. If you find it interesting please
> reach out!

Support for:

- Up to two independent video streams
- Camera support:
  - Hamamatsu Orca Fusion BT (C15440-20UP) (windows only)
  - Vieworks VC-151MX-M6H00
- Streaming file format support:
  - Tiff
  - Zarr v2

## Installation

```bash
python -m pip install acquire-imaging
```

## Usage

See the tests for more examples.

The provided [napari][] plugin is a good example of how to stream for visualization.

### List devices

```python
import acquire
print(acquire.Runtime().device_manager().devices())
```

### Finite triggered acquisition

```python
import acquire
runtime=acquire.Runtime()
dm=runtime.device_manager()

props=runtime.get_configuration()
# select the first Hamamatsu camera
props.video[0].camera.identifier = dm.select(DeviceKind.Camera, "hamamatsu.*")
# stream to zarr
props.video[0].storage.identifier = dm.select(DeviceKind.Storage, "zarr")
props.video[0].storage.settings.filename="out.zarr"
props.video[0].camera.settings.shape = (2304, 2304)
props.video[0].camera.settings.pixel_type = SampleType.U16
props.video[0].max_frame_count = 100
props=runtime.configure(props)

runtime.start()
runtime.stop() # wait for acquisition to complete
```

# Development

We welcome contributors. The following will help you get started building the
code.

## Environment

Requires

- CMake 3.23+ ([download page](https://cmake.org/download/) or via
  [chocolatey](https://community.chocolatey.org/packages/cmake))
- A C++20 compiler (Microsoft Visual Studio Community [download
  page](https://visualstudio.microsoft.com/downloads/), or clang)
- Rust (via rustup, see [install
  page](https://www.rust-lang.org/tools/install))
- conda (optional; via
  [miniconda](https://docs.conda.io/en/latest/miniconda.html))
- libclang >= v5.0 (on windows via [choco](https://chocolatey.org/) `choco
  install llvm` or, on osx, via [brew](https://brew.sh/) `brew install llvm`)

It's strongly recommended you create a python environment for development

```bash
conda create --name acquire python=3.11
conda activate acquire
```

## Build

```bash
conda activate acquire
git submodule update --init --recursive
pip install maturin
maturin build -i python
```

**Important** When updating the 'acquire-video-runtime' (the c api), to need to manually trigger
a rebuild by touching `wrapper.h`.

```bash
git submodule update # updates acquire-video-runtime
touch wrapper.h # will trigger a rebuild
python -m build
```

## Develop

```bash
pip install -e ".[testing]"
pytest -s --tb-short --log-cli-level=0
```

This project uses [`pre-commit`](https://pre-commit.com/) to run required
checks as git hooks.

```bash
pip install precommit
pre-commit install
```

## Troubleshooting

### Maturin can't find a python interpreter

`Maturin` is a command line tool associated with
[`pyo3`](https://pyo3.rs/v0.16.4/). It helps automate the build and packaging
process. It's invoked by `setuptools` during a build.

1. Double-check you've activated the right conda environment.
2. Try `maturin build -i python`

This seems to happen on windows in anaconda environments when multiple python
interpreters are available on the path.

It seems to happen less frequently when invoked via pip - `pip install -e .`
will end up invoking maturin.

### Working with an editable install, how do I update the build?

It depends on what you changed:

- **acquire-video-runtime** (c/c++ code): `touch wrapper.h; maturin develop`
- **rust code**: `maturin develop`

[napari]: https://github.com/napari/napari
