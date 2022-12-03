use monaco::{api::CodeEditorOptions, sys::editor::BuiltinTheme, yew::CodeEditor};
use std::rc::Rc;
use yew::{html, Component, Context, Html};

const CONTENT: &str = include_str!("main.rs");

fn get_options() -> CodeEditorOptions {
    CodeEditorOptions::default()
        .with_language("rust".to_owned())
        .with_value(CONTENT.to_owned())
        .with_builtin_theme(BuiltinTheme::VsDark)
        .with_automatic_layout(true)
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

    fn changed(&mut self, _context: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        html! {
            <CodeEditor classes={"full-height"} options={ self.options.to_sys_options() } />
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
