def bits_from_file(file_path):
    with open(file_path, "br") as f:
        bytes = f.read()
    print(bytes)
    for b in bytes:
        for i in range(8):
            yield (b >> i) & 1