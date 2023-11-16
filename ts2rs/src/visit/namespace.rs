use crate::visit::{translate::TranslateVisitor, ModuleContext};
use swc_ecma_ast::*;
use swc_ecma_visit::*;

pub struct NamespaceVisitor<'a> {
    context: &'a mut ModuleContext,
    name: Vec<&'a str>,
}

impl<'a> NamespaceVisitor<'a> {
    pub fn new(context: &'a mut ModuleContext) -> Self {
        Self {
            context,
            name: vec![],
        }
    }
}

impl<'a> Visit for NamespaceVisitor<'a> {
    fn visit_ts_module_decl(&mut self, n: &TsModuleDecl) {
        if let Some(id) = n.id.as_ident() {
            let id = id.as_ref();

            let name = self.name.iter().map(|s| *s).chain([id]).collect::<Vec<_>>();

            let context = self.context.push(id);
            n.visit_children_with(&mut NamespaceVisitor { context, name });
            n.visit_children_with(&mut TranslateVisitor { context });
        }
    }

    fn visit_ts_namespace_decl(&mut self, n: &TsNamespaceDecl) {
        let id = n.id.as_ref();
        let name = self.name.iter().map(|s| *s).chain([id]).collect::<Vec<_>>();

        let context = self.context.push(id);
        n.visit_children_with(&mut NamespaceVisitor { context, name });
        n.visit_children_with(&mut TranslateVisitor { context });
    }
}
