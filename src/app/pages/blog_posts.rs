use crate::app::ui::{card, format_publication_date, hero, link, selectable_text};
use crate::app::App;
use crate::resume::wordpress_publications;
use gpui_kit::{div, AnyElement, IntoElement, ParentElement, Styled};

impl App {
    pub(crate) fn blog_posts(&self) -> AnyElement {
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
                .child(link(
                    "wordpress-source".to_owned(),
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
                    .child(link(
                        format!("blog-link-{index}"),
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
}