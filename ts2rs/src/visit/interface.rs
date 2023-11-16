use crate::{
    comments::insert_comments,
    gen::{function_parameters_ts, unique_fn},
    ident::ident,
    types,
    visit::ModuleContext,
};
use convert_case::{Case, Casing};
use std::{collections::HashMap, ops::DerefMut};
use swc_common::comments::*;
use swc_ecma_ast::*;
use swc_ecma_visit::*;

pub struct InterfaceVisitor<'a> {
    pub name: &'a str,
    pub context: &'a mut ModuleContext,
    pub fn_names: HashMap<String, usize>,
}

impl<'a> InterfaceVisitor<'a> {
    pub fn new(name: &'a str, context: &'a mut ModuleContext) -> Self {
        Self {
            name,
            context,
            fn_names: Default::default(),
        }
    }
}

impl<'a> Visit for InterfaceVisitor<'a> {
    fn visit_ts_method_signature(&mut self, n: &TsMethodSignature) {
        let name = if let Some(id) = n.key.as_ident() {
            id.as_ref()
        } else {
            return;
        };

        let fn_name = unique_fn(ident(name), &mut self.fn_names);

        // writeln!(self.context, "// optional: {}", n.optional).unwrap();
        // writeln!(self.context, "// readonly: {}", n.readonly).unwrap();

        let comments = self.context.comments.get_leading(n.span.comment_range().lo);
        insert_comments(self.context.deref_mut(), comments);

        let r#type = self.name;
        let attrs = vec![
            "method".to_string(),
            format!("js_class = \"{type}\"",),
            format!("js_name = \"{name}\""),
        ];

        write!(
            self.context,
            r#"    #[wasm_bindgen({attrs})]
    pub fn {fn_name}(this: &{type}, "#,
            attrs = attrs.join(", ")
        )
        .unwrap();

        function_parameters_ts(self.context.deref_mut(), &n.params);

        write!(self.context, ")").unwrap();

        let rt = n
            .type_ann
            .as_ref()
            .and_then(|ann| types::as_ret(&ann, n.optional));
        if let Some(rt) = &rt {
            write!(self.context, " -> {rt}").unwrap();
        }

        writeln!(self.context, ";").unwrap();
        writeln!(self.context).unwrap();
    }

    fn visit_ts_property_signature(&mut self, n: &TsPropertySignature) {
        let name = if let Some(id) = n.key.as_ident() {
            id.as_ref()
        } else {
            return;
        };

        let rt = n.type_ann.as_ref().map(|ann| types::type_ann_name(&ann));

        // writeln!(self.context, "// optional: {}", n.optional).unwrap();
        // writeln!(self.context, "// readonly: {}", n.readonly).unwrap();

        let comments = self.context.comments.get_leading(n.span.comment_range().lo);
        insert_comments(self.context.deref_mut(), comments);

        let r#type = self.name;
        let attrs = vec![
            "method".to_string(),
            format!("js_class = \"{type}\"",),
            format!("js_name = \"{name}\""),
            "getter".to_string(),
        ];

        write!(
            self.context,
            r#"    #[wasm_bindgen({attrs})]
    pub fn {name}(this: &{type})"#,
            name = ident(name),
            attrs = attrs.join(", ")
        )
        .unwrap();

        if let Some(rt) = &rt {
            write!(self.context, " -> {rt}").unwrap();
        }

        writeln!(self.context, ";").unwrap();

        if !n.readonly {
            if let Some(rt) = rt {
                let attrs = vec![
                    "method".to_string(),
                    format!("js_class = \"{type}\"",),
                    format!("js_name = \"{name}\""),
                    "setter".to_string(),
                ];

                write!(
                    self.context,
                    r#"
    #[wasm_bindgen({attrs})]
    pub fn set_{fn_name}(this: &{type}, {name}: {rt})"#,
                    fn_name = name.to_case(Case::Snake),
                    name = ident(&name.to_case(Case::Snake)),
                    attrs = attrs.join(", ")
                )
                .unwrap();

                writeln!(self.context, ";").unwrap();
                writeln!(self.context).unwrap();
            }
        }
    }
}
