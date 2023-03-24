/// Provides utilities for creating [block elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma block elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::block::Block;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Block>{"This is some text in a block."}</Block>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/block/
pub mod block;
/// Provides utilities for creating [box elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma box elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::r#box::Box;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Box>{"This is some text in a box."}</Box>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/box/
pub mod r#box;
/// Provides utilities for creating [button elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma button elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::button::Button;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Button>{"This is some text in a button."}</Button>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/button/
pub mod button;
/// Provides utilities for creating [content elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma content elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::content::Content;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Content>
///             <p>{"This is some text in a content."}</p>
///         </Content>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/content/
pub mod content;
/// Provides utilities for creating [delete elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma delete elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::delete::Delete;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Delete />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/delete/
pub mod delete;
/// Provides utilities for creating [icon elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma icon elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::icon::Icon;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Icon
///             icon={html! {
///                 <i class="fas fa-home"></i>
///             }} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/icon/
pub mod icon;
/// Provides utilities for creating [image elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma image and figure elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::image::{Figure, Image};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Figure>
///             <Image src={"media/images/img.png"} />
///         </Figure>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/image/
pub mod image;
/// Provides utilities for creating [notification elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma notification elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::notification::Notification;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Notification>{"Hello, world!"}</Notification>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/notification/
pub mod notification;
/// Provides utilities for creating [progress elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma progress elements][bd], such as the progress bar in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::progress::ProgressBar;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <ProgressBar value={15.0} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/progress/
pub mod progress;
/// Provides utilities for creating [table elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma table elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::table::{Table, TableHeader, TableRow, TableData};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Table>
///             <TableHeader>{ "One" }</TableHeader>
///             <TableHeader>{ "Two" }</TableHeader>
///
///             <TableRow>
///                 <TableData>{ "Three" }</TableData>
///                 <TableData>{ "Four" }</TableData>
///             </TableRow>
///             <TableRow>
///                 <TableData>{ "Five" }</TableData>
///                 <TableData>{ "Six" }</TableData>
///             </TableRow>
///             <TableRow>
///                 <TableData>{ "Seven" }</TableData>
///                 <TableData>{ "Eight" }</TableData>
///             </TableRow>
///         </Table>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/table/
pub mod table;
/// Provides utilities for creating [tag elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma tag elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
// use yew::prelude::*;
// use yew_and_bulma::elements::tag::Tag;
//
// #[function_component(App)]
// fn app() -> Html {
//     html! {
//         <Tag>{"Tag label"}</Tag>
//     }
// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/tag/
pub mod tag;
/// Provides utilities for creating [title elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma title and subtitle elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::title::Title;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Title>{"Hello, world!"}</Title>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/title/
pub mod title;
