use swc_ecma_ast::*;

fn expand_name(name: &TsEntityName) -> String {
    match name {
        TsEntityName::Ident(id) => id.sym.as_ref().to_string(),
        TsEntityName::TsQualifiedName(id) => {
            format!("{}::{}", expand_name(&id.left), id.right.sym.as_ref())
        }
    }
}

pub fn make_optional(name: String, optional: bool) -> String {
    if optional && (name != "JsValue" && name != "ProviderResult") {
        format!("Option<{name}>")
    } else {
        // JsValue is optional
        name
    }
}

pub fn as_ret(ann: &TsTypeAnn, optional: bool) -> Option<String> {
    let name = type_ann_name(ann);
    if name == "()" {
        None
    } else {
        Some(make_optional(name, optional))
    }
}

pub fn type_name(t: &TsType) -> String {
    let mut result = "JsValue".to_string();

    match t {
        TsType::TsKeywordType(TsKeywordType { kind, .. }) => match kind {
            TsKeywordTypeKind::TsBooleanKeyword => result = "bool".to_string(),
            TsKeywordTypeKind::TsStringKeyword => result = "String".to_string(),
            TsKeywordTypeKind::TsNumberKeyword => result = "f64".to_string(),
            TsKeywordTypeKind::TsVoidKeyword => result = "()".to_string(),
            TsKeywordTypeKind::TsAnyKeyword => {}
            n => log::warn!("Unknown keyword: {n:?}"),
        },
        TsType::TsArrayType(_) => {
            // FIXME: yes, we can do better
            result = "js_sys::Array".to_string();
        }
        TsType::TsTypeLit(_) => {
            // FIXME: we could create a dedicate type for this
            result = "js_sys::Object".to_string();
        }
        TsType::TsTypeRef(r) => result = expand_name(&r.type_name),
        n => {
            log::warn!("Unsupported annotation type: {n:?}")
        }
    }

    fixes(result)
}

pub fn type_ann_name(ann: &TsTypeAnn) -> String {
    type_name(ann.type_ann.as_ref())
}

/// Apply some fixes which seem to end up with.
fn fixes(mut name: String) -> String {
    // fix enum literal as type
    name = fix_enum_literal(name);
    // fix type argument
    name = fix_type_argument(name);
    // done
    name
}

fn fix_enum_literal(name: String) -> String {
    if let Some((first, second)) = name.rsplit_once("::") {
        if second.chars().all(|c| c.is_uppercase() || c == '_') {
            return first.to_string();
        }
    }

    name
}

fn fix_type_argument(name: String) -> String {
    if name.len() == 1 && name.chars().all(char::is_uppercase) {
        "JsValue".to_string()
    } else {
        name
    }
}
