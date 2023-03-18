use yew::{function_component, html};
use yew::{html::ChildrenRenderer, virtual_dom::VChild, AttrValue, Children, Html, Properties};

use crate::utils::class::ClassBuilder;

#[derive(Properties, PartialEq)]
pub struct TableProperties {
    #[prop_or_default]
    pub scrollable: bool,
    #[prop_or_default]
    pub bordered: bool,
    #[prop_or_default]
    pub striped: bool,
    #[prop_or_default]
    pub narrow: bool,
    #[prop_or_default]
    pub hoverable: bool,
    #[prop_or_default]
    pub full_width: bool,
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
            modifier_classes.push_str(" is-narrow");
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

#[derive(Clone, PartialEq)]
pub enum TableItem {
    TableHeader(VChild<TableHeader>),
    TableFooter(VChild<TableFooter>),
    TableRow(VChild<TableRow>),
    TableData(VChild<TableData>),
}

impl TableItem {
    pub fn is_header(&self) -> bool {
        matches!(self, TableItem::TableHeader(_))
    }

    pub fn is_footer(&self) -> bool {
        matches!(self, TableItem::TableFooter(_))
    }

    pub fn is_row(&self) -> bool {
        matches!(self, TableItem::TableRow(_))
    }

    pub fn is_data(&self) -> bool {
        matches!(self, TableItem::TableData(_))
    }
}

impl From<VChild<TableHeader>> for TableItem {
    fn from(value: VChild<TableHeader>) -> Self {
        TableItem::TableHeader(value)
    }
}

impl From<VChild<TableFooter>> for TableItem {
    fn from(value: VChild<TableFooter>) -> Self {
        TableItem::TableFooter(value)
    }
}

impl From<VChild<TableRow>> for TableItem {
    fn from(value: VChild<TableRow>) -> Self {
        TableItem::TableRow(value)
    }
}

impl From<VChild<TableData>> for TableItem {
    fn from(value: VChild<TableData>) -> Self {
        TableItem::TableData(value)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Html> for TableItem {
    fn into(self) -> Html {
        match self {
            TableItem::TableHeader(th) => th.into(),
            TableItem::TableFooter(tf) => tf.into(),
            TableItem::TableRow(tr) => tr.into(),
            TableItem::TableData(td) => td.into(),
        }
    }
}

#[function_component(Table)]
pub fn table(props: &TableProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class(&String::from(props))
        .build();
    let headers: Vec<_> = props.children.iter().filter(|ti| ti.is_header()).collect();
    let footers: Vec<_> = props.children.iter().filter(|ti| ti.is_footer()).collect();
    let data: Vec<_> = props
        .children
        .iter()
        .filter(|ti| ti.is_row() || ti.is_data())
        .collect();

    let table_html = html! {
        <table {class}>
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
        </table>
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

#[derive(Properties, PartialEq)]
pub struct TableHeaderProperties {
    #[prop_or_default]
    pub abbreviation: Option<AttrValue>,
    pub data: AttrValue,
}

#[function_component(TableHeader)]
pub fn table_header(props: &TableHeaderProperties) -> Html {
    let abbr = &props.abbreviation;
    let data = &props.data;

    html! {
        <th>
            if let Some(abbr) = &abbr {
                <abbr title={data}>{ &abbr }</abbr>
            } else {
                { data }
            }
        </th>
    }
}

#[function_component(TableFooter)]
pub fn table_footer(props: &TableHeaderProperties) -> Html {
    let abbr = &props.abbreviation;
    let data = &props.data;

    html! {
        <th>
            if let Some(abbr) = &abbr {
                <abbr title={data}>{ &abbr }</abbr>
            } else {
                { data }
            }
        </th>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableRowProperties {
    #[prop_or_default]
    pub selected: bool,
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

#[function_component(TableRow)]
pub fn table_row(props: &TableRowProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class(&String::from(props))
        .build();

    html! {
        <tr {class}>
            { for props.children.iter() }
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableDataProperties {
    pub children: Children,
}

#[function_component(TableData)]
pub fn table_data(props: &TableDataProperties) -> Html {
    html! {
        <td>
            { for props.children.iter() }
        </td>
    }
}
