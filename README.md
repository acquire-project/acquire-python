# cpx-python

Python interface for calliphlox.

## Build environment

Requires
* [CMake 3.23+](https://cmake.org/download/)
* A C++ compiler
* [Rust](https://www.rust-lang.org/tools/install)
* conda (optional; via [miniconda](https://docs.conda.io/en/latest/miniconda.html))



```
conda create --name calliphlox python=3.10
conda activate calliphlox
pip install maturin
```

## Build

```bash
git submodule update --init --recursive
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
