import dataclasses
import re
from typing import List, Optional, Tuple

from .helpers import (
    MatchError,
    ModSet,
    add_line_prefix,
    build_wasm_bindgen_attr,
    camel_to_snake_case,
    consume_empty_lines,
    consume_match,
    join_nonempty_lines,
    read_until_closing_bracket,
    split_trim,
)
from .js_type import JsType
from .models import Documented


@dataclasses.dataclass()
class JsMember(Documented):
    class_: str
    ident: str

    @staticmethod
    def consume(s: str, class_: str) -> Tuple["JsMember", str]:
        try:
            return JsMethod.consume(s, class_)
        except MatchError:
            return JsProperty.consume(s, class_)

    def this_parameter(self) -> str:
        return f"this: &{self.class_}"

    def build_wasm_bindgen_attr(self, *args: str, **kwargs: str) -> str:
        return build_wasm_bindgen_attr(
            *args, js_class=f'"{self.class_}"', js_name=f'"{self.ident}"', **kwargs
        )


_PATTERN_PROPERTY = re.compile(
    r"^ *(?P<mods>(?:static |readonly )*)(?P<ident>\w+)(?P<optional>\??):\s*(?P<type>.+?);\n",
    re.DOTALL,
)


@dataclasses.dataclass()
class JsProperty(JsMember):
    type_: JsType
    static: bool
    readonly: bool
    optional: bool

    @classmethod
    def consume(cls, s: str, class_: str) -> Tuple["JsProperty", str]:
        doc, s = Documented.consume(s)
        match, s = consume_match(_PATTERN_PROPERTY, s)
        s = consume_empty_lines(s)

        mods = ModSet.create(match["mods"])
        static = mods.pop("static")
        readonly = mods.pop("readonly")
        mods.assert_empty()

        optional = bool(match["optional"])

        method = cls(
            documentation=doc,
            class_=class_,
            ident=match["ident"],
            type_=JsType(match["type"]),
            static=static,
            readonly=readonly,
            optional=optional,
        )
        return method, s

    def rust_type(self, owned: bool) -> str:
        ty = self.type_.to_rust(owned)
        return f"Option<{ty}>" if self.optional else ty

    def type_documentation(self) -> Optional[str]:
        if doc := self.type_.to_rust(True).documentation:
            raw = f"\nType: {doc}"
            doc = add_line_prefix(raw, "/// ", empty_lines=True)
        return doc

    def rust_documentation(self) -> str:
        doc = super().rust_documentation()
        return join_nonempty_lines((doc, self.type_documentation()))

    def this_parameter(self) -> str:
        if not self.static:
            return super().this_parameter()

        return ""

    def getter_signature(self) -> str:
        return f"pub fn {camel_to_snake_case(self.ident)}({self.this_parameter()}) -> {self.rust_type(True)}"

    def setter_signature(self) -> str:
        return f"pub fn set_{camel_to_snake_case(self.ident)}({self.this_parameter()}, val: {self.rust_type(False)})"

    def to_rust(self) -> str:
        method = f"static_method_of = {self.class_}" if self.static else "method"

        code = join_nonempty_lines(
            (
                self.rust_documentation(),
                self.build_wasm_bindgen_attr(method, getter=self.ident),
                f"{self.getter_signature()};",
            )
        )

        if not self.readonly:
            code = join_nonempty_lines(
                (
                    code,
                    f"/// Set the `{self.ident}` property.",
                    self.build_wasm_bindgen_attr("method", setter=self.ident),
                    f"{self.setter_signature()};",
                )
            )

        return code


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
        match, s = consume_match(_PATTERN_PARAM, s)
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
            return f"* `{camel_to_snake_case(self.ident)}` - {type_doc}"

        return None

    def to_rust(self) -> str:
        rust_ident = camel_to_snake_case(self.ident)
        return f"{rust_ident}: {self.type_.to_rust(False)}"


_PATTERN_METHOD = re.compile(
    r"^ *(?P<mods>(?:get |set |static )*)(?P<ident>\w+)(?P<optional>\??)\((?P<params>.*?)\)(?:: (?P<res>.+?))?;\n",
    re.DOTALL,
)


@dataclasses.dataclass()
class JsMethod(JsMember):
    params: List[JsParameter]
    return_type: Optional[JsType]
    is_static: bool
    is_getter: bool
    is_setter: bool

    @classmethod
    def consume(cls, s: str, class_: str) -> Tuple["JsMethod", str]:
        doc, s = Documented.consume(s)
        match, s = consume_match(_PATTERN_METHOD, s)
        s = consume_empty_lines(s)

        if match["optional"]:
            raise ValueError("can't handle optional functions right now")

        mods = ModSet.create(match["mods"])
        is_static = mods.pop("static")
        is_getter = mods.pop("get")
        is_setter = mods.pop("set")
        mods.assert_empty()

        params = JsParameter.parse_multiple(match["params"])

        if return_type := match["res"]:
            if return_type == "void":
                return_type = None
            else:
                return_type = JsType(return_type)

        method = cls(
            documentation=doc,
            class_=class_,
            ident=match["ident"],
            params=params,
            return_type=return_type,
            is_static=is_static,
            is_getter=is_getter,
            is_setter=is_setter,
        )
        return method, s

    def argument_documentation(self) -> Optional[str]:
        args = list(filter(None, (param.type_documentation() for param in self.params)))
        if not args:
            return None
        raw = "\n".join(("", "# Arguments", "", *args,))
        return add_line_prefix(raw, "/// ", empty_lines=True)

    def rust_documentation(self) -> str:
        doc = super().rust_documentation()
        return join_nonempty_lines((doc, self.argument_documentation()))

    def params_to_rust(self) -> List[str]:
        params = [param.to_rust() for param in self.params]
        if not self.is_static:
            params.insert(0, self.this_parameter())

        return params

    def return_type_to_rust(self) -> Optional[str]:
        if ty := self.return_type:
            return ty.to_rust(True)
        return None

    def wasm_bindgen_attr(self) -> str:
        args = []
        kwargs = {}
        if self.is_static:
            kwargs["static_method_of"] = self.class_
        else:
            args.append("method")

        if self.is_getter:
            kwargs["getter"] = self.ident
        if self.is_setter:
            kwargs["setter"] = self.ident

        return self.build_wasm_bindgen_attr(*args, **kwargs)

    def to_rust_signature(self) -> str:
        params = ", ".join(self.params_to_rust())
        ident = camel_to_snake_case(self.ident)
        if self.is_setter:
            ident = f"set_{ident}"

        signature = f"pub fn {ident}({params})"

        ret_type = self.return_type_to_rust()
        if ret_type:
            signature += f" -> {ret_type}"

        return signature

    def to_rust(self) -> str:
        signature = self.to_rust_signature()
        return join_nonempty_lines(
            (self.rust_documentation(), self.wasm_bindgen_attr(), f"{signature};",)
        )


_PATTERN_OBJECT_OPEN = re.compile(
    r"^ *export (?P<type>interface|class) (?P<ident>\w+)\s+(?:extends (?P<extends>\w+(?:,\s*\w+\s*)*) )?(?:implements (?P<implements>\w+(?:,\s*\w+\s*)*) )?{\n"
)


@dataclasses.dataclass()
class JsObject(Documented):
    ident: str
    members: List[JsMember]
    extends: List[str]
    implements: List[str]

    @staticmethod
    def consume(s: str) -> Tuple["JsObject", str]:
        doc, s = Documented.consume(s)
        match, s = consume_match(_PATTERN_OBJECT_OPEN, s)
        s = consume_empty_lines(s)

        ident = match["ident"]

        if extends := match["extends"]:
            extends = split_trim(extends, ",")

        if implements := match["implements"]:
            implements = split_trim(implements, ",")

        cls = JsClass if match["type"] == "class" else JsInterface

        members = []
        obj = cls(
            documentation=doc,
            ident=ident,
            members=members,
            extends=extends,
            implements=implements,
        )

        body, s = read_until_closing_bracket(s)
        s = consume_empty_lines(s)
        while body:
            member, body = JsMember.consume(body, ident)
            members.append(member)

        return obj, s

    def wasm_bindgen_attr(self, /, extends: List[str] = None) -> str:
        if extends is None:
            extends = []

        if v := self.extends:
            extends.extend(v)
        if v := self.implements:
            extends.extend(v)
        if extends:
            return build_wasm_bindgen_attr(extends=extends)

        return ""

    def to_rust(self) -> str:
        return join_nonempty_lines(
            (
                self.rust_documentation(),
                f"#[derive(Debug)]",
                self.wasm_bindgen_attr(),
                f"pub type {self.ident};",
                join_nonempty_lines(member.to_rust() for member in self.members),
            )
        )


@dataclasses.dataclass()
class JsClass(JsObject):
    ...


@dataclasses.dataclass()
class JsInterface(JsObject):
    def wasm_bindgen_attr(self) -> str:
        return super().wasm_bindgen_attr(extends=["Object"])
