use crate::app::ui::{bubble, compact_bubble, link, section_eyebrow, selectable_text, ACTIVE, SIDEBAR_BACKGROUND};
use crate::app::App;
use crate::resume::profile_publications;
use gpui_kit::component::group_box::{GroupBox, GroupBoxVariants};
use gpui_kit::component::table::DataTable;
use gpui_kit::{div, AnyElement, InteractiveElement, IntoElement, ParentElement, Styled};

impl App {
    pub(crate) fn profile(&self) -> AnyElement {
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
                link(
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
                    .child(link(
                        "network-profile",
                        item.username.clone(),
                        item.url.clone(),
                    ))
                    .id(format!("network-{index}")),
            );
        }
        view.into_any_element()
    }
}