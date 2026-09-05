---
component: mj-carousel
version: 1
node_scope: column_child
insert:
  parent: mj-column | mj-hero
  defaults:
    thumbnails: hidden
    images: []
fields:
  - id: align
    required: false
    type: enum
    options: ["left", "center", "right"]
    maps_to: "mj-carousel align"
  - id: padding
    required: false
    type: string
    hint: "1-4 values with px or %"
    example: "10px 20px"
    maps_to: "mj-carousel padding"
  - id: border_radius
    required: false
    type: string
    hint: "px or %"
    maps_to: "mj-carousel border-radius"
  - id: thumbnails
    required: false
    type: enum
    options: ["visible", "hidden", "supported"]
    default: hidden
    maps_to: "mj-carousel thumbnails"
  - id: images
    required: true
    type: list
    maps_to: "mj-carousel-image children"
---

Children are `mj-carousel-image` only.
