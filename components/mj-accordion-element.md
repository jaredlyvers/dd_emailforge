---
component: mj-accordion-element
version: 1
node_scope: accordion_child
insert:
  parent: mj-accordion
  defaults:
    title: "Title"
    content: "Write something."
fields:
  - id: title
    required: true
    type: string
    maps_to: "mj-accordion-title inner text"
  - id: content
    required: false
    type: string
    maps_to: "mj-accordion-text inner HTML (allowlisted tags)"
  - id: background_color
    required: false
    type: string
    maps_to: "mj-accordion-element background-color"
---

Not insertable under `mj-column` or `mj-section`.
