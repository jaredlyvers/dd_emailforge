---
component: mj-accordion
version: 1
node_scope: column_child
insert:
  parent: mj-column | mj-hero
  defaults:
    elements: []
fields:
  - id: border
    required: false
    type: string
    maps_to: "mj-accordion border (CSS border)"
  - id: padding
    required: false
    type: string
    hint: "1-4 values with px or %"
    example: "10px 20px"
    maps_to: "mj-accordion padding"
  - id: font_family
    required: false
    type: string
    maps_to: "mj-accordion font-family"
  - id: icon_position
    required: false
    type: enum
    options: ["left", "right"]
    maps_to: "mj-accordion icon-position"
  - id: icon_width
    required: false
    type: string
    maps_to: "mj-accordion icon-width"
  - id: icon_height
    required: false
    type: string
    maps_to: "mj-accordion icon-height"
  - id: icon_wrapped_url
    required: false
    type: url
    maps_to: "mj-accordion icon-wrapped-url"
  - id: icon_unwrapped_url
    required: false
    type: url
    maps_to: "mj-accordion icon-unwrapped-url"
  - id: css_class
    required: false
    type: string
    maps_to: "mj-accordion css-class"
  - id: elements
    required: true
    type: list
    maps_to: "mj-accordion-element children"
---

Children are `mj-accordion-element` only. Title and body live on the element; `mj-accordion-title` / `mj-accordion-text` are not separately insertable.
