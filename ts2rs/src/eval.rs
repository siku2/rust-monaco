use swc_ecma_ast::*;

pub fn eval_const(expr: &Expr) -> String {
    match expr {
        Expr::Lit(Lit::Num(n)) => format!("{n}"),
        Expr::Lit(Lit::Str(s)) => format!("\"{}\"", s.value.as_ref()),
        Expr::Unary(UnaryExpr {
            op: UnaryOp::Minus,
            arg,
            ..
        }) => match arg.as_ref() {
            Expr::Lit(Lit::Num(n)) => format!("-{n}"),
            _ => unimplemented!("Only numeric literals are supported: {expr:?}"),
        },

        Expr::Ident(id) => id.sym.to_string(),

        _ => unimplemented!("Only numeric literals are supported: {expr:?}"),
    }
}
