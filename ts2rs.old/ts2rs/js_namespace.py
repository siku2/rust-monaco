import dataclasses
import re
from typing import List, Optional, Tuple, Union

from . import helpers
from .helpers import MatchError
from .js_enum import JsEnum
from .js_function import JsFunction
from .js_object import JsClass, JsObject
from .js_type import JsTypeAlias
from .models import Context, NamespaceContext, ToRust

NamespaceMember = Union[JsEnum, JsFunction, JsObject, JsTypeAlias]


def consume_namespace_member(s: str) -> Tuple[NamespaceMember, str]:
    return helpers.consume_first(s, JsEnum, JsFunction, JsObject, JsTypeAlias)


_PATTERN_NAMESPACE_OPEN = re.compile(r"^ *declare namespace (?P<name>(?:\w|\.)+) {\n")


@dataclasses.dataclass()
class JsNamespace:
    name: str
    functions: List[JsFunction]
    objects: List[JsObject]
    others: List[ToRust]

    @classmethod
    def consume(cls, s: str) -> Tuple["JsNamespace", str]:
        match, s = helpers.consume_match(_PATTERN_NAMESPACE_OPEN, s)

        functions = []
        objects = []
        others = []
        ns = cls(
            name=match["name"], functions=functions, objects=objects, others=others
        )

        body, s = helpers.read_until_closing_bracket(s)
        while body:
            mem, body = consume_namespace_member(body)
            if isinstance(mem, JsFunction):
                functions.append(mem)
            elif isinstance(mem, JsObject):
                objects.append(mem)
            elif isinstance(mem, (JsEnum, JsTypeAlias)):
                others.append(mem)
            else:
                raise TypeError(f"unexpected value: {mem!r}")

        return ns, s

    def _is_copy_type(self, ty: str) -> bool:
        for other in self.others:
            if not isinstance(other, JsEnum):
                continue

            if other.ident == ty:
                return True

        return False

    def get_namespace(self) -> Optional[str]:
        _, _, namespace = self.name.partition(".")
        return namespace or None

    def to_rust(self) -> str:
        additional_helpers: List[ToRust] = []
        ns_ctx = NamespaceContext(
            namespace=self.get_namespace(),
            helpers=additional_helpers,
            is_copy_type=self._is_copy_type,
        )
        ctx = Context(ns=ns_ctx, path=())

        import_types: List[str] = []
        helper_types: List[str] = []

        for v in self.others:
            helper_types.append(v.to_rust(ctx))

        for func in self.functions:
            import_types.append(func.to_rust(ctx))

        for obj in self.objects:
            s = obj.to_rust(ctx)
            if isinstance(obj, JsClass):
                import_types.append(s)
                continue

            helper_types.append(to_rust_block(s))

        for helper in additional_helpers:
            helper_types.append(helper.to_rust(ctx))

        import_block = to_rust_block("\n\n".join(import_types))
        helper_block = "\n\n".join(helper_types)

        return helpers.join_nonempty_lines((import_block, helper_block))


def to_rust_block(v: str) -> str:
    return f"""#[wasm_bindgen]\nextern "C" {{\n{helpers.add_indent(v)}\n}}"""
