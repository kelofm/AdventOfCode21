print((lambda d: sum(True for l, r in zip(d[:-1],d[1:]) if l<r))([float(item) for item in open("input").read().strip().split('\n')]))
