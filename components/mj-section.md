---
component: mj-section
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
    maps_to: "mj-section background-color"
  - id: padding
    required: false
    type: string
    maps_to: "mj-section padding"
  - id: full_width
    required: false
    type: bool
    default: false
    maps_to: "mj-section full-width=full-width"
  - id: children
    required: true
    type: list
    maps_to: "mj-column | mj-group"
---
