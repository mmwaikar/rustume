use crate::resume::{
    company_months, geography_graph, skills_graph, wordpress_publications, Resume, YearMonth,
};
use gpui_kit::component::{
    dock::{DockArea, DockSkin},
    link::Link,
    Root,
};
use gpui_kit::{
    div, AnyElement, AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement,
    Render, StatefulInteractiveElement, Styled, Window,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Section {
    Overview,
    Experience,
    Skills,
    Geography,
    Profile,
    Education,
    Publications,
    Languages,
    Network,
    Projects,
    BlogPosts,
}

pub struct App {
    dock: Entity<DockArea>,
    resume: Resume,
    section: Section,
    selected_skill: Option<usize>,
}

impl App {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let (dock, _) = DockSkin::dock_area("rustume", Some(1), window, cx);
        let resume = crate::resume::parse_resume(include_str!("../assets/resume.json"))
            .expect("bundled resume must be valid");
        Self {
            dock,
            resume,
            section: Section::Overview,
            selected_skill: None,
        }
    }

    fn nav_button(
        &self,
        label: &'static str,
        section: Section,
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
            .child(label)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.section = section;
                cx.notify();
            }))
    }

    fn sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let mut sidebar = div()
            .w_64()
            .p_4()
            .bg(gpui_kit::rgb(0xd7d8bd))
            .child(self.nav_button("Overview", Section::Overview, cx))
            .child(self.nav_button("Experience", Section::Experience, cx))
            .child(self.nav_button("Skills", Section::Skills, cx))
            .child(self.nav_button("Geography", Section::Geography, cx))
            .child(self.nav_button("Profile", Section::Profile, cx));
        if !self.resume.education.is_empty() {
            sidebar = sidebar.child(self.nav_button("Education", Section::Education, cx));
        }
        if !self.resume.publications.is_empty() {
            sidebar = sidebar.child(self.nav_button("Publications", Section::Publications, cx));
        }
        if !self.resume.languages.is_empty() {
            sidebar = sidebar.child(self.nav_button("Languages", Section::Languages, cx));
        }
        if crate::resume::has_network_profile(&self.resume) {
            sidebar = sidebar.child(self.nav_button("Network", Section::Network, cx));
        }
        if !self.resume.projects.is_empty() {
            sidebar = sidebar.child(self.nav_button("Projects", Section::Projects, cx));
        }
        if !wordpress_publications(&self.resume).is_empty()
            || self
                .resume
                .basics
                .profiles
                .iter()
                .any(|profile| profile.network.eq_ignore_ascii_case("wordpress blog"))
        {
            sidebar = sidebar.child(self.nav_button("Blog Posts", Section::BlogPosts, cx));
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
            Section::Education => self.education(),
            Section::Publications => self.publications(),
            Section::Languages => self.languages(),
            Section::Network => self.network(),
            Section::Projects => self.projects(),
            Section::BlogPosts => self.blog_posts(),
        }
    }

    fn skills(&self, cx: &mut Context<Self>) -> AnyElement {
        let graph = skills_graph(&self.resume);
        let mut view = div()
            .child(div().text_2xl().child("Skills relationships"))
            .child(div().mt_2().child(format!(
                "{} skill nodes | {} relationships",
                graph.nodes.len(),
                graph.edges.len()
            )));
        for (index, skill) in self.resume.skills.iter().enumerate() {
            let selected = self.selected_skill == Some(index);
            let keywords = skill.keywords.join("  *  ");
            let details = if selected {
                format!("{}: {}", skill.level, keywords)
            } else {
                skill.level.clone()
            };
            view = view.child(
                div()
                    .id(format!("skill-{index}"))
                    .mt_4()
                    .px_3()
                    .py_2()
                    .bg(if selected {
                        gpui_kit::rgb(0xefb15d)
                    } else {
                        gpui_kit::rgb(0xe7e8d2)
                    })
                    .child(skill.name.clone())
                    .child(div().mt_1().child(details))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.selected_skill = Some(index);
                        cx.notify();
                    })),
            );
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
        let mut view = div()
            .child(div().text_2xl().child("Profile"))
            .child(div().mt_4().text_xl().child(basics.name.clone()))
            .child(div().mt_2().child(basics.label.clone()))
            .child(div().mt_4().child(basics.summary.clone()))
            .child(div().mt_4().child(location));
        if !basics.email.is_empty() {
            view = view.child(div().mt_2().child(Self::link(
                "email",
                "Email".to_owned(),
                format!("mailto:{}", basics.email),
            )));
        }
        if !basics.phone.is_empty() {
            view = view.child(div().mt_2().child(Self::link(
                "phone",
                "Phone".to_owned(),
                format!("tel:{}", basics.phone),
            )));
        }
        if !basics.website.is_empty() {
            view = view.child(div().mt_2().child(Self::link(
                "website",
                "Website".to_owned(),
                basics.website.clone(),
            )));
        }
        view.into_any_element()
    }

    fn link(id: &'static str, label: String, href: String) -> AnyElement {
        Link::new(id).href(href).child(label).into_any_element()
    }

    fn education(&self) -> AnyElement {
        let mut view = div().child(div().text_2xl().child("Education"));
        for item in &self.resume.education {
            view = view.child(
                div()
                    .mt_4()
                    .child(item.study_type.clone())
                    .child(div().mt_1().child(item.institution.clone()))
                    .child(div().mt_1().child(format!(
                        "{} - {}",
                        item.start_date.clone().unwrap_or_default(),
                        item.end_date.clone().unwrap_or_default()
                    )))
                    .child(div().mt_1().child(item.area.clone())),
            );
        }
        view.into_any_element()
    }

    fn publications(&self) -> AnyElement {
        let mut view = div().child(div().text_2xl().child("Publications"));
        for item in &self.resume.publications {
            let source = if item.url.is_empty() {
                div().into_any_element()
            } else {
                Self::link(
                    "publication-source",
                    "Open publication".to_owned(),
                    item.url.clone(),
                )
            };
            view = view.child(
                div()
                    .mt_4()
                    .child(item.name.clone())
                    .child(div().mt_1().child(format!(
                        "{} | {}",
                        item.publisher,
                        item.release_date.clone().unwrap_or_default()
                    )))
                    .child(div().mt_1().child(item.summary.clone()))
                    .child(div().mt_1().child(source)),
            );
        }
        view.into_any_element()
    }

    fn languages(&self) -> AnyElement {
        let mut view = div().child(div().text_2xl().child("Languages"));
        for item in &self.resume.languages {
            view = view.child(
                div()
                    .mt_3()
                    .child(format!("{} - {}", item.language, item.fluency)),
            );
        }
        view.into_any_element()
    }

    fn network(&self) -> AnyElement {
        let mut view = div().child(div().text_2xl().child("Network"));
        for (index, item) in self.resume.basics.profiles.iter().enumerate() {
            view = view.child(
                div()
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
        let mut view = div().child(div().text_2xl().child("Projects"));
        if self.resume.projects.is_empty() {
            return view
                .child(div().mt_4().child("No projects are available."))
                .into_any_element();
        }
        for item in &self.resume.projects {
            let highlights = item.highlights.join("  *  ");
            let source = if item.url.is_empty() {
                div().into_any_element()
            } else {
                Self::link(
                    "project-source",
                    "Open project".to_owned(),
                    item.url.clone(),
                )
            };
            view = view.child(
                div()
                    .mt_4()
                    .child(item.name.clone())
                    .child(div().mt_1().child(item.description.clone()))
                    .child(div().mt_1().child(highlights))
                    .child(div().mt_1().child(source)),
            );
        }
        view.into_any_element()
    }

    fn blog_posts(&self) -> AnyElement {
        let mut view = div().child(div().text_2xl().child("Blog Posts"));
        let posts = wordpress_publications(&self.resume);
        for item in &posts {
            view = view.child(
                div()
                    .mt_4()
                    .child(item.name.clone())
                    .child(
                        div()
                            .mt_1()
                            .child(item.release_date.clone().unwrap_or_default()),
                    )
                    .child(div().mt_1().child(item.summary.clone()))
                    .child(div().mt_1().child(Self::link(
                        "blog-source",
                        "Read post".to_owned(),
                        item.url.clone(),
                    ))),
            );
        }
        if posts.is_empty() {
            view = view.child(div().mt_4().child("No blog posts are available."));
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
                            .bg(gpui_kit::rgb(0xd96c3f)),
                    )
                    .child(format!("{months} months")),
            );
        }
        view.into_any_element()
    }
}

impl Render for App {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
            .bg(gpui_kit::rgb(0xf4efe6))
            .child(
                div()
                    .px_6()
                    .py_4()
                    .bg(gpui_kit::rgb(0x203a36))
                    .text_color(gpui_kit::rgb(0xf4efe6))
                    .child(div().text_xl().child(candidate))
                    .child(self.resume.basics.label.clone()),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .child(self.sidebar(cx))
                    .child(div().flex_1().p_6().child(self.content(cx))),
            )
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
    cx.new(|cx| Root::new(view, window, cx))
}
