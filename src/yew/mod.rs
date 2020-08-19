//! Monaco editor as a [Yew](https://yew.rs) component.
//! Requires the `yew` feature.
use crate::api::{CodeEditor as CodeEditorModel, CodeEditorOptions};
use std::rc::Rc;
use web_sys::HtmlElement;
use yew::{html, Component, ComponentLink, Html, NodeRef, Properties, ShouldRender};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct CodeEditorProps {
    #[prop_or_default]
    pub options: Option<Rc<CodeEditorOptions>>,
}

/// CodeEditor component.
#[derive(Debug)]
pub struct CodeEditor {
    props: CodeEditorProps,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    model: Option<CodeEditorModel>,
}
impl Component for CodeEditor {
    type Message = ();
    type Properties = CodeEditorProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            node_ref: NodeRef::default(),
            model: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        todo!("handle update");
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!("handle prop change");
    }

    fn view(&self) -> Html {
        html! {
            <div ref=self.node_ref.clone() style="width:800px; height:600px; border:1px solid #ccc;" />
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let el = self
                .node_ref
                .cast::<HtmlElement>()
                .expect("failed to resolve editor element");
            let model = CodeEditorModel::create(&el, self.props.options.as_deref());
            self.model = Some(model);
        }
    }
}
