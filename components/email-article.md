---
component: email-article
version: 1
node_scope: body_node
insert:
  defaults:
    image_src: ""
    image_alt: ""
    title: "Title"
    copy: ""
    link_label: "Read more"
    link_href: "https://example.com"
    image_position: top
fields:
  - id: image_src
    required: false
    type: string
    maps_to: "mj-image src (omitted when empty)"
  - id: image_alt
    required: false
    type: string
    maps_to: "mj-image alt (required iff image_src is set)"
  - id: title
    required: false
    type: string
    maps_to: "mj-text font-size=20px font-weight=bold"
  - id: copy
    required: false
    type: string
    maps_to: "mj-text"
  - id: link_label
    required: false
    type: string
    maps_to: "mj-button inner"
  - id: link_href
    required: false
    type: string
    maps_to: "mj-button href"
  - id: image_position
    required: false
    type: enum
    options: ["top", "left", "right"]
    default: top
    maps_to: "layout only (not an MJML attribute)"
---
