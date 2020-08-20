//! Monaco editor as a [Yew](https://yew.rs) component.
//! Requires the "yew" feature.
use crate::api::{CodeEditor as CodeEditorModel, CodeEditorOptions, TextModel};
use std::{cell::RefCell, fmt::Write, rc::Rc};
use web_sys::HtmlElement;
use yew::{html, Component, ComponentLink, Html, NodeRef, Properties, ShouldRender};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CodeEditorProps {
    #[prop_or_default]
    pub link: Option<CodeEditorLink>,
    #[prop_or_default]
    pub options: Option<Rc<CodeEditorOptions>>,
    #[prop_or_default]
    pub height: Option<String>,
    #[prop_or_default]
    pub width: Option<String>,
}
impl CodeEditorProps {
    fn build_style(&self) -> String {
        let mut style = String::new();
        if let Some(v) = &self.height {
            write!(style, "height:{};", v).ok();
        }
        if let Some(v) = &self.width {
            write!(style, "width:{};", v).ok();
        }

        style
    }
}

/// CodeEditor component.
#[derive(Debug)]
pub struct CodeEditor {
    props: CodeEditorProps,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    editor: Option<CodeEditorModel>,
}
impl Component for CodeEditor {
    type Message = ();
    type Properties = CodeEditorProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        if let Some(editor_link) = &props.link {
            editor_link.connect(link.clone());
        }

        Self {
            props,
            link,
            node_ref: NodeRef::default(),
            editor: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!("prop changes aren't allowed yet");
    }

    fn view(&self) -> Html {
        let Self {
            props, node_ref, ..
        } = self;

        html! {
            <div ref=node_ref.clone() style=props.build_style() />
        }
    }

    fn rendered(&mut self, first_render: bool) {
        debug_assert!(first_render, "component should never render more than once");

        let el = self
            .node_ref
            .cast::<HtmlElement>()
            .expect("failed to resolve editor element");
        let model = CodeEditorModel::create(&el, self.props.options.as_deref());
        self.editor = Some(model);
    }
}

/// Link to control a [`CodeEditor`].
#[derive(Clone, Debug, Default)]
pub struct CodeEditorLink(Rc<RefCell<Option<ComponentLink<CodeEditor>>>>);
impl CodeEditorLink {
    fn connect(&self, link: ComponentLink<CodeEditor>) {
        self.0.borrow_mut().replace(link);
    }

    fn with_link<T>(&self, f: impl FnOnce(&ComponentLink<CodeEditor>) -> T) -> Option<T> {
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

    fn get_model(&self) -> Option<TextModel> {
        self.with_editor(|editor| editor.get_model()).flatten()
    }

    /// Get the value of the current model.
    pub fn get_value(&self) -> Option<String> {
        self.get_model().map(|model| model.get_value())
    }
}
impl PartialEq for CodeEditorLink {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
