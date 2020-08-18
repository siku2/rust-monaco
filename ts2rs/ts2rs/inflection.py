import re

_PATTERN_UPPER_LOWER = re.compile(r"(.)([A-Z][a-z]+)")
_PATTERN_LOWER_UPPER = re.compile(r"([a-z0-9])([A-Z])")


def camel_to_snake_case(s: str) -> str:
    s = _PATTERN_UPPER_LOWER.sub(r"\1_\2", s)
    s = _PATTERN_LOWER_UPPER.sub(r"\1_\2", s)
    return s.lower()


def snake_to_camel_case(s: str) -> str:
    return "".join(part.title() for part in s.split("_"))

