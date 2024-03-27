def bits_from_file_end_first(file_path):
    with open(file_path, "br") as f:
        bits = []
        for _ in range(17 ** 2):
            bits.append(f.read(1) == b'1')

    for bit in reversed(bits):
        yield bit
