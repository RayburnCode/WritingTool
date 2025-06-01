#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    class: Option<String>,
    children: Element,
    striped: Option<bool>,
    hoverable: Option<bool>,
    compact: Option<bool>,
    bordered: Option<bool>,
    responsive: Option<bool>,
    caption: Option<Element>,
}

#[component]
pub fn Table(props: TableProps) -> Element {
    let striped = props.striped.unwrap_or(false);
    let hoverable = props.hoverable.unwrap_or(false);
    let compact = props.compact.unwrap_or(false);
    let bordered = props.bordered.unwrap_or(true);
    let responsive = props.responsive.unwrap_or(true);
    
    let base_class = "w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400";
    
    let table_class = format!(
        "{} {} {} {} {}",
        base_class,
        if striped { "table-striped" } else { "" },
        if hoverable { "hover:bg-gray-50 dark:hover:bg-gray-600" } else { "" },
        if compact { "table-compact" } else { "" },
        if bordered { "border-collapse border border-gray-200 dark:border-gray-700" } else { "" },
        // props.class.clone().unwrap_or_default()
    );
    
    let wrapper = if responsive {
        rsx! {
            div { class: "relative overflow-x-auto shadow-md sm:rounded-lg",
                table { class: "{table_class}",
                    if let Some(caption) = &props.caption {
                        caption { class: "p-5 text-lg font-semibold text-left rtl:text-right text-gray-900 bg-white dark:text-white dark:bg-gray-800",
                            {caption}
                        }
                    }
                    {props.children}
                }
            }
        }
    } else {
        rsx! {
            table { class: "{table_class}",
                if let Some(caption) = &props.caption {
                    caption { class: "p-5 text-lg font-semibold text-left rtl:text-right text-gray-900 bg-white dark:text-white dark:bg-gray-800",
                        {caption}
                    }
                }
                {props.children}
            }
        }
    };
    
    wrapper
}

#[derive(Props, Clone, PartialEq)]
pub struct TableHeadProps {
    class: Option<String>,
    children: Element,
    sticky: Option<bool>,
}

#[component]
pub fn TableHead(props: TableHeadProps) -> Element {
    let sticky = props.sticky.unwrap_or(false);
    let base_class = "text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400";
    let class = format!(
        "{} {} {}", 
        base_class, 
        if sticky { "sticky top-0" } else { "" },
        props.class.clone().unwrap_or_default()
    );
    
    rsx!(
        thead { class: "{class}", {props.children} }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TableBodyProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn TableBody(props: TableBodyProps) -> Element {
    let class = format!("{}", props.class.clone().unwrap_or_default());
    
    rsx!(
        tbody { class: "{class}", {props.children} }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TableFootProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn TableFoot(props: TableFootProps) -> Element {
    let base_class = "text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400";
    let class = format!("{} {}", base_class, props.class.clone().unwrap_or_default());
    
    rsx!(
        tfoot { class: "{class}", {props.children} }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TableRowProps {
    class: Option<String>,
    children: Element,
    selected: Option<bool>,
}

#[component]
pub fn TableRow(props: TableRowProps) -> Element {
    let selected = props.selected.unwrap_or(false);
    let base_class = "bg-white border-b dark:bg-gray-800 dark:border-gray-700";
    let class = format!(
        "{} {} {}", 
        base_class, 
        if selected { "bg-blue-50 dark:bg-blue-900/20" } else { "" },
        props.class.clone().unwrap_or_default()
    );
    
    rsx!(
        tr { class: "{class}", {props.children} }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TableHeaderCellProps {
    class: Option<String>,
    children: Element,
    scope: Option<String>,
    colspan: Option<u32>,
    rowspan: Option<u32>,
    sortable: Option<bool>,
}

#[component]
pub fn TableHeaderCell(props: TableHeaderCellProps) -> Element {
    let scope = props.scope.clone().unwrap_or("col".to_string());
    let sortable = props.sortable.unwrap_or(false);
    let base_class = "px-6 py-3";
    let class = format!(
        "{} {} {}", 
        base_class, 
        if sortable { "cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-600" } else { "" },
        props.class.clone().unwrap_or_default()
    );
    
    rsx!(
        th {
            class: "{class}",
            scope: "{scope}",
            colspan: props.colspan.map(|c| c.to_string()),
            rowspan: props.rowspan.map(|r| r.to_string()),
            {props.children}
            if sortable {
                span { class: "ml-1 inline-block", "↕" }
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TableCellProps {
    class: Option<String>,
    children: Element,
    colspan: Option<u32>,
    rowspan: Option<u32>,
}

#[component]
pub fn TableCell(props: TableCellProps) -> Element {
    let base_class = "px-6 py-4";
    let class = format!("{} {}", base_class, props.class.clone().unwrap_or_default());
    
    rsx!(
        td {
            class: "{class}",
            colspan: props.colspan.map(|c| c.to_string()),
            rowspan: props.rowspan.map(|r| r.to_string()),
            {props.children}
        }
    )
}

// Example usage:
/*
fn app() -> Element {
    rsx!(
        Table {
            striped: true,
            hoverable: true,
            caption: rsx! { "Employee Information" },
            TableHead {
                TableRow {
                    TableHeaderCell { "Name" }
                    TableHeaderCell { "Position" }
                    TableHeaderCell { "Department" }
                    TableHeaderCell { "Salary" }
                }
            }
            TableBody {
                TableRow {
                    TableCell { "John Doe" }
                    TableCell { "Software Engineer" }
                    TableCell { "Engineering" }
                    TableCell { "$95,000" }
                }
                TableRow {
                    TableCell { "Jane Smith" }
                    TableCell { "Product Manager" }
                    TableCell { "Product" }
                    TableCell { "$105,000" }
                }
                TableRow {
                    TableCell { "Bob Johnson" }
                    TableCell { "UX Designer" }
                    TableCell { "Design" }
                    TableCell { "$85,000" }
                }
            }
            TableFoot {
                TableRow {
                    TableHeaderCell { colspan: 3, "Total" }
                    TableCell { "$285,000" }
                }
            }
        }
    )
}
*/