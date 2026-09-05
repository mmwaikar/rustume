use crate::resume::{company_months, geography_graph, skills_graph, Resume, YearMonth};
use gpui_kit::{div, AnyElement, AppContext, Context, Entity, IntoElement, InteractiveElement, ParentElement, Render, StatefulInteractiveElement, Styled, Window};
use gpui_kit::component::{dock::{DockArea, DockSkin}, Root};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Section { Overview, Experience, Skills, Geography }

pub struct App {
	dock: Entity<DockArea>,
	resume: Resume,
	section: Section,
}

impl App {
	pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
		let (dock, _) = DockSkin::dock_area("rustume", Some(1), window, cx);
		let resume = crate::resume::parse_resume(include_str!("../assets/resume.json"))
			.expect("bundled resume must be valid");
		Self { dock, resume, section: Section::Overview }
	}

	fn nav_button(&self, label: &'static str, section: Section, cx: &mut Context<Self>) -> impl IntoElement {
		let active = self.section == section;
		div().id(label).w_full().px_3().py_3().mb_2()
			.bg(if active { gpui_kit::rgb(0xefb15d) } else { gpui_kit::rgb(0xe7e8d2) })
			.text_color(gpui_kit::rgb(0x203a36)).child(label)
			.on_click(cx.listener(move |this, _, _, cx| { this.section = section; cx.notify(); }))
	}

	fn sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
		div().w_64().p_4().bg(gpui_kit::rgb(0xd7d8bd))
			.child(self.nav_button("Overview", Section::Overview, cx))
			.child(self.nav_button("Experience", Section::Experience, cx))
			.child(self.nav_button("Skills", Section::Skills, cx))
			.child(self.nav_button("Geography", Section::Geography, cx))
	}

	fn content(&self) -> AnyElement {
		match self.section {
			Section::Overview => div().child(div().text_2xl().child("Overview")).child(div().mt_4().text_lg().child(self.resume.basics.summary.clone())).into_any_element(),
			Section::Experience => self.experience(),
			Section::Skills => { let graph = skills_graph(&self.resume); div().child(div().text_2xl().child("Skills relationships")).child(div().mt_4().child(format!("{} skill nodes | {} relationships", graph.nodes.len(), graph.edges.len()))).child(div().mt_4().child(graph.nodes.into_iter().map(|node| node.label).collect::<Vec<_>>().join("  *  "))).into_any_element() },
			Section::Geography => { let graph = geography_graph(&self.resume); div().child(div().text_2xl().child("Experience by country")).child(div().mt_4().child(format!("{} companies and countries | {} relationships", graph.nodes.len(), graph.edges.len()))).child(div().mt_4().child(graph.nodes.into_iter().map(|node| node.label).collect::<Vec<_>>().join("  *  "))).into_any_element() },
		}
	}

	fn experience(&self) -> AnyElement {
		let totals = company_months(&self.resume, YearMonth::current()).unwrap_or_default();
		let max = totals.values().copied().max().unwrap_or(1);
		let mut view = div().child(div().text_2xl().child("Experience by company"));
		if totals.is_empty() { return view.child(div().mt_6().child("No work history is available to chart.")).into_any_element(); }
		for (company, months) in totals {
			view = view.child(div().mt_4().child(company).child(div().h_4().w(gpui_kit::px((80 + months * 320 / max) as f32)).bg(gpui_kit::rgb(0xd96c3f))).child(format!("{months} months")));
		}
		view.into_any_element()
	}
}

impl Render for App {
	fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
		let candidate = if self.resume.basics.name.is_empty() { "Resume visualizer".to_owned() } else { self.resume.basics.name.clone() };
		div().flex().flex_col().size_full().bg(gpui_kit::rgb(0xf4efe6))
			.child(div().px_6().py_4().bg(gpui_kit::rgb(0x203a36)).text_color(gpui_kit::rgb(0xf4efe6)).child(div().text_xl().child(candidate)).child(self.resume.basics.label.clone()))
			.child(div().flex().flex_1().child(self.sidebar(cx)).child(div().flex_1().p_6().child(self.content())))
			.child(div().px_6().py_3().bg(gpui_kit::rgb(0x203a36)).text_color(gpui_kit::rgb(0xd7d8bd)).child("Rustume | Built with Rust and GPUI"))
	}
}

pub fn root_view(window: &mut Window, cx: &mut gpui_kit::App) -> Entity<Root> {
	let view = cx.new(|cx| App::new(window, cx));
	cx.new(|cx| Root::new(view, window, cx))
}
