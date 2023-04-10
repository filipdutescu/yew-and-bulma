use yew::prelude::*;
use yew_and_bulma::{
    elements::r#box::Box,
    layout::{
        container::Container,
        tile::{Relation, Tile},
    },
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Container>
            <Tile relation={Relation::Ancestor}>
                <Tile relation={Relation::Parent} vertical={true}>
                    <Tile relation={Relation::Child}>
                        <Box>{"This is some text in a box inside a container."}</Box>
                    </Tile>

                    <Tile relation={Relation::Child}>
                        <Box>{"This is some text in a box inside a container."}</Box>
                    </Tile>
                </Tile>

                <Tile relation={Relation::Parent}>
                    <Tile relation={Relation::Child}>
                        <Box>
                            {"Lorem ipsum dolor sit amet, consectetur adipiscing
                            elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Sociis
                            natoque penatibus et magnis dis parturient montes nascetur. Eros donec ac odio
                            tempor orci dapibus ultrices. Volutpat odio facilisis mauris sit amet massa
                            vitae tortor condimentum. Tortor pretium viverra suspendisse potenti. Lorem
                            ipsum dolor sit amet. Sociis natoque penatibus et magnis dis. Eleifend quam
                            adipiscing vitae proin sagittis nisl rhoncus. Orci porta non pulvinar neque
                            laoreet suspendisse interdum consectetur libero. Interdum velit euismod in
                            pellentesque massa placerat. Enim tortor at auctor urna nunc id cursus metus.
                            Faucibus pulvinar elementum integer enim neque volutpat. Sit amet mauris
                            commodo quis imperdiet. Leo integer malesuada nunc vel risus commodo viverra.
                            Volutpat diam ut venenatis tellus in metus. Pretium fusce id velit ut tortor
                            pretium. Vitae tortor condimentum lacinia quis vel eros donec ac odio. Iaculis
                            urna id volutpat lacus laoreet non. Consectetur adipiscing elit pellentesque
                            habitant morbi. Tincidunt arcu non sodales neque sodales. Ut porttitor leo a
                            diam sollicitudin. Sit amet mattis vulputate enim. Auctor eu augue ut lectus
                            arcu bibendum. Egestas quis ipsum suspendisse ultrices. Ipsum dolor sit amet
                            consectetur adipiscing elit pellentesque. Eleifend quam adipiscing vitae proin
                            sagittis nisl rhoncus. At elementum eu facilisis sed odio morbi. Varius sit amet
                            mattis vulputate. Accumsan lacus vel facilisis volutpat est. Enim nulla aliquet
                            porttitor lacus. Sit amet purus gravida quis. Cum sociis natoque penatibus et
                            magnis dis parturient montes nascetur. Et netus et malesuada fames ac turpis.
                            Nulla porttitor massa id neque aliquam vestibulum morbi. Eu non diam phasellus
                            vestibulum lorem sed risus ultricies tristique. Iaculis eu non diam phasellus
                            vestibulum lorem sed risus. Mattis rhoncus urna neque viverra justo nec ultrices
                            dui. Lobortis elementum nibh tellus molestie nunc non blandit massa."}
                        </Box>
                    </Tile>
                </Tile>
            </Tile>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
