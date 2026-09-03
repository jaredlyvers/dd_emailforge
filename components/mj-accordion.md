---
component: mj-accordion
version: 1
node_scope: column_child
insert:
  parent: mj-column | mj-hero
  defaults:
    elements: []
fields:
  - id: border
    required: false
    type: string
    maps_to: "mj-accordion border (CSS border)"
  - id: padding
    required: false
    type: string
    maps_to: "mj-accordion padding"
  - id: elements
    required: true
    type: list
    maps_to: "mj-accordion-element children"
---

Children are `mj-accordion-element` only. Title and body live on the element; `mj-accordion-title` / `mj-accordion-text` are not separately insertable.
