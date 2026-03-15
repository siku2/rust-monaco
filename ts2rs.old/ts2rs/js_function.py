import dataclasses
import re
from typing import Iterable, List, Optional, Tuple

from . import helpers, inflection
from .js_type import JsType, TypeWithDocumentation
from .models import Context, Documented, ToRust


@dataclasses.dataclass()
class RustParam:
    ident: str
    ty: str
    documentation: Optional[str] = None

    def __str__(self) -> str:
        return f"{self.ident}: {self.ty}"


_PATTERN_PARAM = re.compile(
    r"^\s*(?P<variadic>\.\.\.)?(?P<ident>\w+)(?P<optional>\??): (?P<type>.+?)(?:,\s*|\s*$)",
    re.DOTALL,
)


@dataclasses.dataclass()
class JsParameter:
    ident: str
    type_: JsType
    variadic: bool
    optional: bool

    @classmethod
    def consume(cls, s: str) -> Tuple["JsParameter", str]:
        match, s = helpers.consume_match(
            _PATTERN_PARAM, s, skip_non_content_after=False
        )
        optional = bool(match["optional"])
        param = cls(
            ident=match["ident"],
            type_=JsType(match["type"]),
            variadic=bool(match["variadic"]),
            optional=optional,
        )
        return param, s

    @classmethod
    def parse_multiple(cls, s: str) -> List["JsParameter"]:
        params = []
        while s:
            param, s = cls.consume(s)
            params.append(param)

        return params

    def type_documentation(self, ty: TypeWithDocumentation) -> Optional[str]:
        if type_doc := ty.documentation:
            return f"* `{inflection.camel_to_snake_case(self.ident)}` - {type_doc}"

        return None

    def to_rust(self, ctx: Context) -> RustParam:
        ident = inflection.camel_to_snake_case(self.ident)
        ctx = ctx.push(ident)
        ty = self.type_.to_rust(ctx, False)
        if self.optional:
            ty = ty.to_option()
        return RustParam(ident=ident, ty=ty, documentation=self.type_documentation(ty))


_PATTERN_FUNCTION = re.compile(
    r"^ *export function (?P<ident>\w+)\((?P<params>.*?)\)(?:: (?P<ret>.+?))?;",
    re.DOTALL,
)


@dataclasses.dataclass()
class JsFunction(Documented, ToRust):
    ident: str
    params: List[JsParameter]
    return_type: Optional[JsType]

    @classmethod
    def from_match(
        cls,
        *,
        documentation: str,
        ident: str,
        params: str,
        ret: Optional[str],
        **kwargs,
    ) -> "JsFunction":
        params = JsParameter.parse_multiple(params)
        if return_type := ret:
            if return_type == "void":
                return_type = None
            else:
                return_type = JsType(return_type)

        return cls(
            documentation=documentation,
            ident=ident,
            params=params,
            return_type=return_type,
            **kwargs,
        )

    @classmethod
    def consume(cls, s: str) -> Tuple["JsFunction", str]:
        doc, s = Documented.consume(s)
        match, s = helpers.consume_match(_PATTERN_FUNCTION, s)
        func = cls.from_match(
            documentation=doc,
            ident=match["ident"],
            params=match["params"],
            ret=match["ret"],
        )
        return func, s

    def argument_documentation(self, params: Iterable[RustParam]) -> Optional[str]:
        args = list(filter(None, (param.documentation for param in params)))
        if not args:
            return None
        raw = "\n".join(("", "# Arguments", "", *args,))
        return helpers.add_line_prefix(raw, "/// ", empty_lines=True)

    def rust_documentation(
        self, params: Iterable[RustParam], ret: Optional[TypeWithDocumentation]
    ) -> str:
        doc = super().rust_documentation()
        if ret := (ret and ret.documentation):
            raw = "\n".join(("", "# Returns", "", ret))
            return_doc = helpers.add_line_prefix(raw, "/// ", empty_lines=True)
        else:
            return_doc = None

        return helpers.join_nonempty_lines(
            (doc, self.argument_documentation(params), return_doc)
        )

    def ident_to_rust(self) -> str:
        return inflection.camel_to_snake_case(self.ident)

    def params_to_rust(self, ctx: Context) -> List[RustParam]:
        return [param.to_rust(ctx) for param in self.params]

    def return_type_to_rust(self, ctx: Context) -> Optional[TypeWithDocumentation]:
        if ty := self.return_type:
            return ty.to_rust(ctx, True)
        return None

    def to_rust_signature(
        self, ident: str, params: Iterable[RustParam], ret: Optional[str]
    ) -> str:
        params = ", ".join(map(str, params))
        signature = f"pub fn {ident}({params})"

        if ret := ret:
            signature += f" -> {ret}"

        return signature

    def wasm_bindgen_attr(self) -> str:
        variadic = "variadic" if any(param.variadic for param in self.params) else None
        return helpers.build_wasm_bindgen_attr(variadic, js_name=f'"{self.ident}"')

    def to_rust(self, ctx: Context) -> str:
        ident = self.ident_to_rust()
        ctx = ctx.push(ident)

        params = self.params_to_rust(ctx)
        ret = self.return_type_to_rust(ctx)

        signature = self.to_rust_signature(ident, params, ret)
        return helpers.join_nonempty_lines(
            (
                self.rust_documentation(params, ret),
                self.wasm_bindgen_attr(),
                f"{signature};",
            )
        )
