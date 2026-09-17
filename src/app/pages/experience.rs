use crate::app::ui::{selectable_text, SIDEBAR_BACKGROUND, UNKNOWN_COUNTRY_COLOR};
use crate::app::App;
use crate::resume::{country_for_work, work_duration_months, Resume, YearMonth};
use gpui_kit::component::chart::BarChart;
use gpui_kit::component::plot::shape::BarAlignment;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::{div, AnyElement, IntoElement, ParentElement, Styled};
use std::collections::BTreeMap;

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

impl App {
    pub(crate) fn experience(&self) -> AnyElement {
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

#[cfg(test)]
mod tests {
    use super::{experience_chart_layout, experience_entries, ExperienceChartLayout};
    use crate::app::ui::UNKNOWN_COUNTRY_COLOR;
    use crate::resume::{Location, Resume, WorkEntry, YearMonth};
    use std::collections::BTreeMap;

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
}