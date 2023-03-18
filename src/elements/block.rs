use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct BlockProperties {
    pub children: Children,
}

#[function_component(Block)]
pub fn block(props: &BlockProperties) -> Html {
    html! {
        <div class="block">
            { for props.children.iter() }
        </div>
    }
}
