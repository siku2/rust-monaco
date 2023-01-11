use monaco::{
    api::{CodeEditorOptions, TextModel},
    sys::editor::{BuiltinTheme, IStandaloneCodeEditor},
    yew::{CodeEditor, CodeEditorLink},
};
use wasm_bindgen::closure::Closure;
use yew::prelude::*;

use wasm_bindgen::JsCast;

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
    on_editor_created: Callback<CodeEditorLink>,
    text_model: TextModel,
}

///
/// This is really just a helper component, so we can pass in props easier.
/// It makes it much easier to use, as we can pass in what we need, and it
/// will only re-render if the props change.
///
#[function_component(CustomEditor)]
pub fn custom_editor(props: &CustomEditorProps) -> Html {
    let CustomEditorProps {
        on_editor_created,
        text_model,
    } = props;

    html! {
        <CodeEditor classes={"full-height"} options={ get_options().to_sys_options() } {on_editor_created} model={text_model.clone()} />
    }
}

#[function_component(App)]
fn app() -> Html {
    // We need to create a new text model, so we can pass it to Monaco.
    // We use use_state_eq, as this allows us to only use it when it changes.
    let text_model = use_state_eq(|| TextModel::create(CONTENT, Some("rust"), None).unwrap());

    // This is the current code output. As it's static from the example, we set it to the content.
    let code = use_state_eq(|| String::from(CONTENT));

    // Here we setup the Callback for when the editor is created.
    let on_editor_created = {
        // We need to clone the text_model/code so we can use them.
        let text_model = text_model.clone();
        let code = code.clone();

        // This is a javascript closure, used to pass to Monaco, using wasm-bindgen.
        let js_closure = {
            let text_model = text_model.clone();

            // We update the code state when the Monaco model changes.
            // See https://yew.rs/docs/0.20.0/concepts/function-components/pre-defined-hooks
            Closure::<dyn Fn()>::new(move || {
                code.set(text_model.get_value());
            })
        };

        // Here we define our callback, we use use_callback as we want to re-render when dependencies change.
        // See https://yew.rs/docs/concepts/function-components/state#general-view-of-how-to-store-state
        use_callback(
            move |editor_link: CodeEditorLink, _text_model| {
                editor_link.with_editor(|editor| {
                    // Registers Ctrl/Cmd + Enter hotkey
                    let keycode = monaco::sys::KeyCode::Enter.to_value()
                        | (monaco::sys::KeyMod::ctrl_cmd() as u32);
                    let raw_editor: &IStandaloneCodeEditor = editor.as_ref();

                    raw_editor.add_command(
                        keycode.into(),
                        js_closure.as_ref().unchecked_ref(),
                        None,
                    );
                });
            },
            text_model,
        )
    };
    html! {
        <div id="code-wrapper">
            <div id="code-editor">
                <CustomEditor {on_editor_created} text_model={(*text_model).clone()} />
            </div>
            <div id="event-log-wrapper">
                <div id="event-log">
                    <h2>{"Code (press CTRL+Enter / Command+Enter to view)"}</h2>
                    <pre>{code.to_string()}</pre>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
