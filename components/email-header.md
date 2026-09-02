---
component: email-header
version: 1
node_scope: body_node
insert:
  defaults:
    logo_src: ""
    logo_alt: ""
    logo_width: "160px"
fields:
  - id: logo_src
    required: false
    type: string
    maps_to: "mj-image src (omitted when empty)"
  - id: logo_alt
    required: false
    type: string
    maps_to: "mj-image alt (required iff logo_src is set)"
  - id: logo_href
    required: false
    type: string
    maps_to: "mj-image href"
  - id: logo_width
    required: false
    type: string
    default: "160px"
    maps_to: "mj-image width"
  - id: background_color
    required: false
    type: string
    maps_to: "mj-section background-color"
---

Emits `mj-section` → one `mj-column` → optional left-aligned `mj-image`.
