"""sort: sort lines of text files.
Supports: -r (reverse), -n (numeric), -V (version), -M (month),
-c (check), -u (unique), -f (fold case),
-k POS1[,POS2] (key sort with optional n/r modifiers)
"""
import sys
from functools import cmp_to_key



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
def _lexical_compare(a: str, b: str, fold_case: bool) -> int:
    if fold_case:
        a = a.lower()
        b = b.lower()
    return (a > b) - (a < b)


def _numeric_compare(a: str, b: str, fold_case: bool) -> int:
    a_has_num, a_num = _numeric_key(a)
    b_has_num, b_num = _numeric_key(b)
    if a_has_num and b_has_num:
        return (a_num > b_num) - (a_num < b_num)
    if a_has_num:
        return -1
    if b_has_num:
        return 1
    return _lexical_compare(a, b, fold_case)


_MONTH_NUMBERS = {
    'jan': 1,
    'feb': 2,
    'mar': 3,
    'apr': 4,
    'may': 5,
    'jun': 6,
    'jul': 7,
    'aug': 8,
    'sep': 9,
    'oct': 10,
    'nov': 11,
    'dec': 12,
}


def _month_number(s: str) -> int:
    return _MONTH_NUMBERS.get(s.lstrip()[:3].lower(), 0)


def _month_compare(a: str, b: str, fold_case: bool) -> int:
    a_month = _month_number(a)
    b_month = _month_number(b)
    if a_month != b_month:
        return (a_month > b_month) - (a_month < b_month)
    return _lexical_compare(a, b, fold_case)


def _version_char_compare(a: str, b: str) -> int:
    if a == b:
        return 0
    if a == '~':
        return -1
    if b == '~':
        return 1
    return (a > b) - (a < b)


def _version_compare(a: str, b: str, fold_case: bool) -> int:
    if fold_case:
        a = a.lower()
        b = b.lower()
    ai = 0
    bi = 0
    while ai < len(a) and bi < len(b):
        a_is_digit = '0' <= a[ai] <= '9'
        b_is_digit = '0' <= b[bi] <= '9'
        if a_is_digit and b_is_digit:
            a_start = ai
            b_start = bi
            while ai < len(a) and '0' <= a[ai] <= '9':
                ai += 1
            while bi < len(b) and '0' <= b[bi] <= '9':
                bi += 1
            a_digits = a[a_start:ai]
            b_digits = b[b_start:bi]
            a_significant = a_digits.lstrip('0') or '0'
            b_significant = b_digits.lstrip('0') or '0'
            if len(a_significant) != len(b_significant):
                return (len(a_significant) > len(b_significant)) - (
                    len(a_significant) < len(b_significant)
                )
            if a_significant != b_significant:
                return (a_significant > b_significant) - (
                    a_significant < b_significant
                )
            if len(a_digits) != len(b_digits):
                return (len(b_digits) > len(a_digits)) - (
                    len(b_digits) < len(a_digits)
                )
            continue

        cmp = _version_char_compare(a[ai], b[bi])
        if cmp:
            return cmp
        ai += 1
        bi += 1

    if ai == len(a) and bi == len(b):
        return 0
    if ai == len(a):
        return 1 if b[bi] == '~' else -1
    return -1 if a[ai] == '~' else 1


def _compare_values(a: str, b: str, mode: str, fold_case: bool) -> int:
    if mode == 'numeric':
        return _numeric_compare(a, b, fold_case)
    if mode == 'version':
        return _version_compare(a, b, fold_case)
    if mode == 'month':
        return _month_compare(a, b, fold_case)
    return _lexical_compare(a, b, fold_case)



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
    version_sort = False
    month_sort = False
    check = False
    unique = False
    fold_case = False
    key_specs: list[dict] = []
    files = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == '--':
            files.extend(args[i + 1:])
            break
        if arg == '-r':
            reverse = True
        elif arg == '-n':
            numeric = True
        elif arg in ('-V', '--version-sort'):
            version_sort = True
        elif arg in ('-M', '--month-sort'):
            month_sort = True
        elif arg in ('-c', '--check', '--check=quiet', '--check=diagnose-first'):
            check = True
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
        elif arg.startswith('--'):
            print(f"sort: invalid option: {arg}", file=sys.stderr)
            return 1
        elif arg.startswith('-') and len(arg) > 1:
            for ch in arg[1:]:
                if ch == 'r':
                    reverse = True
                elif ch == 'n':
                    numeric = True
                elif ch == 'V':
                    version_sort = True
                elif ch == 'M':
                    month_sort = True
                elif ch == 'c':
                    check = True
                elif ch == 'u':
                    unique = True
                elif ch == 'f':
                    fold_case = True
                elif ch == 'k':
                    # Combined like -k2.
                    spec = arg[arg.index('k') + 1:]
                    if not spec:
                        print("sort: option requires an argument: -k", file=sys.stderr)
                        return 1
                    ks = _parse_key_spec(spec)
                    if ks is None:
                        print(f"sort: invalid key specification: {spec}", file=sys.stderr)
                        return 1
                    key_specs.append(ks)
                    break
                else:
                    print(f"sort: invalid option: -{ch}", file=sys.stderr)
                    return 1
        else:
            files.append(arg)
        i += 1

    lines = _read_lines(files)
    if version_sort:
        global_mode = 'version'
    elif month_sort:
        global_mode = 'month'
    elif numeric:
        global_mode = 'numeric'
    else:
        global_mode = 'lexical'

    def compare_lines(a: str, b: str) -> int:
        for ks in key_specs:
            mode = 'numeric' if ks['numeric'] else global_mode
            cmp = _compare_values(
                _extract_key(a, ks),
                _extract_key(b, ks),
                mode,
                fold_case,
            )
            if cmp:
                if ks['reverse'] or reverse:
                    return -cmp
                return cmp

        cmp = _compare_values(a, b, global_mode, fold_case)
        return -cmp if reverse else cmp

    if check:
        for previous, current in zip(lines, lines[1:]):
            if compare_lines(previous, current) > 0:
                return 1
        return 0

    lines.sort(key=cmp_to_key(compare_lines))

    if unique:
        deduped = []
        for line in lines:
            if not deduped or line != deduped[-1]:
                deduped.append(line)
        lines = deduped

    for line in lines:
        print(line)

    return 0
