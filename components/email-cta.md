---
component: email-cta
version: 1
node_scope: body_node
insert:
  defaults:
    heading: "Call to action"
    copy: ""
    button_label: "Learn more"
    button_href: "https://example.com"
fields:
  - id: heading
    required: false
    type: string
    maps_to: "mj-text font-size=22px font-weight=bold"
  - id: copy
    required: false
    type: string
    maps_to: "mj-text"
  - id: button_label
    required: false
    type: string
    maps_to: "mj-button inner"
  - id: button_href
    required: true
    type: string
    maps_to: "mj-button href"
  - id: background_color
    required: false
    type: string
    maps_to: "mj-section background-color"
---
