---
component: mj-group
version: 1
node_scope: section_child
insert:
  defaults:
    children: []
fields:
  - id: width
    required: false
    type: string
    maps_to: "mj-group width"
  - id: background_color
    required: false
    type: string
    maps_to: "mj-group background-color"
  - id: children
    required: true
    type: list
    maps_to: "mj-column list (no type tags)"
---
