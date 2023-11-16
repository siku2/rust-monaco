use crate::{
    comments::insert_comments,
    gen::{function_parameters, unique_fn},
    ident::ident,
    types,
    visit::ModuleContext,
};
use std::{collections::HashMap, ops::DerefMut};
use swc_ecma_ast::*;
use swc_ecma_visit::*;

pub struct ClassVisitor<'a> {
    pub name: &'a str,
    pub context: &'a mut ModuleContext,
    pub fn_names: HashMap<String, usize>,
}

impl<'a> ClassVisitor<'a> {
    pub fn new(name: &'a str, context: &'a mut ModuleContext) -> Self {
        Self {
            name,
            context,
            fn_names: Default::default(),
        }
    }
}

impl<'a> Visit for ClassVisitor<'a> {
    fn visit_class_method(&mut self, n: &ClassMethod) {
        if let Some(name) = n.key.as_ident() {
            let name = name.as_ref();
            let fn_name = unique_fn(ident(name), &mut self.fn_names);

            let r#type = self.name;

            // println!("Method: {type}#{name}");

            let comments = self.context.comments.get_leading(n.span.comment_range().lo);
            insert_comments(self.context.deref_mut(), comments);

            let mut attrs = vec![
                r#"js_class = "{type}""#.to_string(),
                r#"js_name = "{name}""#.to_string(),
            ];
            if n.kind == MethodKind::Getter {
                attrs.push(format!("getter = {name}"));
            }
            if n.is_static {
                attrs.push(format!("static_method_of = {type}"));
            } else {
                attrs.push("method".to_string())
            }

            write!(
                self.context,
                r#"
    #[wasm_bindgen({attrs})]
    pub fn {fn_name}("#,
                attrs = attrs.join(", "),
            )
            .unwrap();

            if !n.is_static {
                write!(self.context, "this: &{}, ", self.name).unwrap();
            }

            function_parameters(self.context.deref_mut(), &n.function.params);

            write!(self.context, r#")"#).unwrap();

            if let Some(rt) = n
                .function
                .return_type
                .as_ref()
                .and_then(|t| types::as_ret(&t, n.is_optional))
            {
                write!(self.context, " -> {rt}").unwrap();
            }

            writeln!(self.context, r#";"#).unwrap();
            writeln!(self.context).unwrap();

            // FIXME: handle setter
        }
    }

    fn visit_constructor(&mut self, n: &Constructor) {
        let r#type = self.name;

        let comments = self.context.comments.get_leading(n.span.comment_range().lo);
        insert_comments(self.context.deref_mut(), comments);

        write!(
            self.context,
            r#"  #[wasm_bindgen(constructor, js_class = "{type}")]
  pub fn new("#
        )
        .unwrap();

        n.visit_children_with(self);

        writeln!(
            self.context,
            r#") -> {type};
"#
        )
        .unwrap();
    }
}
