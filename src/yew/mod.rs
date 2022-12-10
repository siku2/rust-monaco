//! Monaco editor as a [Yew](https://yew.rs) component.
//! Requires the "yew" feature.
use crate::{
    api::{CodeEditor as CodeEditorModel, TextModel},
    sys::editor::IStandaloneEditorConstructionOptions,
};
use std::{cell::RefCell, rc::Rc};
use web_sys::HtmlElement;
use yew::{html, Callback, Classes, Component, Context, Html, NodeRef, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CodeEditorProps<OPT: std::cmp::PartialEq + Clone + Into<IStandaloneEditorConstructionOptions> = IStandaloneEditorConstructionOptions> {
    #[prop_or_default]
    pub link: Option<CodeEditorLink>,
    /// Changing the options will cause the editor to be re-created.
    #[prop_or_default]
    pub options: Option<OPT>,
    #[prop_or_default]
    pub model: Option<TextModel>,
    /// This could be called multiple times if the `options` field changes.
    /// You can use this to initialise the editor
    #[prop_or_default]
    pub on_editor_created: Callback<CodeEditorLink>,
    #[prop_or_default]
    pub classes: Classes,
}

type ModelCell = Rc<RefCell<Option<CodeEditorModel>>>;

/// CodeEditor component.
#[derive(Debug)]
pub struct CodeEditor {
    node_ref: NodeRef,
    editor: ModelCell,
}
impl CodeEditor {
    fn emit_editor_created(&self, ctx: &Context<Self>) {
        let CodeEditorProps {
            link,
            on_editor_created,
            ..
        } = &ctx.props();
        // reuse the link we were given or create a new one
        let link = link
            .clone()
            .unwrap_or_else(|| CodeEditorLink::new_connected(self.editor.clone()));
        on_editor_created.emit(link);
    }
}
impl Component for CodeEditor {
    type Message = ();
    type Properties = CodeEditorProps;

    fn create(ctx: &Context<Self>) -> Self {
        let editor = ModelCell::default();
        if let Some(editor_link) = &ctx.props().link {
            editor_link.connect(editor.clone());
        }
        Self {
            node_ref: NodeRef::default(),
            editor,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        // these are the new values
        let CodeEditorProps {
            link,
            options,
            model,
            on_editor_created: _,
            ..
        } = &ctx.props();

        let mut should_render = false;
        if link != &old_props.link {
            // make sure to connect the new link to this component
            if let Some(link) = &link {
                link.connect(self.editor.clone());
            }
        }
        // changing options requires re-create
        if options != &old_props.options {
            should_render = true;
        }
        // change the attached model
        if model != &old_props.model {
            if let Some(editor) = &mut *self.editor.borrow_mut() {
                match model {
                    Some(model) => editor.set_model(model),
                    None => {
                        editor.detach_model();
                    }
                }
            }
        }

        if should_render {
            // if we're gonna re-create the editor we need to clean up the old one first so
            // as not to cause issues
            self.editor.replace(None);
        }

        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self {
            node_ref, editor, ..
        } = self;
        let props = ctx.props();

        debug_assert!(
            editor.borrow().is_none(),
            "previous editor must be disposed before re-creating"
        );

        html! {
            <div ref={node_ref.clone()} class={props.classes.clone()} />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let el = self
            .node_ref
            .cast::<HtmlElement>()
            .expect("failed to resolve editor element");

        let props = ctx.props();
        let editor = CodeEditorModel::create(&el, props.options.clone());

        if let Some(model) = &props.model {
            // initially we only update the model if it was actually given as a prop.
            // this way a value or model can be given in the options and it won't be
            // detached immediately
            editor.set_model(model)
        }

        self.editor.replace(Some(editor));
        self.emit_editor_created(ctx);
    }
}

/// Link to control a [`CodeEditor`].
#[derive(Clone, Debug, Default)]
pub struct CodeEditorLink(RefCell<ModelCell>);

impl CodeEditorLink {
    pub fn new() -> Self {
        Self(RefCell::default())
    }

    fn new_connected(model_cell: ModelCell) -> Self {
        Self(RefCell::new(model_cell))
    }

    fn connect(&self, model_cell: ModelCell) {
        self.0.replace(model_cell);
    }

    /// Get access to the underlying [`CodeEditor`].
    /// The return value is `None` if the link isn't connected.
    pub fn with_editor<T>(&self, f: impl FnOnce(&CodeEditorModel) -> T) -> Option<T> {
        self.0.borrow().borrow().as_ref().map(f)
    }
}
impl PartialEq for CodeEditorLink {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0.borrow(), &other.0.borrow())
    }
}
