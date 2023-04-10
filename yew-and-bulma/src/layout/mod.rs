/// Provides utilities for creating [container elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma container elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::container::Container;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Container>{"This is some text in a container."}</Container>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/container/
pub mod container;
/// Provides utilities for creating [footer elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma footer elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::footer::Footer;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Footer>{"This is some text in a footer."}</Footer>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/footer/
pub mod footer;
/// Provides utilities for creating [level elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma level elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::level::{Level, LevelItem};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Level>
///             <LevelItem>{"This is some text in a level."}</LevelItem>
///         </Level>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/level/
pub mod level;
/// Provides utilities for creating [media elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma media elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaContent, MediaLeft, MediaRight};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaLeft>{"Avatar goes here."}</MediaLeft>
///
///             <MediaContent>{"Content goes here."}</MediaContent>
///
///             <MediaRight>{"Dismiss goes here for example."}</MediaRight>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
pub mod media;
/// Provides utilities for creating [section elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma section elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::section::Section;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Section>{"This is some text in a section."}</Section>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/section/
pub mod section;
/// Provides utilities for creating [tile elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma tile elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::tile::Tile;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Tile>{"This is some text in a tile."}</Tile>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/tiles/
pub mod tile;
