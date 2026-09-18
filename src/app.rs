use crate::app::ui::{
    education_rows, nav_icon, publication_rows, selectable_text, EducationTableDelegate,
    PublicationTableDelegate, ACTIVE, DEFAULT_LINK_BLUE, FOOTER_GPUI_KIT_URL, FOOTER_RUST_URL, INK,
    PAGE_BACKGROUND, SIDEBAR_BACKGROUND,
};
use crate::resume::{wordpress_publications, Resume};
use gpui_kit::component::dock::{
    panel_handle, BasePanel, DockArea, DockLayout, DockPlacement, DockSkin, Panel as DockPanel,
    PanelEvent,
};
use gpui_kit::component::{
    link::Link, scroll::{Scrollbar, ScrollbarMode}, status_bar::StatusBar, table::TableState,
    Icon, Root,
};
use gpui_kit::{
    div, px, AnyElement, App as GpuiApp, AppContext, Context, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement, IntoElement, ParentElement, Render, ScrollHandle,
    StatefulInteractiveElement, Styled, Window,
};
use std::collections::BTreeSet;

mod pages;
mod ui;

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
    expanded_skills: BTreeSet<String>,
    expanded_countries: BTreeSet<String>,
    geography_countries_expanded: bool,
    experience_expanded: bool,
    section: Section,
}

struct SidebarPanel {
    app: Entity<App>,
    focus_handle: FocusHandle,
}

struct ContentPanel {
    app: Entity<App>,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
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
            scroll_handle: ScrollHandle::default(),
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
        let scroll_handle = self.scroll_handle.clone();
        self.app.update(cx, |app, cx| {
            div()
                .relative()
                .size_full()
                .min_h_0()
                .child(
                    div()
                        .id("content-scroll")
                        .relative()
                        .size_full()
                        .min_h_0()
                        .overflow_y_scroll()
                        .track_scroll(&scroll_handle)
                        .child(
                            div()
                                .p_6()
                                .flex()
                                .flex_col()
                                .child(app.content(cx)),
                        ),
                )
                .child(
                    div()
                        .absolute()
                        .top_0()
                        .bottom_0()
                        .right_0()
                        .w(gpui_kit::px(12.0))
                        .child(Scrollbar::vertical(&scroll_handle).mode(ScrollbarMode::Scrolling)),
                )
        })
    }
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
            expanded_skills: BTreeSet::new(),
            expanded_countries: BTreeSet::new(),
            geography_countries_expanded: false,
            experience_expanded: true,
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
                gpui_kit::rgb(ACTIVE)
            } else {
                gpui_kit::rgb(0xe7e8d2)
            })
            .text_color(gpui_kit::rgb(INK))
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

    fn nav_group_button(
        &self,
        label: &'static str,
        icon: &'static str,
        active: bool,
        expanded: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id(label)
            .w_full()
            .px_3()
            .py_3()
            .mb_2()
            .bg(if active {
                gpui_kit::rgb(ACTIVE)
            } else {
                gpui_kit::rgb(0xe7e8d2)
            })
            .text_color(gpui_kit::rgb(INK))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(nav_icon(icon))
                            .child(selectable_text(format!("nav-{label}"), label)),
                    )
                    .child(nav_icon(if expanded { "chevron-down" } else { "chevron-right" })),
            )
            .on_click(cx.listener(move |this, _, _, cx| {
                this.experience_expanded = !this.experience_expanded;
                cx.notify();
            }))
    }

    fn nav_sub_button(
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
            .py_2()
            .mb_1()
            .bg(if active {
                gpui_kit::rgb(ACTIVE)
            } else {
                gpui_kit::rgb(0xd7d8bd)
            })
            .text_color(gpui_kit::rgb(INK))
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
            .bg(gpui_kit::rgb(SIDEBAR_BACKGROUND))
            .child(self.nav_button("Profile", Section::Profile, "user", cx))
            .child(self.nav_group_button(
                "Experience",
                "calendar",
                matches!(self.section, Section::Experience | Section::Geography),
                self.experience_expanded,
                cx,
            ));
        if self.experience_expanded {
            sidebar = sidebar
                .child(self.nav_sub_button(
                    "Companies",
                    Section::Experience,
                    "building-2",
                    cx,
                ))
                .child(self.nav_sub_button("Geography", Section::Geography, "globe", cx));
        }
        sidebar = sidebar.child(self.nav_button("Skills", Section::Skills, "network", cx));
        if !self.resume.projects.is_empty() {
            sidebar =
                sidebar.child(self.nav_button("Projects", Section::Projects, "layout-dashboard", cx));
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

    fn content(&mut self, cx: &mut Context<Self>) -> AnyElement {
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
            Section::Skills => self.skills(cx),
            Section::Geography => self.geography(cx),
            Section::Profile => self.profile(),
            Section::Projects => self.projects(),
            Section::BlogPosts => self.blog_posts(),
        }
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
            dock.set_dock_size(DockPlacement::Left, px(240.0), window, cx);
        });
    });
    cx.new(|cx| Root::new(view, window, cx))
}