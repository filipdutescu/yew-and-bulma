use yew::prelude::*;
use yew_and_bulma::{
    components::pagination::{
        Align, Pagination, PaginationEllipsis, PaginationLink, PaginationList, PaginationNext,
        PaginationPrevious,
    },
    elements::block::Block,
    layout::{container::Container, section::Section},
    utils::size::Size,
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Container>
            <Section>
                 <Pagination>
                     <PaginationList>
                         <PaginationLink page={1} />
                         <PaginationEllipsis />
                         <PaginationLink page={24} />
                         <PaginationLink page={25} current=true />
                         <PaginationLink page={26} />
                         <PaginationEllipsis />
                         <PaginationLink page={50} />
                     </PaginationList>

                     <PaginationNext>{"Next"}</PaginationNext>
                     <PaginationPrevious>{"Previous"}</PaginationPrevious>
                 </Pagination>
                <Block>{"The above is a normal pagination component, without anything special."}</Block>
            </Section>

            <Section>
                 <Pagination rounded=true>
                     <PaginationList>
                         <PaginationLink page={1} />
                         <PaginationEllipsis />
                         <PaginationLink page={24} />
                         <PaginationLink page={25} current=true />
                         <PaginationLink page={26} />
                         <PaginationEllipsis />
                         <PaginationLink page={50} />
                     </PaginationList>

                     <PaginationNext>{"Next"}</PaginationNext>
                     <PaginationPrevious>{"Previous"}</PaginationPrevious>
                 </Pagination>
                 <Pagination>
                     <PaginationList>
                         <PaginationLink page={1} />
                         <PaginationEllipsis />
                         <PaginationLink page={24} />
                         <PaginationLink page={25} />
                         <PaginationLink page={26} />
                         <PaginationEllipsis />
                         <PaginationLink page={50} current=true />
                     </PaginationList>

                     <PaginationNext disabled=true>{"Next"}</PaginationNext>
                     <PaginationPrevious>{"Previous"}</PaginationPrevious>
                 </Pagination>
                 <Pagination>
                     <PaginationList>
                         <PaginationLink page={1} current=true />
                         <PaginationEllipsis />
                         <PaginationLink page={24} />
                         <PaginationLink page={25} />
                         <PaginationLink page={26} />
                         <PaginationEllipsis />
                         <PaginationLink page={50} />
                     </PaginationList>

                     <PaginationNext>{"Next"}</PaginationNext>
                     <PaginationPrevious disabled=true>{"Previous"}</PaginationPrevious>
                 </Pagination>

                <Block>{"The above are pagination with various styles."}</Block>
            </Section>

            <Section>
                 <Pagination align={Align::Center}>
                     <PaginationList>
                         <PaginationLink page={1} />
                         <PaginationEllipsis />
                         <PaginationLink page={24} />
                         <PaginationLink page={25} current=true />
                         <PaginationLink page={26} />
                         <PaginationEllipsis />
                         <PaginationLink page={50} />
                     </PaginationList>

                     <PaginationNext>{"Next"}</PaginationNext>
                     <PaginationPrevious>{"Previous"}</PaginationPrevious>
                 </Pagination>
                 <Pagination align={Align::Right}>
                     <PaginationList>
                         <PaginationLink page={1} />
                         <PaginationEllipsis />
                         <PaginationLink page={24} />
                         <PaginationLink page={25} current=true />
                         <PaginationLink page={26} />
                         <PaginationEllipsis />
                         <PaginationLink page={50} />
                     </PaginationList>

                     <PaginationNext>{"Next"}</PaginationNext>
                     <PaginationPrevious>{"Previous"}</PaginationPrevious>
                 </Pagination>

                <Block>{"The above are pagination with different alignments."}</Block>
            </Section>

            <Section>
                 <Pagination size={Size::Small}>
                     <PaginationList>
                         <PaginationLink page={1} />
                         <PaginationEllipsis />
                         <PaginationLink page={24} />
                         <PaginationLink page={25} current=true />
                         <PaginationLink page={26} />
                         <PaginationEllipsis />
                         <PaginationLink page={50} />
                     </PaginationList>

                     <PaginationNext>{"Next"}</PaginationNext>
                     <PaginationPrevious>{"Previous"}</PaginationPrevious>
                 </Pagination>
                 <Pagination size={Size::Medium}>
                     <PaginationList>
                         <PaginationLink page={1} />
                         <PaginationEllipsis />
                         <PaginationLink page={24} />
                         <PaginationLink page={25} current=true />
                         <PaginationLink page={26} />
                         <PaginationEllipsis />
                         <PaginationLink page={50} />
                     </PaginationList>

                     <PaginationNext>{"Next"}</PaginationNext>
                     <PaginationPrevious>{"Previous"}</PaginationPrevious>
                 </Pagination>
                 <Pagination size={Size::Large}>
                     <PaginationList>
                         <PaginationLink page={1} />
                         <PaginationEllipsis />
                         <PaginationLink page={24} />
                         <PaginationLink page={25} current=true />
                         <PaginationLink page={26} />
                         <PaginationEllipsis />
                         <PaginationLink page={50} />
                     </PaginationList>

                     <PaginationNext>{"Next"}</PaginationNext>
                     <PaginationPrevious>{"Previous"}</PaginationPrevious>
                 </Pagination>

                <Block>{"The above are pagination with different sizes."}</Block>
            </Section>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
