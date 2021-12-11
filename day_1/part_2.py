print((lambda d: sum(True for l, r in zip(d[:-1],d[1:]) if l<r))((lambda d: [sum(t) for t in zip(d,d[1:],d[2:])])([float(item) for item in open("input").read().strip().split('\n')])))
