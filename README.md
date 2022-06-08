# cpx-python

Python interface for calliphlox.

## Build environment

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

```ipython
In [1]: import calliphlox

In [2]: calliphlox.Trigger(enable=True,line=0,event="AcquisitionStart",kind="Input",edge="Rising")
Out[2]: Trigger(enable='True',line='0',event='AcquisitionStart',kind='Input',edge='Rising')
```
