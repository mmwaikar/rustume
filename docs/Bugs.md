# Bugs

## 17-Sep-2026

- OpenCode fixed the Geography graph to show edges between the nodes
- OpenCode refactored the entire codebase into smaller modules
- The icons in the skills tree view are not visible in the web version, but they show up fine in the desktop version

## 12-Sep-2026

### Round 3 - After AI implementation

- the tree view does not show any icons for nodes - it should show open and closed folder icons for nodes and file icon (or a similar skill icon) for sub-skills
- there also is no indentation in the tree nodes - the child nodes should be indented one level inside the main skill node
- initially all the skill nodes should be collapsed
- the tree view does not cover the remaining screen area, so it shows a scroll bar, it's height can be increased
- the tree node icons are showing up after the text (i.e. to the right of the text), the icon should show up on the left and after that the text should appear
- the icon and the node text are not aligned horizontally, the icon appears slightly higher than the node text
- the tree view height does not occupy the remaining screen size
- there should be some padding between the node icon and the text, right now, they are too close to each other

### Round 2 - After AI implementation

- why does the skill tree view have only 2 nodes per skill - one for the main skill and only one for all the sub-skills? I want one node for the skill, and one each for each sub-skill
- have you used the gpui_kit::component::tree::tree for showing skills and sub-skills? if not please use it

### Round 1 - Pre implementation review

- Right now, all the bubbles are of the same size - the bubble in profile, language fluency, the skill level, project tags. However, the language fluency, skill level and project tags bubble should be smaller
- the skill tree view, shows the sub-skills as a single child node of the main skill node, however each sub-skill should be a separate child node

## 10-Sep-2026

### Round 3 - After one more AI implementation

- Regression - Although the ordering is correct, the experience by company bars are now in the same color, not colors per country. And the legend at the bottom says Unknown (the color of the legend and the bars is the same). Can't the legend appear somewhere on the chart, instead of at the bottom?
- Regression - all the other links, anywhere in the app (like links to blog posts, or projects) do not appear in the default blue color

### Round 2 - After AI implementation

- Why don't we use [Icons and Assets](https://gpui-kit.com/docs/assets/) as described in this link? If we use this, we'll be able to use heart, user-round or any other icons from this create directly
- Regression - The links in the footer do not appear in the default blue color
- Regression - The heart emoji (or icon) does not appear in red color (it's black right now)
- Regression - The text in the language fluency bubbles is displayed vertically (so the bubble itself displays vertically, not horizontally)
- Regression - The experience by company should order the experience in the newest first order. Right now it seems to be sorted by company name. Also can we show the experience by each country in a different color (so all US experience in a separate color, and so on)? And can we also include a legend for countries?

### Round 1 - Pre implementation review

- Regression - The vertical bars don't look good because the company names are too big. If the name can't be made diagonal, then maybe we should keep the bar chart horizontal. Keep using the chart component - https://gpui-kit.com/component/chart/. Either show the company name in the bar, or do not stack the company name and the bar vertically (one below the other), but stacked horizontally (left to right)
- The footer currently says "Built with Rust and GPUI" - I want it to say "Made with :heart-emoji: using Rust (with link https://rust-lang.org/) and gpui-kit (with link https://gpui-kit.com)"

## 9-Sep-2026

### Round 1 - Pre implementation review

- I want to use a StatusBar (https://gpui-kit.com/component/status-bar/) for the footer
- Right now, none of the text is selectable, can you please use the Text Selection (https://gpui-kit.com/base/text-selection/) everywhere
- Can you use fonts along with a Theme as described in https://gpui-kit.com/docs/fonts/? I wan to try the Inter font as described, the current font looks too big
- Icon Regression (from Round 2, 8-Sep-2026) - please use the Icons & Assets as described here - https://gpui-kit.com/docs/assets/. Right now the icons in the sidebar are still not visible - I only see circles or squares
- Experience shows a bar chart where the bars are horizontal. I want to show vertical bars with rounded corners (see Bar Chart Bottom aligned). Also the names of the companies is long, so if possible, the names should be diagonally tilted
- In the skills treeview, the bubble is bigger than the skill name, Make it similar to the Language fluency bubble. Also the sub-skills are shown in a single node separated by *. Each sub-skill should be a child node of the main skill node
- When I run the application, the window should be maximized

## 8-Sep-2026

### Round 2 - After AI implementation

- The icons in the sidebar are still not visible - I only see circles or squares
- Why don't you use a DataTable for showing education?
- The name of the publication and the publisher / release date appear in 2 different lines. But the name is aligned in center, whereas the second line is aligned to the left. Similarly, the date should be formatted as Mon Year.

### Round 1 - Pre implementation review

1. Why is there a label called sidebar in sidebar and a label called content in the main content area
2. The dock works fine but why is the entire sidebar not in green color? The green background is only limited till the content
3. Why do the Education, Publications, Network and Languages panel each have an extra label outside the panel?
4. Show Education and Publications in one row and Network and Languages in the next row
5. In the Network panel, the name of the network e.g. LinkedIn or GitHub is in a separate line and it's value is in the next line. This is wrong and they should be on the same line
6. In the languages panel, the fluency should be shown as a small bubble
7. Can we show education in a data grid?
8. In the Publications panel:
    - Blog posts should be filtered out
    - The title itself should be a link, instead of a separate link for Open Publication
    - The description of the publication should come as a popover

## 7-Sep-2026

1. There should be no Overview link in the sidebar
2. The Profile link should be the first in the sidebar
3. The icons do not show up properly in the sidebar links
4. The right side content should be scrollable (currently, it is not in any of the pages)
5. The date in Blog Posts should be formatted as Mon Year or Month Year
6. The text "Articles published on the WordPress blog." is unnecessarily wrapped
7. In the Profile section:
    - the text "speaker at international conferences" etc should appear in a badge
    - email, phone number and address should appear along side name etc. in a 2 column layout
    - the text "25+ years of ..." should wrap
    - it should show Education, Publications, Network and Languages in a GroupBox
8. Why have you created your own badge function when there already is a Bubble control?
9. Can the left sidebar be in a Dock control, so that it can be width-adjusted?
