---
component: mj-table
version: 1
node_scope: column_child
insert:
  defaults:
    content: "<table><tr><td></td></tr></table>"
fields:
  - id: content
    required: true
    type: string
    ui:
      control: textarea
    maps_to: "mj-table inner — single <table>…</table> fragment"
  - id: font_size
    required: false
    type: string
    maps_to: "mj-table font-size"
  - id: color
    required: false
    type: string
    maps_to: "mj-table color"
  - id: padding
    required: false
    type: string
    maps_to: "mj-table padding"
---
