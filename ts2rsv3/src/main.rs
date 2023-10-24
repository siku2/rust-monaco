mod comments;
mod eval;
mod gen;
mod ident;
mod types;
mod visit;

use crate::visit::{namespace::NamespaceVisitor, ModuleContext};
use std::{fs::File, path::PathBuf, rc::Rc, sync::Arc};
use swc_common::{
    comments::SingleThreadedComments, input::StringInput, FilePathMapping, SourceMap,
};
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::{lexer::Lexer, Parser, Syntax, TsConfig};
use swc_ecma_visit::Visit;

const SOURCE: &str = "../build/node_modules/monaco-editor/monaco.d.ts";

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let fm = cm.load_file(&PathBuf::from(SOURCE))?;

    let comments = SingleThreadedComments::default();

    let syntax = Syntax::Typescript(TsConfig::default());
    let lexer = Lexer::new(
        syntax,
        EsVersion::Es2022,
        StringInput::from(&*fm),
        Some(&comments),
    );
    let mut parser = Parser::new_from(lexer);

    let module = parser.parse_module().unwrap();

    let s = &format!("{module:#?}");
    std::fs::write("ts.dump", s)?;

    let comments = Rc::new(comments);

    let mut context = ModuleContext::new(comments);
    NamespaceVisitor::new(&mut context).visit_module(&module);

    {
        let mut out = File::create("bindings/src/monaco.rs")?;
        context.flush(&mut out)?;
    }

    log::info!("Done");

    Ok(())
}
