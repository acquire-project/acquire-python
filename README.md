# cpx-python

Python interface for calliphlox.

## Build environment

Requires
* CMake 3.23+ ([download page](https://cmake.org/download/) or via [chocolatey](https://community.chocolatey.org/packages/cmake))
* A C++ compiler (Microsoft Visual Studio Community [download page](https://visualstudio.microsoft.com/downloads/)) 
* Rust (via rustup, see [install page](https://www.rust-lang.org/tools/install))
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

## LESSONS/TODO

- Should've implemented serialize and deserialize for the c api objects instead
  of python objects (recommendation: rework later)
    - Then dict to python looks like: dict <-> c api <-> python object
    - More naturally reuse the serde code for format support
    - serde chokes on the python types bc they erase types which makes the code
      more complicated.
