use yew::prelude::*;
use yew_and_bulma::{
    elements::{
        content::Content,
        image::{Figure, Image, Size},
        title::{Size as TitleSize, Title},
    },
    layout::container::Container,
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Container>
            <Content>
                <Title size={TitleSize::Three}>{"Image placeholders taken from Bulma's website"}</Title>
                <p>
                    {"All image placeholder are hosted on and taken from Bulma's documentation website. You can find them at "}
                    <a href="https://bulma.io/documentation/elements/image/">{"the image section in their website"}</a>{"."}
                </p>
            </Content>

            <Figure size={Size::Pixels128x128}>
                <Image src={"https://bulma.io/images/placeholders/128x128.png"} />
            </Figure>

            <Figure size={Size::Pixels128x128}>
                <Image rounded=true src={"https://bulma.io/images/placeholders/480x480.png"} />
            </Figure>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
