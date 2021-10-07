
with open('./opcodes_dirty') as f:
    data = f.read()
    lines = [line.split('\t') for line in data.splitlines()]

opcodes = [line[1] for line in lines]
sizes = [line[2] if line[2] != '' else '1' for line in lines]

d8 = '0abH'
bytes8 = str(int(d8[1:-1], 16))
d16 = '0abcdH'
bytes16 = [str(int(d16[3:5], 16)), str(int(d16[1:3], 16))]

opcodes = list(map(lambda x: x.replace('D8', d8).replace('D16', d16).replace('adr', d16), opcodes))

testdata = []
for opcode, (instruction, size) in enumerate(zip(opcodes, sizes)):
    ml = str(opcode)
    if size == '2':
        ml += f",{bytes8}"
    elif size == '3':
        ml += f",{','.join(bytes16)}"
    testdata.append((size, ml, instruction))
    
with open('./test_input', 'w') as f:
    for sample in testdata:
        f.write(f"{sample[1]}:{sample[2]}\n")
