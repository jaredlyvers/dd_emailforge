---
component: mj-navbar
version: 1
node_scope: column_child
insert:
  parent: mj-column | mj-hero
  defaults:
    hamburger: false
    links: []
fields:
  - id: hamburger
    required: false
    type: bool
    default: false
    maps_to: 'mj-navbar hamburger="hamburger" when true'
  - id: ico_color
    required: false
    type: string
    maps_to: "mj-navbar ico-color"
  - id: base_url
    required: false
    type: string
    maps_to: "mj-navbar base-url"
  - id: align
    required: false
    type: enum
    options: ["left", "center", "right"]
    maps_to: "mj-navbar align"
  - id: padding
    required: false
    type: string
    hint: "1-4 values with px or %"
    example: "10px 20px"
    maps_to: "mj-navbar padding"
  - id: ico_align
    required: false
    type: enum
    options: ["left", "center", "right"]
    maps_to: "mj-navbar ico-align (hamburger only)"
  - id: ico_font_size
    required: false
    type: string
    maps_to: "mj-navbar ico-font-size (hamburger only)"
  - id: ico_padding
    required: false
    type: string
    maps_to: "mj-navbar ico-padding (hamburger only)"
  - id: ico_open
    required: false
    type: string
    maps_to: "mj-navbar ico-open (hamburger only)"
  - id: ico_close
    required: false
    type: string
    maps_to: "mj-navbar ico-close (hamburger only)"
  - id: css_class
    required: false
    type: string
    maps_to: "mj-navbar css-class"
  - id: links
    required: true
    type: list
    maps_to: "mj-navbar-link children"
---

Children are `mj-navbar-link` only. Insert a link when the navbar (or an existing link) is selected.
