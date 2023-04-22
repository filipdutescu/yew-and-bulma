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
                 <Pagination role="navigation" aria_label="pagination">
                     <PaginationList>
                         <PaginationLink page={1} aria_label="Goto page 1" />
                         <PaginationEllipsis />
                         <PaginationLink page={24} aria_label="Goto page 24" />
                         <PaginationLink page={25} current=true aria_label="Page 25" aria_current="page" />
                         <PaginationLink page={26} aria_label="Goto page 26" />
                         <PaginationEllipsis />
                         <PaginationLink page={50} aria_label="Goto page 50" />
                     </PaginationList>

                     <PaginationNext>{"Next"}</PaginationNext>
                     <PaginationPrevious>{"Previous"}</PaginationPrevious>
                 </Pagination>
                <Block>{"The above is a normal pagination component, without anything special."}</Block>
            </Section>

            <Section>
                 <Pagination rounded=true role="navigation" aria_label="pagination">
                     <PaginationList>
                         <PaginationLink page={1} aria_label="Goto page 1" />
                         <PaginationEllipsis />
                         <PaginationLink page={24} aria_label="Goto page 24" />
                         <PaginationLink page={25} current=true aria_label="Page 25" aria_current="page" />
                         <PaginationLink page={26} aria_label="Goto page 26" />
                         <PaginationEllipsis />
                         <PaginationLink page={50} aria_label="Goto page 50" />
                     </PaginationList>

                     <PaginationNext>{"Next"}</PaginationNext>
                     <PaginationPrevious>{"Previous"}</PaginationPrevious>
                 </Pagination>
                 <Pagination role="navigation" aria_label="pagination">
                     <PaginationList>
                         <PaginationLink page={1} aria_label="Goto page 1" />
                         <PaginationEllipsis />
                         <PaginationLink page={24} aria_label="Goto page 24" />
                         <PaginationLink page={25} aria_label="Goto page 25" />
                         <PaginationLink page={26} aria_label="Goto page 26" />
                         <PaginationEllipsis />
                         <PaginationLink page={50} current=true aria_label="Page 50" aria_current="page" />
                     </PaginationList>

                     <PaginationNext disabled=true>{"Next"}</PaginationNext>
                     <PaginationPrevious>{"Previous"}</PaginationPrevious>
                 </Pagination>
                 <Pagination role="navigation" aria_label="pagination">
                     <PaginationList>
                         <PaginationLink page={1} current=true aria_label="Page 1" aria_current="page" />
                         <PaginationEllipsis />
                         <PaginationLink page={24} aria_label="Goto page 24" />
                         <PaginationLink page={25} aria_label="Goto page 25" />
                         <PaginationLink page={26} aria_label="Goto page 26" />
                         <PaginationEllipsis />
                         <PaginationLink page={50} aria_label="Goto page 50" />
                     </PaginationList>

                     <PaginationNext>{"Next"}</PaginationNext>
                     <PaginationPrevious disabled=true>{"Previous"}</PaginationPrevious>
                 </Pagination>

                <Block>{"The above are pagination with various styles."}</Block>
            </Section>

            <Section>
                 <Pagination align={Align::Center} role="navigation" aria_label="pagination">
                     <PaginationList>
                         <PaginationLink page={1} aria_label="Goto page 1" />
                         <PaginationEllipsis />
                         <PaginationLink page={24} aria_label="Goto page 24" />
                         <PaginationLink page={25} current=true aria_label="Page 25" aria_current="page" />
                         <PaginationLink page={26} aria_label="Goto page 26" />
                         <PaginationEllipsis />
                         <PaginationLink page={50} aria_label="Goto page 50" />
                     </PaginationList>

                     <PaginationNext>{"Next"}</PaginationNext>
                     <PaginationPrevious>{"Previous"}</PaginationPrevious>
                 </Pagination>
                 <Pagination align={Align::Right} role="navigation" aria_label="pagination">
                     <PaginationList>
                         <PaginationLink page={1} aria_label="Goto page 1" />
                         <PaginationEllipsis />
                         <PaginationLink page={24} aria_label="Goto page 24" />
                         <PaginationLink page={25} current=true aria_label="Page 25" aria_current="page" />
                         <PaginationLink page={26} aria_label="Goto page 26" />
                         <PaginationEllipsis />
                         <PaginationLink page={50} aria_label="Goto page 50" />
                     </PaginationList>

                     <PaginationNext>{"Next"}</PaginationNext>
                     <PaginationPrevious>{"Previous"}</PaginationPrevious>
                 </Pagination>

                <Block>{"The above are pagination with different alignments."}</Block>
            </Section>

            <Section>
                 <Pagination size={Size::Small} role="navigation" aria_label="pagination">
                     <PaginationList>
                         <PaginationLink page={1} aria_label="Goto page 1" />
                         <PaginationEllipsis />
                         <PaginationLink page={24} aria_label="Goto page 24" />
                         <PaginationLink page={25} current=true aria_label="Page 25" aria_current="page" />
                         <PaginationLink page={26} aria_label="Goto page 26" />
                         <PaginationEllipsis />
                         <PaginationLink page={50} aria_label="Goto page 50" />
                     </PaginationList>

                     <PaginationNext>{"Next"}</PaginationNext>
                     <PaginationPrevious>{"Previous"}</PaginationPrevious>
                 </Pagination>
                 <Pagination size={Size::Medium} role="navigation" aria_label="pagination">
                     <PaginationList>
                         <PaginationLink page={1} aria_label="Goto page 1" />
                         <PaginationEllipsis />
                         <PaginationLink page={24} aria_label="Goto page 24" />
                         <PaginationLink page={25} current=true aria_label="Page 25" aria_current="page" />
                         <PaginationLink page={26} aria_label="Goto page 26" />
                         <PaginationEllipsis />
                         <PaginationLink page={50} aria_label="Goto page 50" />
                     </PaginationList>

                     <PaginationNext>{"Next"}</PaginationNext>
                     <PaginationPrevious>{"Previous"}</PaginationPrevious>
                 </Pagination>
                 <Pagination size={Size::Large} role="navigation" aria_label="pagination">
                     <PaginationList>
                         <PaginationLink page={1} aria_label="Goto page 1" />
                         <PaginationEllipsis />
                         <PaginationLink page={24} aria_label="Goto page 24" />
                         <PaginationLink page={25} current=true aria_label="Page 25" aria_current="page" />
                         <PaginationLink page={26} aria_label="Goto page 26" />
                         <PaginationEllipsis />
                         <PaginationLink page={50} aria_label="Goto page 50" />
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
