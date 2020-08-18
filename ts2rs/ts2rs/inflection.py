import re

_PATTERN_UPPER_LOWER = re.compile(r"(.)([A-Z][a-z]+)")
_PATTERN_LOWER_UPPER = re.compile(r"([a-z0-9])([A-Z])")


def camel_to_snake_case(s: str) -> str:
    s = _PATTERN_UPPER_LOWER.sub(r"\1_\2", s)
    s = _PATTERN_LOWER_UPPER.sub(r"\1_\2", s)
    return s.lower()


_PATTERN_NON_ALPHANUM = re.compile(r"(?:[^a-zA-Z0-9]+)")


def any_to_camel_case(s: str) -> str:
    s = camel_to_snake_case(s)
    parts = _PATTERN_NON_ALPHANUM.split(s)
    return "".join(part.title() for part in parts if part)


def snake_to_camel_case(s: str) -> str:
    return "".join(part.title() for part in s.split("_"))

