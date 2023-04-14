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
