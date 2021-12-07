print((lambda f,l:sum(f(v,80) for v in l))((lambda R:lambda c,r:R(R,c,r))(lambda s,c,r:1 if r<c else 1+sum(s(s,9,q) for q in range(r-c,0,-7))),[int(v) for v in open("input").read().split(',')]))
