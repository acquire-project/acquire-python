[![Build](https://github.com/acquire-project/acquire-python/actions/workflows/build.yml/badge.svg)](https://github.com/acquire-project/acquire-python/actions/workflows/build.yml)
[![Test](https://github.com/acquire-project/acquire-python/actions/workflows/test_pr.yml/badge.svg)](https://github.com/acquire-project/acquire-python/actions/workflows/test_pr.yml)
[![DOI](https://zenodo.org/badge/632689876.svg)](https://zenodo.org/badge/latestdoi/632689876)
[![Chat](https://img.shields.io/badge/zulip-join_chat-brightgreen.svg)](https://acquire-imaging.zulipchat.com/)

# Acquire (`acquire-imaging` on PyPI)

Acquire ([`acquire-imaging` on PyPI](https://pypi.org/project/acquire-imaging/)) provides high-speed, multi-camera, video streaming for up to **2** cameras and image acquisition with a programming interface for streaming video data directly to Python, cloud-friendly file formats, and visualization platforms, such as [napari](https://napari.org/stable/).

> **Note** This is an early stage project. If you find it interesting please
> reach out!

Acquire supports the following cameras (currently only on Windows):

- [Hamamatsu Orca Fusion BT (C15440-20UP)](https://www.hamamatsu.com/eu/en/product/cameras/cmos-cameras/C15440-20UP.html)
- [Vieworks VC-151MX-M6H00](https://www.visionsystech.com/products/cameras/vieworks-vc-151mx-sony-imx411-sensor-ultra-high-resolution-cmos-camera-151-mp)
- [FLIR Blackfly USB3 (BFLY-U3-23S6M-C)](https://www.flir.com/products/blackfly-usb3/?model=BFLY-U3-23S6M-C&vertical=machine+vision&segment=iis)
- [FLIR Oryx 10GigE (ORX-10GS-51S5M-C)](https://www.flir.com/products/oryx-10gige/?model=ORX-10GS-51S5M-C&vertical=machine+vision&segment=iis)

Acquire also supports the following output file formats:

- [Tiff](https://en.wikipedia.org/wiki/TIFF)
- [Zarr](https://zarr.dev/)

For testing and demonstration purposes, Acquire provides a few simulated video sources.

## Installation

```bash
python -m pip install acquire-imaging
```

## Usage

Check out our documentation [here](https://acquire-project.github.io/acquire-docs/). 

The provided [napari](https://napari.org/stable/) plugin ([code here](https://github.com/acquire-project/acquire-python/blob/main/python/acquire/__init__.py#L131)) is a good example of how to stream for visualization.

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

**Important** When updating the 'acquire-video-runtime' (the c api), you need to manually trigger
a rebuild by touching `wrapper.h`.

```bash
git submodule update # updates acquire-video-runtime
touch wrapper.h # will trigger a rebuild
python -m build
```

This package depends on a submodule ([acquire-video-runtime](https://github.com/acquire-project/acquire-video-runtime))
and binaries from the following Acquire drivers:
- [acquire-driver-common](https://github.com/acquire-project/acquire-driver-common)
- [acquire-driver-hdcam](https://github.com/acquire-project/acquire-driver-hdcam)
- [acquire-driver-egrabber](https://github.com/acquire-project/acquire-driver-egrabber)
- [acquire-driver-zarr](https://github.com/acquire-project/acquire-driver-zarr)
- [acquire-driver-spinnaker](https://github.com/acquire-project/acquire-driver-spinnaker)

The build script will automatically try to fetch the binaries from GitHub releases.
In order to configure which release of each driver to use, you can set the value in `drivers.json`:

```json
{
  "acquire-driver-common": "0.1.0",
  "acquire-driver-hdcam": "0.1.0",
  "acquire-driver-egrabber": "0.1.0",
  "acquire-driver-zarr": "0.1.0",
  "acquire-driver-spinnaker": "0.1.0"
}
```

These values can be set to a specific version, or to `nightly` for nightly builds.

## Develop

```bash
pip install -e ".[testing]"
pytest -s --tb=short --log-cli-level=0
```

This project uses [`pre-commit`](https://pre-commit.com/) to run required
checks as git hooks.

```bash
pip install pre-commit
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

### Zarr V3 tests are failing

You should make sure that the following environment variables are set:

```
ZARR_V3_EXPERIMENTAL_API: 1
ZARR_V3_SHARDING: 1
```

[napari]: https://github.com/napari/napari
