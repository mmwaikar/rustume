use crate::resume::{profile_publications, Resume};
use gpui_kit::base::SelectableText;
use gpui_kit::component::{
    bubble::{Bubble, BubbleContent, BubbleVariant},
    link::Link,
    table::{Column, TableDelegate, TableState},
    Icon,
};
use gpui_kit::{
    div, AnyElement, App as GpuiApp, Context, IntoElement, ParentElement, Styled, Window,
};

pub(crate) const PAGE_BACKGROUND: u32 = 0xf4efe6;
pub(crate) const PANEL_BACKGROUND: u32 = 0xfffcf6;
pub(crate) const SIDEBAR_BACKGROUND: u32 = 0xd7d8bd;
pub(crate) const INK: u32 = 0x203a36;
pub(crate) const MUTED_INK: u32 = 0x67735d;
pub(crate) const ACTIVE: u32 = 0xefb15d;
pub(crate) const DEFAULT_LINK_BLUE: u32 = 0x2563eb;
pub(crate) const UNKNOWN_COUNTRY_COLOR: u32 = 0x6b7280;
pub(crate) const FOOTER_RUST_URL: &str = "https://rust-lang.org/";
pub(crate) const FOOTER_GPUI_KIT_URL: &str = "https://gpui-kit.com";

pub(crate) fn nav_icon(name: &'static str) -> AnyElement {
    Icon::new(Icon::empty().path(format!("icons/{name}.svg")))
        .size_4()
        .text_color(gpui_kit::rgb(INK))
        .into_any_element()
}

pub(crate) fn section_eyebrow(label: &'static str) -> AnyElement {
    div()
        .text_sm()
        .text_color(gpui_kit::rgb(MUTED_INK))
        .child(selectable_text(format!("eyebrow-{label}"), label))
        .into_any_element()
}

pub(crate) fn selectable_text(
    id: impl Into<gpui_kit::ElementId>,
    value: impl Into<String>,
) -> AnyElement {
    SelectableText::new(id, value.into()).into_any_element()
}

pub(crate) fn bubble(label: impl Into<String>) -> AnyElement {
    let label = label.into();
    Bubble::new()
        .with_variant(BubbleVariant::Tinted)
        .mr_2()
        .mb_2()
        .child(
            div()
                .whitespace_nowrap()
                .child(selectable_text(label.clone(), label)),
        )
        .into_any_element()
}

pub(crate) fn compact_bubble(label: impl Into<String>) -> AnyElement {
    let label = label.into();
    Bubble::new()
        .with_variant(BubbleVariant::Tinted)
        .flex_none()
        .ml_2()
        .content(
            BubbleContent::new().px_1().py_0().text_xs().child(
                div()
                    .flex_none()
                    .whitespace_nowrap()
                    .child(selectable_text(format!("compact-bubble-{label}"), label)),
            ),
        )
        .into_any_element()
}

pub(crate) fn link(id: String, label: String, href: String) -> AnyElement {
    Link::new(id.clone())
        .href(href)
        .text_color(gpui_kit::rgb(DEFAULT_LINK_BLUE))
        .child(selectable_text(format!("{id}-label"), label))
        .into_any_element()
}

pub(crate) fn card(content: impl IntoElement) -> AnyElement {
    div()
        .flex_1()
        .min_w(gpui_kit::px(280.0))
        .m_2()
        .p_6()
        .bg(gpui_kit::rgb(PANEL_BACKGROUND))
        .border_color(gpui_kit::rgb(SIDEBAR_BACKGROUND))
        .rounded_lg()
        .child(content)
        .into_any_element()
}

pub(crate) fn hero(eyebrow: &'static str, title: impl Into<String>, copy: impl Into<String>) -> AnyElement {
    div()
        .w_full()
        .p_6()
        .mb_4()
        .bg(gpui_kit::rgb(SIDEBAR_BACKGROUND))
        .border_1()
        .border_color(gpui_kit::rgb(ACTIVE))
        .rounded_lg()
        .child(section_eyebrow(eyebrow))
        .child(
            div()
                .w_full()
                .mt_2()
                .text_3xl()
                .text_color(gpui_kit::rgb(INK))
                .child(selectable_text("hero-title", title.into())),
        )
        .child(
            div()
                .mt_3()
                .text_lg()
                .text_color(gpui_kit::rgb(INK))
                .child(selectable_text("hero-copy", copy.into())),
        )
        .into_any_element()
}

pub(crate) fn format_publication_date(value: Option<&str>) -> String {
    let Some(value) = value else {
        return String::new();
    };
    if let Ok(date) = chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        return date.format("%b %Y").to_string();
    }
    if let Ok(date) = chrono::NaiveDate::parse_from_str(&format!("{value}-01"), "%Y-%m-%d") {
        return date.format("%B %Y").to_string();
    }
    value.to_owned()
}

#[derive(Clone, Debug)]
pub(crate) struct EducationRow {
    institution: String,
    study: String,
    dates: String,
    area: String,
}

pub(crate) struct EducationTableDelegate {
    rows: Vec<EducationRow>,
    columns: Vec<Column>,
}

impl EducationTableDelegate {
    pub(crate) fn new(rows: Vec<EducationRow>) -> Self {
        let columns = vec![
            Column::new("institution", "Institution")
                .width(350.)
                .min_width(220.),
            Column::new("study", "Study").width(400.).min_width(220.),
            Column::new("dates", "Dates").width(150.).min_width(140.),
            Column::new("area", "Area").width(100.).min_width(140.),
        ];
        Self { rows, columns }
    }
}

impl TableDelegate for EducationTableDelegate {
    fn columns_count(&self, _: &GpuiApp) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _: &GpuiApp) -> usize {
        self.rows.len()
    }

    fn column(&self, col_ix: usize, _: &GpuiApp) -> Column {
        self.columns[col_ix].clone()
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _: &mut Window,
        _: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        let row = &self.rows[row_ix];
        let value = match col_ix {
            0 => row.institution.clone(),
            1 => row.study.clone(),
            2 => row.dates.clone(),
            3 => row.area.clone(),
            _ => String::new(),
        };

        selectable_text(format!("education-{row_ix}-{col_ix}"), value)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PublicationRow {
    name: String,
    publisher: String,
    published: String,
    url: String,
}

pub(crate) struct PublicationTableDelegate {
    rows: Vec<PublicationRow>,
    columns: Vec<Column>,
}

impl PublicationTableDelegate {
    pub(crate) fn new(rows: Vec<PublicationRow>) -> Self {
        let columns = vec![
            Column::new("name", "Name").width(700.).min_width(420.),
            Column::new("publisher", "Publisher")
                .width(150.)
                .min_width(110.),
            Column::new("published", "Published")
                .width(150.)
                .min_width(110.),
        ];
        Self { rows, columns }
    }
}

impl TableDelegate for PublicationTableDelegate {
    fn columns_count(&self, _: &GpuiApp) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _: &GpuiApp) -> usize {
        self.rows.len()
    }

    fn column(&self, col_ix: usize, _: &GpuiApp) -> Column {
        self.columns[col_ix].clone()
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _: &mut Window,
        _: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        let row = &self.rows[row_ix];
        let value = match col_ix {
            0 => row.name.clone(),
            1 => row.publisher.clone(),
            2 => row.published.clone(),
            _ => String::new(),
        };
        if col_ix == 0 && !row.url.is_empty() {
            Link::new(format!("publication-table-title-{row_ix}"))
                .href(row.url.clone())
                .child(selectable_text(
                    format!("publication-title-{row_ix}"),
                    value,
                ))
                .into_any_element()
        } else {
            selectable_text(format!("publication-{row_ix}-{col_ix}"), value)
        }
    }
}

pub(crate) fn education_rows(resume: &Resume) -> Vec<EducationRow> {
    resume
        .education
        .iter()
        .map(|item| EducationRow {
            institution: item.institution.clone(),
            study: item.study_type.clone(),
            dates: match (item.start_date.as_deref(), item.end_date.as_deref()) {
                (Some(start), Some(end)) if !start.is_empty() && !end.is_empty() => format!(
                    "{} - {}",
                    format_publication_date(Some(start)),
                    format_publication_date(Some(end))
                ),
                (Some(start), _) if !start.is_empty() => format_publication_date(Some(start)),
                (_, Some(end)) if !end.is_empty() => format_publication_date(Some(end)),
                _ => String::new(),
            },
            area: item.area.clone(),
        })
        .collect()
}

pub(crate) fn publication_rows(resume: &Resume) -> Vec<PublicationRow> {
    profile_publications(resume)
        .into_iter()
        .map(|item| PublicationRow {
            name: item.name.clone(),
            publisher: item.publisher.clone(),
            published: format_publication_date(item.release_date.as_deref()),
            url: item.url.clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn footer_uses_official_project_urls() {
        assert_eq!(FOOTER_RUST_URL, "https://rust-lang.org/");
        assert_eq!(FOOTER_GPUI_KIT_URL, "https://gpui-kit.com");
    }

    #[test]
    fn shared_links_use_default_blue() {
        assert_eq!(DEFAULT_LINK_BLUE, 0x2563eb);
    }

    #[test]
    fn formats_full_and_partial_publication_dates() {
        assert_eq!(format_publication_date(Some("2024-02-17")), "Feb 2024");
        assert_eq!(format_publication_date(Some("2024-02")), "February 2024");
    }

    #[test]
    fn preserves_missing_or_unknown_publication_dates() {
        assert_eq!(format_publication_date(None), "");
        assert_eq!(format_publication_date(Some("Spring 2024")), "Spring 2024");
    }
}