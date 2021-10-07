
with open('./opcodes_dirty') as f:
    data = f.read()
    lines = [line.split('\t') for line in data.splitlines()]

opcodes = [line[1] for line in lines]
sizes = [line[2] for line in lines]

d8 = '0aaH'
bytes8 = d8[1:-1]
d16 = '0ccccH'
bytes16 = d16[3:5] + d16[1:3]
print(bytes8)
print(bytes16)

opcodes = list(map(lambda x: x.replace('D8', d8).replace('D16', d16).replace('adr', d16), opcodes))

testdata = []
for opcode, (instruction, size) in enumerate(zip(opcodes, sizes)):
    ml = format(opcode, '02x')
    if size == '2':
        ml += bytes8
    elif size == '3':
        ml += bytes16
    testdata.append((int(ml,  16), instruction))
    
with open('./test_input', 'w') as f:
    for sample in testdata:
        f.write(f"{sample[0]}:{sample[1]}\n")
