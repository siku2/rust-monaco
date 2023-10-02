use crate::{
    comments::insert_comments,
    eval::eval_const,
    types::type_name,
    visit::{class::*, interface::InterfaceVisitor, r#enum::EnumVisitor, ModuleContext},
};
use std::ops::DerefMut;
use swc_ecma_ast::*;
use swc_ecma_visit::*;

pub struct TranslateVisitor<'a> {
    pub context: &'a mut ModuleContext,
}

impl<'a> Visit for TranslateVisitor<'a> {
    fn visit_ts_module_decl(&mut self, _: &TsModuleDecl) {}

    fn visit_ts_namespace_decl(&mut self, _: &TsNamespaceDecl) {}

    fn visit_ts_type_alias_decl(&mut self, n: &TsTypeAliasDecl) {
        let name = n.id.as_ref();

        println!("Type alias: {name}");

        let comments = self.context.comments.get_leading(n.span.comment_range().lo);
        insert_comments(self.context.deref_mut(), comments);

        writeln!(
            self.context,
            r#"pub type {name} = {}"#,
            type_name(&n.type_ann)
        )
        .unwrap();

        writeln!(self.context, r#";"#).unwrap();
    }

    fn visit_ts_enum_decl(&mut self, n: &TsEnumDecl) {
        let name = n.id.as_ref();

        println!("Enum: {name}");

        let comments = self.context.comments.get_leading(n.span.comment_range().lo);
        insert_comments(self.context.deref_mut(), comments);

        writeln!(
            self.context,
            r#"
            int_enum! {{
#[allow(non_camel_case_types)]
pub enum {name} {{"#
        )
        .unwrap();
        n.visit_children_with(&mut EnumVisitor {
            name,
            context: self.context,
        });
        writeln!(
            self.context,
            r#"
  }}
}}"#
        )
        .unwrap();
        writeln!(self.context).unwrap();
    }

    fn visit_class_decl(&mut self, n: &ClassDecl) {
        println!("Class: {}", n.ident.as_ref());

        let name = n.ident.as_ref();

        let comments = self
            .context
            .comments
            .get_leading(n.class.span.comment_range().lo);
        insert_comments(self.context.deref_mut(), comments);

        let mut extends = vec!["js_sys::Object".to_string()];

        if let Some(s) = &n.class.super_class {
            extends.push(eval_const(&s));
        }

        let extends = extends
            .into_iter()
            .map(|base| format!("extends = {base}"))
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(
            self.context,
            r#"
#[wasm_bindgen]
extern "C" {{
    #[derive(Debug)]
    #[wasm_bindgen({extends})]
    pub type {name};
"#
        )
        .unwrap();

        n.visit_children_with(&mut ClassVisitor::new(name, self.context));

        writeln!(
            self.context,
            r#"
}}
"#
        )
        .unwrap();
    }

    fn visit_ts_interface_decl(&mut self, n: &TsInterfaceDecl) {
        println!("Interface: {}", n.id.as_ref());

        let comments = self.context.comments.get_leading(n.span.comment_range().lo);
        insert_comments(self.context.deref_mut(), comments);

        let name = n.id.as_ref();

        writeln!(
            self.context,
            r#"
#[wasm_bindgen]
extern "C" {{
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type {name};
"#
        )
        .unwrap();

        n.visit_children_with(&mut InterfaceVisitor::new(name, self.context));

        writeln!(
            self.context,
            r#"
}}
"#
        )
        .unwrap();
    }
}
