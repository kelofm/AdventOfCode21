print((lambda l,m:sum((lambda d:d*(d+1)//2)(abs(v-m)) for v in l))(*(lambda l:(l,round(sum(l)/len(l)-0.5)))([int(v) for v in open("input").read().split(',')])))
