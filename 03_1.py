# Parse binary strings as decimal integers
ints = [int(l,2) for l in open("input")]

# Compute most common bits (gamma)
gamma = sum(1 << p for p in range(12) if len(ints)/2.0 < sum(1 for n in ints if n&(1<<p)))

# Gamma's unsigned complement (epsilon)
epsilon = gamma ^ (1 << (gamma-1).bit_length())-1

# Result
print(gamma * epsilon)

# One-liner
#print((lambda g: g * (g ^ (1 << (g-1).bit_length())-1))((lambda l: sum(1<<p for p in range(12) if len(l)/2 < sum(1 for n in l if n&(1<<p))))([int(s,2) for s in open("input")])))
