---
component: mj-text
version: 1
node_scope: column_child
insert:
  defaults:
    content: "Write something."
fields:
  - id: content
    required: true
    type: string
    ui:
      control: textarea
    maps_to: "mj-text inner (plain or allowlisted br|a|b|strong|em|span|u)"
  - id: align
    required: false
    type: enum
    options: ["left", "center", "right"]
    maps_to: "mj-text align"
  - id: font_size
    required: false
    type: string
    maps_to: "mj-text font-size"
  - id: font_family
    required: false
    type: string
    maps_to: "mj-text font-family (omit → mj-all / brand)"
  - id: color
    required: false
    type: string
    maps_to: "mj-text color"
  - id: padding
    required: false
    type: string
    maps_to: "mj-text padding"
---
