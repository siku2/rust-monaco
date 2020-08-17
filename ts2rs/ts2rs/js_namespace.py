import dataclasses
import re
from typing import List, Tuple, Union

from .helpers import (
    MatchError,
    add_indent,
    consume_empty_lines,
    consume_match,
    join_nonempty_lines,
    read_until_closing_bracket,
)
from .js_enum import JsEnum
from .js_object import JsClass, JsObject

NamespaceMember = Union[JsEnum, JsObject]


def consume_namespace_member(s: str) -> Tuple[NamespaceMember, str]:
    try:
        return JsEnum.consume(s)
    except MatchError:
        return JsObject.consume(s)


_PATTERN_NAMESPACE_OPEN = re.compile(r"^ *declare namespace (?P<name>(?:\w|\.)+) {\n")


@dataclasses.dataclass()
class JsNamespace:
    name: str
    enums: List[JsEnum]
    objects: List[JsObject]

    @classmethod
    def consume(cls, s: str) -> Tuple["JsNamespace", str]:
        match, s = consume_match(_PATTERN_NAMESPACE_OPEN, s)
        s = consume_empty_lines(s)

        enums = []
        objects = []
        ns = cls(name=match["name"], enums=enums, objects=objects)

        body, s = read_until_closing_bracket(s)
        s = consume_empty_lines(s)
        while body:
            mem, body = consume_namespace_member(body)
            if isinstance(mem, JsEnum):
                enums.append(mem)
            elif isinstance(mem, JsObject):
                objects.append(mem)
            else:
                raise TypeError

        return ns, s

    def to_rust(self) -> str:
        import_types: List[str] = []
        helper_types: List[str] = []

        for obj in self.objects:
            if isinstance(obj, JsClass):
                import_types.append(obj.to_rust())
                continue

            helper_types.append(to_rust_block(obj))

        for enum in self.enums:
            helper_types.append(enum.to_rust())

        import_block = to_rust_block("\n\n".join(import_types))
        helper_block = "\n\n".join(helper_types)

        return join_nonempty_lines((import_block, helper_block))


def to_rust_block(v: Union[NamespaceMember, str]) -> str:
    if not isinstance(v, str):
        v = v.to_rust()
    return f"""#[wasm_bindgen]\nextern "C" {{\n{add_indent(v)}\n}}"""
