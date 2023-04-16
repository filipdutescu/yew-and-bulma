/// Provides utilities for creating [breadcrumb components][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma breadcrumb components][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::breadcrumb::{Breadcrumb, Crumb};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let crumbs = vec![
///         Crumb(AttrValue::from("#"), html!{ {"Trail"} }),
///         Crumb(AttrValue::from("#"), html!{ {"of"} }),
///         Crumb(AttrValue::from("#"), html!{ {"breadcrumbs"} }),
///     ];
///
///     html! {
///         <Breadcrumb {crumbs} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/breadcrumb/
pub mod breadcrumb;
/// Provides utilities for creating [pagination components][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma pagination components][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
///};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/
pub mod pagination;
/// Provides utilities for creating [tabs components][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma tabs components][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::tabs::{tabs, Crumb};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let crumbs = vec![
///         Crumb(AttrValue::from("#"), html!{ {"Trail"} }),
///         Crumb(AttrValue::from("#"), html!{ {"of"} }),
///         Crumb(AttrValue::from("#"), html!{ {"tabss"} }),
///     ];
///
///     html! {
///         <tabs {crumbs} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/tabs/
pub mod tabs;
