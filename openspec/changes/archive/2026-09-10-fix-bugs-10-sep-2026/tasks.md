## 1. Experience Chart Layout

- [x] 1.1 Inspect the current GPUI Kit chart API and isolate a deterministic label-layout decision based on company-name length and available chart width; verify the chosen mode can be exercised by focused tests.
- [x] 1.2 Update the experience view to retain company-month calculations and common scaling while rendering rounded bars with readable horizontal, side-by-side, or in-bar labels when vertical labels would overlap; verify proportional sizing and the long-name scenario with focused Rust tests or a rendered view check.
- [x] 1.3 Preserve and verify the explicit empty-state rendering when no company has chartable duration; verify it through the existing experience view test surface or a manual narrow-data run.

## 2. Footer Attribution

- [x] 2.1 Replace the StatusBar's single footer copy with selectable heart-marked attribution and separate Rust and gpui-kit links; verify the rendered footer remains in the persistent StatusBar and exposes the exact two URLs.
- [x] 2.2 Add focused assertions for the footer link destinations and visible attribution text where the current test harness permits; verify both links remain independently activatable.

## 3. Validation

- [x] 3.1 Run `cargo fmt --check` and the focused Rust test suite, then fix any formatting or regression failures in the touched rendering path.
- [x] 3.2 Run the available native and web build checks and manually verify wide and narrow layouts, long company names, rounded proportional bars, empty experience, and both footer links. Native, wasm, binding-generation, and Vite builds passed; wide and narrow browser screenshots confirmed shell layout and footer links, while GPUI canvas interaction and detailed chart inspection remained limited by wasm asset-loader warnings.
