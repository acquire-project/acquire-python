from calliphlox import Trigger;

def test_set():
    t=Trigger()
    assert t.enable==False
    t.enable=True
    assert t.enable==True