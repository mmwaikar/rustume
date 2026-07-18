use leptos::prelude::*;

const NAV_ITEMS: [(&str, &str); 5] = [
    ("/profile", "Profile"),
    ("/experience", "Experience"),
    ("/skills", "Skills"),
    ("/projects", "Projects"),
    ("/blog-posts", "Blog Posts"),
];

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="shell">
            <aside class="sidebar">
                <div class="brand">
                    <p class="brand-kicker">"rustume"</p>
                    <h1>"Resume visualizer"</h1>
                    <p class="brand-copy">"Leptos + Spectrum 2 style shell with Rust-native data shaping."</p>
                </div>
                    <nav class="nav" aria-label="Primary">
                        <ul>
                            {NAV_ITEMS.iter().map(|(href, label)| view! {
                                <li><a href={*href}>{*label}</a></li>
                            }).collect_view()}
                        </ul>
                    </nav>
            </aside>

            <main class="content">
                <header class="hero">
                    <p class="eyebrow">"Rust port"</p>
                    <h2>"Resume data from a separate GitHub repo"</h2>
                    <p>
                        "This scaffold establishes the repo boundary, navigation shell, and app entry point. "
                        "Next steps will wire the JSON model, analytics, and Cytoscape payloads in Rust."
                    </p>
                </header>

                <section class="grid">
                    <article class="card">
                        <h3>"Data model"</h3>
                        <p>"Port the JSON Resume schema and summary logic from the Blazor app into serde-based types."</p>
                    </article>
                    <article class="card">
                        <h3>"Graphs"</h3>
                        <p>"Keep Cytoscape, but feed it Rust-built nodes, edges, and summaries instead of repetitive C# interop structs."</p>
                    </article>
                    <article class="card">
                        <h3>"Styling"</h3>
                        <p>"Use a new Spectrum 2-oriented visual language with a green or yellow accent palette."</p>
                    </article>
                </section>
            </main>
        </div>
    }
}
