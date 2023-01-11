use monaco::{
    api::{CodeEditorOptions, TextModel},
    sys::editor::BuiltinTheme,
    yew::CodeEditor,
};

use rand::{distributions::Alphanumeric, Rng};
use yew::prelude::*;

const CONTENT: &str = include_str!("main.rs");

fn get_options() -> CodeEditorOptions {
    CodeEditorOptions::default()
        .with_language("rust".to_owned())
        .with_value(CONTENT.to_owned())
        .with_builtin_theme(BuiltinTheme::VsDark)
        .with_automatic_layout(true)
}

#[derive(PartialEq, Properties)]
pub struct CustomEditorProps {
    text_model: TextModel,
}

/// This is really just a helper component, so we can pass in props easier.
/// It makes it much easier to use, as we can pass in what we need, and it
/// will only re-render if the props change.
#[function_component(CustomEditor)]
pub fn custom_editor(props: &CustomEditorProps) -> Html {
    let CustomEditorProps { text_model } = props;

    html! {
        <CodeEditor classes={"full-height"} options={ get_options().to_sys_options() } model={text_model.clone()} />
    }
}

// inefficient and silly, but fine for demo :-)
fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(50)
        .map(char::from)
        .collect()
}

#[function_component(App)]
fn app() -> Html {
    // We need to create a new text model, so we can pass it to Monaco.
    // We use use_state_eq, as this allows us to only use it when it changes.
    let text_model =
        use_state_eq(|| TextModel::create(&random_string(), Some("rust"), None).unwrap());

    // Here we define what we want to do when we click the button, which is bound to
    // the button below. We use the `use_callback` function in Yew to bind the
    // function to the button. We can then clone the text model, and use it in
    // the callback to set the value to whatever we want.
    let on_run_clicked = {
        let text_model = text_model.clone();
        use_callback(
            move |_, text_model| {
                let s: String = random_string();
                // Here we have full access to the text model. We can do whatever we want with
                // it. For this example, we'll just set the value to a random
                // string.
                text_model.set_value(&s);
            },
            text_model,
        )
    };

    html! {
        <div id="code-wrapper">
            <div id="code-editor">
                <CustomEditor text_model={(*text_model).clone()} />
            </div>
            <div id="event-log-wrapper">
                <div id="event-log">
                    <button onclick={on_run_clicked}>{ "Random code" }</button>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
