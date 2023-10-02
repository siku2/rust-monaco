use swc_ecma_ast::*;

fn expand_name(name: &TsEntityName) -> String {
    match name {
        TsEntityName::Ident(id) => id.sym.as_ref().to_string(),
        TsEntityName::TsQualifiedName(id) => {
            format!("{}::{}", expand_name(&id.left), id.right.sym.as_ref())
        }
    }
}

pub fn as_ret(ann: &TsTypeAnn, optional: bool) -> Option<String> {
    let name = type_ann_name(ann);
    if name == "()" {
        None
    } else if optional {
        if name != "JsValue" {
            // JsValue is optional
            Some(format!("Option<{name}>"))
        } else {
            Some(name)
        }
    } else {
        Some(name)
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

    result
}

pub fn type_ann_name(ann: &TsTypeAnn) -> String {
    type_name(ann.type_ann.as_ref())
}
