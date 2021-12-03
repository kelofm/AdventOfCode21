# Parse input to command char - value pairs
commands = [[l[0],int(l.strip()[l.rfind(' ')+1:])] for l in open("input").readlines()]

# Compute aim at "forward" commands
# and store it along with the forward value
forwardCommands = [[sum(v[1] for v in commands[:I] if v[0]=='d')-sum(v[1] for v in commands[:I] if v[0]=='u'),V[1]] for I,V in enumerate(commands) if V[0]=='f']

# Execute "forward" commands
# Position represented on the complex plane:
#   real part: depth
#   imag part: horizontal distance
position = sum(pair[0]*pair[1] + pair[1]*1j for pair in forwardCommands)

# Result
print(position.real * position.imag)

# One-liner
#print((lambda c: c.real*c.imag)((lambda l: sum(v[0]*v[1]+v[1]*1j for v in l))((lambda l: [[sum(v[1] for v in l[:I] if v[0]=='d')-sum(v[1] for v in l[:I] if v[0]=='u'),V[1]] for I,V in enumerate(l) if V[0]=='f'])([[l[0],int(l.strip()[l.rfind(' ')+1:])] for l in open("input").readlines()]))))
