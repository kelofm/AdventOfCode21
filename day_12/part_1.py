print(sum(1 for p in (lambda R,N:[R,[[R.append(r+[n]) for n in N[r[-1]] if n.isupper() or not (n in r)] for r in R if r[-1]!="end"]][0])([["start"]],(lambda N,F:[F(a,b.strip(),N) for a,b in (tuple(l.split('-') for l in open("input") if l))][0])(dict(),lambda a,b,N:[N,N.setdefault(a,[]).append(b),N.setdefault(b,[]).append(a)][0]))if p[-1]=="end"))