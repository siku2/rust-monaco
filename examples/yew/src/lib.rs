use monaco::{api::CodeEditorOptions, sys::editor::BuiltinTheme, yew::CodeEditor};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use yew::{html, Component, Html, Context};

const CONTENT: &str = include_str!("lib.rs");

fn get_options() -> CodeEditorOptions {
    CodeEditorOptions::default()
        .with_language("rust".to_owned())
        .with_value(CONTENT.to_owned())
        .with_builtin_theme(BuiltinTheme::VsDark)
}

struct App {
    options: Rc<CodeEditorOptions>,
}
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_context: &Context<Self>) -> Self {
        Self {
            options: Rc::new(get_options()),
        }
    }

    fn changed(&mut self, _context: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        html! {
            <CodeEditor options={ Rc::clone(&self.options) } />
        }
    }
}

#[wasm_bindgen(start)]
pub fn start_app() {
    yew::start_app::<App>();
}
