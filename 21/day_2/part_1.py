print((lambda c: c.real*c.imag)(sum({"f": lambda v: v, "d": lambda v: v*1j, "u": lambda v: -v*1j}[l[0]](int(l.strip()[l.rfind(' ')+1:])) for l in open("input",'r').readlines())))
