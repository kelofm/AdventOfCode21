print((lambda s:sum(next((c for c in l if (s.append(c) if c<0 else s.pop()==0 if s and c==-s[-1] else True)),0) for l in (lambda m:((m[c] for c in l.strip()) for l in open("input").readlines() if l))({'(':-3,'[':-57,'{':-1197,'<':-25137,')':3,']':57,'}':1197,'>':25137})))([]))
