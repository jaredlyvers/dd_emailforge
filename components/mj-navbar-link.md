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
  - id: padding
    required: false
    type: string
    maps_to: "mj-navbar-link padding"
---

Ending tag. Not insertable under `mj-column` or `mj-section`.
