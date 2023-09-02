use monaco::{
    api::{CodeEditorOptions, TextModel},
    sys::{
        editor::{BuiltinTheme, IMarkerData, IModelDecorationOptions, IModelDeltaDecoration},
        IMarkdownString, MarkerSeverity, Range,
    },
    yew::CodeEditor,
};
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
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

#[function_component(App)]
fn app() -> Html {
    // Here's some "code" that will showcase some of
    // the decorations that can be applied to the text model.
    let code = String::from("This is just a normal line.\nThis is an error.\nThis line is highlighted.\nHover over this line to see a secret message.");

    // We need to create a new text model, so we can pass it to Monaco.
    // We use use_mut_ref, as this allows us to only use it 
    // when it changes and for us to call the same text model between the two callbacks.
    let text_model = use_mut_ref(|| TextModel::create(&code, Some("rust"), None).unwrap());

    // Setup the arrays that would store decorations applied to the text model.
    let marker_jsarray = js_sys::Array::new();
    let old_decor_jsarray = use_mut_ref(js_sys::Array::new);
    let new_decor_jsarray = js_sys::Array::new();

    // Here we define what we want to do when we click the button, which is bound to
    // the button below. We use the `use_callback` function in Yew to bind the
    // function to the button. We can then clone the text model, and use it in
    // the callback to set the value to whatever we want.
    let on_decor_clicked = {
        let text_model = Rc::clone(&text_model);
        let marker_jsarray = marker_jsarray.clone();
        let old_decor_jsarray = old_decor_jsarray.clone();
        let new_decor_jsarray = new_decor_jsarray.clone();

        use_callback(
            move |_, text_model| {
                let text_model = text_model.borrow_mut();
                let mut old_decor_jsarray = old_decor_jsarray.borrow_mut();

                // Make a text model reference to pass the decorations to.
                let curr_model = text_model.as_ref();

                // Here we are going to add a simple error decoration onto a line.
                // The process of applying this requires the IMarkerData type to pass
                // in the parameters of the message, the severity, and the range of the error itself.
                // It's important to note that the line-number/column index for Monaco starts at 1, and
                // the values have to be in f64 vs. number in JavaScript. 
                // With columns, the end index of a line is the last character on that line plus one.
                // Once we have that setup, push the marker onto a JsArray.
                let error_marker: IMarkerData = new_object().into();
                error_marker.set_message("This is the error's message.");
                error_marker.set_severity(MarkerSeverity::Error);
                error_marker.set_start_line_number(2.0);
                error_marker.set_start_column(12.0);
                error_marker.set_end_line_number(2.0);
                error_marker.set_end_column(17.0);
                marker_jsarray.push(&error_marker);

                // Then, to have it applied to our editor, we set it as so.
                // This will show the word "error" being underlined as an error
                monaco::sys::editor::set_model_markers(curr_model, "owner", &marker_jsarray);

                // Here we are applying a highlight decoration on a line.
                // This requires some basic styling which can be seen in this example's index.html.
                // This uses the IModelDeltaDecoration type, which is versatile for a lot of the
                // features you could implement like line & in-line decorations.
                // NOTE: delta_decorations has been replaced by createDecorationsCollection as of version 0.37.
                // Since rust-monaco is based on version 0.32, this is the primary function for applying decorations.
                
                // Setting up the options/parameters which will highlight the third line in the example code.
                let highlight_decor = monaco::sys::editor::IModelDecorationOptions::default();
                highlight_decor.set_is_whole_line(true.into());
                highlight_decor.set_inline_class_name("myInlineDecoration".into());
                let highlight_range = Range::new(3.0, 0.0, 3.0, 0.0);
                
                // We create the IModelDeltaDecoration to store our styling.
                let highlight_line: IModelDeltaDecoration = new_object().into();
                highlight_line.set_options(&highlight_decor);
                // Then, we convert the range and decoration to JsValues to be store in the JsArray
                let range_js = highlight_range
                    .dyn_into::<JsValue>()
                    .expect("Range is not found.");
                highlight_line.set_range(&monaco::sys::IRange::from(range_js));
                let highlight_js = highlight_line
                    .dyn_into::<JsValue>()
                    .expect("Highlight is not found.");
                // Push the final decoration to the JsArray
                new_decor_jsarray.push(&highlight_js);

                // Here we are applying a hover decoration on a line. It is similar to the previous one,
                // but it has more steps. Create the IModelDeltaDecoration for the hover 
                // provider to store options and range.
                let hover_decoration: IModelDeltaDecoration = new_object().into();
                let hover_range = Range::new(4.0, 0.0, 4.0, 0.0);
                let hover_range_js = hover_range
                    .dyn_into::<JsValue>()
                    .expect("Range is not found.");
                hover_decoration.set_range(&monaco::sys::IRange::from(hover_range_js));
                let hover_opts: IModelDecorationOptions = new_object().into();
                hover_opts.set_is_whole_line(true.into());
                // Since we do not have a server to request for information, we will use the 
                // Reflect.set() function to set the message for the hover provider as a JsValue 
                // and to store a message in it. Take note of the use of unwrap() to get the resulting JsValue.
                let hover_message: IMarkdownString = new_object().into();
                js_sys::Reflect::set(
                    &hover_message,
                    &JsValue::from_str("value"),
                    &JsValue::from_str("hi :)"),
                )
                .unwrap();
                // Continue on as usual with converting the rest of the parameters into JsValues.
                hover_opts.set_hover_message(&hover_message);
                hover_decoration.set_options(&hover_opts);
                let hover_js = hover_decoration
                    .dyn_into::<JsValue>()
                    .expect("Hover is not found.");
                // Then, push it to the JsArray.
                new_decor_jsarray.push(&hover_js);
                
                // This is the culimination of setting up all the IModelDeltaDecorations as we call delta_decorations.
                // To keep it simple, when it is called it will apply all the decorations from the "new" JsArray
                // with them onto the editor. Then, it will store the handles in the "old" JsArray. To prevent
                // any sort of oddities like duplicate decorations, we store the resulting handles to a "temp" JsArray
                // then pass that to the "old" JsArray. 
                let temp_decor_array =
                    curr_model.delta_decorations(&old_decor_jsarray, &new_decor_jsarray, None);
                *old_decor_jsarray = temp_decor_array;

                // After the decorations are applied to the editor, we remove the decorations from the arrays.
                // However, it does not mean that they are removed from the editor since we did not call set_model_markers and delta_decorations.
                new_decor_jsarray.set_length(0);
                marker_jsarray.pop();
            },
            text_model,
        )
    };

    // Here is where we will call set_model_markers and delta_decorations again to remove the decorations from the editor.
    let on_nodecor_clicked = {
        let text_model = Rc::clone(&text_model);
        let marker_jsarray = marker_jsarray.clone();
        let old_decor_jsarray = old_decor_jsarray.clone();
        let new_decor_jsarray = new_decor_jsarray.clone();

        use_callback(
            move |_, text_model| {
                let text_model = text_model.borrow_mut();
                let curr_model = text_model.as_ref();
                let old_decor_jsarray = old_decor_jsarray.borrow();

                monaco::sys::editor::set_model_markers(curr_model, "owner", &marker_jsarray);

                curr_model.delta_decorations(&old_decor_jsarray, &new_decor_jsarray, None);
            },
            text_model,
        )
    };

    html! {
        <div id="code-wrapper">
            <div id="code-editor">
                <CustomEditor text_model={text_model.borrow().clone()} />
            </div>
            <div id="event-log-wrapper">
                <div id="event-log">
                    <button onclick={on_decor_clicked}>{ "Show Decors" }</button>
                    <button onclick={on_nodecor_clicked}>{ "Remove Decors" }</button>
                </div>
            </div>
        </div>
    }
}

// Creates a new `JsValue`. Done for convenience and readability.
fn new_object() -> JsValue {
    js_sys::Object::new().into()
}

fn main() {
    yew::Renderer::<App>::new().render();
}
