---
component: mj-spacer
version: 1
node_scope: column_child
insert:
  defaults:
    height: "24px"
fields:
  - id: height
    required: true
    type: string
    default: "24px"
    maps_to: "mj-spacer height"
  - id: padding
    required: false
    type: string
    hint: "1-4 values with px or %"
    maps_to: "mj-spacer padding"
---
