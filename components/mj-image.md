---
component: mj-image
version: 1
node_scope: column_child
insert:
  defaults:
    src: "https://dummyimage.com/600x200/cccccc/000000"
    alt: "Image"
    fluid_on_mobile: true
fields:
  - id: src
    required: true
    type: string
    maps_to: "mj-image src (rewritten in preview/export)"
  - id: alt
    required: true
    type: string
    maps_to: "mj-image alt"
  - id: href
    required: false
    type: string
    maps_to: "mj-image href"
  - id: width
    required: false
    type: string
    maps_to: "mj-image width"
  - id: align
    required: false
    type: enum
    options: ["left", "center", "right"]
    maps_to: "mj-image align"
  - id: fluid_on_mobile
    required: false
    type: bool
    default: true
    maps_to: "mj-image fluid-on-mobile=true"
  - id: padding
    required: false
    type: string
    maps_to: "mj-image padding"
---
