use crate::app::ui::{card, compact_bubble, hero, link, selectable_text};
use crate::app::App;
use gpui_kit::{div, AnyElement, IntoElement, ParentElement, Styled};

impl App {
    pub(crate) fn projects(&self) -> AnyElement {
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
                    .child(link(
                        format!("project-link-{index}"),
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
}