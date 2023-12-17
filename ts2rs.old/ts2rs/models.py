import abc
import dataclasses
import re
from typing import Callable, List, Match, Optional, Tuple

from . import helpers, inflection
from .helpers import MatchError


# Matches multiline documentation in the form of
# /**
#  * Hello world!
#  */
_PATTERN_DOC = re.compile(r"^ *\/\*\*\n(?: *\*(?: .*)?\n)+ *\*\/\n")
_PATTERN_SINGLE_DOC = re.compile(r"^ *\/\*\* *(?:.*) *\*\/\n")


def match_doc(s: str) -> Tuple[Match, str]:
    try:
        return helpers.consume_match(_PATTERN_DOC, s, skip_non_content_after=False)
    except MatchError:
        return helpers.consume_match(
            _PATTERN_SINGLE_DOC, s, skip_non_content_after=False
        )


@dataclasses.dataclass()
class Documented:
    documentation: str

    @staticmethod
    def consume(s: str) -> Tuple[str, str]:
        try:
            match, s = match_doc(s)
        except MatchError:
            return "", s

        lines = helpers.remove_indent(match[0]).splitlines()
        doc = "\n".join(line[3:] for line in lines[1:-1])
        return doc, s

    def rust_documentation(self) -> str:
        return "\n".join(f"/// {line}" for line in self.documentation.splitlines())


@dataclasses.dataclass()
class NamespaceContext:
    namespace: Optional[str]
    helpers: List["ToRust"]
    is_copy_type: Callable[[str], bool]

    def add_helper(self, helper: "ToRust"):
        self.helpers.append(helper)


@dataclasses.dataclass(frozen=True)
class Context:
    ns: NamespaceContext
    path: Tuple[str, ...]

    def push(self, ident: str) -> "Context":
        return dataclasses.replace(self, path=(*self.path, ident))

    def build_ident(self) -> str:
        return "".join(inflection.any_to_camel_case(seg) for seg in self.path)


class ToRust(abc.ABC):
    @abc.abstractmethod
    def to_rust(self, ctx: Context) -> str:
        ...


@dataclasses.dataclass()
class StringEnum(ToRust):
    ident: str
    variants: List[str]

    @classmethod
    def create(cls, ident: str, value: str) -> "JsEnum":
        raw_variants = helpers.split_trim(value, "|")
        variants = list(filter(None, (raw[1:-1] for raw in raw_variants)))
        return cls(ident=ident, variants=variants)

    def _variants_to_rust(self) -> str:
        variants_it = (
            f'{inflection.any_to_camel_case(variant)} = "{variant}",'
            for variant in self.variants
        )
        return "\n".join(variants_it)

    def to_rust(self, ctx: Context) -> str:
        enum_body = self._variants_to_rust()
        macro_body = f"pub enum {self.ident} {{\n{helpers.add_indent(enum_body)}\n}}"
        return f"str_enum! {{\n{helpers.add_indent(macro_body)}\n}}"
