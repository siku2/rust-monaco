use crate::types::make_optional;
use crate::{ident::ident, types};
use std::{collections::HashMap, io::Write};
use swc_ecma_ast::*;

pub fn unique_fn(mut name: String, names: &mut HashMap<String, usize>) -> String {
    let c = names.entry(name.clone()).or_default();

    if *c > 0 {
        name = format!("{name}_{c}");
    }

    *c += 1;

    name
}

pub fn function_parameters(w: &mut dyn Write, params: &[Param]) {
    for p in params {
        match &p.pat {
            Pat::Ident(id) => {
                // println!("Param: {:?}", &id.as_ref());
                let mut r#type = "JsValue".to_string();

                if let Some(ann) = &id.type_ann {
                    r#type = types::type_ann_name(&ann);
                }

                r#type = make_optional(r#type, id.optional);

                write!(w, "{}: {type}, ", ident(id.as_ref())).unwrap();
            }
            _ => {}
        }
    }
}

pub fn function_parameters_ts(w: &mut dyn Write, params: &[TsFnParam]) {
    for p in params {
        match p {
            TsFnParam::Ident(id) => {
                // println!("Param: {:?}", &id.as_ref());
                let mut r#type = "JsValue".to_string();

                if let Some(ann) = &id.type_ann {
                    r#type = types::type_ann_name(&ann);
                }

                r#type = make_optional(r#type, id.optional);

                write!(w, "{}: {type}, ", ident(id.as_ref())).unwrap();
            }
            _ => {}
        }
    }
}
