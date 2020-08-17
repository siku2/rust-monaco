from .js_enum import JsEnum
from .js_namespace import JsNamespace
from .js_object import JsObject

# TODO use copy type when appropriate (keep track of which idents resolve to enums)
# TODO Add "_" to Rust idents which conflict with keywords
#! TODO handle nested namespaces using js_namespace


# TODO maybe parse docstring to extract @param
#! FIXME: check the docstring for "throws" and have them return Result<>
