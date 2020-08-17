from typing import List, Optional
from .helpers import split_trim


class TypeWithDocumentation(str):
    documentation: Optional[str] = None

    def __new__(cls, *args, documentation: str = None, **kwargs):
        s = super().__new__(cls, *args, **kwargs)
        s.documentation = documentation
        return s


JS2RUST_TYPE = {
    "number": "f64",
    "boolean": "bool",
}


class JsType(str):
    def split_union(self) -> List["JsType"]:
        return list(map(self.__class__, split_trim(self, "|")))

    def is_none(self) -> bool:
        return self == "null" or self == "undefined"

    def is_function(self) -> bool:
        return "=>" in self

    def is_object(self) -> bool:
        return self.startswith("{") and self.endswith("}")

    def is_type_assertion(self) -> bool:
        return " is " in self

    def array_item(self) -> Optional["JsType"]:
        if self.endswith("[]"):
            return self.__class__(self[:-2])

        return None

    def __union_type(
        self, sub_types: List["JsType"], owned: bool
    ) -> TypeWithDocumentation:
        is_optional = False
        for i in reversed(range(len(sub_types))):
            if sub_types[i].is_none():
                del sub_types[i]
                is_optional = True

        rust_types = [ty.to_rust(owned) for ty in sub_types]
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

        return TypeWithDocumentation(
            maybe_ref("Array", owned), documentation=f"`{array_item}[]`"
        )

    def __object_type(self, owned: bool) -> TypeWithDocumentation:
        return TypeWithDocumentation(
            maybe_ref("Object", owned), documentation=f"`{self}`"
        )

    def to_rust(self, owned: bool) -> TypeWithDocumentation:
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

        union_types = self.split_union()
        if len(union_types) > 1:
            return self.__union_type(union_types, owned)

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

        return TypeWithDocumentation(maybe_ref(ty, owned), documentation=documentation)


def maybe_ref(s: str, owned: bool) -> str:
    return s if owned else f"&{s}"
