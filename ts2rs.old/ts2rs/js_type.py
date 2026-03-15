import dataclasses
import re
from typing import List, Optional, Tuple

from . import helpers
from .models import Context, Documented, StringEnum, ToRust


class TypeWithDocumentation(str):
    documentation: Optional[str] = None

    def __new__(cls, v, documentation: str = None):
        s = super().__new__(cls, v)
        s.documentation = documentation
        return s

    def replace_text(self, s: str) -> "TypeWithDocumentation":
        return TypeWithDocumentation(s, documentation=self.documentation)

    def to_option(self) -> "TypeWithDocumentation":
        if self.startswith("Option<") and self.endswith(">"):
            return self
        return self.replace_text(f"Option<{self}>")


_PATTERN_STR_ENUM = re.compile(r"(?:\|\s*)?\".+?\"(?:\s*\|\s*\".+?\")*", re.DOTALL)

JS2RUST_TYPE = {
    "number": "f64",
    "boolean": "bool",
}


class JsType(str):
    def is_str_enum(self) -> bool:
        return bool(_PATTERN_STR_ENUM.match(self))

    def is_none(self) -> bool:
        return self == "null" or self == "undefined"

    def is_function(self) -> bool:
        return "=>" in self

    def is_object(self) -> bool:
        return self.startswith("{") and self.endswith("}")

    def is_type_assertion(self) -> bool:
        return " is " in self

    def split_union(self) -> List["JsType"]:
        return list(map(self.__class__, helpers.split_trim(self, "|")))

    def array_item(self) -> Optional["JsType"]:
        if self.endswith("[]"):
            return self.__class__(self[:-2])

        return None

    def __str_enum(
        self, ctx: Context, *, create_helpers: bool
    ) -> TypeWithDocumentation:
        ident = ctx.build_ident()
        if create_helpers:
            ctx.ns.add_helper(StringEnum.create(ident, self))
        return TypeWithDocumentation(ident)

    def __union_type(
        self,
        ctx: Context,
        sub_types: List["JsType"],
        owned: bool,
        *,
        create_helpers: bool,
    ) -> TypeWithDocumentation:
        is_optional = False
        for i in reversed(range(len(sub_types))):
            if sub_types[i].is_none():
                del sub_types[i]
                is_optional = True

        rust_types = [
            ty.to_rust(ctx, owned, create_helpers=create_helpers) for ty in sub_types
        ]
        # can't create enums on the fly
        ty = " | ".join(rust_types)

        if is_optional:
            ty = f"Option<{ty}>"

        ty = TypeWithDocumentation(ty)
        if any(ty.documentation for ty in rust_types):
            ty.documentation = f"`{' | '.join(sub_types)}`"

        return ty

    def __array_type(self, array_item: "JsType", owned: bool) -> TypeWithDocumentation:
        if array_item == "number":
            return TypeWithDocumentation("Vec<f64>" if owned else "&[f64]")
        if array_item == "any":
            return TypeWithDocumentation("Vec<JsValue>" if owned else "&[JsValue]")

        return TypeWithDocumentation(
            maybe_ref("Array", owned), documentation=f"`{array_item}[]`"
        )

    def __object_type(self, owned: bool) -> TypeWithDocumentation:
        return TypeWithDocumentation(
            maybe_ref("Object", owned), documentation=f"`{self}`"
        )

    def to_rust(
        self, ctx: Context, owned: bool, *, create_helpers=True
    ) -> TypeWithDocumentation:
        try:
            ty = JS2RUST_TYPE[self]
        except KeyError:
            pass
        else:
            return TypeWithDocumentation(ty)

        if self == "string":
            return TypeWithDocumentation("String" if owned else "&str")

        if self.is_object():
            return self.__object_type(owned)

        if self.is_str_enum():
            return self.__str_enum(ctx, create_helpers=create_helpers)

        union_types = self.split_union()
        if len(union_types) > 1:
            return self.__union_type(
                ctx, union_types, owned, create_helpers=create_helpers
            )

        if array_item := self.array_item():
            return self.__array_type(array_item, owned)

        if self.is_type_assertion():
            return TypeWithDocumentation("bool")

        ty = str(self)
        documentation = None

        if self == "any":
            ty = "JsValue"
        elif self.is_function():
            ty = "Function"
            documentation = f"`{self}`"

        return TypeWithDocumentation(
            maybe_ref(ty, owned, ctx=ctx), documentation=documentation
        )


def maybe_ref(s: str, owned: bool, *, ctx: Context = None) -> str:
    if ctx and not owned:
        owned = ctx.ns.is_copy_type(s)

    return s if owned else f"&{s}"


_PATTERN_TYPE_ALIAS = re.compile(
    r"^ *export type (?P<ident>\w+)\s*=\s*(?P<type>.+?);", re.DOTALL
)


@dataclasses.dataclass()
class JsTypeAlias(Documented, ToRust):
    ident: str
    type_: JsType

    @classmethod
    def consume(cls, s: str) -> Tuple["JsTypeAlias", str]:
        doc, s = Documented.consume(s)
        match, s = helpers.consume_match(_PATTERN_TYPE_ALIAS, s)
        alias = cls(
            documentation=doc, ident=match["ident"], type_=JsType(match["type"])
        )
        return alias, s

    def to_rust(self, ctx: Context) -> str:
        if not self.type_.is_str_enum():
            raise TypeError(
                f"only string enum type aliases are supported. Type: `{self.type_}`"
            )
        return StringEnum.create(self.ident, self.type_).to_rust(ctx)
