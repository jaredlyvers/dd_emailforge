//! Populate FormEdit from a tree selection and write it back on Ctrl+S.
use anyhow::{anyhow, Context, Result};

use crate::model::{
    Align, BodyNode, ColumnChild, EmailArticle, EmailCta, EmailFooter, EmailHeader, EmailHero,
    HeroMode, ImagePosition, MjAccordion, MjAccordionElement, MjCarousel, MjCarouselImage,
    MjColumn, MjGroup, MjHero, MjNavbar, MjNavbarLink, MjSocial, MjSocialElement, SectionChild,
    SocialMode, SocialNetwork, Template, Thumbnails, WebFont,
};

use super::editform::{self, EditFormState};
use super::tree::{Step, TreeId};

pub fn form_for(template: &Template, id: &TreeId) -> Option<EditFormState> {
    match id {
        TreeId::Head => Some(head_to_form(template)),
        TreeId::Brand => Some(brand_to_form(template)),
        TreeId::Body => Some(body_to_form(template)),
        TreeId::Path(path) => path_to_form(template, path),
    }
}

pub fn apply_form(template: &mut Template, id: &TreeId, state: &EditFormState) -> Result<()> {
    match id {
        TreeId::Head => apply_head(template, state),
        TreeId::Brand => apply_brand(template, state),
        TreeId::Body => apply_body(template, state),
        TreeId::Path(path) => apply_path(template, path, state),
    }
}

fn head_to_form(t: &Template) -> EditFormState {
    let mut s = EditFormState::new(&editform::HEAD_FORM);
    s.set("subject", t.subject.clone());
    s.set("preheader", t.preheader.clone());
    s.set("lang", t.lang.clone());
    s.set("title", t.head.title.clone());
    s.set("breakpoint", t.head.breakpoint.clone());
    s.set("base_url", t.base_url.clone());
    s.set("json_ld", t.head.json_ld.clone());
    s.set("css", t.head.css.clone());
    s.set("css_inline", bool_str(t.head.css_inline));
    let mut fonts = Vec::new();
    for font in &t.head.fonts {
        let mut item = EditFormState::new(&editform::FONT_ITEM_FORM);
        item.set("name", font.name.clone());
        item.set("href", font.href.clone());
        fonts.push(item);
    }
    s.sub_state.insert("fonts".into(), fonts);
    s.selected_sub_item.insert("fonts".into(), 0);
    s
}

fn apply_head(t: &mut Template, state: &EditFormState) -> Result<()> {
    t.subject = state.get("subject").to_string();
    t.preheader = state.get("preheader").to_string();
    t.lang = state.get("lang").to_string();
    t.head.title = state.get("title").to_string();
    t.head.breakpoint = state.get("breakpoint").to_string();
    t.base_url = state.get("base_url").to_string();
    t.head.json_ld = state.get("json_ld").to_string();
    t.head.css = state.get("css").to_string();
    t.head.css_inline = parse_bool(state.get("css_inline"))?;
    t.head.fonts.clear();
    if let Some(items) = state.sub_state.get("fonts") {
        for item in items {
            t.head.fonts.push(WebFont {
                name: item.get("name").trim().to_string(),
                href: item.get("href").trim().to_string(),
            });
        }
    }
    Ok(())
}

fn brand_to_form(t: &Template) -> EditFormState {
    let mut s = EditFormState::new(&editform::BRAND_FORM);
    s.set("font_family", t.brand.font_family.clone());
    s.set("text_color", t.brand.text_color.clone());
    s.set("background_color", t.brand.background_color.clone());
    s.set("content_width", t.brand.content_width.to_string());
    s.set("button_background", t.brand.button_background.clone());
    s.set("button_color", t.brand.button_color.clone());
    s
}

fn apply_brand(t: &mut Template, state: &EditFormState) -> Result<()> {
    t.brand.font_family = state.get("font_family").to_string();
    t.brand.text_color = state.get("text_color").trim().to_string();
    t.brand.background_color = state.get("background_color").trim().to_string();
    t.brand.content_width = state
        .get("content_width")
        .trim()
        .parse::<u32>()
        .context("content_width must be a number")?;
    t.brand.button_background = state.get("button_background").trim().to_string();
    t.brand.button_color = state.get("button_color").trim().to_string();
    Ok(())
}

fn body_to_form(t: &Template) -> EditFormState {
    let mut s = EditFormState::new(&editform::BODY_FORM);
    s.set("background_color", t.body.background_color.clone());
    s
}

fn apply_body(t: &mut Template, state: &EditFormState) -> Result<()> {
    t.body.background_color = state.get("background_color").trim().to_string();
    Ok(())
}

fn path_to_form(t: &Template, path: &[Step]) -> Option<EditFormState> {
    match locate(t, path)? {
        Located::BodyNode(n) => Some(body_node_to_form(n)),
        Located::Column(c) => Some(column_to_form(c)),
        Located::Group(g) => Some(group_to_form(g)),
        Located::Leaf(c) => Some(leaf_to_form(c)),
        Located::NavbarLink(l) => Some(navbar_link_to_form(l)),
        Located::AccordionEl(e) => Some(accordion_el_to_form(e)),
        Located::CarouselImg(i) => Some(carousel_img_to_form(i)),
    }
}

fn apply_path(t: &mut Template, path: &[Step], state: &EditFormState) -> Result<()> {
    match locate_mut(t, path).ok_or_else(|| anyhow!("missing node"))? {
        LocatedMut::BodyNode(n) => apply_body_node(n, state),
        LocatedMut::Column(c) => apply_column(c, state),
        LocatedMut::Group(g) => apply_group(g, state),
        LocatedMut::Leaf(c) => apply_leaf(c, state),
        LocatedMut::NavbarLink(l) => apply_navbar_link(l, state),
        LocatedMut::AccordionEl(e) => apply_accordion_el(e, state),
        LocatedMut::CarouselImg(i) => apply_carousel_img(i, state),
    }
}

enum Located<'a> {
    BodyNode(&'a BodyNode),
    Column(&'a MjColumn),
    Group(&'a MjGroup),
    Leaf(&'a ColumnChild),
    NavbarLink(&'a MjNavbarLink),
    AccordionEl(&'a MjAccordionElement),
    CarouselImg(&'a MjCarouselImage),
}

enum LocatedMut<'a> {
    BodyNode(&'a mut BodyNode),
    Column(&'a mut MjColumn),
    Group(&'a mut MjGroup),
    Leaf(&'a mut ColumnChild),
    NavbarLink(&'a mut MjNavbarLink),
    AccordionEl(&'a mut MjAccordionElement),
    CarouselImg(&'a mut MjCarouselImage),
}

fn locate<'a>(t: &'a Template, path: &[Step]) -> Option<Located<'a>> {
    let mut steps = path.iter();
    let Step::BodyNode(i) = steps.next()? else {
        return None;
    };
    let mut node = t.body.nodes.get(*i)?;
    loop {
        match steps.next() {
            None => return Some(Located::BodyNode(node)),
            Some(Step::WrapperChild(j)) => {
                let BodyNode::MjWrapper(w) = node else {
                    return None;
                };
                node = w.children.get(*j)?;
            }
            Some(Step::SectionChild(j)) => {
                let BodyNode::MjSection(s) = node else {
                    return None;
                };
                return match s.children.get(*j)? {
                    SectionChild::MjColumn(c) => locate_column(c, steps),
                    SectionChild::MjGroup(g) => locate_group(g, steps),
                };
            }
            Some(Step::HeroChild(j)) => {
                let BodyNode::MjHero(h) = node else {
                    return None;
                };
                return locate_column_child(h.children.get(*j)?, steps);
            }
            _ => return None,
        }
    }
}

fn locate_column<'a>(
    c: &'a MjColumn,
    mut steps: std::slice::Iter<'_, Step>,
) -> Option<Located<'a>> {
    match steps.next() {
        None => Some(Located::Column(c)),
        Some(Step::ColComp(i)) => locate_column_child(c.components.get(*i)?, steps),
        _ => None,
    }
}

fn locate_column_child<'a>(
    c: &'a ColumnChild,
    mut steps: std::slice::Iter<'_, Step>,
) -> Option<Located<'a>> {
    match steps.next() {
        None => Some(Located::Leaf(c)),
        Some(Step::NavbarLink(i)) => {
            let ColumnChild::MjNavbar(n) = c else {
                return None;
            };
            Some(Located::NavbarLink(n.links.get(*i)?))
        }
        Some(Step::AccordionEl(i)) => {
            let ColumnChild::MjAccordion(n) = c else {
                return None;
            };
            Some(Located::AccordionEl(n.elements.get(*i)?))
        }
        Some(Step::CarouselImg(i)) => {
            let ColumnChild::MjCarousel(n) = c else {
                return None;
            };
            Some(Located::CarouselImg(n.images.get(*i)?))
        }
        _ => None,
    }
}

fn locate_group<'a>(g: &'a MjGroup, mut steps: std::slice::Iter<'_, Step>) -> Option<Located<'a>> {
    match steps.next() {
        None => Some(Located::Group(g)),
        Some(Step::GroupCol(i)) => locate_column(g.children.get(*i)?, steps),
        _ => None,
    }
}

fn locate_mut<'a>(t: &'a mut Template, path: &[Step]) -> Option<LocatedMut<'a>> {
    let mut steps = path.iter();
    let Step::BodyNode(i) = steps.next()? else {
        return None;
    };
    let mut node = t.body.nodes.get_mut(*i)?;
    loop {
        match steps.next() {
            None => return Some(LocatedMut::BodyNode(node)),
            Some(Step::WrapperChild(j)) => {
                let BodyNode::MjWrapper(w) = node else {
                    return None;
                };
                node = w.children.get_mut(*j)?;
            }
            Some(Step::SectionChild(j)) => {
                let BodyNode::MjSection(s) = node else {
                    return None;
                };
                return match s.children.get_mut(*j)? {
                    SectionChild::MjColumn(c) => locate_column_mut(c, steps),
                    SectionChild::MjGroup(g) => locate_group_mut(g, steps),
                };
            }
            Some(Step::HeroChild(j)) => {
                let BodyNode::MjHero(h) = node else {
                    return None;
                };
                return locate_column_child_mut(h.children.get_mut(*j)?, steps);
            }
            _ => return None,
        }
    }
}

fn locate_column_mut<'a>(
    c: &'a mut MjColumn,
    mut steps: std::slice::Iter<'_, Step>,
) -> Option<LocatedMut<'a>> {
    match steps.next() {
        None => Some(LocatedMut::Column(c)),
        Some(Step::ColComp(i)) => locate_column_child_mut(c.components.get_mut(*i)?, steps),
        _ => None,
    }
}

fn locate_column_child_mut<'a>(
    c: &'a mut ColumnChild,
    mut steps: std::slice::Iter<'_, Step>,
) -> Option<LocatedMut<'a>> {
    match steps.next() {
        None => Some(LocatedMut::Leaf(c)),
        Some(Step::NavbarLink(i)) => {
            let ColumnChild::MjNavbar(n) = c else {
                return None;
            };
            Some(LocatedMut::NavbarLink(n.links.get_mut(*i)?))
        }
        Some(Step::AccordionEl(i)) => {
            let ColumnChild::MjAccordion(n) = c else {
                return None;
            };
            Some(LocatedMut::AccordionEl(n.elements.get_mut(*i)?))
        }
        Some(Step::CarouselImg(i)) => {
            let ColumnChild::MjCarousel(n) = c else {
                return None;
            };
            Some(LocatedMut::CarouselImg(n.images.get_mut(*i)?))
        }
        _ => None,
    }
}

fn locate_group_mut<'a>(
    g: &'a mut MjGroup,
    mut steps: std::slice::Iter<'_, Step>,
) -> Option<LocatedMut<'a>> {
    match steps.next() {
        None => Some(LocatedMut::Group(g)),
        Some(Step::GroupCol(i)) => locate_column_mut(g.children.get_mut(*i)?, steps),
        _ => None,
    }
}

fn body_node_to_form(n: &BodyNode) -> EditFormState {
    match n {
        BodyNode::MjSection(s) => {
            let mut st = EditFormState::new(&editform::SECTION_FORM);
            st.set("background_color", opt_get(&s.background_color));
            st.set("padding", opt_get(&s.padding));
            st.set("full_width", bool_str(s.full_width));
            st
        }
        BodyNode::MjWrapper(w) => {
            let mut st = EditFormState::new(&editform::WRAPPER_FORM);
            st.set("background_color", opt_get(&w.background_color));
            st.set("padding", opt_get(&w.padding));
            st.set("full_width", bool_str(w.full_width));
            st
        }
        BodyNode::MjHero(h) => hero_to_form(h),
        BodyNode::EmailHeader(h) => email_header_to_form(h),
        BodyNode::EmailHero(h) => email_hero_to_form(h),
        BodyNode::EmailCta(c) => email_cta_to_form(c),
        BodyNode::EmailArticle(a) => email_article_to_form(a),
        BodyNode::EmailFooter(f) => email_footer_to_form(f),
    }
}

fn apply_body_node(n: &mut BodyNode, state: &EditFormState) -> Result<()> {
    match n {
        BodyNode::MjSection(s) => {
            s.background_color = opt_set(state, "background_color");
            s.padding = opt_set(state, "padding");
            s.full_width = parse_bool(state.get("full_width"))?;
            Ok(())
        }
        BodyNode::MjWrapper(w) => {
            w.background_color = opt_set(state, "background_color");
            w.padding = opt_set(state, "padding");
            w.full_width = parse_bool(state.get("full_width"))?;
            Ok(())
        }
        BodyNode::MjHero(h) => apply_hero(h, state),
        BodyNode::EmailHeader(h) => apply_email_header(h, state),
        BodyNode::EmailHero(h) => apply_email_hero(h, state),
        BodyNode::EmailCta(c) => apply_email_cta(c, state),
        BodyNode::EmailArticle(a) => apply_email_article(a, state),
        BodyNode::EmailFooter(f) => apply_email_footer(f, state),
    }
}

fn column_to_form(c: &MjColumn) -> EditFormState {
    let mut st = EditFormState::new(&editform::COLUMN_FORM);
    st.set("width", opt_get(&c.width));
    st.set("background_color", opt_get(&c.background_color));
    st.set("padding", opt_get(&c.padding));
    st.set("inner_background_color", opt_get(&c.inner_background_color));
    st
}

fn apply_column(c: &mut MjColumn, state: &EditFormState) -> Result<()> {
    c.width = opt_set(state, "width");
    c.background_color = opt_set(state, "background_color");
    c.padding = opt_set(state, "padding");
    c.inner_background_color = opt_set(state, "inner_background_color");
    Ok(())
}

fn group_to_form(g: &MjGroup) -> EditFormState {
    let mut st = EditFormState::new(&editform::GROUP_FORM);
    st.set("width", opt_get(&g.width));
    st.set("background_color", opt_get(&g.background_color));
    st
}

fn apply_group(g: &mut MjGroup, state: &EditFormState) -> Result<()> {
    g.width = opt_set(state, "width");
    g.background_color = opt_set(state, "background_color");
    Ok(())
}

fn hero_to_form(h: &MjHero) -> EditFormState {
    let mut st = EditFormState::new(&editform::HERO_FORM);
    st.set("mode", hero_mode_str(h.mode));
    st.set("background_url", opt_get(&h.background_url));
    st.set("background_color", opt_get(&h.background_color));
    st.set("background_height", opt_get(&h.background_height));
    st.set("width", opt_get(&h.width));
    st.set("height", opt_get(&h.height));
    st
}

fn apply_hero(h: &mut MjHero, state: &EditFormState) -> Result<()> {
    h.mode = parse_hero_mode(state.get("mode"))?;
    h.background_url = opt_set(state, "background_url");
    h.background_color = opt_set(state, "background_color");
    h.background_height = opt_set(state, "background_height");
    h.width = opt_set(state, "width");
    h.height = opt_set(state, "height");
    Ok(())
}

fn leaf_to_form(c: &ColumnChild) -> EditFormState {
    match c {
        ColumnChild::MjText(t) => {
            let mut st = EditFormState::new(&editform::TEXT_FORM);
            st.set("content", t.content.clone());
            st.set("align", align_str(t.align));
            st.set("font_size", opt_get(&t.font_size));
            st.set("font_family", opt_get(&t.font_family));
            st.set("color", opt_get(&t.color));
            st.set("padding", opt_get(&t.padding));
            st
        }
        ColumnChild::MjButton(b) => {
            let mut st = EditFormState::new(&editform::BUTTON_FORM);
            st.set("content", b.content.clone());
            st.set("href", b.href.clone());
            st.set("background_color", opt_get(&b.background_color));
            st.set("color", opt_get(&b.color));
            st.set("align", align_str(b.align));
            st.set("font_family", opt_get(&b.font_family));
            st.set("border_radius", opt_get(&b.border_radius));
            st.set("width", opt_get(&b.width));
            st.set("padding", opt_get(&b.padding));
            st
        }
        ColumnChild::MjImage(i) => {
            let mut st = EditFormState::new(&editform::IMAGE_FORM);
            st.set("src", i.src.clone());
            st.set("alt", i.alt.clone());
            st.set("href", opt_get(&i.href));
            st.set("width", opt_get(&i.width));
            st.set("align", align_str(i.align));
            st.set("fluid_on_mobile", bool_str(i.fluid_on_mobile));
            st.set("padding", opt_get(&i.padding));
            st
        }
        ColumnChild::MjDivider(d) => {
            let mut st = EditFormState::new(&editform::DIVIDER_FORM);
            st.set("border_color", opt_get(&d.border_color));
            st.set("border_width", opt_get(&d.border_width));
            st.set("padding", opt_get(&d.padding));
            st
        }
        ColumnChild::MjSpacer(s) => {
            let mut st = EditFormState::new(&editform::SPACER_FORM);
            st.set("height", s.height.clone());
            st
        }
        ColumnChild::MjSocial(s) => social_to_form(s),
        ColumnChild::MjTable(tb) => {
            let mut st = EditFormState::new(&editform::TABLE_FORM);
            st.set("content", tb.content.clone());
            st.set("font_size", opt_get(&tb.font_size));
            st.set("color", opt_get(&tb.color));
            st.set("padding", opt_get(&tb.padding));
            st
        }
        ColumnChild::MjNavbar(n) => navbar_to_form(n),
        ColumnChild::MjAccordion(a) => accordion_to_form(a),
        ColumnChild::MjCarousel(c) => carousel_to_form(c),
    }
}

fn apply_leaf(c: &mut ColumnChild, state: &EditFormState) -> Result<()> {
    match c {
        ColumnChild::MjText(t) => {
            t.content = state.get("content").to_string();
            t.align = parse_align(state.get("align"))?;
            t.font_size = opt_set(state, "font_size");
            t.font_family = opt_set(state, "font_family");
            t.color = opt_set(state, "color");
            t.padding = opt_set(state, "padding");
            Ok(())
        }
        ColumnChild::MjButton(b) => {
            b.content = state.get("content").to_string();
            b.href = state.get("href").to_string();
            b.background_color = opt_set(state, "background_color");
            b.color = opt_set(state, "color");
            b.align = parse_align(state.get("align"))?;
            b.font_family = opt_set(state, "font_family");
            b.border_radius = opt_set(state, "border_radius");
            b.width = opt_set(state, "width");
            b.padding = opt_set(state, "padding");
            Ok(())
        }
        ColumnChild::MjImage(i) => {
            i.src = state.get("src").trim().to_string();
            i.alt = state.get("alt").to_string();
            i.href = opt_set(state, "href");
            i.width = opt_set(state, "width");
            i.align = parse_align(state.get("align"))?;
            i.fluid_on_mobile = parse_bool(state.get("fluid_on_mobile"))?;
            i.padding = opt_set(state, "padding");
            Ok(())
        }
        ColumnChild::MjDivider(d) => {
            d.border_color = opt_set(state, "border_color");
            d.border_width = opt_set(state, "border_width");
            d.padding = opt_set(state, "padding");
            Ok(())
        }
        ColumnChild::MjSpacer(s) => {
            s.height = state.get("height").trim().to_string();
            Ok(())
        }
        ColumnChild::MjSocial(s) => apply_social(s, state),
        ColumnChild::MjTable(tb) => {
            tb.content = state.get("content").to_string();
            tb.font_size = opt_set(state, "font_size");
            tb.color = opt_set(state, "color");
            tb.padding = opt_set(state, "padding");
            Ok(())
        }
        ColumnChild::MjNavbar(n) => apply_navbar(n, state),
        ColumnChild::MjAccordion(a) => apply_accordion(a, state),
        ColumnChild::MjCarousel(c) => apply_carousel(c, state),
    }
}

fn navbar_to_form(n: &MjNavbar) -> EditFormState {
    let mut st = EditFormState::new(&editform::NAVBAR_FORM);
    st.set("hamburger", bool_str(n.hamburger));
    st.set("ico_color", opt_get(&n.ico_color));
    st.set("base_url", opt_get(&n.base_url));
    st.set("align", align_str(n.align));
    st.set("padding", opt_get(&n.padding));
    st
}

fn apply_navbar(n: &mut MjNavbar, state: &EditFormState) -> Result<()> {
    n.hamburger = parse_bool(state.get("hamburger"))?;
    n.ico_color = opt_set(state, "ico_color");
    n.base_url = opt_set(state, "base_url");
    n.align = parse_align(state.get("align"))?;
    n.padding = opt_set(state, "padding");
    Ok(())
}

fn navbar_link_to_form(l: &MjNavbarLink) -> EditFormState {
    let mut st = EditFormState::new(&editform::NAVBAR_LINK_FORM);
    st.set("href", l.href.clone());
    st.set("content", l.content.clone());
    st.set("color", opt_get(&l.color));
    st.set("padding", opt_get(&l.padding));
    st
}

fn apply_navbar_link(l: &mut MjNavbarLink, state: &EditFormState) -> Result<()> {
    l.href = state.get("href").to_string();
    l.content = state.get("content").to_string();
    l.color = opt_set(state, "color");
    l.padding = opt_set(state, "padding");
    Ok(())
}

fn accordion_to_form(a: &MjAccordion) -> EditFormState {
    let mut st = EditFormState::new(&editform::ACCORDION_FORM);
    st.set("border", opt_get(&a.border));
    st.set("padding", opt_get(&a.padding));
    st
}

fn apply_accordion(a: &mut MjAccordion, state: &EditFormState) -> Result<()> {
    a.border = opt_set(state, "border");
    a.padding = opt_set(state, "padding");
    Ok(())
}

fn accordion_el_to_form(e: &MjAccordionElement) -> EditFormState {
    let mut st = EditFormState::new(&editform::ACCORDION_ELEMENT_FORM);
    st.set("title", e.title.clone());
    st.set("content", e.content.clone());
    st.set("background_color", opt_get(&e.background_color));
    st
}

fn apply_accordion_el(e: &mut MjAccordionElement, state: &EditFormState) -> Result<()> {
    e.title = state.get("title").to_string();
    e.content = state.get("content").to_string();
    e.background_color = opt_set(state, "background_color");
    Ok(())
}

fn carousel_to_form(c: &MjCarousel) -> EditFormState {
    let mut st = EditFormState::new(&editform::CAROUSEL_FORM);
    st.set("align", align_str(c.align));
    st.set("padding", opt_get(&c.padding));
    st.set("thumbnails", thumbnails_str(c.thumbnails));
    st
}

fn apply_carousel(c: &mut MjCarousel, state: &EditFormState) -> Result<()> {
    c.align = parse_align(state.get("align"))?;
    c.padding = opt_set(state, "padding");
    c.thumbnails = parse_thumbnails(state.get("thumbnails"))?;
    Ok(())
}

fn carousel_img_to_form(i: &MjCarouselImage) -> EditFormState {
    let mut st = EditFormState::new(&editform::CAROUSEL_IMAGE_FORM);
    st.set("src", i.src.clone());
    st.set("alt", i.alt.clone());
    st.set("href", opt_get(&i.href));
    st.set("thumbnails_src", opt_get(&i.thumbnails_src));
    st
}

fn apply_carousel_img(i: &mut MjCarouselImage, state: &EditFormState) -> Result<()> {
    i.src = state.get("src").trim().to_string();
    i.alt = state.get("alt").to_string();
    i.href = opt_set(state, "href");
    i.thumbnails_src = opt_set(state, "thumbnails_src");
    Ok(())
}

fn social_to_form(s: &MjSocial) -> EditFormState {
    let mut st = EditFormState::new(&editform::SOCIAL_FORM);
    st.set("mode", social_mode_str(s.mode));
    st.set("align", align_str(s.align));
    st.set("icon_size", s.icon_size.clone());
    st.sub_state
        .insert("elements".into(), social_items(&s.elements));
    st.selected_sub_item.insert("elements".into(), 0);
    st
}

fn apply_social(s: &mut MjSocial, state: &EditFormState) -> Result<()> {
    s.mode = parse_social_mode(state.get("mode"))?;
    s.align = parse_align(state.get("align"))?;
    s.icon_size = state.get("icon_size").trim().to_string();
    s.elements = parse_social_items(state.sub_state.get("elements"))?;
    Ok(())
}

fn email_header_to_form(h: &EmailHeader) -> EditFormState {
    let mut st = EditFormState::new(&editform::EMAIL_HEADER_FORM);
    st.set("logo_src", h.logo_src.clone());
    st.set("logo_alt", h.logo_alt.clone());
    st.set("logo_href", opt_get(&h.logo_href));
    st.set("logo_width", h.logo_width.clone());
    st.set("background_color", opt_get(&h.background_color));
    st
}

fn apply_email_header(h: &mut EmailHeader, state: &EditFormState) -> Result<()> {
    h.logo_src = state.get("logo_src").trim().to_string();
    h.logo_alt = state.get("logo_alt").to_string();
    h.logo_href = opt_set(state, "logo_href");
    h.logo_width = state.get("logo_width").trim().to_string();
    h.background_color = opt_set(state, "background_color");
    Ok(())
}

fn email_hero_to_form(h: &EmailHero) -> EditFormState {
    let mut st = EditFormState::new(&editform::EMAIL_HERO_FORM);
    st.set("image_src", h.image_src.clone());
    st.set("image_alt", h.image_alt.clone());
    st.set("heading", h.heading.clone());
    st.set("subheading", h.subheading.clone());
    st.set("background_color", opt_get(&h.background_color));
    st
}

fn apply_email_hero(h: &mut EmailHero, state: &EditFormState) -> Result<()> {
    h.image_src = state.get("image_src").trim().to_string();
    h.image_alt = state.get("image_alt").to_string();
    h.heading = state.get("heading").to_string();
    h.subheading = state.get("subheading").to_string();
    h.background_color = opt_set(state, "background_color");
    Ok(())
}

fn email_cta_to_form(c: &EmailCta) -> EditFormState {
    let mut st = EditFormState::new(&editform::EMAIL_CTA_FORM);
    st.set("heading", c.heading.clone());
    st.set("copy", c.copy.clone());
    st.set("button_label", c.button_label.clone());
    st.set("button_href", c.button_href.clone());
    st.set("background_color", opt_get(&c.background_color));
    st
}

fn apply_email_cta(c: &mut EmailCta, state: &EditFormState) -> Result<()> {
    c.heading = state.get("heading").to_string();
    c.copy = state.get("copy").to_string();
    c.button_label = state.get("button_label").to_string();
    c.button_href = state.get("button_href").to_string();
    c.background_color = opt_set(state, "background_color");
    Ok(())
}

fn email_article_to_form(a: &EmailArticle) -> EditFormState {
    let mut st = EditFormState::new(&editform::EMAIL_ARTICLE_FORM);
    st.set("image_src", a.image_src.clone());
    st.set("image_alt", a.image_alt.clone());
    st.set("title", a.title.clone());
    st.set("copy", a.copy.clone());
    st.set("link_label", a.link_label.clone());
    st.set("link_href", a.link_href.clone());
    st.set("image_position", image_pos_str(a.image_position));
    st
}

fn apply_email_article(a: &mut EmailArticle, state: &EditFormState) -> Result<()> {
    a.image_src = state.get("image_src").trim().to_string();
    a.image_alt = state.get("image_alt").to_string();
    a.title = state.get("title").to_string();
    a.copy = state.get("copy").to_string();
    a.link_label = state.get("link_label").to_string();
    a.link_href = state.get("link_href").to_string();
    a.image_position = parse_image_pos(state.get("image_position"))?;
    Ok(())
}

fn email_footer_to_form(f: &EmailFooter) -> EditFormState {
    let mut st = EditFormState::new(&editform::EMAIL_FOOTER_FORM);
    st.set("company_name", f.company_name.clone());
    st.set("address_lines", f.address_lines.join("\n"));
    st.set("unsubscribe_label", f.unsubscribe_label.clone());
    st.set("unsubscribe_href", f.unsubscribe_href.clone());
    st.set("copyright", opt_get(&f.copyright));
    st.sub_state
        .insert("social".into(), social_items(&f.social));
    st.selected_sub_item.insert("social".into(), 0);
    st
}

fn apply_email_footer(f: &mut EmailFooter, state: &EditFormState) -> Result<()> {
    f.company_name = state.get("company_name").to_string();
    f.address_lines = state
        .get("address_lines")
        .split('\n')
        .map(str::to_string)
        .filter(|l| !l.trim().is_empty())
        .collect();
    f.unsubscribe_label = state.get("unsubscribe_label").to_string();
    f.unsubscribe_href = state.get("unsubscribe_href").to_string();
    f.copyright = opt_set(state, "copyright");
    f.social = parse_social_items(state.sub_state.get("social"))?;
    Ok(())
}

fn social_items(elements: &[MjSocialElement]) -> Vec<EditFormState> {
    let mut items = Vec::new();
    for el in elements {
        let mut item = EditFormState::new(&editform::SOCIAL_ITEM_FORM);
        item.set("name", social_net_str(el.name));
        item.set("href", el.href.clone());
        item.set("src", opt_get(&el.src));
        items.push(item);
    }
    items
}

fn parse_social_items(items: Option<&Vec<EditFormState>>) -> Result<Vec<MjSocialElement>> {
    let mut out = Vec::new();
    if let Some(items) = items {
        for item in items {
            out.push(MjSocialElement {
                name: parse_social_net(item.get("name"))?,
                href: item.get("href").to_string(),
                src: opt_set(item, "src"),
            });
        }
    }
    Ok(out)
}

fn opt_get(v: &Option<String>) -> String {
    v.clone().unwrap_or_default()
}

fn opt_set(state: &EditFormState, id: &str) -> Option<String> {
    let t = state.get(id).trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

fn bool_str(v: bool) -> &'static str {
    if v {
        "true"
    } else {
        "false"
    }
}

fn parse_bool(s: &str) -> Result<bool> {
    match s.trim() {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(anyhow!("expected true/false, got {other}")),
    }
}

fn align_str(a: Option<Align>) -> &'static str {
    match a {
        Some(Align::Left) => "left",
        Some(Align::Center) => "center",
        Some(Align::Right) => "right",
        None => "",
    }
}

fn parse_align(s: &str) -> Result<Option<Align>> {
    match s.trim() {
        "" => Ok(None),
        "left" => Ok(Some(Align::Left)),
        "center" => Ok(Some(Align::Center)),
        "right" => Ok(Some(Align::Right)),
        other => Err(anyhow!("invalid align: {other}")),
    }
}

fn hero_mode_str(m: HeroMode) -> &'static str {
    match m {
        HeroMode::FluidHeight => "fluid-height",
        HeroMode::FixedHeight => "fixed-height",
    }
}

fn parse_hero_mode(s: &str) -> Result<HeroMode> {
    match s.trim() {
        "fluid-height" => Ok(HeroMode::FluidHeight),
        "fixed-height" => Ok(HeroMode::FixedHeight),
        other => Err(anyhow!("invalid hero mode: {other}")),
    }
}

fn social_mode_str(m: SocialMode) -> &'static str {
    match m {
        SocialMode::Horizontal => "horizontal",
        SocialMode::Vertical => "vertical",
    }
}

fn parse_social_mode(s: &str) -> Result<SocialMode> {
    match s.trim() {
        "horizontal" => Ok(SocialMode::Horizontal),
        "vertical" => Ok(SocialMode::Vertical),
        other => Err(anyhow!("invalid social mode: {other}")),
    }
}

fn image_pos_str(p: ImagePosition) -> &'static str {
    match p {
        ImagePosition::Top => "top",
        ImagePosition::Left => "left",
        ImagePosition::Right => "right",
    }
}

fn parse_image_pos(s: &str) -> Result<ImagePosition> {
    match s.trim() {
        "top" => Ok(ImagePosition::Top),
        "left" => Ok(ImagePosition::Left),
        "right" => Ok(ImagePosition::Right),
        other => Err(anyhow!("invalid image_position: {other}")),
    }
}

fn thumbnails_str(t: Thumbnails) -> &'static str {
    match t {
        Thumbnails::Visible => "visible",
        Thumbnails::Hidden => "hidden",
        Thumbnails::Supported => "supported",
    }
}

fn parse_thumbnails(s: &str) -> Result<Thumbnails> {
    match s.trim() {
        "visible" => Ok(Thumbnails::Visible),
        "hidden" => Ok(Thumbnails::Hidden),
        "supported" => Ok(Thumbnails::Supported),
        other => Err(anyhow!("invalid thumbnails: {other}")),
    }
}

fn social_net_str(n: SocialNetwork) -> &'static str {
    match n {
        SocialNetwork::Facebook => "facebook",
        SocialNetwork::Instagram => "instagram",
        SocialNetwork::Linkedin => "linkedin",
        SocialNetwork::X => "x",
        SocialNetwork::Github => "github",
        SocialNetwork::Web => "web",
    }
}

fn parse_social_net(s: &str) -> Result<SocialNetwork> {
    match s.trim() {
        "facebook" => Ok(SocialNetwork::Facebook),
        "instagram" => Ok(SocialNetwork::Instagram),
        "linkedin" => Ok(SocialNetwork::Linkedin),
        "x" => Ok(SocialNetwork::X),
        "github" => Ok(SocialNetwork::Github),
        "web" => Ok(SocialNetwork::Web),
        other => Err(anyhow!("invalid social network: {other}")),
    }
}
