import dataclasses
import re
import textwrap
from typing import Optional, Iterable, List, Match, Pattern, Tuple, Type, TypeVar, Union


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


def read_until_closing_bracket(
    s: str, *, skip_non_content_after=True
) -> Tuple[str, str]:
    (a, _, b) = read_until_closing(s, r"\{", r"\}")
    if skip_non_content_after:
        b = skip_non_content(b)
    return a, b


def build_wasm_bindgen_attr(
    *args: Union[str, None], **kwargs: Union[str, Iterable[str], None]
) -> str:
    args = list(args)
    for key, value in kwargs.items():
        if not value:
            continue

        if isinstance(value, str):
            args.append(f"{key} = {value}")
        else:
            args.extend(f"{key} = {v}" for v in value)

    return f"#[wasm_bindgen({', '.join(filter(None, args))})]"


_PATTERN_COMMENT = re.compile(r"^ *\/\/.*\n")


def consume_comments(s: str) -> str:
    while match := _PATTERN_COMMENT.match(s):
        s = s[match.end() :]

    return s


_PATTERN_EMPTY_LINES = re.compile(r"^ *(?:\n|$)")


def consume_empty_lines(s: str) -> str:
    while match := _PATTERN_EMPTY_LINES.match(s):
        s = s[match.end() :]
        if not s:
            break

    return s


def skip_non_content(s: str) -> str:
    while True:
        new = consume_comments(consume_empty_lines(s))
        if new == s:
            break

        s = new

    return s


@dataclasses.dataclass()
class MatchError(Exception):
    s: str
    pattern: Optional[Pattern] = None
    info: Optional[str] = None

    def __str__(self) -> str:
        s = self.preview_s()
        if info := self.info:
            return f"failed to parse: {info}:\n{s}"
        elif pattern := self.pattern:
            return f"didn't match pattern: `{pattern.pattern}`:\n{s}"
        else:
            return "{s}"

    def preview_s(self) -> str:
        lines = self.s.splitlines()
        if len(lines) > 8:
            lines = lines[:8]
            lines.append("... TRUNCATED")

        s = "\n".join(lines)
        hor_line = 80 * "="
        return f"{hor_line}\n{s}\n{hor_line}"


def consume_match(
    pattern: Pattern, s: str, *, skip_non_content_after=True, info: str = None,
) -> Tuple[Match, str]:
    if match := pattern.match(s):
        remainder = s[match.end() :]
        if skip_non_content_after:
            remainder = skip_non_content(remainder)

        return match, remainder

    raise MatchError(s=s, pattern=pattern, info=info)


T = TypeVar("T")


def consume_first(s: str, *consumers: Type[T], args=None) -> Tuple[T, str]:
    assert consumers, "need at least one consumer"

    if args is None:
        args = ()

    error = None
    for consumer in consumers:
        try:
            return consumer.consume(s, *args)
        except MatchError as e:
            e.__context__ = error
            error = e

    raise MatchError(
        s=s, info=" | ".join(f"`{consumer.__qualname__}`" for consumer in consumers)
    ) from error


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

