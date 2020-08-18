import dataclasses
import re
from typing import List, Optional, Tuple

from . import helpers, inflection
from .js_type import JsType
from .models import Documented


_PATTERN_PARAM = re.compile(
    r"^\s*(?P<ident>\w+)(?P<optional>\??): (?P<type>.+?)(?:,\s*|\s*$)", re.DOTALL
)


@dataclasses.dataclass()
class JsParameter:
    ident: str
    type_: JsType
    optional: bool

    @classmethod
    def consume(cls, s: str) -> Tuple["JsParameter", str]:
        match, s = helpers.consume_match(
            _PATTERN_PARAM, s, skip_empty_lines_after=False
        )
        optional = bool(match["optional"])
        param = cls(
            ident=match["ident"], type_=JsType(match["type"]), optional=optional,
        )
        return param, s

    @classmethod
    def parse_multiple(cls, s: str) -> List["JsParameter"]:
        params = []
        while s:
            param, s = cls.consume(s)
            params.append(param)

        return params

    def type_documentation(self) -> Optional[str]:
        type_doc = self.type_.to_rust(True).documentation
        if type_doc:
            return f"* `{inflection.camel_to_snake_case(self.ident)}` - {type_doc}"

        return None

    def to_rust(self) -> str:
        rust_ident = inflection.camel_to_snake_case(self.ident)
        return f"{rust_ident}: {self.type_.to_rust(False)}"


_PATTERN_FUNCTION = re.compile(
    r"^ *export function (?P<ident>\w+)\((?P<params>.*?)\)(?:: (?P<ret>.+?))?;",
    re.DOTALL,
)


@dataclasses.dataclass()
class JsFunction(Documented):
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

    def argument_documentation(self) -> Optional[str]:
        args = list(filter(None, (param.type_documentation() for param in self.params)))
        if not args:
            return None
        raw = "\n".join(("", "# Arguments", "", *args,))
        return helpers.add_line_prefix(raw, "/// ", empty_lines=True)

    def rust_documentation(self) -> str:
        doc = super().rust_documentation()
        return helpers.join_nonempty_lines((doc, self.argument_documentation()))

    def ident_to_rust(self) -> str:
        return inflection.camel_to_snake_case(self.ident)

    def params_to_rust(self) -> List[str]:
        return [param.to_rust() for param in self.params]

    def return_type_to_rust(self) -> Optional[str]:
        if ty := self.return_type:
            return ty.to_rust(True)
        return None

    def to_rust_signature(self) -> str:
        ident = self.ident_to_rust()
        params = ", ".join(self.params_to_rust())
        signature = f"pub fn {ident}({params})"

        if ret := self.return_type_to_rust():
            signature += f" -> {ret}"

        return signature

    def wasm_bindgen_attr(self) -> str:
        return helpers.build_wasm_bindgen_attr(js_name=f'"{self.ident}"')

    def to_rust(self) -> str:
        signature = self.to_rust_signature()
        return helpers.join_nonempty_lines(
            (self.rust_documentation(), self.wasm_bindgen_attr(), f"{signature};")
        )
