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
  - id: border_radius
    required: false
    type: string
    hint: "px or %"
    maps_to: "mj-social border-radius"
  - id: padding
    required: false
    type: string
    hint: "1-4 values with px or %"
    maps_to: "mj-social padding"
  - id: icon_padding
    required: false
    type: string
    hint: "1-4 values with px or %"
    maps_to: "mj-social icon-padding"
  - id: inner_padding
    required: false
    type: string
    hint: "1-4 values with px or %"
    maps_to: "mj-social inner-padding"
  - id: font_size
    required: false
    type: string
    maps_to: "mj-social font-size"
  - id: color
    required: false
    type: string
    maps_to: "mj-social color"
  - id: elements
    required: true
    type: list
    maps_to: "mj-social-element name+href (+ src for web)"
---

JSON `"name": "x"` emits MJML `name="twitter"`. Extra networks: youtube, pinterest, google, tumblr, snapchat, vimeo, medium, soundcloud, dribbble, xing. `web` still needs `src`.
