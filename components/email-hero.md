---
component: email-hero
version: 1
node_scope: body_node
insert:
  defaults:
    image_src: ""
    image_alt: ""
    heading: "Headline"
    subheading: ""
fields:
  - id: image_src
    required: false
    type: string
    maps_to: "mj-image src (omitted when empty)"
  - id: image_alt
    required: false
    type: string
    maps_to: "mj-image alt (required iff image_src is set)"
  - id: heading
    required: false
    type: string
    maps_to: "mj-text font-size=28px font-weight=bold"
  - id: subheading
    required: false
    type: string
    maps_to: "mj-text"
  - id: background_color
    required: false
    type: string
    maps_to: "mj-section background-color"
---
