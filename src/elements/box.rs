use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct BoxProperties {
    pub children: Children,
}

#[function_component(Box)]
pub fn block(props: &BoxProperties) -> Html {
    html! {
        <div class="box">
            { for props.children.iter() }
        </div>
    }
}
