![Build](https://github.com/calliphlox/cpx-python/actions/workflows/CI.yml/badge.svg)

# cpx-python

Python interface for calliphlox.

## Build environment

Requires

- CMake 3.23+ ([download page](https://cmake.org/download/) or via
  [chocolatey](https://community.chocolatey.org/packages/cmake))
- A C++ compiler (Microsoft Visual Studio Community [download
  page](https://visualstudio.microsoft.com/downloads/))
- Rust (via rustup, see [install
  page](https://www.rust-lang.org/tools/install))
- conda (optional; via
  [miniconda](https://docs.conda.io/en/latest/miniconda.html))
- libclang >= v5.0 (on windows via [choco](https://chocolatey.org/) `choco
  install llvm` or, on osx, via [brew](https://brew.sh/) `brew install llvm`)

```
conda create --name calliphlox python=3.10
conda activate calliphlox
```

## Build

```bash
git submodule update --init --recursive
pip install build
python -m build
```

**Important** When updating the 'cpx' (the c api), to need to manually trigger
a rebuild by touching `wrapper.h`.

```bash
git submodule update # updates cpx
touch wrapper.h # will trigger a rebuild
python -m build
```

## Develop

```bash
pip install -e '.[testing]'
pytest
```

Example use in a python console:

```pycon
>>> import calliphlox
>>> calliphlox.Trigger(enable=True,line=0,event="AcquisitionStart",kind="Input",edge="Rising")
Trigger(enable='True',line='0',event='AcquisitionStart',kind='Input',edge='Rising')
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

### Working with an editable install, how do I update the build?

It depends on what you changed:

- **cpx** (c/c++ code): `touch wrapper.h; maturin develop`
- **rust code**: `maturin develop`
- **python code**: nothing
