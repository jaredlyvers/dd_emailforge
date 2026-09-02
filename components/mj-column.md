---
component: mj-column
version: 1
node_scope: section_child
insert:
  defaults:
    components: []
fields:
  - id: width
    required: false
    type: string
    maps_to: "mj-column width"
  - id: background_color
    required: false
    type: string
    maps_to: "mj-column background-color"
  - id: padding
    required: false
    type: string
    maps_to: "mj-column padding"
  - id: inner_background_color
    required: false
    type: string
    maps_to: "mj-column inner-background-color"
  - id: components
    required: true
    type: list
    maps_to: "column children (mj-text, mj-button, …)"
---
