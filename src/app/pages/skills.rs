use crate::app::ui::{compact_bubble, hero, nav_icon, selectable_text};
use crate::app::App;
use crate::resume::{skills_graph, GraphNode, GraphPayload, Resume};
use gpui_kit::component::list::ListItem;
use gpui_kit::component::tree::TreeItem;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, AnyElement, Context, IntoElement, MouseButton, ParentElement, SharedString, Styled};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn skill_tree_icon_name(is_folder: bool, is_expanded: bool) -> &'static str {
    match (is_folder, is_expanded) {
        (true, true) => "folder-open",
        (true, false) => "folder-closed",
        (false, _) => "file",
    }
}

fn skill_tree_icon(is_folder: bool, is_expanded: bool) -> AnyElement {
    nav_icon(skill_tree_icon_name(is_folder, is_expanded))
}

pub(crate) fn skill_tree_children(graph: &GraphPayload, skill_name: &str) -> Vec<GraphNode> {
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

pub(crate) fn skill_tree_items(resume: &Resume) -> Vec<TreeItem> {
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

impl App {
    pub(crate) fn skills(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let graph = skills_graph(&self.resume);
        let skill_levels = self
            .resume
            .skills
            .iter()
            .map(|skill| (skill.name.clone(), skill.level.clone()))
            .collect::<BTreeMap<_, _>>();
        let expanded = self.expanded_skills.clone();
        let mut rows = Vec::new();
        for item in skill_tree_items(&self.resume) {
            collect_skill_rows(&item, 0, &expanded, &mut rows);
        }
        let skills_tree = div()
            .flex()
            .flex_col()
            .children(rows.into_iter().map(|(id, label, depth, is_folder, is_expanded)| {
                let mut content = div()
                    .flex()
                    .items_center()
                    .gap_x_2()
                    .ml_2()
                    .child(skill_tree_icon(is_folder, is_expanded));
                if depth > 0 {
                    content = content.ml_4();
                }
                content = content.child(selectable_text(
                    format!("skill-tree-{}", id),
                    label.clone(),
                ));
                if depth == 0 {
                    if let Some(level) = skill_levels.get(label.as_ref()).filter(|level| !level.is_empty()) {
                        content = content.child(compact_bubble(level.clone()));
                    }
                }
                let toggle_id = id.clone();
                ListItem::new(id)
                    .child(content)
                    .when(is_folder, |item| {
                        item.on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                            if this.expanded_skills.contains(toggle_id.as_ref()) {
                                this.expanded_skills.remove(toggle_id.as_ref());
                            } else {
                                this.expanded_skills.insert(toggle_id.to_string());
                            }
                            cx.notify();
                        }))
                    })
            }));
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
                    .h(gpui_kit::px(560.0))
                    .min_h_0()
                    .w_full()
                    .child(skills_tree),
            )
            .into_any_element()
    }
}

fn collect_skill_rows(
    item: &TreeItem,
    depth: usize,
    expanded: &BTreeSet<String>,
    out: &mut Vec<(SharedString, SharedString, usize, bool, bool)>,
) {
    let is_folder = item.is_folder();
    let is_expanded = is_folder && expanded.contains(item.id.as_ref());
    out.push((item.id.clone(), item.label.clone(), depth, is_folder, is_expanded));
    if is_expanded {
        for child in &item.children {
            collect_skill_rows(child, depth + 1, expanded, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{skill_tree_children, skill_tree_icon_name, skill_tree_items};
    use crate::resume::{skills_graph, Resume, Skill};

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