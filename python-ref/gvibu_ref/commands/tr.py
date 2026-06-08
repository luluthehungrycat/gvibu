import sys


def expand_set(s: str) -> list[int]:
    result = []
    i = 0
    while i < len(s):
        if i + 2 < len(s) and s[i + 1] == "-" and ord(s[i]) < ord(s[i + 2]):
            for c in range(ord(s[i]), ord(s[i + 2]) + 1):
                result.append(c)
            i += 3
        elif s[i] == "\\" and i + 1 < len(s):
            esc = {"n": "\n", "t": "\t", "r": "\r", "\\": "\\", "0": "\0"}
            result.append(ord(esc.get(s[i + 1], s[i + 1])))
            i += 2
        else:
            result.append(ord(s[i]))
            i += 1

    # Deduplicate preserving order
    seen = set()
    return [c for c in result if not (c in seen or seen.add(c))]


def build_char_set(s: str, complement: bool) -> set[int]:
    chars = set(expand_set(s))
    if complement:
        all_chars = set(range(256))
        return all_chars - chars
    return chars


def run(args: list[str]) -> int:
    delete = False
    squeeze = False
    complement = False
    sets = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "-d":
            delete = True
        elif arg == "-s":
            squeeze = True
        elif arg in ("-c", "-C"):
            complement = True
        elif arg.startswith("-") and len(arg) > 1:
            print(f"tr: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            sets.append(arg)
        i += 1

    if (delete and len(sets) < 1) or (not delete and len(sets) < 2):
        print("tr: missing operand", file=sys.stderr)
        return 1

    set1 = build_char_set(sets[0] if sets else "", complement)

    if delete:
        delete_set = set1
        translate_map = {}
    else:
        set2_list = expand_set(sets[1] if len(sets) > 1 else "")
        if not set2_list and set1:
            set2_list = [sorted(set1)[-1]] * len(set1)
        translate_map = {}
        for idx, c in enumerate(sorted(set1)):
            translate_map[c] = set2_list[idx % len(set2_list)] if set2_list else c
        delete_set = set()

    input_data = sys.stdin.buffer.read()
    output = bytearray()
    prev = None

    for byte in input_data:
        if byte in delete_set:
            continue

        translated = translate_map.get(byte, byte)

        if squeeze and translated == prev:
            continue

        prev = translated
        output.append(translated)

    sys.stdout.buffer.write(output)
    return 0
