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
    maps_to: "mj-navbar padding"
  - id: links
    required: true
    type: list
    maps_to: "mj-navbar-link children"
---

Children are `mj-navbar-link` only. Insert a link when the navbar (or an existing link) is selected.
