import abc
import dataclasses
import re
from typing import List, Optional, Tuple

from . import helpers, inflection
from .helpers import MatchError, ModSet
from .js_function import JsFunction, RustParam
from .js_type import JsType, TypeWithDocumentation
from .models import Context, Documented, ToRust


@dataclasses.dataclass()
class JsMember(Documented, ToRust, abc.ABC):
    class_: str
    ident: str

    @staticmethod
    def consume(s: str, class_: str) -> Tuple["JsMember", str]:
        return helpers.consume_first(s, JsMethod, JsProperty, args=(class_,))

    def this_parameter(self) -> RustParam:
        return RustParam(ident="this", ty=f"&{self.class_}")

    def build_wasm_bindgen_attr(self, *args: str, **kwargs: str) -> str:
        return helpers.build_wasm_bindgen_attr(
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
        match, s = helpers.consume_match(_PATTERN_PROPERTY, s)

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

    def type_to_rust(self, ctx: Context, owned: bool) -> TypeWithDocumentation:
        ty = self.type_.to_rust(ctx, owned, create_helpers=owned)
        return (
            ty.to_option()
            if self.optional
            else ty
        )

    def type_documentation(self, ty: TypeWithDocumentation) -> Optional[str]:
        if doc := ty.documentation:
            raw = f"\nType: {doc}"
            doc = helpers.add_line_prefix(raw, "/// ", empty_lines=True)
        return doc

    def rust_documentation(self, ty: TypeWithDocumentation) -> str:
        doc = super().rust_documentation()
        return helpers.join_nonempty_lines((doc, self.type_documentation(ty)))

    def this_parameter(self) -> Optional[RustParam]:
        if not self.static:
            return super().this_parameter()

        return None

    def to_rust(self, ctx: Context) -> str:
        method = f"static_method_of = {self.class_}" if self.static else "method"
        getter_ident = f"{inflection.camel_to_snake_case(self.ident)}"
        ctx = ctx.push(getter_ident)

        if this_param := self.this_parameter():
            this_param = str(this_param)
        else:
            this_param = ""

        ty = self.type_to_rust(ctx, True)
        code = helpers.join_nonempty_lines(
            (
                self.rust_documentation(ty),
                self.build_wasm_bindgen_attr(method, getter=self.ident),
                f"pub fn {getter_ident}({this_param}) -> {ty};",
            )
        )

        if not self.readonly:
            ty = self.type_to_rust(ctx, False)
            code = helpers.join_nonempty_lines(
                (
                    code,
                    f"/// Set the `{self.ident}` property.",
                    self.build_wasm_bindgen_attr(method, setter=self.ident),
                    f"pub fn set_{getter_ident}({this_param}, val: {ty});",
                )
            )

        return code


_PATTERN_METHOD = re.compile(
    r"^ *(?P<mods>(?:get |set |static )*)(?P<ident>\w+)(?P<optional>\??)\((?P<params>.*?)\)(?:: (?P<ret>.+?))?;\n",
    re.DOTALL,
)


@dataclasses.dataclass()
class JsMethod(JsMember, JsFunction):
    is_static: bool
    is_getter: bool
    is_setter: bool

    @classmethod
    def consume(cls, s: str, class_: str) -> Tuple["JsMethod", str]:
        doc, s = Documented.consume(s)
        match, s = helpers.consume_match(_PATTERN_METHOD, s)

        if match["optional"]:
            raise ValueError("can't handle optional functions right now")

        mods = ModSet.create(match["mods"])
        is_static = mods.pop("static")
        is_getter = mods.pop("get")
        is_setter = mods.pop("set")
        mods.assert_empty()

        method = cls.from_match(
            documentation=doc,
            ident=match["ident"],
            params=match["params"],
            ret=match["ret"],
            class_=class_,
            is_static=is_static,
            is_getter=is_getter,
            is_setter=is_setter,
        )
        return method, s

    def ident_to_rust(self) -> str:
        ident = super().ident_to_rust()
        if self.is_setter:
            ident = f"set_{ident}"
        return ident

    def params_to_rust(self, ctx: Context) -> List[RustParam]:
        params = super().params_to_rust(ctx)
        if not self.is_static:
            params.insert(0, self.this_parameter())

        return params

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


_PATTERN_OBJECT_OPEN = re.compile(
    r"^ *export (?P<type>interface|class) (?P<ident>\w+)\s+(?:extends (?P<extends>\w+(?:,\s*\w+\s*)*) )?(?:implements (?P<implements>\w+(?:,\s*\w+\s*)*) )?{\n"
)


@dataclasses.dataclass()
class JsObject(Documented, ToRust):
    ident: str
    members: List[JsMember]
    extends: List[str]
    implements: List[str]

    @staticmethod
    def consume(s: str) -> Tuple["JsObject", str]:
        doc, s = Documented.consume(s)
        match, s = helpers.consume_match(_PATTERN_OBJECT_OPEN, s)

        ident = match["ident"]

        if extends := match["extends"]:
            extends = helpers.split_trim(extends, ",")

        if implements := match["implements"]:
            implements = helpers.split_trim(implements, ",")

        cls = JsClass if match["type"] == "class" else JsInterface

        members = []
        obj = cls(
            documentation=doc,
            ident=ident,
            members=members,
            extends=extends,
            implements=implements,
        )

        body, s = helpers.read_until_closing_bracket(s)
        while body:
            member, body = JsMember.consume(body, ident)
            members.append(member)

        return obj, s

    def wasm_bindgen_attr(self, *, extends: List[str] = None) -> str:
        if extends is None:
            extends = []

        if v := self.extends:
            extends.extend(v)
        if v := self.implements:
            extends.extend(v)
        if extends:
            return helpers.build_wasm_bindgen_attr(extends=extends)

        return ""

    def to_rust(self, ctx: Context) -> str:
        ctx = ctx.push(self.ident)
        return helpers.join_nonempty_lines(
            (
                self.rust_documentation(),
                f"#[derive(Debug)]",
                self.wasm_bindgen_attr(),
                f"pub type {self.ident};",
                *(member.to_rust(ctx) for member in self.members),
            )
        )


@dataclasses.dataclass()
class JsClass(JsObject):
    ...


@dataclasses.dataclass()
class JsInterface(JsObject):
    def wasm_bindgen_attr(self) -> str:
        return super().wasm_bindgen_attr(extends=["Object"])
