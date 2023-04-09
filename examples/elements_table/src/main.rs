use yew::prelude::*;
use yew_and_bulma::{
    elements::{table::*, title::Title},
    layout::container::Container,
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Container>
            <Title>{"Selected row table"}</Title>
            <Table striped={true}>
                <TableHeader>{"One"}</TableHeader>
                <TableHeader>{"Two"}</TableHeader>

                <TableRow>
                    <TableData>{ "Three" }</TableData>
                    <TableData>{ "Four" }</TableData>
                </TableRow>
                <TableRow selected={true}>
                    <TableData>{ "Five" }</TableData>
                    <TableData>{ "Six" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Seven" }</TableData>
                    <TableData>{ "Eight" }</TableData>
                </TableRow>
            </Table>

            <hr />

            <Title>{"Bordered table"}</Title>
            <Table bordered={true}>
                <TableHeader>{"One"}</TableHeader>
                <TableHeader>{"Two"}</TableHeader>

                <TableData>{ "Three" }</TableData>
                <TableData>{ "Four" }</TableData>
            </Table>

            <hr />

            <Title>{"Striped table"}</Title>
            <Table striped={true}>
                <TableHeader>{"One"}</TableHeader>
                <TableHeader>{"Two"}</TableHeader>

                <TableRow>
                    <TableData>{ "Three" }</TableData>
                    <TableData>{ "Four" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Five" }</TableData>
                    <TableData>{ "Six" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Seven" }</TableData>
                    <TableData>{ "Eight" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Nine" }</TableData>
                    <TableData>{ "Ten" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Eleven" }</TableData>
                    <TableData>{ "Twelve" }</TableData>
                </TableRow>
            </Table>

            <hr />

            <Title>{"Narrow table"}</Title>
            <Table narrow={true}>
                <TableHeader>{"One"}</TableHeader>
                <TableHeader>{"Two"}</TableHeader>

                <TableRow>
                    <TableData>{ "Three" }</TableData>
                    <TableData>{ "Four" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Five" }</TableData>
                    <TableData>{ "Six" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Seven" }</TableData>
                    <TableData>{ "Eight" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Nine" }</TableData>
                    <TableData>{ "Ten" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Eleven" }</TableData>
                    <TableData>{ "Twelve" }</TableData>
                </TableRow>
            </Table>

            <hr />

            <Title>{"Hoverable table"}</Title>
            <Table hoverable={true}>
                <TableHeader>{"One"}</TableHeader>
                <TableHeader>{"Two"}</TableHeader>

                <TableRow>
                    <TableData>{ "Three" }</TableData>
                    <TableData>{ "Four" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Five" }</TableData>
                    <TableData>{ "Six" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Seven" }</TableData>
                    <TableData>{ "Eight" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Nine" }</TableData>
                    <TableData>{ "Ten" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Eleven" }</TableData>
                    <TableData>{ "Twelve" }</TableData>
                </TableRow>
            </Table>

            <hr />

            <Title>{"Full width table"}</Title>
            <Table full_width={true}>
                <TableHeader>{"One"}</TableHeader>
                <TableHeader>{"Two"}</TableHeader>

                <TableRow>
                    <TableData>{ "Three" }</TableData>
                    <TableData>{ "Four" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Five" }</TableData>
                    <TableData>{ "Six" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Seven" }</TableData>
                    <TableData>{ "Eight" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Nine" }</TableData>
                    <TableData>{ "Ten" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Eleven" }</TableData>
                    <TableData>{ "Twelve" }</TableData>
                </TableRow>
            </Table>

            <hr />

            <Title>{"Combined modifiers table"}</Title>
            <Table
                bordered={true}
                striped={true}
                narrow={true}
                hoverable={true}
                full_width={true}>
                <TableHeader>{"One"}</TableHeader>
                <TableHeader>{"Two"}</TableHeader>

                <TableRow>
                    <TableData>{ "Three" }</TableData>
                    <TableData>{ "Four" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Five" }</TableData>
                    <TableData>{ "Six" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Seven" }</TableData>
                    <TableData>{ "Eight" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Nine" }</TableData>
                    <TableData>{ "Ten" }</TableData>
                </TableRow>
                <TableRow>
                    <TableData>{ "Eleven" }</TableData>
                    <TableData>{ "Twelve" }</TableData>
                </TableRow>
            </Table>

            <hr />

            <Title>{"Scrollable table"}</Title>
            <Table scrollable={true}>
                <TableRow>
                    { for (1..101).into_iter().map(|i| html!{<TableData>{i}</TableData>}) }
                </TableRow>

                <TableRow>
                    { for (1..101).into_iter().map(|i| html!{<TableData>{i * 2}</TableData>}) }
                </TableRow>

                <TableRow>
                    { for (1..101).into_iter().map(|i| html!{<TableData>{i * 3}</TableData>}) }
                </TableRow>

                <TableRow>
                    { for (1..101).into_iter().map(|i| html!{<TableData>{i * 4}</TableData>}) }
                </TableRow>

                <TableRow>
                    { for (1..101).into_iter().map(|i| html!{<TableData>{i * 5}</TableData>}) }
                </TableRow>
            </Table>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
