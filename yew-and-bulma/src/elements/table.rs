use yew::html;
use yew::{
    function_component, html::ChildrenRenderer, virtual_dom::VChild, AttrValue, Children, Html,
    Properties,
};
use yew_and_bulma_macros::{base_component_properties, TypedChildren};

use crate::utils::class::ClassBuilder;
use crate::utils::constants::IS_NARROW;
use crate::utils::BaseComponent;

/// Defines the properties of the [Bulma table element][bd].
///
/// Defines the properties of the table element, based on the specification
/// found in the [Bulma table element documentation][bd].
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
///             <TableHeader>{"One"}</TableHeader>
///             <TableHeader>{"Two"}</TableHeader>
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
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct TableProperties {
    /// Whether or not the [Bulma table element][bd] should be scrollable.
    ///
    /// Whether or not the [Bulma table element][bd], which will receive these
    /// properties, will be scrollable.
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
    ///         <Table scrollable=true>
    ///             <TableHeader>{"One"}</TableHeader>
    ///             <TableHeader>{"Two"}</TableHeader>
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
    /// [bd]: https://bulma.io/documentation/elements/table/#table-container
    #[prop_or_default]
    pub scrollable: bool,
    /// Whether or not the [Bulma table element][bd] should be bordered.
    ///
    /// Whether or not the [Bulma table element][bd], which will receive these
    /// properties, will be bordered.
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
    ///         <Table bordered=true>
    ///             <TableHeader>{"One"}</TableHeader>
    ///             <TableHeader>{"Two"}</TableHeader>
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
    /// [bd]: https://bulma.io/documentation/elements/table/#modifiers
    #[prop_or_default]
    pub bordered: bool,
    /// Whether or not the [Bulma table element][bd] should be striped.
    ///
    /// Whether or not the [Bulma table element][bd], which will receive these
    /// properties, will be striped.
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
    ///         <Table striped=true>
    ///             <TableHeader>{"One"}</TableHeader>
    ///             <TableHeader>{"Two"}</TableHeader>
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
    /// [bd]: https://bulma.io/documentation/elements/table/#modifiers
    #[prop_or_default]
    pub striped: bool,
    /// Whether or not the [Bulma table element][bd] should be narrow.
    ///
    /// Whether or not the [Bulma table element][bd], which will receive these
    /// properties, will be narrow.
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
    ///         <Table narrow=true>
    ///             <TableHeader>{"One"}</TableHeader>
    ///             <TableHeader>{"Two"}</TableHeader>
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
    /// [bd]: https://bulma.io/documentation/elements/table/#modifiers
    #[prop_or_default]
    pub narrow: bool,
    /// Whether or not the [Bulma table element][bd] should be hoverable.
    ///
    /// Whether or not the [Bulma table element][bd], which will receive these
    /// properties, will be hoverable.
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
    ///         <Table hoverable=true>
    ///             <TableHeader>{"One"}</TableHeader>
    ///             <TableHeader>{"Two"}</TableHeader>
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
    /// [bd]: https://bulma.io/documentation/elements/table/#modifiers
    #[prop_or_default]
    pub hoverable: bool,
    /// Whether or not the [Bulma table element][bd] should be full width.
    ///
    /// Whether or not the [Bulma table element][bd], which will receive these
    /// properties, will be full width.
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
    ///         <Table full_width=true>
    ///             <TableHeader>{"One"}</TableHeader>
    ///             <TableHeader>{"Two"}</TableHeader>
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
    /// [bd]: https://bulma.io/documentation/elements/table/#modifiers
    #[prop_or_default]
    pub full_width: bool,
    /// The list of elements found inside the [table element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma table element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/table/
    pub children: ChildrenRenderer<TableItem>,
}

impl From<&TableProperties> for String {
    fn from(value: &TableProperties) -> Self {
        let mut modifier_classes = String::with_capacity(80);

        modifier_classes.push_str("table");
        if value.bordered {
            modifier_classes.push_str(" is-bordered");
        }
        if value.striped {
            modifier_classes.push_str(" is-striped");
        }
        if value.narrow {
            modifier_classes.push_str(IS_NARROW);
        }
        if value.hoverable {
            modifier_classes.push_str(" is-hoverable");
        }
        if value.full_width {
            modifier_classes.push_str(" is-fullwidth");
        }

        modifier_classes
    }
}

/// Defines the possible types of children from a [Bulma table element][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma table element][bd].
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
///             <TableHeader>{"One"}</TableHeader>
///             <TableHeader>{"Two"}</TableHeader>
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
#[derive(Clone, PartialEq, TypedChildren)]
pub enum TableItem {
    TableHeader(VChild<TableHeader>),
    TableFooter(VChild<TableFooter>),
    TableRow(VChild<TableRow>),
    TableData(VChild<TableData>),
}

impl TableItem {
    /// Determines if the table item is a [`crate::elements::table::TableHeader`].
    pub fn is_header(&self) -> bool {
        matches!(self, TableItem::TableHeader(_))
    }

    /// Determines if the table item is a [`crate::elements::table::TableFooter`].
    pub fn is_footer(&self) -> bool {
        matches!(self, TableItem::TableFooter(_))
    }

    /// Determines if the table item is a [`crate::elements::table::TableRow`].
    pub fn is_row(&self) -> bool {
        matches!(self, TableItem::TableRow(_))
    }

    /// Determines if the table item is a [`crate::elements::table::TableData`].
    pub fn is_data(&self) -> bool {
        matches!(self, TableItem::TableData(_))
    }
}

/// Yew implementation of the [Bulma table element][bd].
///
/// Yew implementation of the table element, based on the specification found
/// in the [Bulma table element documentation][bd].
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
///             <TableHeader>{"One"}</TableHeader>
///             <TableHeader>{"Two"}</TableHeader>
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
#[function_component(Table)]
pub fn table(props: &TableProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class(&String::from(props))
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();
    let headers: Vec<_> = props.children.iter().filter(|ti| ti.is_header()).collect();
    let footers: Vec<_> = props.children.iter().filter(|ti| ti.is_footer()).collect();
    let data: Vec<_> = props
        .children
        .iter()
        .filter(|ti| ti.is_row() || ti.is_data())
        .collect();

    let table_html = html! {
        <BaseComponent tag="table" {class} ..props.into()>
            if !headers.is_empty() {
                <thead>
                    { for headers }
                </thead>
            }

            if !footers.is_empty() {
                <tfoot>
                    { for footers }
                </tfoot>
            }

            <tbody>
                { for data }
            </tbody>
        </BaseComponent>
    };

    if props.scrollable {
        html! {
            <div class="table-container">
                {table_html}
            </div>
        }
    } else {
        table_html
    }
}

/// Defines the properties of the [Bulma table header element][bd].
///
/// Defines the properties of the table header element, based on the
/// specification found in the [Bulma table element documentation][bd].
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
///             <TableHeader>{"One"}</TableHeader>
///             <TableHeader>{"Two"}</TableHeader>
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
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct TableHeaderProperties {
    /// Sets the abbreviation of [Bulma table header element][bd].
    ///
    /// Sets the abbreviation of the [Bulma table header element][bd] which will
    /// receive these properties.
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
    ///             <TableHeader abbreviation={"1"}>{"One"}</TableHeader>
    ///             <TableHeader abbreviation={"1"}>{"Two"}</TableHeader>
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
    #[prop_or_default]
    pub abbreviation: Option<AttrValue>,
    /// The list of elements found inside the [table header element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma table header element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/table/
    pub children: Children,
}

/// Yew implementation of the [Bulma table header element][bd].
///
/// Yew implementation of the table header element, based on the specification
/// found in the [Bulma table element documentation][bd].
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
///             <TableHeader>{"One"}</TableHeader>
///             <TableHeader>{"Two"}</TableHeader>
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
#[function_component(TableHeader)]
pub fn table_header(props: &TableHeaderProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();
    let abbr = &props.abbreviation;

    html! {
        <BaseComponent tag="th" {class} ..props.into()>
            if let Some(abbr) = &abbr {
                <abbr {abbr}>{ for props.children.iter() }</abbr>
            } else {
                { for props.children.iter() }
            }
        </BaseComponent>
    }
}

/// Yew implementation of the [Bulma table footer element][bd].
///
/// Yew implementation of the table footer element, based on the specification
/// found in the [Bulma table element documentation][bd]. Has the same
/// properties as [`crate::elements::table::TableHeader`].
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
///             <TableHeader>{"One"}</TableHeader>
///             <TableHeader>{"Two"}</TableHeader>
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
#[function_component(TableFooter)]
pub fn table_footer(props: &TableHeaderProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();
    let abbr = &props.abbreviation;

    html! {
        <BaseComponent tag="th" {class} ..props.into()>
            if let Some(abbr) = &abbr {
                <abbr {abbr}>{ for props.children.iter() }</abbr>
            } else {
                { for props.children.iter() }
            }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma table row element][bd].
///
/// Defines the properties of the table row element, based on the
/// specification found in the [Bulma table element documentation][bd].
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
///             <TableHeader>{"One"}</TableHeader>
///             <TableHeader>{"Two"}</TableHeader>
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
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct TableRowProperties {
    /// Whether or not the [Bulma table row element][bd] should be selected.
    ///
    /// Whether or not the [Bulma table row element][bd], which will receive these
    /// properties, will be selected.
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
    ///             <TableHeader>{"One"}</TableHeader>
    ///             <TableHeader>{"Two"}</TableHeader>
    ///
    ///             <TableRow>
    ///                 <TableData>{ "Three" }</TableData>
    ///                 <TableData>{ "Four" }</TableData>
    ///             </TableRow>
    ///             <TableRow selected=true>
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
    #[prop_or_default]
    pub selected: bool,
    /// The list of elements found inside the [table row element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma table row element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/table/
    pub children: Children,
}

impl From<&TableRowProperties> for String {
    fn from(value: &TableRowProperties) -> Self {
        if value.selected {
            "is-selected".to_owned()
        } else {
            String::new()
        }
    }
}

/// Yew implementation of the [Bulma table row element][bd].
///
/// Yew implementation of the table row element, based on the specification
/// found in the [Bulma table element documentation][bd].
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
///             <TableHeader>{"One"}</TableHeader>
///             <TableHeader>{"Two"}</TableHeader>
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
#[function_component(TableRow)]
pub fn table_row(props: &TableRowProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class(&String::from(props))
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <BaseComponent tag="tr" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma table data element][bd].
///
/// Defines the properties of the table data element, based on the
/// specification found in the [Bulma table element documentation][bd].
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
///             <TableHeader>{"One"}</TableHeader>
///             <TableHeader>{"Two"}</TableHeader>
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
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct TableDataProperties {
    /// The list of elements found inside the [table data element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma table data element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/table/
    pub children: Children,
}

/// Yew implementation of the [Bulma table data element][bd].
///
/// Yew implementation of the table data element, based on the specification
/// found in the [Bulma table element documentation][bd].
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
///             <TableHeader>{"One"}</TableHeader>
///             <TableHeader>{"Two"}</TableHeader>
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
#[function_component(TableData)]
pub fn table_data(props: &TableDataProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <BaseComponent tag="td" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}
