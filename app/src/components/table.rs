use leptos::*;
use leptos_dom::*;
use leptos_router::*;

#[component]
pub fn TableCell(
    cx: Scope,
    children: Children,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! { cx, <td class=format!("p-3 md:p-4 {class}")>{children(cx)}</td> }
}

#[component]
pub fn TableRow(
    cx: Scope,
    children: Children,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! { cx,
        <tr class=format!(
            "my-4 bg-white dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 {class}"
        )>{children(cx)}</tr>
    }
}

#[derive(Clone, Debug)]
pub struct ColumnHeader {
    pub id:    String,
    pub label: String,
    pub width: Option<u32>,
}

impl IntoView for ColumnHeader {
    fn into_view(self, cx: Scope) -> View {
        match self.width {
            Some(width) => view! { cx,
                <th id=self.id scope="col" class=format!("p-3 md:p-4 w-{}%", width)>
                    {self.label}
                </th>
            },
            None => view! { cx,
                <th id=self.id scope="col" class="p-3 md:p-4">
                    {self.label}
                </th>
            },
        }.into_view(cx)
    }
}

#[component]
pub fn Table(
    cx: Scope,
    children: Children,
    // column_headers: Vec<String>,
    column_headers: Vec<ColumnHeader>,
) -> impl IntoView {
    view! { cx,
        <table class="w-full text-left text-gray-500 table-fixed dark:text-gray-400">
            <thead class="hidden text-gray-700 uppercase bg-gray-50 md:table-header-group dark:text-gray-400 dark:bg-gray-700">
                <tr>
                    {column_headers}
                </tr>
            </thead>
            <tbody>{children(cx)}</tbody>
        </table>
    }
}
