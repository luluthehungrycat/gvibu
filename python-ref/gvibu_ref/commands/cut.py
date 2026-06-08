import sys


def parse_list(list_str: str) -> list[int] | None:
    indices = set()
    for part in list_str.split(","):
        part = part.strip()
        if not part:
            return None
        if "-" in part:
            start, end = part.split("-", 1)
            start = start.strip()
            end = end.strip()
            if not start or not end:
                return None
            try:
                s = int(start)
                e = int(end)
            except ValueError:
                return None
            if s == 0 or e == 0 or s > e:
                return None
            for i in range(s, e + 1):
                indices.add(i)
        else:
            try:
                n = int(part)
            except ValueError:
                return None
            if n == 0:
                return None
            indices.add(n)

    return sorted(indices)


def run(args: list[str]) -> int:
    delimiter = "\t"
    field_indices = None
    files = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "-d":
            i += 1
            if i >= len(args):
                print("cut: option requires an argument: -d", file=sys.stderr)
                return 1
            delimiter = args[i]
            if not delimiter:
                print("cut: invalid delimiter", file=sys.stderr)
                return 1
        elif arg == "-f":
            i += 1
            if i >= len(args):
                print("cut: option requires an argument: -f", file=sys.stderr)
                return 1
            result = parse_list(args[i])
            if result is None:
                print(f"cut: invalid field list: {args[i]}", file=sys.stderr)
                return 1
            field_indices = result
        elif arg.startswith("-") and len(arg) > 1:
            print(f"cut: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            files.append(arg)
        i += 1

    if field_indices is None:
        print("cut: you must specify a list of fields with -f", file=sys.stderr)
        return 1

    if not files:
        files = ["-"]

    for fname in files:
        if fname == "-":
            process_reader(sys.stdin, delimiter, field_indices)
        else:
            try:
                with open(fname) as f:
                    process_reader(f, delimiter, field_indices)
            except OSError as e:
                print(f"cut: {fname}: {e}", file=sys.stderr)
                return 1

    return 0


def process_reader(reader, delimiter: str, indices: list[int]) -> None:
    for line in reader:
        line = line.rstrip("\n")
        fields = line.split(delimiter)
        selected = [fields[i - 1] for i in indices if 1 <= i <= len(fields)]
        if selected:
            print(delimiter.join(selected))
