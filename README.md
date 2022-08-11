![Build](https://github.com/calliphlox/cpx-python/actions/workflows/CI.yml/badge.svg)
# cpx-python

Python interface for calliphlox.

## Build environment

Requires
* CMake 3.23+ ([download page](https://cmake.org/download/) or via [chocolatey](https://community.chocolatey.org/packages/cmake))
* A C++ compiler (Microsoft Visual Studio Community [download page](https://visualstudio.microsoft.com/downloads/)) 
* Rust (via rustup, see [install page](https://www.rust-lang.org/tools/install))
* conda (optional; via [miniconda](https://docs.conda.io/en/latest/miniconda.html))
* libclang >= v5.0 (on windows via [choco](https://chocolatey.org/) `choco install llvm` or, on osx, via [brew](https://brew.sh/) `brew install llvm`)

```
conda create --name calliphlox python=3.10
conda activate calliphlox
pip install maturin
```

`Maturin` is a command line tool associated with [`pyo3`](https://pyo3.rs/v0.16.4/). It 
helps automate the build and packaging process.

## Build

```bash
git submodule update --init --recursive
maturin build
```

**Important** When updating the 'cpx' (the c api), to need to manually trigger a 
rebuild by touching `wrapper.h`.

```bash
git submodule update # updates cpx
touch wrapper.h # will trigger a rebuild
maturin build
```

## Develop

```bash
maturin develop
ipython
```

```pycon
>>> import calliphlox
>>> calliphlox.Trigger(enable=True,line=0,event="AcquisitionStart",kind="Input",edge="Rising")
Trigger(enable='True',line='0',event='AcquisitionStart',kind='Input',edge='Rising')
```

## Troubleshooting

### Maturin can't find a python interpreter.

1. Double-check you've activated the right conda environment.
2. Try `maturin build -i python`

This seems to happen on windows in anaconda environments when multiple python interpreter is available on the path.
