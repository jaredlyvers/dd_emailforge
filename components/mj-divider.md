---
component: mj-divider
version: 1
node_scope: column_child
insert:
  defaults: {}
fields:
  - id: border_color
    required: false
    type: string
    maps_to: "mj-divider border-color"
  - id: border_width
    required: false
    type: string
    maps_to: "mj-divider border-width"
  - id: border_style
    required: false
    type: enum
    options: ["solid", "dashed", "dotted", "none"]
    maps_to: "mj-divider border-style"
  - id: width
    required: false
    type: string
    maps_to: "mj-divider width"
  - id: align
    required: false
    type: enum
    options: ["left", "center", "right"]
    maps_to: "mj-divider align"
  - id: padding
    required: false
    type: string
    hint: "1-4 values with px or %"
    example: "10px 20px"
    maps_to: "mj-divider padding"
---
