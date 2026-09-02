---
component: mj-social
version: 1
node_scope: column_child
insert:
  defaults:
    mode: horizontal
    icon_size: "32px"
    elements: []
fields:
  - id: mode
    required: false
    type: enum
    options: ["horizontal", "vertical"]
    maps_to: "mj-social mode"
  - id: align
    required: false
    type: enum
    options: ["left", "center", "right"]
    maps_to: "mj-social align"
  - id: icon_size
    required: false
    type: string
    default: "32px"
    maps_to: "mj-social icon-size"
  - id: elements
    required: true
    type: list
    maps_to: "mj-social-element name+href (+ src for web)"
---

JSON `"name": "x"` emits MJML `name="twitter"` unless a later compiler check finds a built-in `x`.
