//! Monaco editor as a [Yew](https://yew.rs) component.
//! Requires the "yew" feature.
use crate::{
    api::{CodeEditor as CodeEditorModel, TextModel},
    sys::editor::IStandaloneEditorConstructionOptions,
};
use std::{cell::RefCell, mem, rc::Rc};
use web_sys::HtmlElement;
use yew::{html, html::Scope, Callback, Classes, Component, Context, Html, NodeRef, Properties};

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

/// CodeEditor component.
///
/// Use the `link` prop to pass down a [`CodeEditorLink`] which can be used to
/// access the [`CodeEditor`](CodeEditorModel).
#[derive(Debug)]
pub struct CodeEditor {
    props: CodeEditorProps,
    node_ref: NodeRef,
    editor: Option<CodeEditorModel>,
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
            .unwrap_or_else(|| CodeEditorLink::new_connected(ctx.link().clone()));

        on_editor_created.emit(link);
    }
}
impl Component for CodeEditor {
    type Message = ();
    type Properties = CodeEditorProps;

    fn create(ctx: &Context<Self>) -> Self {
        if let Some(editor_link) = &ctx.props().link {
            editor_link.connect(ctx.link().clone());
        }

        Self {
            props: ctx.props().clone(),
            node_ref: NodeRef::default(),
            editor: None,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let old_props = mem::replace(&mut self.props, ctx.props().clone());
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
                link.connect(ctx.link().clone());
            }
        }
        // changing options requires re-create
        if options != &old_props.options {
            should_render = true;
        }
        // change the attached model
        if model != &old_props.model {
            if let Some(editor) = &self.editor {
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
            self.editor = None;
        }

        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self {
            node_ref, editor, ..
        } = self;
        let props = ctx.props();

        debug_assert!(
            editor.is_none(),
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

        self.editor = Some(editor);
        self.emit_editor_created(ctx);
    }
}

/// Link to control a [`CodeEditor`].
#[derive(Clone, Debug, Default)]
pub struct CodeEditorLink(Rc<RefCell<Option<Scope<CodeEditor>>>>);
impl CodeEditorLink {
    fn new_connected(link: Scope<CodeEditor>) -> Self {
        Self(Rc::new(RefCell::new(Some(link))))
    }

    fn connect(&self, link: Scope<CodeEditor>) {
        self.0.borrow_mut().replace(link);
    }

    fn with_link<T>(&self, f: impl FnOnce(&Scope<CodeEditor>) -> T) -> Option<T> {
        (*self.0.borrow()).as_ref().map(f)
    }

    fn with_component<T>(&self, f: impl FnOnce(&CodeEditor) -> T) -> Option<T> {
        self.with_link(|link| link.get_component().as_deref().map(f))
            .flatten()
    }

    /// Get access to the underlying [`CodeEditor`].
    /// The return value is `None` if the link isn't connected.
    pub fn with_editor<T>(&self, f: impl FnOnce(&CodeEditorModel) -> T) -> Option<T> {
        self.with_component(|comp| comp.editor.as_ref().map(f))
            .flatten()
    }
}
impl PartialEq for CodeEditorLink {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
