"""sort: sort lines of text files.
Supports: -r (reverse), -n (numeric), -u (unique), -f (fold case),
-k POS1[,POS2] (key sort with optional n/r modifiers)
"""
import sys


def _read_lines(files: list[str]) -> list[str]:
    lines = []
    if not files:
        lines = sys.stdin.read().splitlines(keepends=False)
    else:
        for fname in files:
            if fname == '-':
                lines.extend(sys.stdin.read().splitlines(keepends=False))
            else:
                try:
                    with open(fname) as f:
                        lines.extend(f.read().splitlines(keepends=False))
                except OSError as e:
                    print(f"sort: {fname}: {e}", file=sys.stderr)
    return lines


def _numeric_key(s: str) -> tuple[bool, float]:
    s = s.strip()
    if not s:
        return (False, 0.0)
    i = 0
    if i < len(s) and s[i] == '-':
        i += 1
    while i < len(s) and s[i].isdigit():
        i += 1
    if i < len(s) and s[i] == '.':
        i += 1
        while i < len(s) and s[i].isdigit():
            i += 1
    if i == 0:
        return (False, 0.0)
    try:
        return (True, float(s[:i]))
    except ValueError:
        return (False, 0.0)


def _parse_key_spec(spec: str) -> dict | None:
    """Parse a key spec like '2', '2,4', '2.3', '2.3,4.5', '2n', '2nr'"""
    spec_str = spec
    numeric = False
    reverse = False

    # Extract trailing modifiers
    while spec_str:
        ch = spec_str[-1]
        if ch == 'n':
            numeric = True
            spec_str = spec_str[:-1]
        elif ch == 'r':
            reverse = True
            spec_str = spec_str[:-1]
        else:
            break

    if not spec_str:
        return None

    parts = spec_str.split(',')
    if not parts or not parts[0]:
        return None

    # Parse POS1: field[.char]
    pos1_parts = parts[0].split('.')
    try:
        field1 = int(pos1_parts[0])
    except ValueError:
        return None
    if field1 < 1:
        return None
    char1 = int(pos1_parts[1]) if len(pos1_parts) > 1 else 1

    # Parse optional POS2
    field2 = None
    char2 = None
    if len(parts) > 1 and parts[1]:
        pos2_parts = parts[1].split('.')
        try:
            f2 = int(pos2_parts[0])
        except ValueError:
            return None
        if f2 < 1:
            return None
        field2 = f2
        char2 = int(pos2_parts[1]) if len(pos2_parts) > 1 else 1

    return {
        'field1': field1,
        'char1': char1,
        'field2': field2,
        'char2': char2,
        'numeric': numeric,
        'reverse': reverse,
    }


def _split_fields(line: str) -> list[str]:
    """Split into whitespace-separated fields (like GNU sort default)."""
    return line.split()


def _extract_key(line: str, spec: dict) -> str:
    """Extract the key portion of a line according to a KeySpec dict."""
    fields = _split_fields(line)

    if not fields:
        return ''

    start_field = min(spec['field1'] - 1, len(fields) - 1)
    if start_field >= len(fields):
        return ''

    end_field = spec['field2']
    if end_field is not None:
        end_field = min(end_field - 1, len(fields) - 1)

    # Single field case
    if end_field is None or end_field == start_field:
        field = fields[start_field]
        start_char = min(spec['char1'] - 1, len(field))
        if spec['char2'] is not None and end_field is not None:
            limit = min(spec['char2'] - 1, len(field))
        else:
            limit = len(field)
        if start_char >= limit:
            return ''
        return field[start_char:limit]

    # Multiple field case
    end = end_field or (len(fields) - 1)
    parts = []
    for idx in range(start_field, end + 1):
        f = fields[idx]
        if idx == start_field:
            start_char = min(spec['char1'] - 1, len(f))
            parts.append(f[start_char:])
        elif idx == end and spec['char2'] is not None:
            limit = min(spec['char2'] - 1, len(f))
            parts.append(f[:limit])
        else:
            parts.append(f)
    return ' '.join(parts)


def run(args: list[str]) -> int:
    reverse = False
    numeric = False
    unique = False
    fold_case = False
    key_specs: list[dict] = []
    files = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == '-r':
            reverse = True
        elif arg == '-n':
            numeric = True
        elif arg == '-u':
            unique = True
        elif arg == '-f':
            fold_case = True
        elif arg == '-k':
            i += 1
            if i >= len(args):
                print("sort: option requires an argument: -k", file=sys.stderr)
                return 1
            ks = _parse_key_spec(args[i])
            if ks is None:
                print(f"sort: invalid key specification: {args[i]}", file=sys.stderr)
                return 1
            key_specs.append(ks)
        elif arg.startswith('-') and len(arg) > 1:
            combined_key = False
            for ch in arg[1:]:
                if ch == 'r':
                    reverse = True
                elif ch == 'n':
                    numeric = True
                elif ch == 'u':
                    unique = True
                elif ch == 'f':
                    fold_case = True
                elif ch == 'k':
                    # Combined like -k2
                    spec = arg[arg.index('k') + 1:]
                    if not spec:
                        print("sort: option requires an argument: -k", file=sys.stderr)
                        return 1
                    ks = _parse_key_spec(spec)
                    if ks is None:
                        print(f"sort: invalid key specification: {spec}", file=sys.stderr)
                        return 1
                    key_specs.append(ks)
                    combined_key = True
                    break
                else:
                    print(f"sort: invalid option: -{ch}", file=sys.stderr)
                    return 1
            if combined_key:
                break
        else:
            files.append(arg)
        i += 1

    lines = _read_lines(files)

    def make_sort_key(line: str) -> tuple:
        """Build a sort key tuple: (key_values..., whole_line)"""
        keys = tuple(_extract_key(line, ks) for ks in key_specs)
        return keys + (line,)

    # Sort with key
    def cmp_key(item: str) -> list:
        """Return a list of comparison keys for the item."""
        keys = []
        for ks in key_specs:
            k = _extract_key(item, ks)
            use_numeric = ks['numeric'] or numeric
            if use_numeric:
                has_num, val = _numeric_key(k)
                if has_num:
                    keys.append((0, val))
                else:
                    keys.append((1, 0.0))
            else:
                keys.append((2, k.lower() if fold_case else k))
        # Whole line tiebreaker
        keys.append((3, line.lower() if fold_case else line))
        return keys

    if key_specs:
        lines.sort(key=cmp_key, reverse=reverse)
    else:
        def sort_key(line: str):
            if numeric:
                has_num, val = _numeric_key(line)
                if has_num:
                    return (0, val, '')
                else:
                    return (1, 0.0, line.lower() if fold_case else line)
            key = line.lower() if fold_case else line
            return (0, 0.0, key)

        lines.sort(key=sort_key, reverse=reverse)

    if unique:
        deduped = []
        for line in lines:
            if not deduped or line != deduped[-1]:
                deduped.append(line)
        lines = deduped

    for line in lines:
        print(line)

    return 0
