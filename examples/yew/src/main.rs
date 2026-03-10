use monaco::{api::CodeEditorOptions, sys::editor::BuiltinTheme, yew::CodeEditor};
use yew::prelude::*;

const CONTENT: &str = include_str!("main.rs");

fn get_options() -> CodeEditorOptions {
    CodeEditorOptions::default()
        .with_language("rust".to_owned())
        .with_value(CONTENT.to_owned())
        .with_builtin_theme(BuiltinTheme::VsDark)
        .with_automatic_layout(true)
}

#[component]
fn App() -> Html {
    let options = get_options().to_sys_options();

    html! {
        <CodeEditor classes={"full-height"} options={options} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
