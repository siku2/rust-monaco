use convert_case::{Case, Casing};

pub fn ident(name: &str) -> String {
    match name {
        "type" | "loop" | "for" => format!("r#{name}"),
        name => name.to_case(Case::Snake),
    }
}
