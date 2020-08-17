import re
import textwrap
from typing import Optional, Iterable, List, Match, Pattern, Tuple, Union

_PATTERN_UPPER_LOWER = re.compile(r"(.)([A-Z][a-z]+)")
_PATTERN_LOWER_UPPER = re.compile(r"([a-z0-9])([A-Z])")


def camel_to_snake_case(s: str) -> str:
    s = _PATTERN_UPPER_LOWER.sub(r"\1_\2", s)
    s = _PATTERN_LOWER_UPPER.sub(r"\1_\2", s)
    return s.lower()


def snake_to_camel_case(s: str) -> str:
    return "".join(part.title() for part in s.split("_"))


def add_line_prefix(s: str, prefix: str, /, empty_lines=False) -> str:
    if empty_lines:
        predicate = lambda line: True
    else:
        predicate = None

    return textwrap.indent(s, prefix, predicate=predicate)


def add_indent(s: str, levels=1) -> str:
    level_prefix = 4 * " "
    return add_line_prefix(s, levels * level_prefix)


def remove_indent(s: str) -> str:
    return textwrap.dedent(s)


def split_trim(s: str, delim: Optional[str]) -> List[str]:
    return [s.strip() for s in s.split(delim)]


def join_nonempty_lines(lines: Iterable[Optional[str]]) -> str:
    return "\n".join(filter(None, (line.strip() for line in lines if line)))


def read_until_closing(s: str, open: str, close: str) -> Tuple[str, str, str]:
    open_pattern = re.compile(open)
    pattern = re.compile(f"(?:{open_pattern.pattern})|(?:{close})")

    start_pos = end_pos = 0
    depth = 1
    while depth:
        match = pattern.search(s, end_pos)
        if not match:
            raise ValueError(f"missing closing bracket (expected {depth} closing)")

        start_pos, end_pos = match.start(), match.end()
        if open_pattern.match(match[0]):
            depth += 1
        else:
            depth -= 1

    return s[:start_pos], s[start_pos:end_pos], s[end_pos:]


def read_until_closing_bracket(s: str) -> Tuple[str, str]:
    (a, _, b) = read_until_closing(s, r"\{", r"\}")
    return a, b


def build_wasm_bindgen_attr(*args: str, **kwargs: Union[str, Iterable[str]]) -> str:
    args = list(args)
    for key, value in kwargs.items():
        if isinstance(value, str):
            args.append(f"{key} = {value}")
        else:
            args.extend(f"{key} = {v}" for v in value)

    return f"#[wasm_bindgen({', '.join(args)})]"


_PATTERN_EMPTY_LINES = re.compile(r"^ *(?:\n|$)")


def consume_empty_lines(s: str) -> str:
    while match := _PATTERN_EMPTY_LINES.match(s):
        s = s[match.end() :]
        if not s:
            break

    return s


class MatchError(Exception):
    ...


def consume_match(pattern: Pattern, s: str) -> Tuple[Match, str]:
    if match := pattern.match(s):
        remainder = s[match.end() :]
        return match, remainder

    raise MatchError(f"{s[:50]!r} didn't match pattern: `{pattern.pattern}`")


class ModSet:
    def __init__(self, mods: Iterable[str]) -> None:
        self._mods = set(mods)

    @classmethod
    def create(cls, s: str):
        return cls(s.split())

    def pop(self, mod: str) -> bool:
        try:
            self._mods.remove(mod)
        except KeyError:
            return False
        else:
            return True

    def assert_empty(self):
        if self._mods:
            raise ValueError(f"unhandled modifiers: {self._mods}")

