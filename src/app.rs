use crate::resume::{
    country_for_work, geography_graph, skills_graph, wordpress_publications,
    work_duration_months, GraphEdge, GraphNode, GraphPayload, Publication, Resume, YearMonth,
};
use gpui_kit::base::SelectableText;
use gpui_kit::component::plot::shape::BarAlignment;
use gpui_kit::component::{
    bubble::{Bubble, BubbleContent, BubbleVariant},
    chart::BarChart,
    dock::{
        panel_handle, BasePanel, DockArea, DockLayout, DockPlacement, DockSkin, Panel as DockPanel,
        PanelEvent,
    },
    group_box::{GroupBox, GroupBoxVariants},
    link::Link,
    list::ListItem,
    scroll::ScrollableElement,
    status_bar::StatusBar,
    table::{Column, DataTable, TableDelegate, TableState},
    tree::{tree, TreeItem, TreeState},
    Icon, Root,
};
use gpui_kit::{
    div, AnyElement, App as GpuiApp, AppContext, Context, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement, IntoElement, ParentElement, Render, StatefulInteractiveElement,
    Styled, Window,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Section {
    Overview,
    Experience,
    Skills,
    Geography,
    Profile,
    Projects,
    BlogPosts,
}

pub struct App {
    dock: Entity<DockArea>,
    resume: Resume,
    education_table: Entity<TableState<EducationTableDelegate>>,
    publication_table: Entity<TableState<PublicationTableDelegate>>,
    skill_tree: Entity<TreeState>,
    expanded_countries: BTreeSet<String>,
    section: Section,
}

struct SidebarPanel {
    app: Entity<App>,
    focus_handle: FocusHandle,
}

struct ContentPanel {
    app: Entity<App>,
    focus_handle: FocusHandle,
}

impl SidebarPanel {
    fn new(app: Entity<App>, cx: &mut Context<Self>) -> Self {
        Self {
            app,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl ContentPanel {
    fn new(app: Entity<App>, cx: &mut Context<Self>) -> Self {
        Self {
            app,
            focus_handle: cx.focus_handle(),
        }
    }
}

macro_rules! impl_dock_panel {
    ($panel:ty, $name:literal) => {
        impl EventEmitter<PanelEvent> for $panel {}

        impl Focusable for $panel {
            fn focus_handle(&self, _: &GpuiApp) -> FocusHandle {
                self.focus_handle.clone()
            }
        }

        impl BasePanel for $panel {
            fn panel_name(&self) -> &'static str {
                $name
            }
        }

        impl DockPanel for $panel {
            fn title(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
            }

            fn zoom_control(&self, _: &GpuiApp) -> Option<gpui_kit::component::dock::PanelControl> {
                None
            }
        }
    };
}

impl_dock_panel!(SidebarPanel, "sidebar");
impl_dock_panel!(ContentPanel, "content");

impl Render for SidebarPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.app
            .update(cx, |app, cx| app.sidebar(cx).into_any_element())
    }
}

impl Render for ContentPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.app.update(cx, |app, cx| {
            div()
                .id("content-scroll")
                .size_full()
                .min_h_0()
                .overflow_y_scroll()
                .child(
                    div()
                        .p_6()
                        .flex()
                        .flex_col()
                        .size_full()
                        .min_h_0()
                        .child(app.content(cx)),
                )
                .into_any_element()
        })
    }
}

const PAGE_BACKGROUND: u32 = 0xf4efe6;
const PANEL_BACKGROUND: u32 = 0xfffcf6;
const SIDEBAR_BACKGROUND: u32 = 0xd7d8bd;
const INK: u32 = 0x203a36;
const MUTED_INK: u32 = 0x67735d;
const ACTIVE: u32 = 0xefb15d;
const DEFAULT_LINK_BLUE: u32 = 0x2563eb;
const UNKNOWN_COUNTRY_COLOR: u32 = 0x6b7280;
const FOOTER_RUST_URL: &str = "https://rust-lang.org/";
const FOOTER_GPUI_KIT_URL: &str = "https://gpui-kit.com";

fn nav_icon(name: &'static str) -> AnyElement {
    Icon::new(Icon::empty().path(format!("icons/{name}.svg")))
        .size_4()
        .text_color(gpui_kit::rgb(INK))
        .into_any_element()
}

fn skill_tree_icon_name(is_folder: bool, is_expanded: bool) -> &'static str {
    match (is_folder, is_expanded) {
        (true, true) => "folder-open",
        (true, false) => "folder-closed",
        (false, _) => "file",
    }
}

fn skill_tree_icon(is_folder: bool, is_expanded: bool) -> AnyElement {
    nav_icon(skill_tree_icon_name(is_folder, is_expanded))
}

fn section_eyebrow(label: &'static str) -> AnyElement {
    div()
        .text_sm()
        .text_color(gpui_kit::rgb(MUTED_INK))
        .child(selectable_text(format!("eyebrow-{label}"), label))
        .into_any_element()
}

fn selectable_text(id: impl Into<gpui_kit::ElementId>, value: impl Into<String>) -> AnyElement {
    SelectableText::new(id, value.into()).into_any_element()
}

fn bubble(label: impl Into<String>) -> AnyElement {
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

fn compact_bubble(label: impl Into<String>) -> AnyElement {
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

fn skill_tree_children(graph: &GraphPayload, skill_name: &str) -> Vec<GraphNode> {
    let Some(parent_id) = graph
        .nodes
        .iter()
        .find(|node| node.group == "skill" && node.label == skill_name)
        .map(|node| node.id.as_str())
    else {
        return Vec::new();
    };

    graph
        .edges
        .iter()
        .filter(|edge| edge.source == parent_id && edge.label.as_deref() == Some("sub-skill"))
        .filter_map(|edge| graph.nodes.iter().find(|node| node.id == edge.target))
        .cloned()
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CountryGraphProjection {
    countries: Vec<GraphNode>,
    companies: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CountryGraphLayout {
    Stacked,
    Split,
}

fn country_graph_projection(
    graph: &GraphPayload,
    expanded_countries: &BTreeSet<String>,
) -> CountryGraphProjection {
    let countries = graph
        .nodes
        .iter()
        .filter(|node| node.group == "country")
        .cloned()
        .collect::<Vec<_>>();
    let edges = graph
        .edges
        .iter()
        .filter(|edge| expanded_countries.contains(&edge.target))
        .cloned()
        .collect::<Vec<_>>();
    let visible_companies = edges
        .iter()
        .map(|edge| edge.source.as_str())
        .collect::<BTreeSet<_>>();
    let companies = graph
        .nodes
        .iter()
        .filter(|node| node.group == "company" && visible_companies.contains(node.id.as_str()))
        .cloned()
        .collect();

    CountryGraphProjection {
        countries,
        companies,
        edges,
    }
}

fn country_graph_layout(longest_label: usize, available_width: u32) -> CountryGraphLayout {
    if available_width >= 680 && longest_label <= 36 {
        CountryGraphLayout::Split
    } else {
        CountryGraphLayout::Stacked
    }
}

fn skill_tree_items(resume: &Resume) -> Vec<TreeItem> {
    let graph = skills_graph(resume);
    resume
        .skills
        .iter()
        .map(|skill| {
            let children = skill_tree_children(&graph, &skill.name)
                .into_iter()
                .map(|child| TreeItem::new(child.id, child.label))
                .collect::<Vec<_>>();
            let parent = graph
                .nodes
                .iter()
                .find(|node| node.group == "skill" && node.label == skill.name)
                .expect("skills graph must contain every resume skill");
            TreeItem::new(parent.id.clone(), parent.label.clone())
                .children(children)
        })
        .collect()
}

#[derive(Clone, Debug)]
struct EducationRow {
    institution: String,
    study: String,
    dates: String,
    area: String,
}

struct EducationTableDelegate {
    rows: Vec<EducationRow>,
    columns: Vec<Column>,
}

impl EducationTableDelegate {
    fn new(rows: Vec<EducationRow>) -> Self {
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
struct PublicationRow {
    name: String,
    publisher: String,
    published: String,
    url: String,
}

struct PublicationTableDelegate {
    rows: Vec<PublicationRow>,
    columns: Vec<Column>,
}

impl PublicationTableDelegate {
    fn new(rows: Vec<PublicationRow>) -> Self {
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

fn education_rows(resume: &Resume) -> Vec<EducationRow> {
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

fn publication_rows(resume: &Resume) -> Vec<PublicationRow> {
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

fn card(content: impl IntoElement) -> AnyElement {
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

fn hero(eyebrow: &'static str, title: impl Into<String>, copy: impl Into<String>) -> AnyElement {
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

fn profile_publications(resume: &Resume) -> Vec<&Publication> {
    resume
        .publications
        .iter()
        .filter(|publication| !publication.publisher.eq_ignore_ascii_case("wordpress"))
        .collect()
}

impl App {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let (dock, _) = DockSkin::dock_area("rustume", Some(1), window, cx);
        let resume = crate::resume::parse_resume(include_str!("../assets/resume.json"))
            .expect("bundled resume must be valid");
        let education_delegate = EducationTableDelegate::new(education_rows(&resume));
        let education_table = cx.new(|cx| {
            TableState::new(education_delegate, window, cx)
                .row_selectable(false)
                .col_selectable(false)
                .sortable(false)
                .col_movable(false)
                .col_resizable(true)
        });
        let publication_delegate = PublicationTableDelegate::new(publication_rows(&resume));
        let publication_table = cx.new(|cx| {
            TableState::new(publication_delegate, window, cx)
                .row_selectable(false)
                .col_selectable(false)
                .sortable(false)
                .col_movable(false)
                .col_resizable(true)
        });
        let skill_tree = cx.new(|cx| TreeState::new(cx).items(skill_tree_items(&resume)));
        Self {
            dock,
            resume,
            education_table,
            publication_table,
            skill_tree,
            expanded_countries: BTreeSet::new(),
            section: Section::Overview,
        }
    }

    fn nav_button(
        &self,
        label: &'static str,
        section: Section,
        icon: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let active = self.section == section;
        div()
            .id(label)
            .w_full()
            .px_3()
            .py_3()
            .mb_2()
            .bg(if active {
                gpui_kit::rgb(0xefb15d)
            } else {
                gpui_kit::rgb(0xe7e8d2)
            })
            .text_color(gpui_kit::rgb(0x203a36))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(nav_icon(icon))
                    .child(selectable_text(format!("nav-{label}"), label)),
            )
            .on_click(cx.listener(move |this, _, _, cx| {
                this.section = section;
                cx.notify();
            }))
    }

    fn sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let mut sidebar = div()
            .w_64()
            .size_full()
            .p_4()
            .bg(gpui_kit::rgb(0xd7d8bd))
            .child(self.nav_button("Profile", Section::Profile, "user", cx))
            .child(self.nav_button("Experience", Section::Experience, "building-2", cx))
            .child(self.nav_button("Skills", Section::Skills, "chart-pie", cx))
            .child(self.nav_button("Geography", Section::Geography, "globe", cx));
        if !self.resume.projects.is_empty() {
            sidebar = sidebar.child(self.nav_button("Projects", Section::Projects, "folder", cx));
        }
        if !wordpress_publications(&self.resume).is_empty()
            || self
                .resume
                .basics
                .profiles
                .iter()
                .any(|profile| profile.network.eq_ignore_ascii_case("wordpress blog"))
        {
            sidebar =
                sidebar.child(self.nav_button("Blog Posts", Section::BlogPosts, "book-open", cx));
        }
        sidebar
    }

    fn content(&self, cx: &mut Context<Self>) -> AnyElement {
        match self.section {
            Section::Overview => div()
                .child(
                    div()
                        .text_2xl()
                        .child(selectable_text("overview-heading", "Overview")),
                )
                .child(div().mt_4().text_lg().child(selectable_text(
                    "overview-summary",
                    self.resume.basics.summary.clone(),
                )))
                .into_any_element(),
            Section::Experience => self.experience(),
            Section::Skills => self.skills(),
            Section::Geography => self.geography(cx),
            Section::Profile => self.profile(),
            Section::Projects => self.projects(),
            Section::BlogPosts => self.blog_posts(),
        }
    }

    fn geography(&self, cx: &mut Context<Self>) -> AnyElement {
        let graph = geography_graph(&self.resume);
        let projection = country_graph_projection(&graph, &self.expanded_countries);
        let mut view = div()
            .child(div().text_2xl().child(selectable_text(
                "geography-heading",
                "Experience by country",
            )))
            .child(div().mt_2().text_color(gpui_kit::rgb(MUTED_INK)).child(
                selectable_text(
                    "geography-summary",
                    "Select a country to reveal the companies connected to it.",
                ),
            ));

        if projection.countries.is_empty() {
            return view
                .child(div().mt_6().child(selectable_text(
                    "geography-empty",
                    "No work history is available to group by country.",
                )))
                .into_any_element();
        }

        let longest_label = projection
            .countries
            .iter()
            .chain(projection.companies.iter())
            .map(|node| node.label.len())
            .max()
            .unwrap_or_default();
        let layout = country_graph_layout(longest_label, 720);
        let mut countries = div().flex_1().min_w(gpui_kit::px(220.0));
        for country in &projection.countries {
            let country_id = country.id.clone();
            let expanded = self.expanded_countries.contains(&country_id);
            let company_count = graph
                .edges
                .iter()
                .filter(|edge| edge.target == country_id)
                .count();
            countries = countries.child(
                div()
                    .id(format!("country-node-{}", country.id))
                    .w_full()
                    .mb_3()
                    .p_4()
                    .bg(gpui_kit::rgb(if expanded { ACTIVE } else { SIDEBAR_BACKGROUND }))
                    .border_1()
                    .border_color(gpui_kit::rgb(INK))
                    .rounded_lg()
                    .text_color(gpui_kit::rgb(INK))
                    .child(div().flex().items_center().justify_between().child(
                        selectable_text(
                            format!("country-label-{}", country.id),
                            country.label.clone(),
                        ),
                    ).child(selectable_text(
                        format!("country-count-{}", country.id),
                        format!("{} {company_count}", if expanded { "Hide" } else { "Show" }),
                    )))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if !this.expanded_countries.insert(country_id.clone()) {
                            this.expanded_countries.remove(&country_id);
                        }
                        cx.notify();
                    })),
            );
        }

        let mut companies = div().flex_1().min_w(gpui_kit::px(260.0));
        if projection.companies.is_empty() {
            companies = companies.child(div().mt_2().text_color(gpui_kit::rgb(MUTED_INK)).child(
                selectable_text("geography-companies-empty", "Choose a country to view its companies."),
            ));
        } else {
            for edge in &projection.edges {
                let company = projection
                    .companies
                    .iter()
                    .find(|company| company.id == edge.source)
                    .expect("projected edge must have a visible company");
                let country = projection
                    .countries
                    .iter()
                    .find(|country| country.id == edge.target)
                    .expect("projected edge must have a country");
                companies = companies.child(
                    div()
                        .flex()
                        .items_center()
                        .mb_3()
                        .child(div().w_8().h_1().bg(gpui_kit::rgb(ACTIVE)))
                        .child(
                            div()
                                .flex_1()
                                .p_4()
                                .bg(gpui_kit::rgb(PANEL_BACKGROUND))
                                .border_1()
                                .border_color(gpui_kit::rgb(SIDEBAR_BACKGROUND))
                                .rounded_lg()
                                .child(selectable_text(
                                    format!("company-label-{}", company.id),
                                    company.label.clone(),
                                ))
                                .child(div().mt_1().text_sm().text_color(gpui_kit::rgb(MUTED_INK)).child(
                                    selectable_text(
                                        format!("company-country-{}", company.id),
                                        country.label.clone(),
                                    ),
                                )),
                        ),
                );
            }
        }

        let graph_view = match layout {
            CountryGraphLayout::Split => div().flex().flex_wrap().gap_6().child(countries).child(companies),
            CountryGraphLayout::Stacked => div().flex().flex_col().gap_4().child(countries).child(companies),
        };
        view = view.child(div().mt_6().p_4().bg(gpui_kit::rgb(0xe7e8d2)).rounded_lg().child(graph_view));
        view.into_any_element()
    }

    fn skills(&self) -> AnyElement {
        let graph = skills_graph(&self.resume);
        let skill_levels = self
            .resume
            .skills
            .iter()
            .map(|skill| (skill.name.clone(), skill.level.clone()))
            .collect::<BTreeMap<_, _>>();
        let skills_tree = tree(&self.skill_tree, move |_, entry, _, _, _| {
            let label = entry.item().label.to_string();
            let mut content = div()
                .flex()
                .items_center()
                .gap_x_2()
                .ml_2()
                .child(skill_tree_icon(entry.is_folder(), entry.is_expanded()))
                .child(selectable_text(
                    format!("skill-tree-{}", entry.item().id),
                    label.clone(),
                ));
            if entry.depth() > 0 {
                content = content.ml_4();
            }
            if entry.is_root() {
                if let Some(level) = skill_levels.get(&label).filter(|level| !level.is_empty()) {
                    content = content.child(compact_bubble(level.clone()));
                }
            }
            ListItem::new(entry.item().id.clone()).child(content)
        });
        div()
            .flex()
            .flex_col()
            .size_full()
            .min_h_0()
            .child(hero(
                "SKILLS",
                "Technical strengths",
                "Skills with their related keywords.",
            ))
            .child(div().mb_4().child(selectable_text(
                "skills-summary",
                format!(
                    "{} skill nodes | {} relationships",
                    graph.nodes.len(),
                    graph.edges.len()
                ),
            )))
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .child(skills_tree),
            )
            .into_any_element()
    }

    fn profile(&self) -> AnyElement {
        let basics = &self.resume.basics;
        let location = basics
            .location
            .as_ref()
            .map(|location| {
                format!(
                    "{}, {} {}",
                    location.city, location.region, location.country_code
                )
            })
            .unwrap_or_default();
        let mut headline_row = div().flex().flex_wrap();
        for value in basics.label.split(',') {
            headline_row = headline_row.child(bubble(value.trim().to_owned()));
        }
        let contact = div()
            .min_w(gpui_kit::px(150.0))
            .child(if basics.email.is_empty() {
                div().into_any_element()
            } else {
                Self::link(
                    "email",
                    basics.email.clone(),
                    format!("mailto:{}", basics.email),
                )
            })
            .child(
                div()
                    .mt_3()
                    .child(selectable_text("profile-phone", basics.phone.clone())),
            )
            .child(
                div()
                    .mt_3()
                    .child(selectable_text("profile-location", location)),
            );
        let mut hero_panel = div()
            .w_full()
            .p_6()
            .mb_4()
            .bg(gpui_kit::rgb(SIDEBAR_BACKGROUND))
            .border_1()
            .border_color(gpui_kit::rgb(ACTIVE))
            .rounded_lg()
            .child(section_eyebrow("PROFILE"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .mt_2()
                    .child(
                        div()
                            .flex_1()
                            .min_w(gpui_kit::px(200.0))
                            .child(
                                div()
                                    .text_3xl()
                                    .child(selectable_text("profile-name", basics.name.clone())),
                            )
                            .child(headline_row),
                    )
                    .child(contact),
            );
        hero_panel = hero_panel.child(
            div()
                .mt_6()
                .text_lg()
                .child(selectable_text("profile-summary", basics.summary.clone())),
        );
        let mut view = div().child(hero_panel);
        let mut first_row = div().w_full().flex().flex_wrap();
        let mut second_row = div().w_full().flex().flex_wrap();
        let mut first_row_has_groups = false;
        let mut second_row_has_groups = false;
        if !self.resume.education.is_empty() {
            first_row = first_row.child(self.supporting_group(self.education(), "Education", 7.0));
            first_row_has_groups = true;
        }
        if !self.resume.basics.profiles.is_empty() {
            first_row = first_row.child(self.supporting_group(self.network(), "Network", 3.0));
            first_row_has_groups = true;
        }
        if !profile_publications(&self.resume).is_empty() {
            second_row =
                second_row.child(self.supporting_group(self.publications(), "Publications", 7.0));
            second_row_has_groups = true;
        }
        if !self.resume.languages.is_empty() {
            second_row =
                second_row.child(self.supporting_group(self.languages(), "Languages", 3.0));
            second_row_has_groups = true;
        }
        if first_row_has_groups {
            view = view.child(first_row);
        }
        if second_row_has_groups {
            view = view.child(second_row);
        }
        view.into_any_element()
    }

    fn supporting_group(&self, content: AnyElement, title: &'static str, grow: f32) -> AnyElement {
        div()
            .flex_basis(gpui_kit::px(0.0))
            .flex_grow(grow)
            .min_w(gpui_kit::px(280.0))
            .m_2()
            .child(GroupBox::new().outline().title(title).child(content))
            .into_any_element()
    }

    fn link(id: &'static str, label: String, href: String) -> AnyElement {
        Link::new(id)
            .href(href)
            .text_color(gpui_kit::rgb(DEFAULT_LINK_BLUE))
            .child(selectable_text(format!("{id}-label"), label))
            .into_any_element()
    }

    fn education(&self) -> AnyElement {
        div()
            .w_full()
            .h(gpui_kit::px(150.0))
            .child(
                DataTable::new(&self.education_table)
                    .bordered(true)
                    .stripe(false)
                    .scrollbar_visible(true, false),
            )
            .into_any_element()
    }

    fn publications(&self) -> AnyElement {
        div()
            .w_full()
            .h(gpui_kit::px(150.0))
            .child(
                DataTable::new(&self.publication_table)
                    .bordered(true)
                    .stripe(false)
                    .scrollbar_visible(true, false),
            )
            .into_any_element()
    }

    fn languages(&self) -> AnyElement {
        let mut view = div();
        for (index, item) in self.resume.languages.iter().enumerate() {
            view = view.child(
                div()
                    .flex()
                    .items_start()
                    .mt_3()
                    .child(selectable_text(
                        format!("language-{index}"),
                        item.language.clone(),
                    ))
                    .child(compact_bubble(item.fluency.clone())),
            );
        }
        view.into_any_element()
    }

    fn network(&self) -> AnyElement {
        let mut view = div();
        for (index, item) in self.resume.basics.profiles.iter().enumerate() {
            view = view.child(
                div()
                    .flex()
                    .items_center()
                    .mt_3()
                    .child(selectable_text(
                        format!("network-name-{index}"),
                        format!("{}: ", item.network),
                    ))
                    .child(Self::link(
                        "network-profile",
                        item.username.clone(),
                        item.url.clone(),
                    ))
                    .id(format!("network-{index}")),
            );
        }
        view.into_any_element()
    }

    fn projects(&self) -> AnyElement {
        let view = div().child(hero("PROJECTS", "Selected works", "GitHub code samples."));
        if self.resume.projects.is_empty() {
            return view
                .child(div().mt_4().child(selectable_text(
                    "projects-empty",
                    "No projects are available.",
                )))
                .into_any_element();
        }
        let mut grid = div().flex().flex_wrap();
        for (index, item) in self.resume.projects.iter().enumerate() {
            let mut highlights = div().flex().flex_wrap();
            for highlight in &item.highlights {
                highlights = highlights.child(compact_bubble(highlight.clone()));
            }
            grid = grid.child(card(
                div()
                    .child(Self::link(
                        "project-name",
                        item.name.clone(),
                        item.url.clone(),
                    ))
                    .child(highlights)
                    .child(div().mt_4().child(selectable_text(
                        format!("project-description-{index}"),
                        item.description.clone(),
                    ))),
            ));
        }
        view.child(grid).into_any_element()
    }

    fn blog_posts(&self) -> AnyElement {
        let blog_source = self
            .resume
            .basics
            .profiles
            .iter()
            .find(|profile| profile.network.eq_ignore_ascii_case("wordpress blog"));
        let intro = if let Some(profile) = blog_source {
            div()
                .flex()
                .items_center()
                .flex_nowrap()
                .child(selectable_text(
                    "blog-intro-before",
                    "Articles published on the ",
                ))
                .child(Self::link(
                    "wordpress-source",
                    "WordPress".to_owned(),
                    profile.url.clone(),
                ))
                .child(selectable_text("blog-intro-after", " blog."))
        } else {
            div()
                .flex()
                .items_center()
                .flex_nowrap()
                .child(selectable_text(
                    "blog-intro-only",
                    "Articles published from the resume's WordPress entries.",
                ))
        };
        let mut view = div()
            .child(hero(
                "BLOG POSTS",
                "Musings on experiences in life and programming.",
                "",
            ))
            .child(div().whitespace_nowrap().child(intro));
        let posts = wordpress_publications(&self.resume);
        let mut grid = div().flex().flex_wrap();
        for (index, item) in posts.iter().enumerate() {
            grid = grid.child(card(
                div()
                    .child(Self::link(
                        "blog-title",
                        item.name.clone(),
                        item.url.clone(),
                    ))
                    .child(div().mt_2().text_sm().child(selectable_text(
                        format!("blog-date-{index}"),
                        format_publication_date(item.release_date.as_deref()),
                    ))),
            ));
        }
        if posts.is_empty() {
            view = view.child(div().mt_4().child(selectable_text(
                "blog-empty",
                "No blog posts are available.",
            )));
        } else {
            view = view.child(grid);
        }
        view.into_any_element()
    }

    fn experience(&self) -> AnyElement {
        let entries = experience_entries(&self.resume, YearMonth::current());
        let mut view = div().child(div().text_2xl().child(selectable_text(
            "experience-heading",
            "Experience by company",
        )));
        if entries.is_empty() {
            return view
                .child(div().mt_6().child(selectable_text(
                    "experience-empty",
                    "No work history is available to chart.",
                )))
                .into_any_element();
        }

        let companies = entries
            .iter()
            .map(|entry| entry.company.clone())
            .collect::<Vec<_>>();
        let layout = experience_chart_layout(&companies, 720);
        let chart_data = entries.clone();
        let chart_height = match layout {
            ExperienceChartLayout::Horizontal => {
                gpui_kit::px((chart_data.len() as f32 * 56.0 + 48.0).max(180.0))
            }
            ExperienceChartLayout::Vertical => gpui_kit::px(380.0),
        };
        let alignment = match layout {
            ExperienceChartLayout::Horizontal => BarAlignment::Left,
            ExperienceChartLayout::Vertical => BarAlignment::Bottom,
        };
        let chart = BarChart::new(chart_data)
            .band(|entry| entry.company.clone())
            .value(|entry| entry.months)
            .label(|entry| format!("{:.0} months", entry.months))
            .fill(|entry, _, _, _| gpui_kit::rgb(entry.color))
            .alignment(alignment)
            .corner_radii(gpui_kit::Corners::all(gpui_kit::px(8.0)))
            .grid(false)
            .value_axis(true);

        let mut legend = div().mt_3().flex().flex_wrap().gap_3();
        let mut seen_countries = BTreeMap::new();
        for entry in &entries {
            if seen_countries
                .insert(entry.country.clone(), entry.color)
                .is_none()
            {
                legend = legend.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .child(
                            div()
                                .w_3()
                                .h_3()
                                .rounded_sm()
                                .bg(gpui_kit::rgb(entry.color)),
                        )
                        .child(selectable_text(
                            format!("experience-country-legend-{}", entry.country),
                            entry.country.clone(),
                        )),
                );
            }
        }
        view = view.child(
            div()
                .mt_6()
                .w_full()
                .h(chart_height)
                .p_4()
                .overflow_x_scrollbar()
                .border_1()
                .border_color(gpui_kit::rgb(SIDEBAR_BACKGROUND))
                .child(chart)
                .child(legend),
        );
        view.into_any_element()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ExperienceEntry {
    company: String,
    months: f64,
    country: String,
    color: u32,
    latest_start: Option<YearMonth>,
}

fn experience_entries(resume: &Resume, current: YearMonth) -> Vec<ExperienceEntry> {
    let mut grouped = BTreeMap::<String, (u32, String, Option<YearMonth>)>::new();
    for work in &resume.work {
        let company = if work.name.is_empty() {
            "Unknown company".to_owned()
        } else {
            work.name.clone()
        };
        let Ok(months) = work_duration_months(work, current) else {
            continue;
        };
        let start = work
            .start_date
            .as_deref()
            .and_then(|value| YearMonth::parse(&value[..7.min(value.len())]).ok());
        let country = country_for_work(work);
        let entry = grouped
            .entry(company)
            .or_insert((0, country.clone(), start));
        entry.0 += months;
        if start > entry.2 {
            entry.1 = country;
            entry.2 = start;
        }
    }

    let mut countries = grouped
        .values()
        .map(|(_, country, _)| country.clone())
        .collect::<Vec<_>>();
    countries.sort();
    countries.dedup();
    let palette = [0xd96c3f, 0x2f7f73, 0xd39b35, 0x5f6fb5, 0x9b5c83, 0x578b5b];
    let colors = countries
        .into_iter()
        .enumerate()
        .map(|(index, country)| {
            let color = if country == "Unknown" {
                UNKNOWN_COUNTRY_COLOR
            } else {
                palette[index % palette.len()]
            };
            (country, color)
        })
        .collect::<BTreeMap<_, _>>();

    let mut entries = grouped
        .into_iter()
        .map(
            |(company, (months, country, latest_start))| ExperienceEntry {
                company,
                months: months as f64,
                color: colors[&country],
                country,
                latest_start,
            },
        )
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| {
        right
            .latest_start
            .cmp(&left.latest_start)
            .then_with(|| left.company.cmp(&right.company))
    });
    entries
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExperienceChartLayout {
    Horizontal,
    Vertical,
}

fn experience_chart_layout(companies: &[String], available_width: u32) -> ExperienceChartLayout {
    let longest_label = companies.iter().map(String::len).max().unwrap_or(0);
    let estimated_width = companies.len() as u32 * 140;
    if longest_label > 20 || estimated_width > available_width {
        ExperienceChartLayout::Horizontal
    } else {
        ExperienceChartLayout::Vertical
    }
}

fn footer_attribution() -> AnyElement {
    div()
        .flex()
        .items_center()
        .gap_1()
        .child(selectable_text("footer-made-with", "Made with"))
        .child(
            Icon::new(Icon::empty().path("icons/heart.svg"))
                .size_3()
                .text_color(gpui_kit::rgb(0xdc2626)),
        )
        .child(selectable_text("footer-using", "using"))
        .child(
            Link::new("footer-rust-link")
                .href(FOOTER_RUST_URL)
                .text_color(gpui_kit::rgb(DEFAULT_LINK_BLUE))
                .child(selectable_text("footer-rust", "Rust")),
        )
        .child(selectable_text("footer-using-gpui-kit", "and"))
        .child(
            Link::new("footer-gpui-kit-link")
                .href(FOOTER_GPUI_KIT_URL)
                .text_color(gpui_kit::rgb(DEFAULT_LINK_BLUE))
                .child(selectable_text("footer-gpui-kit", "gpui-kit")),
        )
        .into_any_element()
}

impl Render for App {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let _ = &self.dock;
        let candidate = if self.resume.basics.name.is_empty() {
            "Resume visualizer".to_owned()
        } else {
            self.resume.basics.name.clone()
        };
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(gpui_kit::rgb(PAGE_BACKGROUND))
            .child(
                div()
                    .px_6()
                    .py_4()
                    .bg(gpui_kit::rgb(0x203a36))
                    .text_color(gpui_kit::rgb(0xf4efe6))
                    .child(
                        div()
                            .text_xl()
                            .child(selectable_text("header-candidate", candidate)),
                    )
                    .child(div().w_full().child(selectable_text(
                        "header-label",
                        self.resume.basics.label.clone(),
                    ))),
            )
            .child(div().flex_1().min_h_0().child(self.dock.clone()))
            .child(
                StatusBar::new()
                    .left(selectable_text("footer-name", "Rustume"))
                    .right(footer_attribution()),
            )
    }
}

pub fn root_view(window: &mut Window, cx: &mut gpui_kit::App) -> Entity<Root> {
    let view = cx.new(|cx| App::new(window, cx));
    let sidebar = cx.new(|cx| SidebarPanel::new(view.clone(), cx));
    let content = cx.new(|cx| ContentPanel::new(view.clone(), cx));
    view.update(cx, |app, cx| {
        app.dock.update(cx, |dock, cx| {
            dock.set_center(
                DockLayout::tabs().panel_view(panel_handle(content), cx),
                window,
                cx,
            );
            dock.set_dock(
                DockPlacement::Left,
                DockLayout::tabs().panel_view(panel_handle(sidebar), cx),
                window,
                cx,
            );
            dock.set_dock_size(DockPlacement::Left, gpui_kit::px(240.0), window, cx);
        });
    });
    cx.new(|cx| Root::new(view, window, cx))
}

fn format_publication_date(value: Option<&str>) -> String {
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

#[cfg(test)]
mod tests {
    use super::{
        country_graph_layout, country_graph_projection, experience_chart_layout, experience_entries,
        format_publication_date, profile_publications, skill_tree_children, skill_tree_icon_name,
        skill_tree_items, CountryGraphLayout, ExperienceChartLayout, DEFAULT_LINK_BLUE,
        FOOTER_GPUI_KIT_URL, FOOTER_RUST_URL, UNKNOWN_COUNTRY_COLOR,
    };
    use crate::resume::{
        geography_graph, skills_graph, Location, Publication, Resume, Skill, WorkEntry, YearMonth,
    };
    use std::collections::{BTreeMap, BTreeSet};

    fn publication(publisher: &str) -> Publication {
        Publication {
            name: publisher.to_owned(),
            publisher: publisher.to_owned(),
            ..Default::default()
        }
    }

    #[test]
    fn profile_publications_excludes_wordpress_entries_case_insensitively() {
        let resume = Resume {
            publications: vec![
                publication("ACM"),
                publication("WordPress"),
                publication("wordpress"),
            ],
            ..Default::default()
        };

        let publications = profile_publications(&resume);

        assert_eq!(publications.len(), 1);
        assert_eq!(publications[0].publisher, "ACM");
    }

    #[test]
    fn profile_publications_is_empty_for_blog_only_or_empty_input() {
        let empty = Resume::default();
        let blog_only = Resume {
            publications: vec![publication("WordPress")],
            ..Default::default()
        };

        assert!(profile_publications(&empty).is_empty());
        assert!(profile_publications(&blog_only).is_empty());
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

    #[test]
    fn long_company_labels_use_horizontal_chart_layout() {
        let companies = vec!["A company with a very long name".to_owned()];
        assert_eq!(
            experience_chart_layout(&companies, 720),
            ExperienceChartLayout::Horizontal
        );
    }

    #[test]
    fn short_company_labels_can_use_vertical_chart_layout_when_wide_enough() {
        let companies = vec!["Rustume".to_owned(), "GPUI".to_owned()];
        assert_eq!(
            experience_chart_layout(&companies, 720),
            ExperienceChartLayout::Vertical
        );
    }

    #[test]
    fn footer_uses_official_project_urls() {
        assert_eq!(FOOTER_RUST_URL, "https://rust-lang.org/");
        assert_eq!(FOOTER_GPUI_KIT_URL, "https://gpui-kit.com");
    }

    #[test]
    fn experience_entries_sort_newest_first_and_color_countries() {
        let resume = Resume {
            work: vec![
                WorkEntry {
                    name: "Older company".to_owned(),
                    start_date: Some("2018-01".to_owned()),
                    end_date: Some("2020-01".to_owned()),
                    location: Some(Location::Structured {
                        country_code: Some("US".to_owned()),
                        country: None,
                        _other: BTreeMap::new(),
                    }),
                    ..Default::default()
                },
                WorkEntry {
                    name: "Newer company".to_owned(),
                    start_date: Some("2022-01".to_owned()),
                    end_date: Some("2024-01".to_owned()),
                    location: Some(Location::Text("India".to_owned())),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let entries = experience_entries(&resume, YearMonth::parse("2024-12").unwrap());

        assert_eq!(entries[0].company, "Newer company");
        assert_eq!(entries[0].country, "India");
        assert_ne!(entries[0].color, entries[1].color);
    }

    #[test]
    fn experience_entries_infer_country_from_company_name_and_keep_unknown_distinct() {
        let resume = Resume {
            work: vec![
                WorkEntry {
                    name: "Company, Berlin, Germany".to_owned(),
                    start_date: Some("2020-01".to_owned()),
                    end_date: Some("2021-01".to_owned()),
                    ..Default::default()
                },
                WorkEntry {
                    name: "Company without location".to_owned(),
                    start_date: Some("2018-01".to_owned()),
                    end_date: Some("2019-01".to_owned()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let entries = experience_entries(&resume, YearMonth::parse("2024-12").unwrap());

        assert_eq!(entries[0].country, "Germany");
        assert_eq!(entries[1].country, "Unknown");
        assert_eq!(entries[1].color, UNKNOWN_COUNTRY_COLOR);
        assert_ne!(entries[0].color, entries[1].color);
    }

    #[test]
    fn shared_links_use_default_blue() {
        assert_eq!(DEFAULT_LINK_BLUE, 0x2563eb);
    }

    #[test]
    fn country_graph_projection_keeps_countries_and_reveals_expanded_companies() {
        let resume = Resume {
            work: vec![
                WorkEntry {
                    name: "London company".to_owned(),
                    location: Some(Location::Text("London, United Kingdom".to_owned())),
                    ..Default::default()
                },
                WorkEntry {
                    name: "Unlocated company".to_owned(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let graph = geography_graph(&resume);
        let empty = country_graph_projection(&graph, &BTreeSet::new());

        assert_eq!(empty.countries.len(), 2);
        assert!(empty.countries.iter().any(|node| node.label == "Unknown"));
        assert!(empty.companies.is_empty());
        assert!(empty.edges.is_empty());

        let expanded = BTreeSet::from(["country:united-kingdom".to_owned()]);
        let projection = country_graph_projection(&graph, &expanded);

        assert_eq!(projection.companies.len(), 1);
        assert_eq!(projection.companies[0].label, "London company");
        assert_eq!(projection.edges.len(), 1);
        assert_eq!(projection.edges[0].target, "country:united-kingdom");
    }

    #[test]
    fn country_graph_layout_preserves_readability_at_wide_and_narrow_widths() {
        assert_eq!(
            country_graph_layout(18, 900),
            CountryGraphLayout::Split
        );
        assert_eq!(
            country_graph_layout(18, 420),
            CountryGraphLayout::Stacked
        );
        assert_eq!(
            country_graph_layout(48, 900),
            CountryGraphLayout::Stacked
        );
    }

    #[test]
    fn country_graph_projection_is_empty_when_work_history_is_empty() {
        let projection = country_graph_projection(&geography_graph(&Resume::default()), &BTreeSet::new());

        assert!(projection.countries.is_empty());
        assert!(projection.companies.is_empty());
        assert!(projection.edges.is_empty());
    }

    #[test]
    fn skill_tree_children_remain_separate_and_case_normalized() {
        let resume = Resume {
            skills: vec![Skill {
                name: "Rust".to_owned(),
                keywords: vec![
                    "Systems".to_owned(),
                    "systems".to_owned(),
                    "Async".to_owned(),
                ],
                ..Default::default()
            }],
            ..Default::default()
        };

        let children = skill_tree_children(&skills_graph(&resume), "Rust");

        assert_eq!(children.len(), 2);
        assert!(children.iter().any(|child| child.label == "Systems"));
        assert!(children.iter().any(|child| child.label == "Async"));

        let tree = skill_tree_items(&resume);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].label, "Rust");
        assert!(!tree[0].is_expanded());
        assert_eq!(
            tree[0]
                .children
                .iter()
                .map(|child| child.label.to_string())
                .collect::<Vec<_>>(),
            vec!["Async", "Systems"]
        );
        assert_eq!(skill_tree_icon_name(true, false), "folder-closed");
        assert_eq!(skill_tree_icon_name(true, true), "folder-open");
        assert_eq!(skill_tree_icon_name(false, false), "file");
    }
}
