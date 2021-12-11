print((lambda g: g * (g ^ (1 << (g-1).bit_length())-1))((lambda l: sum(1<<p for p in range(12) if len(l)/2 < sum(1 for n in l if n&(1<<p))))([int(s,2) for s in open("input")])))
