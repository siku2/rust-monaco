use web_sys::HtmlElement;
use yew::{html, Component, ComponentLink, Html, NodeRef, Properties, ShouldRender};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct EditorProps {}

#[derive(Debug)]
pub struct Editor {
    props: EditorProps,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
}
impl Component for Editor {
    type Message = ();
    type Properties = EditorProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
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
                let options = crate::sys::editor::IStandaloneEditorConstructionOptions::default()
                crate::sys::editor.create(&el, None);
            }
        }
    }
}
