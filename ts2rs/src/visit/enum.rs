use crate::{comments::insert_comments, eval::eval_const, visit::ModuleContext};
use std::ops::DerefMut;
use swc_ecma_ast::*;
use swc_ecma_visit::*;

pub struct EnumVisitor<'a> {
    pub name: &'a str,
    pub context: &'a mut ModuleContext,
}

impl<'a> Visit for EnumVisitor<'a> {
    fn visit_ts_enum_members(&mut self, n: &[TsEnumMember]) {
        for e in n {
            let comments = self.context.comments.get_leading(e.span.comment_range().lo);
            insert_comments(self.context.deref_mut(), comments);

            let name = e.id.as_ref().as_ref();
            let init = match &e.init {
                Some(expr) => format!(" = {}", eval_const(&expr)),
                None => String::new(),
            };
            writeln!(self.context, r#"    {name}{init},"#).unwrap();
        }
    }
}
