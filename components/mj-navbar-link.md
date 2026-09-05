---
component: mj-navbar-link
version: 1
node_scope: navbar_child
insert:
  parent: mj-navbar
  defaults:
    href: "https://example.com"
    content: "Link"
fields:
  - id: href
    required: true
    type: url
    maps_to: "mj-navbar-link href"
  - id: content
    required: true
    type: string
    maps_to: "mj-navbar-link inner text (ending tag)"
  - id: color
    required: false
    type: string
    maps_to: "mj-navbar-link color"
  - id: font_family
    required: false
    type: string
    maps_to: "mj-navbar-link font-family"
  - id: font_size
    required: false
    type: string
    maps_to: "mj-navbar-link font-size"
  - id: font_weight
    required: false
    type: enum
    options: ["normal", "bold", "400", "700"]
    maps_to: "mj-navbar-link font-weight"
  - id: text_decoration
    required: false
    type: enum
    options: ["none", "underline", "overline", "line-through"]
    maps_to: "mj-navbar-link text-decoration"
  - id: text_transform
    required: false
    type: enum
    options: ["none", "uppercase", "lowercase", "capitalize"]
    maps_to: "mj-navbar-link text-transform"
  - id: padding
    required: false
    type: string
    hint: "1-4 values with px or %"
    example: "10px 20px"
    maps_to: "mj-navbar-link padding"
---

Ending tag. Not insertable under `mj-column` or `mj-section`.
