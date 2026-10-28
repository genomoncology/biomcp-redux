"""Check Windows candidate imports without relying on a runner's installed DLLs."""
import re


def imports(data):
    """Read x64 PE import and delay-import descriptors (Microsoft PE format)."""
    def read(offset, size):
        if offset < 0 or offset + size > len(data):
            raise ValueError('invalid Windows PE: truncated data')
        return data[offset:offset + size]

    def number(offset, size=4):
        return int.from_bytes(read(offset, size), 'little')

    if read(0, 2) != b'MZ':
        raise ValueError('invalid Windows PE: missing DOS signature')
    header = number(0x3c)
    if read(header, 4) != b'PE\0\0' or number(header + 4, 2) != 0x8664:
        raise ValueError('invalid Windows PE: expected x64 executable')
    optional = header + 24
    optional_size = number(header + 20, 2)
    read(optional, optional_size)
    if optional_size < 112 or number(optional, 2) != 0x20b:
        raise ValueError('invalid Windows PE: expected PE32+ header')
    count = number(optional + 108)
    if count > (optional_size - 112) // 8:
        raise ValueError('invalid Windows PE: truncated data directories')
    sections = []
    for index in range(number(header + 6, 2)):
        section = optional + optional_size + index * 40
        read(section, 40)
        sections.append((number(section + 12), number(section + 16), number(section + 20)))

    def offset(rva, size):
        for address, length, start in sections:
            if address <= rva and rva + size <= address + length:
                result = start + rva - address
                read(result, size)
                return result
        raise ValueError('invalid Windows PE: unmapped import address')

    names = []
    for directory, width, name_offset in ((1, 20, 12), (13, 32, 4)):
        if directory >= count:
            continue
        rva = number(optional + 112 + directory * 8)
        length = number(optional + 116 + directory * 8)
        if not rva and not length:
            continue
        if not rva or length < width:
            raise ValueError('invalid Windows PE: invalid import directory')
        for position in range(0, length - width + 1, width):
            descriptor = offset(rva + position, width)
            if not any(read(descriptor, width)):
                break
            if directory == 13 and number(descriptor) != 1:
                raise ValueError('invalid Windows PE: unsupported delay import attributes')
            name = number(descriptor + name_offset)
            encoded = bytearray()
            for index in range(256):
                char = read(offset(name + index, 1), 1)
                if char == b'\0':
                    break
                encoded.extend(char)
            else:
                raise ValueError('invalid Windows PE: unterminated DLL name')
            try:
                names.append(encoded.decode('ascii'))
            except UnicodeDecodeError as error:
                raise ValueError('invalid Windows PE: non-ASCII DLL name') from error
            if not names[-1]:
                raise ValueError('invalid Windows PE: empty DLL name')
        else:
            raise ValueError('invalid Windows PE: unterminated import directory')
    return names


def check_runtime(data, platform):
    if not platform.startswith('win_'):
        return
    # Windows supplies UCRT itself; these DLL families require the VC redist.
    forbidden = [name for name in imports(data) if re.fullmatch(
        r'(?:vcruntime|msvcp|msvcr|concrt|vcomp|vcamp|vccorlib)\d+[^/\\]*\.dll', name, re.IGNORECASE)]
    if forbidden:
        raise ValueError('Windows binary requires Visual C++ redistributable: ' + ', '.join(forbidden))
