use crate::resume::{
    company_months, geography_graph, skills_graph, wordpress_publications, Publication, Resume,
    YearMonth,
};
use gpui_kit::component::{
    bubble::{Bubble, BubbleVariant},
    dock::{
        panel_handle, BasePanel, DockArea, DockLayout, DockPlacement, DockSkin, Panel as DockPanel,
        PanelEvent,
    },
    group_box::{GroupBox, GroupBoxVariants},
    link::Link,
    table::{Column, DataTable, TableDelegate, TableState},
    Root,
};
use gpui_kit::{
    div, AnyElement, App as GpuiApp, AppContext, Context, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement, IntoElement, ParentElement, Render, StatefulInteractiveElement,
    Styled, Window,
};

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
    section: Section,
    selected_skill: Option<usize>,
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
                .child(div().p_6().child(app.content(cx)))
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
const ACCENT: u32 = 0xd96c3f;

fn nav_icon(name: &'static str) -> AnyElement {
    let mut icon = div()
        .size_4()
        .flex_none()
        .border_2()
        .border_color(gpui_kit::rgb(INK));
    if matches!(name, "user" | "globe") {
        icon = icon.rounded_full();
    } else {
        icon = icon.rounded_sm();
    }
    if matches!(name, "building-2" | "chart-pie" | "folder") {
        icon = icon.bg(gpui_kit::rgb(INK));
    }
    icon.into_any_element()
}

fn section_eyebrow(label: &'static str) -> AnyElement {
    div()
        .text_sm()
        .text_color(gpui_kit::rgb(MUTED_INK))
        .child(label)
        .into_any_element()
}

fn bubble(label: impl Into<String>) -> AnyElement {
    Bubble::new()
        .with_variant(BubbleVariant::Tinted)
        .mr_2()
        .mb_2()
        .child(div().whitespace_nowrap().child(label.into()))
        .into_any_element()
}

fn compact_bubble(label: impl Into<String>) -> AnyElement {
    div()
        .flex_none()
        .ml_2()
        .px_1()
        .py_0()
        .rounded_sm()
        .bg(gpui_kit::rgb(ACTIVE))
        .text_xs()
        .text_color(gpui_kit::rgb(INK))
        .child(label.into())
        .into_any_element()
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

        div().text_sm().child(value)
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
            Column::new("publisher", "Publisher").width(150.).min_width(110.),
            Column::new("published", "Published").width(150.).min_width(110.),
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
                .child(value)
                .into_any_element()
        } else {
            div().text_sm().child(value).into_any_element()
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
                .child(title.into()),
        )
        .child(
            div()
                .mt_3()
                .text_lg()
                .text_color(gpui_kit::rgb(INK))
                .child(copy.into()),
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
        Self {
            dock,
            resume,
            education_table,
            publication_table,
            section: Section::Overview,
            selected_skill: None,
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
                    .child(label),
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
                .child(div().text_2xl().child("Overview"))
                .child(
                    div()
                        .mt_4()
                        .text_lg()
                        .child(self.resume.basics.summary.clone()),
                )
                .into_any_element(),
            Section::Experience => self.experience(),
            Section::Skills => self.skills(cx),
            Section::Geography => {
                let graph = geography_graph(&self.resume);
                div()
                    .child(div().text_2xl().child("Experience by country"))
                    .child(div().mt_4().child(format!(
                        "{} companies and countries | {} relationships",
                        graph.nodes.len(),
                        graph.edges.len()
                    )))
                    .child(
                        div().mt_4().child(
                            graph
                                .nodes
                                .into_iter()
                                .map(|node| node.label)
                                .collect::<Vec<_>>()
                                .join("  *  "),
                        ),
                    )
                    .into_any_element()
            }
            Section::Profile => self.profile(),
            Section::Projects => self.projects(),
            Section::BlogPosts => self.blog_posts(),
        }
    }

    fn skills(&self, cx: &mut Context<Self>) -> AnyElement {
        let graph = skills_graph(&self.resume);
        let mut view = div()
            .child(hero(
                "SKILLS",
                "Technical strengths",
                "Skills with their related keywords.",
            ))
            .child(div().mb_4().child(format!(
                "{} skill nodes | {} relationships",
                graph.nodes.len(),
                graph.edges.len()
            )));
        for (index, skill) in self.resume.skills.iter().enumerate() {
            let selected = self.selected_skill == Some(index);
            let keywords = skill.keywords.join("  *  ");
            let header = div()
                .flex()
                .child(if selected { "v " } else { "> " })
                .child(skill.name.clone())
                .child(bubble(skill.level.clone()));
            let mut row = div()
                .id(format!("skill-{index}"))
                .w_full()
                .mb_1()
                .px_4()
                .py_3()
                .bg(if selected {
                    gpui_kit::rgb(ACTIVE)
                } else {
                    gpui_kit::rgb(PANEL_BACKGROUND)
                })
                .child(header)
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.selected_skill = if this.selected_skill == Some(index) {
                        None
                    } else {
                        Some(index)
                    };
                    cx.notify();
                }));
            if selected {
                row = row.child(
                    div()
                        .ml_6()
                        .mt_2()
                        .text_sm()
                        .text_color(gpui_kit::rgb(MUTED_INK))
                        .child(keywords),
                );
            }
            view = view.child(row);
        }
        view.into_any_element()
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
        let headlines = basics
            .label
            .split(',')
            .map(|value| bubble(value.trim().to_owned()))
            .collect::<Vec<_>>();
        let mut headline_row = div().flex().flex_wrap();
        for headline in headlines {
            headline_row = headline_row.child(headline);
        }
        let contact = div()
            .min_w(gpui_kit::px(150.0))
            .child(if basics.email.is_empty() {
                div().into_any_element()
            } else {
                Self::link(
                    "email",
                    "Email".to_owned(),
                    format!("mailto:{}", basics.email),
                )
            })
            .child(div().mt_3().child(basics.phone.clone()))
            .child(div().mt_3().child(location));
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
                            .child(div().text_3xl().child(basics.name.clone()))
                            .child(headline_row),
                    )
                    .child(contact),
            );
        hero_panel = hero_panel.child(div().mt_6().text_lg().child(basics.summary.clone()));
        let mut view = div().child(hero_panel);
        let mut first_row = div().w_full().flex().flex_wrap();
        let mut second_row = div().w_full().flex().flex_wrap();
        let mut first_row_has_groups = false;
        let mut second_row_has_groups = false;
        if !self.resume.education.is_empty() {
            first_row = first_row.child(self.supporting_group(
                self.education(),
                "Education",
                7.0,
            ));
            first_row_has_groups = true;
        }
        if !self.resume.basics.profiles.is_empty() {
            first_row = first_row.child(self.supporting_group(self.network(), "Network", 3.0));
            first_row_has_groups = true;
        }
        if !profile_publications(&self.resume).is_empty() {
            second_row = second_row.child(self.supporting_group(
                self.publications(),
                "Publications",
                7.0,
            ));
            second_row_has_groups = true;
        }
        if !self.resume.languages.is_empty() {
            second_row = second_row.child(self.supporting_group(self.languages(), "Languages", 3.0));
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
        Link::new(id).href(href).child(label).into_any_element()
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
        for item in &self.resume.languages {
            view = view.child(
                div()
                    .flex()
                    .items_start()
                    .mt_3()
                    .child(item.language.clone())
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
                    .child(format!("{}: ", item.network))
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
                .child(div().mt_4().child("No projects are available."))
                .into_any_element();
        }
        let mut grid = div().flex().flex_wrap();
        for item in &self.resume.projects {
            let mut highlights = div().flex().flex_wrap();
            for highlight in &item.highlights {
                highlights = highlights.child(bubble(highlight.clone()));
            }
            grid = grid.child(card(
                div()
                    .child(Self::link(
                        "project-name",
                        item.name.clone(),
                        item.url.clone(),
                    ))
                    .child(highlights)
                    .child(div().mt_4().child(item.description.clone())),
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
                .child("Articles published on the ")
                .child(Self::link(
                    "wordpress-source",
                    "WordPress".to_owned(),
                    profile.url.clone(),
                ))
                .child(" blog.")
        } else {
            div()
                .flex()
                .items_center()
                .flex_nowrap()
                .child("Articles published from the resume's WordPress entries.")
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
        for item in &posts {
            grid = grid.child(card(
                div()
                    .child(Self::link(
                        "blog-title",
                        item.name.clone(),
                        item.url.clone(),
                    ))
                    .child(
                        div()
                            .mt_2()
                            .text_sm()
                            .child(format_publication_date(item.release_date.as_deref())),
                    ),
            ));
        }
        if posts.is_empty() {
            view = view.child(div().mt_4().child("No blog posts are available."));
        } else {
            view = view.child(grid);
        }
        view.into_any_element()
    }

    fn experience(&self) -> AnyElement {
        let totals = company_months(&self.resume, YearMonth::current()).unwrap_or_default();
        let max = totals.values().copied().max().unwrap_or(1);
        let mut view = div().child(div().text_2xl().child("Experience by company"));
        if totals.is_empty() {
            return view
                .child(div().mt_6().child("No work history is available to chart."))
                .into_any_element();
        }
        for (company, months) in totals {
            view = view.child(
                div()
                    .mt_4()
                    .child(company)
                    .child(
                        div()
                            .h_4()
                            .w(gpui_kit::px((80 + months * 320 / max) as f32))
                            .bg(gpui_kit::rgb(ACCENT)),
                    )
                    .child(format!("{months} months")),
            );
        }
        view.into_any_element()
    }
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
                    .child(div().text_xl().child(candidate))
                    .child(div().w_full().child(self.resume.basics.label.clone())),
            )
            .child(div().flex_1().min_h_0().child(self.dock.clone()))
            .child(
                div()
                    .px_6()
                    .py_3()
                    .bg(gpui_kit::rgb(0x203a36))
                    .text_color(gpui_kit::rgb(0xd7d8bd))
                    .child("Rustume | Built with Rust and GPUI"),
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
    use super::{format_publication_date, profile_publications};
    use crate::resume::{Publication, Resume};

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

}
