use crate::app::ui::{compact_bubble, hero, nav_icon, selectable_text};
use crate::app::App;
use crate::resume::{skills_graph, GraphNode, GraphPayload, Resume};
use gpui_kit::component::{list::ListItem, tree::{tree, TreeItem}};
use gpui_kit::{div, AnyElement, IntoElement, ParentElement, Styled};
use std::collections::BTreeMap;

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
    pub(crate) fn skills(&self) -> AnyElement {
        let graph = skills_graph(&self.resume);
        let skill_levels = self
            .resume
            .skills
            .iter()
            .map(|skill| (skill.name.clone(), skill.level.clone()))
            .collect::<BTreeMap<_, _>>();
        let skills_tree = tree(&self.skill_tree, move |_, entry, _, _, _| {
            let label = entry.item().label.to_string();
            let mut content = div()
                .flex()
                .items_center()
                .gap_x_2()
                .ml_2()
                .child(skill_tree_icon(entry.is_folder(), entry.is_expanded()))
                .child(selectable_text(
                    format!("skill-tree-{}", entry.item().id),
                    label.clone(),
                ));
            if entry.depth() > 0 {
                content = content.ml_4();
            }
            if entry.is_root() {
                if let Some(level) = skill_levels.get(&label).filter(|level| !level.is_empty()) {
                    content = content.child(compact_bubble(level.clone()));
                }
            }
            ListItem::new(entry.item().id.clone()).child(content)
        });
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
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .child(skills_tree),
            )
            .into_any_element()
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