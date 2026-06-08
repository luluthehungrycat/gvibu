"""join: join lines of two files on a common field."""


def run(args: list[str]) -> int:
    field = 1
    files: list[str] = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "-j":
            i += 1
            if i >= len(args):
                print("join: option requires an argument: -j", file=__import__("sys").stderr)
                return 1
            try:
                n = int(args[i])
                if n <= 0:
                    raise ValueError
                field = n
            except ValueError:
                print(f"join: invalid field number: {args[i]}", file=__import__("sys").stderr)
                return 1
        elif arg.startswith("-") and len(arg) > 1:
            print(f"join: invalid option: {arg}", file=__import__("sys").stderr)
            return 1
        else:
            files.append(arg)
        i += 1

    if len(files) < 2:
        print("join: missing operand", file=__import__("sys").stderr)
        return 1

    file1_path = files[0]
    file2_path = files[1]
    field_idx = field - 1

    try:
        with open(file2_path) as f2:
            lines2 = f2.readlines()
    except OSError as e:
        print(f"join: {file2_path}: {e}", file=__import__("sys").stderr)
        return 1

    map2: dict[str, list[list[str]]] = {}
    for line in lines2:
        fields = line.strip().split()
        if field_idx < len(fields):
            key = fields[field_idx]
            map2.setdefault(key, []).append(fields)

    try:
        with open(file1_path) as f1:
            for line in f1:
                fields = line.strip().split()
                if field_idx >= len(fields):
                    continue
                key = fields[field_idx]
                if key in map2:
                    for m in map2[key]:
                        parts = [key]
                        for j, f in enumerate(fields):
                            if j != field_idx:
                                parts.append(f)
                        for j, f in enumerate(m):
                            if j != field_idx:
                                parts.append(f)
                        print(" ".join(parts))
    except OSError as e:
        print(f"join: {file1_path}: {e}", file=__import__("sys").stderr)
        return 1

    return 0
