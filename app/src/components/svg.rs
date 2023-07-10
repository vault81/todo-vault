use leptos::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Svg {
    AlertCircle,
    CheckmarkChecked,
    CheckmarkUnchecked,
    Edit2,
    FileEdit,
    FilePlus,
    Logo,
    Hamburger,
    Save,
    Search,
    Trash2,
}

impl IntoView for Svg {
    fn into_view(self, cx: Scope) -> View {
        view! { cx,
        <div inner_html=match self {
            Svg::AlertCircle => include_str!("../../../style/inlined/alert-circle.svg"),
            Svg::CheckmarkChecked => include_str!("../../../style/inlined/check-circle.svg"),
            Svg::CheckmarkUnchecked => include_str!("../../../style/inlined/circle.svg"),
            Svg::Edit2 => include_str!("../../../style/inlined/edit-2.svg"),
            Svg::FilePlus => include_str!("../../../style/inlined/file-plus.svg"),
            Svg::FileEdit => include_str!("../../../style/inlined/file-edit.svg"),
            Svg::Logo => include_str!("../../../style/inlined/logo.svg"),
            Svg::Hamburger => include_str!("../../../style/inlined/hamburger.svg"),
            Svg::Save => include_str!("../../../style/inlined/save.svg"),
            Svg::Trash2 => include_str!("../../../style/inlined/trash-2.svg"),
            Svg::Search => include_str!("../../../style/inlined/search.svg"),
        }></div>
    }
        .into_view(cx)
    }
}
