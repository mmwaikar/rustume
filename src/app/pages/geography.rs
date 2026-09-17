use crate::app::ui::{
    selectable_text, ACTIVE, INK, MUTED_INK, PANEL_BACKGROUND, SIDEBAR_BACKGROUND,
};
use crate::app::App;
use crate::resume::{geography_graph, GraphPayload};
use gpui_kit::{
    canvas, div, point, px, AnyElement, Context, InteractiveElement, IntoElement, ParentElement,
    PathBuilder, Pixels, Point, StatefulInteractiveElement, Styled, Window,
};
use std::collections::{BTreeMap, BTreeSet};

const GRAPH_PADDING: f32 = 24.0;
const GRAPH_COLUMN_GAP: f32 = 56.0;
const GRAPH_ROW_GAP: f32 = 14.0;
const COUNTRY_NODE_HEIGHT: f32 = 44.0;
const COMPANY_NODE_HEIGHT: f32 = 36.0;
const GRAPH_AVAILABLE_WIDTH: f32 = 720.0;

#[derive(Clone, Debug, PartialEq)]
struct GraphNodePosition {
    id: String,
    label: String,
    group: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Clone, Debug, PartialEq)]
struct GraphEdgePath {
    from_x: f32,
    from_y: f32,
    to_x: f32,
    to_y: f32,
}

#[derive(Clone, Debug, PartialEq)]
struct GraphLayout {
    nodes: Vec<GraphNodePosition>,
    edges: Vec<GraphEdgePath>,
    container_width: f32,
    container_height: f32,
}

fn graph_node_width(label: &str, min: f32, max: f32) -> f32 {
    (min + label.chars().count() as f32 * 8.5).min(max)
}

fn display_company_name(name: &str) -> String {
    match name.rsplit_once(',') {
        None => name.trim().to_owned(),
        Some((rest, _)) => match rest.trim().rsplit_once(',') {
            None => rest.trim().to_owned(),
            Some((rest, _)) => rest.trim().to_owned(),
        },
    }
}

fn country_company_counts(graph: &GraphPayload) -> BTreeMap<String, usize> {
    graph
        .edges
        .iter()
        .map(|edge| edge.target.clone())
        .fold(BTreeMap::new(), |mut counts, target| {
            *counts.entry(target).or_insert(0) += 1;
            counts
        })
}

fn compute_graph_layout(
    graph: &GraphPayload,
    expanded_countries: &BTreeSet<String>,
    available_width: f32,
) -> GraphLayout {
    let countries = graph
        .nodes
        .iter()
        .filter(|node| node.group == "country")
        .collect::<Vec<_>>();
    let mut country_widths = countries
        .iter()
        .map(|node| graph_node_width(&node.label, 140.0, 300.0))
        .collect::<Vec<_>>();
    let max_country_width = country_widths.iter().cloned().fold(0.0, f32::max);
    let company_column_x = GRAPH_PADDING + max_country_width + GRAPH_COLUMN_GAP;

    country_widths.iter_mut().for_each(|width| {
        *width = width.max(max_country_width);
    });

    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut y = GRAPH_PADDING;
    let mut rightmost = company_column_x;

    for (country, country_width) in countries.iter().zip(&country_widths) {
        let country_y = y;
        nodes.push(GraphNodePosition {
            id: country.id.clone(),
            label: country.label.clone(),
            group: country.group.clone(),
            x: GRAPH_PADDING,
            y: country_y,
            width: *country_width,
            height: COUNTRY_NODE_HEIGHT,
        });
        y += COUNTRY_NODE_HEIGHT;

        if expanded_countries.contains(&country.id) {
            let companies = graph
                .edges
                .iter()
                .filter(|edge| edge.target == country.id)
                .filter_map(|edge| graph.nodes.iter().find(|node| node.id == edge.source).cloned())
                .collect::<Vec<_>>();
            if !companies.is_empty() {
                y += GRAPH_ROW_GAP;
            }
            for company in &companies {
                let label = display_company_name(&company.label);
                let company_width = graph_node_width(&label, 180.0, 320.0);
                rightmost = rightmost.max(company_column_x + company_width);
                let company_y = y;
                nodes.push(GraphNodePosition {
                    id: company.id.clone(),
                    label,
                    group: company.group.clone(),
                    x: company_column_x,
                    y: company_y,
                    width: company_width,
                    height: COMPANY_NODE_HEIGHT,
                });
                edges.push(GraphEdgePath {
                    from_x: company_column_x,
                    from_y: company_y + COMPANY_NODE_HEIGHT / 2.0,
                    to_x: GRAPH_PADDING + *country_width,
                    to_y: country_y + COUNTRY_NODE_HEIGHT / 2.0,
                });
                y += COMPANY_NODE_HEIGHT + GRAPH_ROW_GAP;
            }
            if !companies.is_empty() {
                y -= GRAPH_ROW_GAP;
            }
        }
        y += GRAPH_ROW_GAP;
    }

    GraphLayout {
        nodes,
        edges,
        container_width: (rightmost + GRAPH_PADDING).max(available_width),
        container_height: y + GRAPH_PADDING,
    }
}

fn paint_graph_edge(edge: &GraphEdgePath, origin: Point<Pixels>, window: &mut Window) {
    let from = origin + point(px(edge.from_x), px(edge.from_y));
    let to = origin + point(px(edge.to_x), px(edge.to_y));
    let bend = ((edge.from_x - edge.to_x).abs() * 0.5).max(24.0);
    let mut builder = PathBuilder::stroke(px(2.0));
    builder.move_to(from);
    builder.cubic_bezier_to(
        to,
        origin + point(px(edge.from_x - bend), px(edge.from_y)),
        origin + point(px(edge.to_x + bend), px(edge.to_y)),
    );
    if let Ok(path) = builder.build() {
        window.paint_path(path, gpui_kit::rgb(ACTIVE));
    }
}

impl App {
    pub(crate) fn geography(&self, cx: &mut Context<Self>) -> AnyElement {
        let graph = geography_graph(&self.resume);
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

        if graph.nodes.iter().all(|node| node.group != "country") {
            return view
                .child(div().mt_6().child(selectable_text(
                    "geography-empty",
                    "No work history is available to group by country.",
                )))
                .into_any_element();
        }

        let layout = compute_graph_layout(&graph, &self.expanded_countries, GRAPH_AVAILABLE_WIDTH);
        let edges = layout.edges.clone();
        let company_counts = country_company_counts(&graph);

        let mut graph_container = div()
            .id("geography-graph")
            .relative()
            .w_full()
            .h(gpui_kit::px(layout.container_height))
            .child(
                canvas(
                    move |_, _, _| edges.clone(),
                    move |bounds, edges, window, _| {
                        for edge in &edges {
                            paint_graph_edge(edge, bounds.origin, window);
                        }
                    },
                )
                .absolute()
                .inset_0(),
            );

        for node in &layout.nodes {
            if node.group == "country" {
                let expanded = self.expanded_countries.contains(&node.id);
                let count = company_counts.get(&node.id).copied().unwrap_or_default();
                let node_id = node.id.clone();
                graph_container = graph_container.child(
                    div()
                        .id(format!("country-node-{}", node.id))
                        .absolute()
                        .left(gpui_kit::px(node.x))
                        .top(gpui_kit::px(node.y))
                        .w(gpui_kit::px(node.width))
                        .h(gpui_kit::px(node.height))
                        .px_3()
                        .flex()
                        .items_center()
                        .justify_between()
                        .bg(gpui_kit::rgb(if expanded { ACTIVE } else { SIDEBAR_BACKGROUND }))
                        .border_1()
                        .border_color(gpui_kit::rgb(INK))
                        .rounded_lg()
                        .text_color(gpui_kit::rgb(INK))
                        .child(selectable_text(
                            format!("country-label-{}", node.id),
                            node.label.clone(),
                        ))
                        .child(selectable_text(
                            format!("country-toggle-{}", node.id),
                            format!("{count} {}", if expanded { "hide" } else { "show" }),
                        ))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if !this.expanded_countries.insert(node_id.clone()) {
                                this.expanded_countries.remove(&node_id);
                            }
                            cx.notify();
                        })),
                );
            } else {
                graph_container = graph_container.child(
                    div()
                        .id(format!("company-node-{}", node.id))
                        .absolute()
                        .left(gpui_kit::px(node.x))
                        .top(gpui_kit::px(node.y))
                        .w(gpui_kit::px(node.width))
                        .h(gpui_kit::px(node.height))
                        .px_3()
                        .flex()
                        .items_center()
                        .bg(gpui_kit::rgb(PANEL_BACKGROUND))
                        .border_1()
                        .border_color(gpui_kit::rgb(SIDEBAR_BACKGROUND))
                        .rounded_lg()
                        .text_color(gpui_kit::rgb(INK))
                        .child(selectable_text(
                            format!("company-label-{}", node.id),
                            node.label.clone(),
                        )),
                );
            }
        }

        view = view.child(
            div()
                .mt_6()
                .w_full()
                .p_4()
                .bg(gpui_kit::rgb(SIDEBAR_BACKGROUND))
                .rounded_lg()
                .child(graph_container),
        );
        view.into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        compute_graph_layout, country_company_counts, display_company_name, graph_node_width,
        GraphLayout, GraphNodePosition, COMPANY_NODE_HEIGHT, COUNTRY_NODE_HEIGHT, GRAPH_PADDING,
    };
    use crate::resume::{geography_graph, Location, Resume, WorkEntry};
    use std::collections::BTreeSet;

    fn london_width(layout: &GraphLayout) -> f32 {
        layout
            .nodes
            .iter()
            .find(|node| node.label == "United Kingdom")
            .map(|node| node.width)
            .expect("country node width exists")
    }

    #[test]
    fn compute_graph_layout_places_countries_and_reveals_expanded_companies() {
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
        let empty = compute_graph_layout(&graph, &BTreeSet::new(), 720.0);

        assert_eq!(empty.nodes.len(), 2);
        assert!(empty.nodes.iter().any(|node| node.group == "country"));
        assert!(!empty.nodes.iter().any(|node| node.group == "company"));
        assert!(empty.edges.is_empty());

        let expanded = BTreeSet::from(["country:united-kingdom".to_owned()]);
        let layout = compute_graph_layout(&graph, &expanded, 720.0);

        let london = layout
            .nodes
            .iter()
            .find(|node| node.label == "London company")
            .expect("expanded company node exists");
        assert_eq!(london.group, "company");
        assert!(london.x > GRAPH_PADDING);
        assert_eq!(layout.edges.len(), 1);
        let edge = &layout.edges[0];
        assert_eq!(edge.from_y, london.y + COMPANY_NODE_HEIGHT / 2.0);
        assert_eq!(edge.to_x, GRAPH_PADDING + london_width(&layout));
    }

    #[test]
    fn compute_graph_layout_collapses_companies_for_unexpanded_countries() {
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
        let layout = compute_graph_layout(&graph, &BTreeSet::new(), 720.0);

        assert!(layout.edges.is_empty());
        assert!(!layout.nodes.iter().any(|node| node.group == "company"));
    }

    #[test]
    fn graph_node_width_grows_with_label_length_and_clamps() {
        let short = graph_node_width("DE", 140.0, 300.0);
        let long = graph_node_width("A very long company name indeed", 180.0, 320.0);
        assert_eq!(short, 157.0);
        assert_eq!(long, 320.0);
        assert!(graph_node_width("Germany", 140.0, 300.0) > short);
    }

    #[test]
    fn display_company_name_strips_city_and_country_suffix() {
        assert_eq!(
            display_company_name("DKFZ (German Cancer Research Centre), Heidelberg, Germany"),
            "DKFZ (German Cancer Research Centre)"
        );
        assert_eq!(
            display_company_name("Redcats, New York, USA"),
            "Redcats"
        );
        assert_eq!(
            display_company_name("Company, Inc., New York, USA"),
            "Company, Inc."
        );
        assert_eq!(display_company_name("Pune, India"), "Pune");
        assert_eq!(display_company_name("Single Company"), "Single Company");
        assert_eq!(display_company_name("Unknown company"), "Unknown company");
    }

    #[test]
    fn country_company_counts_total_edges_per_country() {
        let resume = Resume {
            work: vec![
                WorkEntry {
                    name: "Company A".to_owned(),
                    location: Some(Location::Text("London, United Kingdom".to_owned())),
                    ..Default::default()
                },
                WorkEntry {
                    name: "Company B".to_owned(),
                    location: Some(Location::Text("London, United Kingdom".to_owned())),
                    ..Default::default()
                },
                WorkEntry {
                    name: "Company C".to_owned(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let graph = geography_graph(&resume);
        let counts = country_company_counts(&graph);

        assert_eq!(counts.get("country:united-kingdom"), Some(&2));
        assert_eq!(counts.get("country:unknown"), Some(&1));
    }

    #[test]
    fn graph_nodes_are_sorted_with_countries_first() {
        let resume = Resume {
            work: vec![
                WorkEntry {
                    name: "Company A".to_owned(),
                    location: Some(Location::Text("London, United Kingdom".to_owned())),
                    ..Default::default()
                },
                WorkEntry {
                    name: "Company B".to_owned(),
                    location: Some(Location::Text("Berlin, Germany".to_owned())),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let graph = geography_graph(&resume);
        let layout = compute_graph_layout(&graph, &BTreeSet::new(), 720.0);

        assert_eq!(
            layout
                .nodes
                .iter()
                .map(|node: &GraphNodePosition| node.group.as_str())
                .collect::<Vec<_>>(),
            vec!["country", "country"]
        );
        assert_eq!(layout.nodes[0].x, layout.nodes[1].x);
        assert_eq!(layout.nodes[0].height, COUNTRY_NODE_HEIGHT);
        assert!(layout.nodes[1].y > layout.nodes[0].y);
    }

    #[test]
    fn graph_layout_container_height_clears_expanded_companies() {
        let resume = Resume {
            work: vec![
                WorkEntry {
                    name: "London company".to_owned(),
                    location: Some(Location::Text("London, United Kingdom".to_owned())),
                    ..Default::default()
                },
                WorkEntry {
                    name: "Berlin company".to_owned(),
                    location: Some(Location::Text("Berlin, Germany".to_owned())),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let graph = geography_graph(&resume);
        let expanded = BTreeSet::from(["country:united-kingdom".to_owned()]);
        let layout = compute_graph_layout(&graph, &expanded, 720.0);

        let london = layout
            .nodes
            .iter()
            .find(|node| node.label == "London company")
            .expect("expanded company node exists");
        assert!(london.y + COMPANY_NODE_HEIGHT <= layout.container_height);
        let germany = layout
            .nodes
            .iter()
            .find(|node| node.label == "Germany")
            .expect("country node exists");
        assert!(germany.y < london.y);
    }
}