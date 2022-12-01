print(sum(sum(len(v) in (2,3,4,7) for v in p[1]) for p in [[sum(([v.strip()] for v in p.split(' ')),[]) for p in l.split(" | ")] for l in open("input").readlines() if l]))
