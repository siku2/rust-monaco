//! Monaco editor as a [Yew](https://yew.rs) component.
use web_sys::HtmlElement;
use yew::{html, Component, ComponentLink, Html, NodeRef, Properties, ShouldRender};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct CodeEditorProps {}

#[derive(Debug)]
pub struct CodeEditor {
    props: CodeEditorProps,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
}
impl Component for CodeEditor {
    type Message = ();
    type Properties = CodeEditorProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        #[cfg(feature = "embed_workers")]
        crate::embedded::ensure_environment_set();

        Self {
            props,
            link,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div ref=self.node_ref.clone() style="width:800px; height:600px; border:1px solid #ccc;" />
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(el) = self.node_ref.cast::<HtmlElement>() {
                crate::sys::editor::create(&el);
            }
        }
    }
}
