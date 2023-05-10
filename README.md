[![Build](https://github.com/acquire-project/acquire-python/actions/workflows/build.yml/badge.svg)](https://github.com/acquire-project/acquire-python/actions/workflows/build.yml)
[![Test](https://github.com/acquire-project/acquire-python/actions/workflows/test_pr.yml/badge.svg)](https://github.com/acquire-project/acquire-python/actions/workflows/test_pr.yml)

# acquire-python

Python interface for Acquire.

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
conda create --name acquire python=3.11
conda activate acquire
```

The build script also looks for a GitHub personal access token in the environment variable `GH_TOKEN`.
You can set this on the command line, or in a .env file in the root directory of this repository.
Your .env file might look like, for example:

```dotenv
GH_TOKEN=[your token here]
```

## Build

```bash
git submodule update --init
cd acquire-video-runtime
git submodule update --init
pip install build
python -m build
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
pytest
```

Example use in a python console:

```pycon
>>> import acquire
>>> acquire.Trigger(enable=True,line=0,event="AcquisitionStart",kind="Input",edge="Rising")
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

It seems to happen less frequently when invoked via pip - `pip install -e .`
will end up invoking maturin.

### Working with an editable install, how do I update the build?

It depends on what you changed:

- **acquire-video-runtime** (c/c++ code): `touch wrapper.h; maturin develop`
- **rust code**: `maturin develop`
- **python code**: nothing
