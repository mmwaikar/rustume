use crate::app::ui::{
    nav_icon, selectable_text, ACTIVE, INK, MUTED_INK, PANEL_BACKGROUND, SIDEBAR_BACKGROUND,
};
use crate::app::App;
use crate::resume::{geography_graph, GraphPayload};
use gpui_kit::{
    canvas, div, point, px, AnyElement, Context, InteractiveElement, IntoElement, ParentElement,
    PathBuilder, Pixels, Point, StatefulInteractiveElement, Styled, Window,
};
use std::collections::{BTreeMap, BTreeSet};

const GRAPH_PADDING: f32 = 24.0;
const COUNTRY_NODE_HEIGHT: f32 = 44.0;
const COMPANY_NODE_HEIGHT: f32 = 36.0;
const ROOT_NODE_WIDTH: f32 = 220.0;
const MAX_COUNTRY_NODE_WIDTH: f32 = 200.0;
const MIN_COMPANY_NODE_WIDTH: f32 = 160.0;
const MAX_COMPANY_NODE_WIDTH: f32 = 200.0;
const CIRCLE_INNER_RADIUS: f32 = 150.0;
const COMPANY_RING_BASE: f32 = 300.0;
const COMPANY_RING_STEP: f32 = 64.0;
const COMPANY_GAP: f32 = 14.0;
const WEDGE_MARGIN: f32 = 0.28;
const RESOLVE_MAX_ROUNDS: usize = 12;
const GRAPH_AVAILABLE_WIDTH: f32 = 720.0;
const LABEL_CHAR_WIDTH: f32 = 8.5;

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
    (min + label.chars().count() as f32 * LABEL_CHAR_WIDTH).min(max)
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

fn rects_overlap(a: &GraphNodePosition, b: &GraphNodePosition) -> bool {
    let overlap_x = ((a.x + a.width / 2.0) - (b.x + b.width / 2.0)).abs() < (a.width + b.width) / 2.0;
    let overlap_y = ((a.y + a.height / 2.0) - (b.y + b.height / 2.0)).abs()
        < (a.height + b.height) / 2.0;
    overlap_x && overlap_y
}

fn ring_bucket_fits(radius: f32, usable_half: f32, widths: &[f32]) -> bool {
    if widths.len() <= 1 {
        return true;
    }
    let mut max_pair: f32 = 0.0;
    for window in widths.windows(2) {
        max_pair = max_pair.max((window[0] + window[1]) / 2.0 + COMPANY_GAP);
    }
    let step = max_pair / radius;
    (widths.len() - 1) as f32 * step <= 2.0 * usable_half
}

fn compute_graph_layout(
    graph: &GraphPayload,
    countries_expanded: bool,
    expanded_countries: &BTreeSet<String>,
    _available_width: f32,
) -> GraphLayout {
    let countries = graph
        .nodes
        .iter()
        .filter(|node| node.group == "country")
        .collect::<Vec<_>>();

    let mut nodes: Vec<GraphNodePosition> = Vec::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();

    if !countries_expanded {
        nodes.push(GraphNodePosition {
            id: "countries-root".to_owned(),
            label: "Countries".to_owned(),
            group: "root".to_owned(),
            x: GRAPH_PADDING,
            y: GRAPH_PADDING,
            width: ROOT_NODE_WIDTH,
            height: COUNTRY_NODE_HEIGHT,
        });
        return GraphLayout {
            nodes,
            edges: Vec::new(),
            container_width: ROOT_NODE_WIDTH + 2.0 * GRAPH_PADDING,
            container_height: COUNTRY_NODE_HEIGHT + 2.0 * GRAPH_PADDING,
        };
    }

    let center = GraphNodePosition {
        id: "countries-root".to_owned(),
        label: "Countries".to_owned(),
        group: "root".to_owned(),
        x: -ROOT_NODE_WIDTH / 2.0,
        y: -COUNTRY_NODE_HEIGHT / 2.0,
        width: ROOT_NODE_WIDTH,
        height: COUNTRY_NODE_HEIGHT,
    };
    nodes.push(center.clone());

    let country_count = countries.len().max(1) as f32;
    let wedge = std::f32::consts::TAU / country_count;
    let usable_half = wedge / 2.0 - WEDGE_MARGIN;

    for (index, country) in countries.iter().enumerate() {
        let theta =
            -std::f32::consts::FRAC_PI_2 + std::f32::consts::TAU * index as f32 / country_count;
        let country_width = graph_node_width(&country.label, 140.0, MAX_COUNTRY_NODE_WIDTH);
        let country_x = CIRCLE_INNER_RADIUS * theta.cos() - country_width / 2.0;
        let country_y = CIRCLE_INNER_RADIUS * theta.sin() - COUNTRY_NODE_HEIGHT / 2.0;
        nodes.push(GraphNodePosition {
            id: country.id.clone(),
            label: country.label.clone(),
            group: country.group.clone(),
            x: country_x,
            y: country_y,
            width: country_width,
            height: COUNTRY_NODE_HEIGHT,
        });
        let country_index = nodes.len() - 1;
        edges.push((0, country_index));

        if expanded_countries.contains(&country.id) {
            let companies = graph
                .edges
                .iter()
                .filter(|edge| edge.target == country.id)
                .filter_map(|edge| graph.nodes.iter().find(|node| node.id == edge.source).cloned())
                .collect::<Vec<_>>();

            let mut buckets: Vec<Vec<usize>> = vec![Vec::new()];
            let mut bucket_index = 0;
            let mut radius = COMPANY_RING_BASE;
            for company in &companies {
                let label = display_company_name(&company.label);
                let width = graph_node_width(&label, MIN_COMPANY_NODE_WIDTH, MAX_COMPANY_NODE_WIDTH);
                nodes.push(GraphNodePosition {
                    id: company.id.clone(),
                    label,
                    group: company.group.clone(),
                    x: 0.0,
                    y: 0.0,
                    width,
                    height: COMPANY_NODE_HEIGHT,
                });
                let node_index = nodes.len() - 1;
                edges.push((country_index, node_index));
                let widths = buckets[bucket_index]
                    .iter()
                    .map(|&idx| nodes[idx].width)
                    .chain(std::iter::once(width))
                    .collect::<Vec<_>>();
                if ring_bucket_fits(radius, usable_half, &widths) {
                    buckets[bucket_index].push(node_index);
                } else {
                    buckets.push(vec![node_index]);
                    bucket_index += 1;
                    radius += COMPANY_RING_STEP;
                }
            }

            let radius = COMPANY_RING_BASE;
            for (shell, bucket) in buckets.iter().enumerate() {
                let shell_radius = radius + shell as f32 * COMPANY_RING_STEP;
                let mut max_pair: f32 = 0.0;
                let bucket_widths = bucket
                    .iter()
                    .map(|&idx| nodes[idx].width)
                    .collect::<Vec<_>>();
                for window in bucket_widths.windows(2) {
                    max_pair = max_pair.max((window[0] + window[1]) / 2.0 + COMPANY_GAP);
                }
                let step = if bucket.len() > 1 {
                    (max_pair / shell_radius).min(2.0 * usable_half / (bucket.len() - 1) as f32)
                } else {
                    0.0
                };
                for (slot, &node_index) in bucket.iter().enumerate() {
                    let phi = theta + (slot as f32 - (bucket.len() - 1) as f32 / 2.0) * step;
                    nodes[node_index].x = shell_radius * phi.cos() - nodes[node_index].width / 2.0;
                    nodes[node_index].y = shell_radius * phi.sin() - COMPANY_NODE_HEIGHT / 2.0;
                }
            }
        }
    }

    resolve_overlaps(&mut nodes);

    finish_layout(nodes, edges)
}

fn node_index_center(nodes: &[GraphNodePosition], index: usize) -> (f32, f32) {
    let node = &nodes[index];
    (node.x + node.width / 2.0, node.y + node.height / 2.0)
}

fn resolve_overlaps(nodes: &mut Vec<GraphNodePosition>) {
    let mut angle_by_index = BTreeMap::new();
    let mut radius_by_index = BTreeMap::new();
    for (index, node) in nodes.iter().enumerate() {
        if node.group == "company" {
            let (cx, cy) = node_index_center(nodes, index);
            angle_by_index.insert(index, cy.atan2(cx));
            radius_by_index.insert(index, (cx * cx + cy * cy).sqrt());
        }
    }

    for _ in 0..RESOLVE_MAX_ROUNDS {
        let mut collided = None;
        'search: for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                if rects_overlap(&nodes[i], &nodes[j]) {
                    collided = Some((i, j));
                    break 'search;
                }
            }
        }
        let Some((i, j)) = collided else {
            break;
        };
        let (a, b) = if radius_by_index
            .get(&j)
            .copied()
            .unwrap_or(0.0)
            > radius_by_index.get(&i).copied().unwrap_or(0.0)
        {
            (j, i)
        } else {
            (i, j)
        };
        if nodes[a].group == "company" {
            let angle = angle_by_index[&a];
            let radius = radius_by_index[&a] + COMPANY_RING_STEP;
            radius_by_index.insert(a, radius);
            nodes[a].x = radius * angle.cos() - nodes[a].width / 2.0;
            nodes[a].y = radius * angle.sin() - nodes[a].height / 2.0;
        } else if nodes[b].group == "company" {
            let angle = angle_by_index[&b];
            let radius = radius_by_index[&b] + COMPANY_RING_STEP;
            radius_by_index.insert(b, radius);
            nodes[b].x = radius * angle.cos() - nodes[b].width / 2.0;
            nodes[b].y = radius * angle.sin() - nodes[b].height / 2.0;
        } else {
            break;
        }
    }
}

fn finish_layout(
    mut nodes: Vec<GraphNodePosition>,
    structural_edges: Vec<(usize, usize)>,
) -> GraphLayout {
    let min_x = nodes.iter().map(|node| node.x).fold(f32::INFINITY, f32::min);
    let max_x = nodes
        .iter()
        .map(|node| node.x + node.width)
        .fold(f32::NEG_INFINITY, f32::max);
    let min_y = nodes.iter().map(|node| node.y).fold(f32::INFINITY, f32::min);
    let max_y = nodes
        .iter()
        .map(|node| node.y + node.height)
        .fold(f32::NEG_INFINITY, f32::max);

    let shift_x = GRAPH_PADDING - min_x;
    let shift_y = GRAPH_PADDING - min_y;
    for node in &mut nodes {
        node.x += shift_x;
        node.y += shift_y;
    }

    let edges = structural_edges
        .into_iter()
        .map(|(from, to)| {
            let from_center = node_index_center(&nodes, from);
            let to_center = node_index_center(&nodes, to);
            GraphEdgePath {
                from_x: from_center.0,
                from_y: from_center.1,
                to_x: to_center.0,
                to_y: to_center.1,
            }
        })
        .collect();

    GraphLayout {
        nodes,
        edges,
        container_width: (max_x - min_x) + 2.0 * GRAPH_PADDING,
        container_height: (max_y - min_y) + 2.0 * GRAPH_PADDING,
    }
}

fn paint_graph_edge(edge: &GraphEdgePath, origin: Point<Pixels>, window: &mut Window) {
    let from = origin + point(px(edge.from_x), px(edge.from_y));
    let to = origin + point(px(edge.to_x), px(edge.to_y));
    let dx = edge.to_x - edge.from_x;
    let dy = edge.to_y - edge.from_y;
    let length = (dx * dx + dy * dy).sqrt().max(1.0);
    let bend = 12.0;
    let normal_x = -dy / length * bend;
    let normal_y = dx / length * bend;
    let mut builder = PathBuilder::stroke(px(2.0));
    builder.move_to(from);
    builder.cubic_bezier_to(
        to,
        origin + point(
            px(edge.from_x + dx / 3.0 + normal_x),
            px(edge.from_y + dy / 3.0 + normal_y),
        ),
        origin + point(
            px(edge.from_x + dx * 2.0 / 3.0 - normal_x),
            px(edge.from_y + dy * 2.0 / 3.0 - normal_y),
        ),
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

        let layout = compute_graph_layout(
            &graph,
            self.geography_countries_expanded,
            &self.expanded_countries,
            GRAPH_AVAILABLE_WIDTH,
        );
        let edges = layout.edges.clone();
        let company_counts = country_company_counts(&graph);
        let country_total = graph
            .nodes
            .iter()
            .filter(|node| node.group == "country")
            .count();

        let mut graph_container = div()
            .id("geography-graph")
            .relative()
            .w(gpui_kit::px(layout.container_width))
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
            if node.group == "root" {
                let expanded = self.geography_countries_expanded;
                graph_container = graph_container.child(
                    div()
                        .id("country-root-node")
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
                        .child(
                            div()
                                .flex_1()
                                .min_w(gpui_kit::px(0.0))
                                .truncate()
                                .child(selectable_text("country-root-label", node.label.clone())),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(selectable_text(
                                    "country-root-toggle",
                                    format!("{country_total}"),
                                ))
                                .child(nav_icon(if expanded {
                                    "chevron-left"
                                } else {
                                    "chevron-right"
                                })),
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.geography_countries_expanded = !this.geography_countries_expanded;
                            cx.notify();
                        })),
                );
            } else if node.group == "country" {
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
                        .child(
                            div()
                                .flex_1()
                                .min_w(gpui_kit::px(0.0))
                                .truncate()
                                .child(selectable_text(
                                    format!("country-label-{}", node.id),
                                    node.label.clone(),
                                )),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(selectable_text(
                                    format!("country-toggle-{}", node.id),
                                    format!("{count}"),
                                ))
                                .child(nav_icon(if expanded {
                                    "chevron-left"
                                } else {
                                    "chevron-right"
                                })),
                        )
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
                        .child(
                            div()
                                .flex_1()
                                .min_w(gpui_kit::px(0.0))
                                .truncate()
                                .child(selectable_text(
                                    format!("company-label-{}", node.id),
                                    node.label.clone(),
                                )),
                        ),
                );
            }
        }

        view = view.child(
            div()
                .mt_6()
                .w_full()
                .min_w(gpui_kit::px(layout.container_width + 2.0 * GRAPH_PADDING))
                .flex()
                .justify_center()
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
        GraphNodePosition, CIRCLE_INNER_RADIUS, COMPANY_NODE_HEIGHT, COUNTRY_NODE_HEIGHT,
    };
    use crate::resume::{geography_graph, Location, Resume, WorkEntry};
    use std::collections::BTreeSet;

    fn node_center(node: &GraphNodePosition) -> (f32, f32) {
        (node.x + node.width / 2.0, node.y + node.height / 2.0)
    }

    fn distance_from(child: &GraphNodePosition, parent: &GraphNodePosition) -> f32 {
        let (cx, cy) = node_center(child);
        let (px, py) = node_center(parent);
        ((cx - px).powi(2) + (cy - py).powi(2)).sqrt()
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
        let empty = compute_graph_layout(&graph, false, &BTreeSet::new(), 720.0);

        assert_eq!(empty.nodes.len(), 1);
        assert_eq!(empty.nodes[0].group, "root");
        assert_eq!(empty.nodes[0].label, "Countries");
        assert!(!empty.nodes.iter().any(|node| node.group == "country"));
        assert!(!empty.nodes.iter().any(|node| node.group == "company"));
        assert!(empty.edges.is_empty());

        let collapsed_only = compute_graph_layout(&graph, true, &BTreeSet::new(), 720.0);
        assert_eq!(collapsed_only.nodes.len(), 3);
        assert!(
            collapsed_only
                .nodes
                .iter()
                .filter(|node| node.group == "country")
                .count()
                == 2
        );
        assert!(!collapsed_only.nodes.iter().any(|node| node.group == "company"));

        let expanded = BTreeSet::from(["country:united-kingdom".to_owned()]);
        let layout = compute_graph_layout(&graph, true, &expanded, 720.0);

        let root = layout
            .nodes
            .iter()
            .find(|node| node.group == "root")
            .expect("root node exists");
        let london = layout
            .nodes
            .iter()
            .find(|node| node.label == "London company")
            .expect("expanded company node exists");
        let united_kingdom = layout
            .nodes
            .iter()
            .find(|node| node.label == "United Kingdom")
            .expect("country node exists");
        assert_eq!(london.group, "company");
        let (uk_x, uk_y) = node_center(united_kingdom);
        let (company_x, company_y) = node_center(london);
        assert!(
            layout
                .edges
                .iter()
                .any(|edge| edge.from_x == uk_x
                    && edge.from_y == uk_y
                    && edge.to_x == company_x
                    && edge.to_y == company_y)
        );
        assert!(
            distance_from(london, root) > distance_from(united_kingdom, root),
            "companies orbit farther from the root than their country"
        );
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
        let layout = compute_graph_layout(&graph, true, &BTreeSet::new(), 720.0);

        let root = layout
            .nodes
            .iter()
            .find(|node| node.group == "root")
            .expect("root node exists");
        let (root_x, root_y) = node_center(root);
        assert!(!layout.nodes.iter().any(|node| node.group == "company"));
        assert!(
            layout
                .edges
                .iter()
                .all(|edge| edge.from_x == root_x && edge.from_y == root_y),
            "collapsed countries connect straight to the root"
        );
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
    fn expanded_real_resume_has_no_overlapping_nodes() {
        let resume =
            crate::resume::parse_resume(include_str!("../../../assets/resume.json")).unwrap();
        let graph = geography_graph(&resume);
        let expanded = graph
            .nodes
            .iter()
            .filter(|node| node.group == "country")
            .map(|node| node.id.clone())
            .collect::<BTreeSet<_>>();
        let layout = compute_graph_layout(&graph, true, &expanded, 720.0);

        for i in 0..layout.nodes.len() {
            for j in (i + 1)..layout.nodes.len() {
                let a = &layout.nodes[i];
                let b = &layout.nodes[j];
                let overlap_x = ((a.x + a.width / 2.0) - (b.x + b.width / 2.0)).abs()
                    < (a.width + b.width) / 2.0;
                let overlap_y = ((a.y + a.height / 2.0) - (b.y + b.height / 2.0)).abs()
                    < (a.height + b.height) / 2.0;
                assert!(
                    !(overlap_x && overlap_y),
                    "nodes {:?} ({}, {}) and {:?} ({}, {}) overlap",
                    a.label,
                    a.x,
                    a.y,
                    b.label,
                    b.x,
                    b.y
                );
            }
        }
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
        let layout = compute_graph_layout(&graph, true, &BTreeSet::new(), 720.0);

        assert_eq!(
            layout
                .nodes
                .iter()
                .map(|node: &GraphNodePosition| node.group.as_str())
                .collect::<Vec<_>>(),
            vec!["root", "country", "country"]
        );
        let root = &layout.nodes[0];
        let country_distance = |index: usize| distance_from(&layout.nodes[index], root);
        assert_eq!(layout.nodes[1].height, COUNTRY_NODE_HEIGHT);
        assert!(
            (country_distance(1) - country_distance(2)).abs() < 0.01,
            "countries orbit the root on the same ring"
        );
        assert!(
            country_distance(1) > CIRCLE_INNER_RADIUS - 1.0,
            "countries stay clear of the root"
        );
        assert!(
            (node_center(root).0 - layout.container_width / 2.0).abs() < 0.01,
            "root sits at the centre of the orbit"
        );
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
        let layout = compute_graph_layout(&graph, true, &expanded, 720.0);

        let london = layout
            .nodes
            .iter()
            .find(|node| node.label == "London company")
            .expect("expanded company node exists");
        assert!(london.y >= 0.0 && london.y + COMPANY_NODE_HEIGHT <= layout.container_height);
        assert!(london.x >= 0.0 && london.x + london.width <= layout.container_width);
        let germany = layout
            .nodes
            .iter()
            .find(|node| node.label == "Germany")
            .expect("country node exists");
        assert!(germany.y >= 0.0 && germany.y + COUNTRY_NODE_HEIGHT <= layout.container_height);
        assert!(germany.x >= 0.0 && germany.x + germany.width <= layout.container_width);
    }
}