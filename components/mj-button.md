---
component: mj-button
version: 1
node_scope: column_child
insert:
  defaults:
    content: "Read more"
    href: "https://example.com"
fields:
  - id: content
    required: true
    type: string
    maps_to: "mj-button inner text (plain, XML-escaped)"
  - id: href
    required: true
    type: string
    maps_to: "mj-button href"
    ui:
      note: "Absolute https or an opaque merge tag."
  - id: background_color
    required: false
    type: string
    maps_to: "mj-button background-color"
  - id: color
    required: false
    type: string
    maps_to: "mj-button color"
  - id: align
    required: false
    type: enum
    options: ["left", "center", "right"]
    maps_to: "mj-button align"
  - id: font_family
    required: false
    type: string
    maps_to: "mj-button font-family"
  - id: border_radius
    required: false
    type: string
    maps_to: "mj-button border-radius"
  - id: width
    required: false
    type: string
    maps_to: "mj-button width"
  - id: padding
    required: false
    type: string
    maps_to: "mj-button padding"
---
