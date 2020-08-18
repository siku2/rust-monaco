import dataclasses
import re
from typing import List, Tuple, Union

from . import helpers
from .helpers import MatchError
from .js_enum import JsEnum
from .js_function import JsFunction
from .js_object import JsClass, JsObject

NamespaceMember = Union[JsEnum, JsFunction, JsObject]


def consume_namespace_member(s: str) -> Tuple[NamespaceMember, str]:
    return helpers.consume_first(s, JsEnum, JsFunction, JsObject)


_PATTERN_NAMESPACE_OPEN = re.compile(r"^ *declare namespace (?P<name>(?:\w|\.)+) {\n")


@dataclasses.dataclass()
class JsNamespace:
    name: str
    enums: List[JsEnum]
    functions: List[JsFunction]
    objects: List[JsObject]

    @classmethod
    def consume(cls, s: str) -> Tuple["JsNamespace", str]:
        match, s = helpers.consume_match(_PATTERN_NAMESPACE_OPEN, s)

        enums = []
        functions = []
        objects = []
        ns = cls(name=match["name"], enums=enums, functions=functions, objects=objects)

        body, s = helpers.read_until_closing_bracket(s)
        while body:
            mem, body = consume_namespace_member(body)
            if isinstance(mem, JsEnum):
                enums.append(mem)
            elif isinstance(mem, JsFunction):
                functions.append(mem)
            elif isinstance(mem, JsObject):
                objects.append(mem)
            else:
                raise TypeError(f"unexpected value: {mem!r}")

        return ns, s

    def to_rust(self) -> str:
        import_types: List[str] = []
        helper_types: List[str] = []

        for enum in self.enums:
            helper_types.append(enum.to_rust())

        for func in self.functions:
            import_types.append(func.to_rust())

        for obj in self.objects:
            if isinstance(obj, JsClass):
                import_types.append(obj.to_rust())
                continue

            helper_types.append(to_rust_block(obj))

        import_block = to_rust_block("\n\n".join(import_types))
        helper_block = "\n\n".join(helper_types)

        return helpers.join_nonempty_lines((import_block, helper_block))


def to_rust_block(v: Union[NamespaceMember, str]) -> str:
    if not isinstance(v, str):
        v = v.to_rust()
    return f"""#[wasm_bindgen]\nextern "C" {{\n{helpers.add_indent(v)}\n}}"""
