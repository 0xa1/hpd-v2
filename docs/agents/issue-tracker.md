# Issue tracker: Local Markdown

## Publish and fetch

- Publish specs to `.scratch/<feature-slug>/spec.md`.
- Publish one file per implementation ticket at `.scratch/<feature-slug>/issues/<NN>-<slug>.md`, numbered from `01`. Create directories as needed.
- Fetch a ticket by its path or by its number within the feature.
- Record triage in a `Status:` line near the top using `triage-labels.md`. Append conversation history under `## Comments`.

## Wayfinding operations

- Keep the map at `.scratch/<effort>/map.md`, with Notes, Decisions-so-far, and Fog sections.
- Keep child tickets in `.scratch/<effort>/issues/<NN>-<slug>.md`, numbered from `01`. Put the question in the body and record `Type:` (`research`, `prototype`, `grilling`, or `task`).
- Record dependencies as `Blocked by: NN, NN` near the top. A ticket is unblocked when every listed dependency is `resolved`.
- Pick the lowest-numbered open, unblocked, unclaimed ticket. Save `Status: claimed` before starting.
- To resolve a ticket, add an `## Answer`, set `Status: resolved`, and add a summary and link in the map’s Decisions-so-far section.
