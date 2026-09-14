# Requirements

I want to build a resume visualizer in Rust. The app will be a client side app and will be hosted on GitHub pages. It will mimic the visualizer which I built using C# Blazor, called [Compendium](https://www.codionics.com/Compendium/). It should:

- use the rust [gpui](https://gpui.rs/) crate and components from the [gpui-kit](https://github.com/longbridge/gpui-kit)
- although the gpui advertises itself as a desktop library, the [gpui-kit](https://gpui-kit.com/) website runs gpui components in a browser
- have a header, footer, left sidebar and the main area in between
- the left sidebar shows:
  - Profile
  - Experience
    - By Company: shows the experience in a company (in months) in a bar chart 
    - By Country: should show experience in compnies, grouped by countries as a nodes-edges graph using [cytoscape.js](https://js.cytoscape.org/). When a country is expanded, the companies for that country should be shown
    - By Skills: should show experience grouped by skills (same nodes-edges graph as above). When a skill is exanded, the sub-skills should be shown
  - Skills
  - Projects
  - Blog Posts

## Changes

### 5-Sep-2026 compendium-section-visual-refresh

Can you change the following sections:

1. profile section should look like ![profile](screenshots/profile.png)
2. skills section should look like ![skills](screenshots/skills.png)
3. projects section should look like ![projects](screenshots/projects.png)
4. blog posts section should look like ![blog-posts](screenshots/blog-posts.png)

I like the current colors so don't use the blue visual language but keep the current colors.

### 14-Sep-2026

Q1. Earlier I planned to use [cytoscape.js](https://js.cytoscape.org/) but can we try using [gpui-node-graph](https://github.com/ignition-is-go/gpui-node-graph) library to build the Experience by Country graph? Does it work in both desktop and browser?
A1. COOP and COEP are HTTP security headers that enable cross-origin isolation, which some WebAssembly features require:

- `Cross-Origin-Opener-Policy: same-origin` (COOP)
- `Cross-Origin-Embedder-Policy: require-corp` (COEP)

`gpui-node-graph` documents these as necessary for its browser runtime because it uses shared-memory WebAssembly.

GitHub Pages serves static files but does not provide a way to configure arbitrary response headers. So, **if the library truly requires COOP/COEP in Rustume’s browser path, it will not work on GitHub Pages as-is**. HTML meta tags cannot substitute for these response headers.

Desktop is supported by the library. Browser support works on hosts where those headers can be configured, such as Cloudflare Pages, Netlify, or a custom server.

Q2. Shall we try [gpui-flow](https://github.com/pacifio/gpui-flow) then?
A2. I would not choose `gpui-flow` for Rustume’s GitHub Pages target.

It is a good feature fit for a node graph: custom node rendering, edges, pan/zoom, selection, and click handling. But it has no documented browser or `wasm32-unknown-unknown` support, no browser demo, no releases or tags, and depends directly on Zed’s `gpui` Git repository. Its examples also include the native `gpui_macos` dependency.

So compared with `gpui-node-graph`:

- `gpui-node-graph`: explicitly supports desktop and Wasm, but requires COOP/COEP headers that GitHub Pages cannot set.
- `gpui-flow`: no stated Wasm/browser support at all, so it is riskier for GitHub Pages.

For a static GitHub Pages deployment, the practical option remains a browser-safe renderer built with the GPUI Kit version already working in Rustume, unless you move hosting to Cloudflare Pages/Netlify and can use the explicit Wasm support in `gpui-node-graph`.

Q3. How about [ferrum-flow](https://github.com/tu6ge/ferrum-flow) then?
A3. No. `ferrum-flow` is also unsuitable for the GitHub Pages browser requirement today.

It is a capable GPUI graph editor and has stable tags, but its own roadmap lists **“WASM target - When the stack allows”** as a future direction. It does not document a browser build or WebAssembly support, and calls itself alpha with potentially changing APIs.

It could be evaluated for a desktop-only Rust app, but not for Rustume’s required desktop-plus-GitHub-Pages deployment. The realistic choices remain:

- Build the country graph with Rustume’s existing browser-capable GPUI Kit stack.
- Move hosting to a platform supporting COOP/COEP, then use `gpui-node-graph`.
