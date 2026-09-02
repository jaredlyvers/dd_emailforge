---
component: email-footer
version: 1
node_scope: body_node
insert:
  defaults:
    company_name: ""
    address_lines: []
    unsubscribe_label: "Unsubscribe"
    unsubscribe_href: "*|UNSUB|*"
    social: []
fields:
  - id: company_name
    required: false
    type: string
    maps_to: "mj-text"
  - id: address_lines
    required: true
    type: list
    maps_to: "mj-text joined with <br />"
  - id: unsubscribe_label
    required: false
    type: string
    maps_to: "a inner (only when unsubscribe_href is set)"
  - id: unsubscribe_href
    required: false
    type: string
    maps_to: "a href (omit link when empty)"
  - id: social
    required: false
    type: list
    maps_to: "mj-social"
  - id: copyright
    required: false
    type: string
    maps_to: "mj-text"
---

Emits divider, optional social, then a 12px centered text block. Transactional starters ship empty unsub label **and** href.
