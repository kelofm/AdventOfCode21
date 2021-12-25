SQRT = lambda c:(lambda V:[[v,V.append((v+c/v)/2) if abs(v*v-c)>1e-9 else None][0] for v in V ][-1])([1])
SUMMIN = lambda b:int(-(-(SQRT(1+8*b)-1)//2))

print(SUMMIN(20))
print((lambda v:v*(v+1)//2)((lambda b,I:int(I[b:b+I[b:].find('.')])-1)(*(lambda I:(I.find("y=-")+3,I))(open("input").read()))))