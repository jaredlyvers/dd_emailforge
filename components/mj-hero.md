---
component: mj-hero
version: 1
node_scope: body_node
insert:
  defaults:
    mode: fluid-height
    children: []
fields:
  - id: mode
    required: false
    type: enum
    options: ["fluid-height", "fixed-height"]
    maps_to: "mj-hero mode"
  - id: background_url
    required: false
    type: string
    maps_to: "mj-hero background-url (same rewrite as mj-image src)"
  - id: background_color
    required: false
    type: string
    maps_to: "mj-hero background-color"
  - id: background_height
    required: false
    type: string
    maps_to: "mj-hero background-height"
  - id: width
    required: false
    type: string
    maps_to: "mj-hero width"
  - id: height
    required: false
    type: string
    maps_to: "mj-hero height"
  - id: children
    required: true
    type: list
    maps_to: "ColumnChild list (hero is a single column)"
---
