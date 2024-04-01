from typing import Iterator


def bits_from_file_end_first(file_path) -> Iterator[bool]:
    """
    Yields all the bits from a file, last bit first
    :param file_path: path to the file from display/
    :return: bits
    """
    with open(file_path, "br") as f:
        bits = []
        for _ in range(17 ** 2):
            bits.append(f.read(1) == b'1')

    for bit in reversed(bits):
        yield bit
