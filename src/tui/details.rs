//! Read-only inspector: labeled fields + a simple 600px ascii canvas.
use crate::model::{
    BodyNode, ColumnChild, EmailArticle, EmailFooter, EmailHeader, EmailHero, MjColumn, MjGroup,
    MjSection, SectionChild, Template,
};

use super::tree::{Step, TreeId, TreeRow};

pub fn details_title(label: &str) -> String {
    format!("Details — {label}")
}

pub fn details_lines(
    template: Option<&Template>,
    row: Option<&TreeRow>,
    width: usize,
) -> Vec<String> {
    let Some(t) = template else {
        return vec!["No template open.".into()];
    };
    let Some(row) = row else {
        return vec!["Nothing selected.".into()];
    };
    match &row.id {
        TreeId::Head => head_lines(t),
        TreeId::Brand => brand_lines(t),
        TreeId::Body => {
            let mut lines = vec![format!("nodes: {}", t.body.nodes.len())];
            lines.push(String::new());
            lines.extend(email_ascii(t, width));
            lines
        }
        TreeId::Path(path) => path_lines(t, path, width),
    }
}

fn head_lines(t: &Template) -> Vec<String> {
    let fonts = if t.head.fonts.is_empty() {
        "(none)".into()
    } else {
        t.head
            .fonts
            .iter()
            .map(|f| format!("{} ({})", f.name, host_of(&f.href)))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let json_ld = json_ld_summary(&t.head.json_ld);
    let css = if t.head.css.trim().is_empty() {
        "(empty)".into()
    } else {
        format!("{} lines", t.head.css.lines().count())
    };
    vec![
        format!("subject:     {}", dash(&t.subject)),
        format!("preheader:   {}", dash(&t.preheader)),
        format!("lang:        {}", dash(&t.lang)),
        format!("title:       {}", dash(&t.head.title)),
        format!("breakpoint:  {}", dash(&t.head.breakpoint)),
        format!("base_url:    {}", dash(&t.base_url)),
        format!("fonts:       {fonts}"),
        format!("json_ld:     {json_ld}"),
        format!("css:         {css}"),
        format!("css_inline:  {}", t.head.css_inline),
    ]
}

fn brand_lines(t: &Template) -> Vec<String> {
    vec![
        format!("font_family:         {}", t.brand.font_family),
        format!("text_color:          {}", t.brand.text_color),
        format!("background_color:    {}", t.brand.background_color),
        format!("content_width:       {}", t.brand.content_width),
        format!("button_background:   {}", t.brand.button_background),
        format!("button_color:        {}", t.brand.button_color),
    ]
}

fn path_lines(t: &Template, path: &[Step], width: usize) -> Vec<String> {
    match locate(t, path) {
        Located::BodyNode(n) => body_node_lines(n, width, t.brand.content_width),
        Located::Column(c) => column_lines(c),
        Located::Leaf(label, extra) => {
            let mut v = vec![label];
            v.extend(extra);
            v
        }
        Located::Missing => vec!["(missing node)".into()],
    }
}

enum Located<'a> {
    BodyNode(&'a BodyNode),
    Column(&'a MjColumn),
    Leaf(String, Vec<String>),
    Missing,
}

fn locate<'a>(t: &'a Template, path: &[Step]) -> Located<'a> {
    let mut steps = path.iter();
    let Some(Step::BodyNode(i)) = steps.next() else {
        return Located::Missing;
    };
    let mut node = match t.body.nodes.get(*i) {
        Some(n) => n,
        None => return Located::Missing,
    };
    loop {
        match steps.next() {
            None => return Located::BodyNode(node),
            Some(Step::WrapperChild(j)) => {
                let BodyNode::MjWrapper(w) = node else {
                    return Located::Missing;
                };
                node = match w.children.get(*j) {
                    Some(n) => n,
                    None => return Located::Missing,
                };
            }
            Some(Step::SectionChild(j)) => {
                let BodyNode::MjSection(s) = node else {
                    return Located::Missing;
                };
                match s.children.get(*j) {
                    Some(SectionChild::MjColumn(c)) => {
                        return locate_column(c, steps);
                    }
                    Some(SectionChild::MjGroup(g)) => {
                        return locate_group(g, steps);
                    }
                    None => return Located::Missing,
                }
            }
            Some(Step::HeroChild(j)) => {
                let BodyNode::MjHero(h) = node else {
                    return Located::Missing;
                };
                return match h.children.get(*j) {
                    Some(c) => Located::Leaf(column_child_detail(c), vec![]),
                    None => Located::Missing,
                };
            }
            _ => return Located::Missing,
        }
    }
}

fn locate_group<'a>(g: &'a MjGroup, mut steps: std::slice::Iter<'_, Step>) -> Located<'a> {
    match steps.next() {
        None => Located::Leaf(
            "mj-group".into(),
            vec![format!("columns: {}", g.children.len())],
        ),
        Some(Step::GroupCol(i)) => match g.children.get(*i) {
            Some(c) => locate_column(c, steps),
            None => Located::Missing,
        },
        _ => Located::Missing,
    }
}

fn locate_column<'a>(c: &'a MjColumn, mut steps: std::slice::Iter<'_, Step>) -> Located<'a> {
    match steps.next() {
        None => Located::Column(c),
        Some(Step::ColComp(i)) => match c.components.get(*i) {
            Some(ch) => Located::Leaf(column_child_detail(ch), vec![]),
            None => Located::Missing,
        },
        _ => Located::Missing,
    }
}

fn body_node_lines(n: &BodyNode, width: usize, canvas: u32) -> Vec<String> {
    match n {
        BodyNode::MjSection(s) => {
            let mut lines = vec![format!("children: {}", s.children.len())];
            if let Some(bg) = &s.background_color {
                lines.push(format!("background: {bg}"));
            }
            lines.push(String::new());
            lines.extend(section_ascii(s, width, canvas));
            lines
        }
        BodyNode::MjWrapper(w) => vec![
            format!("wrapper children: {}", w.children.len()),
            format!("full_width: {}", w.full_width),
        ],
        BodyNode::MjHero(h) => {
            let mut lines = vec![format!("hero children: {}", h.children.len())];
            if let Some(url) = &h.background_url {
                lines.push(format!("background_url: {url}"));
            }
            lines
        }
        BodyNode::EmailHeader(h) => email_header_lines(h),
        BodyNode::EmailHero(h) => email_hero_lines(h),
        BodyNode::EmailCta(c) => vec![
            format!("heading: {}", dash(&c.heading)),
            format!("copy:    {}", dash(&c.copy)),
            format!(
                "button:  {} → {}",
                dash(&c.button_label),
                dash(&c.button_href)
            ),
        ],
        BodyNode::EmailArticle(a) => email_article_lines(a),
        BodyNode::EmailFooter(f) => email_footer_lines(f),
    }
}

fn email_header_lines(h: &EmailHeader) -> Vec<String> {
    vec![
        format!("logo_src: {}", dash(&h.logo_src)),
        format!("logo_alt: {}", dash(&h.logo_alt)),
        format!("logo_width: {}", h.logo_width),
    ]
}

fn email_hero_lines(h: &EmailHero) -> Vec<String> {
    vec![
        format!("heading:    {}", dash(&h.heading)),
        format!("subheading: {}", dash(&h.subheading)),
        format!("image_src:  {}", dash(&h.image_src)),
    ]
}

fn email_article_lines(a: &EmailArticle) -> Vec<String> {
    vec![
        format!("title:     {}", dash(&a.title)),
        format!("copy:      {}", dash(&a.copy)),
        format!("image:     {}", dash(&a.image_src)),
        format!("position:  {:?}", a.image_position),
    ]
}

fn email_footer_lines(f: &EmailFooter) -> Vec<String> {
    vec![
        format!("company: {}", dash(&f.company_name)),
        format!("address: {}", f.address_lines.join(" / ")),
        format!("unsub:   {}", dash(&f.unsubscribe_href)),
    ]
}

fn column_lines(c: &MjColumn) -> Vec<String> {
    let mut lines = vec![
        format!("width: {}", c.width.as_deref().unwrap_or("(equal split)")),
        format!("components: {}", c.components.len()),
    ];
    for ch in &c.components {
        lines.push(format!("  - {}", column_child_detail(ch)));
    }
    lines
}

fn column_child_detail(c: &ColumnChild) -> String {
    match c {
        ColumnChild::MjText(t) => format!("mj-text  {}", truncate(&t.content, 40)),
        ColumnChild::MjButton(b) => format!("mj-button  {} → {}", b.content, b.href),
        ColumnChild::MjImage(i) => format!("mj-image  {}", i.src),
        ColumnChild::MjDivider(_) => "mj-divider".into(),
        ColumnChild::MjSpacer(s) => format!("mj-spacer  {}", s.height),
        ColumnChild::MjSocial(s) => format!("mj-social  {} icons", s.elements.len()),
        ColumnChild::MjTable(_) => "mj-table".into(),
    }
}

fn email_ascii(t: &Template, width: usize) -> Vec<String> {
    let inner = width.saturating_sub(2).max(8);
    let border = format!("+{}+", "-".repeat(inner));
    let mut lines = vec![border.clone()];
    lines.push(fit_box(
        &format!("{}px canvas", t.brand.content_width),
        inner,
    ));
    if t.body.nodes.is_empty() {
        lines.push(fit_box("(empty body)", inner));
    } else {
        for (i, n) in t.body.nodes.iter().enumerate() {
            lines.push(fit_box(&format!("{}. {}", i + 1, body_kind(n)), inner));
        }
    }
    lines.push(border);
    lines
}

fn section_ascii(s: &MjSection, width: usize, _canvas: u32) -> Vec<String> {
    let inner = width.saturating_sub(2).max(8);
    if s.children.is_empty() {
        return vec!["(no columns)".into()];
    }
    let cols = s.children.len().max(1);
    let cell = ((inner.saturating_sub(cols.saturating_sub(1))) / cols).max(4);
    let mut top = String::from("+");
    for _ in 0..cols {
        top.push_str(&"-".repeat(cell));
        top.push('+');
    }
    let mut mid = String::from("|");
    for child in &s.children {
        let label = match child {
            SectionChild::MjColumn(c) => c
                .width
                .clone()
                .unwrap_or_else(|| format!("col {}", c.components.len())),
            SectionChild::MjGroup(_) => "group".into(),
        };
        mid.push_str(&pad_cell(&label, cell));
        mid.push('|');
    }
    vec![top.clone(), mid, top]
}

fn fit_box(text: &str, inner: usize) -> String {
    format!("|{}|", pad_cell(text, inner))
}

fn pad_cell(text: &str, width: usize) -> String {
    let t: String = text.chars().take(width.saturating_sub(1)).collect();
    format!(" {t:<width$}", width = width.saturating_sub(1))
        .chars()
        .take(width)
        .collect()
}

fn body_kind(n: &BodyNode) -> &'static str {
    match n {
        BodyNode::MjSection(_) => "mj-section",
        BodyNode::MjWrapper(_) => "mj-wrapper",
        BodyNode::MjHero(_) => "mj-hero",
        BodyNode::EmailHeader(_) => "email-header",
        BodyNode::EmailHero(_) => "email-hero",
        BodyNode::EmailCta(_) => "email-cta",
        BodyNode::EmailArticle(_) => "email-article",
        BodyNode::EmailFooter(_) => "email-footer",
    }
}

fn dash(s: &str) -> &str {
    if s.trim().is_empty() {
        "—"
    } else {
        s
    }
}

fn truncate(s: &str, max: usize) -> String {
    let t = s.replace('\n', " ");
    if t.chars().count() <= max {
        t
    } else {
        let mut out: String = t.chars().take(max.saturating_sub(1)).collect();
        out.push('…');
        out
    }
}

fn host_of(href: &str) -> &str {
    href.split('/').nth(2).unwrap_or(href)
}

fn json_ld_summary(raw: &str) -> String {
    let raw = raw.trim();
    if raw.is_empty() {
        return "(empty)".into();
    }
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(raw) {
        if let Some(t) = v.get("@type").and_then(|x| x.as_str()) {
            return t.to_string();
        }
        if let Some(arr) = v.as_array() {
            if let Some(t) = arr
                .iter()
                .find_map(|x| x.get("@type").and_then(|y| y.as_str()))
            {
                return t.to_string();
            }
        }
    }
    "(invalid JSON)".into()
}
