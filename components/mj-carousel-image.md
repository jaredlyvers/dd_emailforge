---
component: mj-carousel-image
version: 1
node_scope: carousel_child
insert:
  parent: mj-carousel
  defaults:
    src: "https://dummyimage.com/600x200/cccccc/000000"
    alt: "Image"
fields:
  - id: src
    required: true
    type: url
    maps_to: "mj-carousel-image src"
  - id: alt
    required: true
    type: string
    maps_to: "mj-carousel-image alt"
  - id: href
    required: false
    type: url
    maps_to: "mj-carousel-image href"
  - id: thumbnails_src
    required: false
    type: url
    maps_to: "mj-carousel-image thumbnails-src"
---

Not insertable under `mj-column` or `mj-section`.
