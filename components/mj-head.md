---
component: mj-head
version: 1
node_scope: head
insert: false
fields:
  - id: dir
    required: false
    type: enum
    options: ["auto", "ltr", "rtl"]
    maps_to: "mjml dir (document-level, edited in HEAD FormEdit)"
  - id: title
    required: true
    type: string
    maps_to: "mj-title"
  - id: breakpoint
    required: false
    type: string
    default: "480px"
    maps_to: "mj-breakpoint width"
  - id: fonts
    required: false
    type: list
    maps_to: "mj-font name + href (Google Fonts CSS only)"
  - id: json_ld
    required: false
    type: string
    ui:
      control: textarea
    maps_to: "mj-raw > script[type=application/ld+json] (emitter-wrapped)"
  - id: css
    required: false
    type: string
    ui:
      control: textarea
    maps_to: "mj-style"
  - id: css_inline
    required: false
    type: bool
    default: false
    maps_to: "mj-style inline=inline"
---

Not insertable. Edited via the `[HEAD]` FormEdit. `fonts[].href` must be `https://fonts.googleapis.com/css?` or `css2?`. JSON-LD is parsed to JSON then pretty-printed; authors do not type `mj-raw`.
