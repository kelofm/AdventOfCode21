print((lambda l,m:sum(abs(v-m) for v in l))(*(lambda l:(l,l[len(l)//2]))(sorted([int(v) for v in open("input").read().split(',')]))))
