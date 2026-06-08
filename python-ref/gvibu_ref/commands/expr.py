"""expr: evaluate expressions."""

import sys
import re


class ExprParser:
    def __init__(self, args: list[str]):
        self.tokens = self._tokenize(args)
        self.pos = 0

    def _tokenize(self, args: list[str]) -> list:
        tokens = []
        for arg in args:
            if arg in ("(", ")"):
                tokens.append(arg)
            elif arg in ("|", "&", "=", "!=", "<", "<=", ">", ">=",
                         "+", "-", "*", "/", "%", ":"):
                tokens.append(arg)
            elif arg == "length":
                tokens.append(("keyword", "length"))
            elif arg == "substr":
                tokens.append(("keyword", "substr"))
            elif arg == "index":
                tokens.append(("keyword", "index"))
            elif arg == "match":
                tokens.append(("keyword", "match"))
            else:
                try:
                    tokens.append(("num", int(arg)))
                except ValueError:
                    tokens.append(("str", arg))
        return tokens

    def peek(self):
        return self.tokens[self.pos] if self.pos < len(self.tokens) else None

    def next_token(self):
        if self.pos < len(self.tokens):
            t = self.tokens[self.pos]
            self.pos += 1
            return t
        return None

    def expect(self, msg):
        t = self.next_token()
        if t is None:
            raise ValueError(f"expr: {msg}")
        return t

    def parse(self) -> str:
        result = self._parse_or()
        return result

    def _parse_or(self) -> str:
        left = self._parse_and()
        while self.peek() == "|":
            self.next_token()
            right = self._parse_and()
            if is_non_zero_non_null(left):
                pass  # keep left
            else:
                left = right
        return left

    def _parse_and(self) -> str:
        left = self._parse_cmp()
        while self.peek() == "&":
            self.next_token()
            right = self._parse_cmp()
            if is_non_zero_non_null(left) and is_non_zero_non_null(right):
                pass  # keep left
            else:
                left = "0"
        return left

    def _parse_cmp(self) -> str:
        left = self._parse_arith()
        op = self.peek()
        if op in ("=", "!=", "<", "<=", ">", ">="):
            self.next_token()
            right = self._parse_arith()
            return compare_str(left, op, right)
        return left

    def _parse_arith(self) -> str:
        left = self._parse_term()
        while self.peek() in ("+", "-"):
            op = self.next_token()
            right = self._parse_term()
            left = arith_op(left, op, right)
        return left

    def _parse_term(self) -> str:
        left = self._parse_factor()
        while self.peek() in ("*", "/", "%", ":"):
            op = self.next_token()
            right = self._parse_factor()
            if op == ":":
                left = regex_match(left, right)
            else:
                left = arith_op(left, op, right)
        return left

    def _parse_factor(self) -> str:
        peek = self.peek()
        if peek is None:
            raise ValueError("missing operand")

        if isinstance(peek, tuple) and peek[0] == "keyword":
            kw = peek[1]
            self.next_token()
            if kw == "length":
                arg = self._parse_factor()
                return str(len(arg))
            elif kw == "substr":
                s = self._token_value(self.expect("expected string"))
                pos_str = self._token_value(self.expect("expected position"))
                len_str = self._token_value(self.expect("expected length"))
                try:
                    pos = int(pos_str)
                    length = int(len_str)
                except ValueError:
                    return ""
                if pos < 1 or pos > len(s) or length <= 0:
                    return ""
                start = pos - 1
                end = min(start + length, len(s))
                return s[start:end]
            elif kw == "index":
                s = self._token_value(self.expect("expected string"))
                chars = self._token_value(self.expect("expected chars"))
                for i, ch in enumerate(s):
                    if ch in chars:
                        return str(i + 1)
                return "0"
            elif kw == "match":
                s = self._token_value(self.expect("expected string"))
                regex = self._token_value(self.expect("expected regex"))
                return regex_match(s, regex)
            else:
                raise ValueError(f"unknown keyword: {kw}")

        if peek == "(":
            self.next_token()
            result = self._parse_or()
            if self.peek() != ")":
                raise ValueError("missing closing parenthesis")
            self.next_token()
            return result

        tok = self.next_token()
        if isinstance(tok, tuple) and tok[0] in ("num", "str"):
            return str(tok[1])

        raise ValueError(f"unexpected token: {tok}")

    def _token_value(self, tok) -> str:
        if isinstance(tok, tuple):
            return str(tok[1])
        return str(tok)


def is_non_zero_non_null(s: str) -> bool:
    if not s:
        return False
    try:
        return int(s) != 0
    except ValueError:
        return True


def compare_str(left: str, op: str, right: str) -> str:
    try:
        l, r = int(left), int(right)
        numeric = True
    except ValueError:
        numeric = False

    if numeric:
        if op == "=":
            result = l == r
        elif op == "!=":
            result = l != r
        elif op == "<":
            result = l < r
        elif op == "<=":
            result = l <= r
        elif op == ">":
            result = l > r
        elif op == ">=":
            result = l >= r
        else:
            result = False
        return "1" if result else "0"

    if op == "=":
        result = left == right
    elif op == "!=":
        result = left != right
    elif op == "<":
        result = left < right
    elif op == "<=":
        result = left <= right
    elif op == ">":
        result = left > right
    elif op == ">=":
        result = left >= right
    else:
        result = False
    return "1" if result else "0"


def arith_op(left: str, op: str, right: str) -> str:
    try:
        l = int(left)
    except ValueError:
        raise ValueError(f"non-numeric argument: {left}")
    try:
        r = int(right)
    except ValueError:
        raise ValueError(f"non-numeric argument: {right}")

    if op == "+":
        return str(l + r)
    elif op == "-":
        return str(l - r)
    elif op == "*":
        return str(l * r)
    elif op == "/":
        if r == 0:
            raise ValueError("division by zero")
        return str(l // r)
    elif op == "%":
        if r == 0:
            raise ValueError("division by zero")
        return str(l % r)
    else:
        raise ValueError(f"unknown operator: {op}")


def regex_match(text: str, pattern: str) -> str:
    """Basic regex match - uses Python's re module."""
    try:
        m = re.search(pattern, text)
        if m:
            return m.group(0)
        else:
            return "0"
    except re.error:
        return "0"


def run(args: list[str]) -> int:
    if not args:
        print("expr: missing operand", file=sys.stderr)
        return 1

    try:
        parser = ExprParser(args)
        result = parser.parse()
        print(result)
        return 0 if is_non_zero_non_null(result) else 1
    except ValueError as e:
        print(f"expr: {e}", file=sys.stderr)
        return 2
    except Exception as e:
        print(f"expr: {e}", file=sys.stderr)
        return 2
