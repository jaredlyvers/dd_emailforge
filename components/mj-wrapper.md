---
component: mj-wrapper
version: 1
node_scope: body_node
insert:
  defaults:
    full_width: false
    children: []
fields:
  - id: background_color
    required: false
    type: string
    maps_to: "mj-wrapper background-color"
  - id: padding
    required: false
    type: string
    maps_to: "mj-wrapper padding"
  - id: full_width
    required: false
    type: bool
    default: false
    maps_to: "mj-wrapper full-width=full-width"
  - id: children
    required: true
    type: list
    maps_to: "mj-section | mj-hero only"
---
