//! Static FormEdit maps. Field ids match JSON / model names.
use super::*;

const fn f(id: &'static str, label: &'static str, kind: FieldKind, required: bool) -> FormField {
    FormField {
        id,
        label,
        kind,
        required,
        visible_when: None,
    }
}

pub const ALIGN_OPTIONS: &[&str] = &["", "left", "center", "right"];
pub const BOOL_OPTIONS: &[&str] = &["false", "true"];
pub const HERO_MODE_OPTIONS: &[&str] = &["fluid-height", "fixed-height"];
pub const SOCIAL_MODE_OPTIONS: &[&str] = &["horizontal", "vertical"];
pub const IMAGE_POS_OPTIONS: &[&str] = &["top", "left", "right"];
pub const SOCIAL_NET_OPTIONS: &[&str] =
    &["facebook", "instagram", "linkedin", "x", "github", "web"];
pub const THUMBNAILS_OPTIONS: &[&str] = &["hidden", "visible", "supported"];

pub static FONT_ITEM_FORM: EditForm = EditForm {
    title: "font",
    fields: &[
        f("name", "Name", FieldKind::Text { default: "Raleway" }, true),
        f(
            "href",
            "Google Fonts href",
            FieldKind::Url {
                default:
                    "https://fonts.googleapis.com/css2?family=Raleway:wght@400;700&display=swap",
            },
            true,
        ),
    ],
};

pub static SOCIAL_ITEM_FORM: EditForm = EditForm {
    title: "social icon",
    fields: &[
        f(
            "name",
            "Network",
            FieldKind::Enum {
                options: SOCIAL_NET_OPTIONS,
                default: "x",
            },
            true,
        ),
        f(
            "href",
            "Href",
            FieldKind::Url {
                default: "https://example.com",
            },
            true,
        ),
        f(
            "src",
            "Icon src (web)",
            FieldKind::Url { default: "" },
            false,
        ),
    ],
};

pub static HEAD_FORM: EditForm = EditForm {
    title: "mj-head",
    fields: &[
        f("subject", "Subject", FieldKind::Text { default: "" }, true),
        f(
            "preheader",
            "Preheader",
            FieldKind::Text { default: "" },
            false,
        ),
        f("lang", "Lang", FieldKind::Text { default: "en" }, true),
        f("title", "Title", FieldKind::Text { default: "" }, true),
        f(
            "breakpoint",
            "Breakpoint",
            FieldKind::Text { default: "480px" },
            false,
        ),
        f(
            "base_url",
            "Base URL",
            FieldKind::Url { default: "" },
            false,
        ),
        f(
            "fonts",
            "Fonts",
            FieldKind::SubForm {
                template: &FONT_ITEM_FORM,
                min_items: 0,
                summary_field_id: "name",
            },
            false,
        ),
        f(
            "json_ld",
            "JSON-LD",
            FieldKind::Textarea {
                rows: 6,
                default: "",
            },
            false,
        ),
        f(
            "css",
            "Custom CSS",
            FieldKind::Textarea {
                rows: 6,
                default: "",
            },
            false,
        ),
        f(
            "css_inline",
            "Inline CSS",
            FieldKind::Enum {
                options: BOOL_OPTIONS,
                default: "false",
            },
            false,
        ),
    ],
};

pub static BRAND_FORM: EditForm = EditForm {
    title: "brand",
    fields: &[
        f(
            "font_family",
            "Font family",
            FieldKind::Text { default: "" },
            true,
        ),
        f(
            "text_color",
            "Text color",
            FieldKind::Text { default: "#1a1a1a" },
            true,
        ),
        f(
            "background_color",
            "Background",
            FieldKind::Text { default: "#f4f4f4" },
            true,
        ),
        f(
            "content_width",
            "Content width",
            FieldKind::Text { default: "600" },
            true,
        ),
        f(
            "button_background",
            "Button background",
            FieldKind::Text { default: "#FFAF46" },
            true,
        ),
        f(
            "button_color",
            "Button color",
            FieldKind::Text { default: "#0F1114" },
            true,
        ),
    ],
};

pub static BODY_FORM: EditForm = EditForm {
    title: "mj-body",
    fields: &[f(
        "background_color",
        "Background",
        FieldKind::Text { default: "" },
        false,
    )],
};

pub static SECTION_FORM: EditForm = EditForm {
    title: "mj-section",
    fields: &[
        f(
            "background_color",
            "Background",
            FieldKind::Text { default: "" },
            false,
        ),
        f("padding", "Padding", FieldKind::Text { default: "" }, false),
        f(
            "full_width",
            "Full width",
            FieldKind::Enum {
                options: BOOL_OPTIONS,
                default: "false",
            },
            false,
        ),
    ],
};

pub static COLUMN_FORM: EditForm = EditForm {
    title: "mj-column",
    fields: &[
        f("width", "Width", FieldKind::Text { default: "" }, false),
        f(
            "background_color",
            "Background",
            FieldKind::Text { default: "" },
            false,
        ),
        f("padding", "Padding", FieldKind::Text { default: "" }, false),
        f(
            "inner_background_color",
            "Inner background",
            FieldKind::Text { default: "" },
            false,
        ),
    ],
};

pub static WRAPPER_FORM: EditForm = EditForm {
    title: "mj-wrapper",
    fields: &[
        f(
            "background_color",
            "Background",
            FieldKind::Text { default: "" },
            false,
        ),
        f("padding", "Padding", FieldKind::Text { default: "" }, false),
        f(
            "full_width",
            "Full width",
            FieldKind::Enum {
                options: BOOL_OPTIONS,
                default: "false",
            },
            false,
        ),
    ],
};

pub static GROUP_FORM: EditForm = EditForm {
    title: "mj-group",
    fields: &[
        f("width", "Width", FieldKind::Text { default: "" }, false),
        f(
            "background_color",
            "Background",
            FieldKind::Text { default: "" },
            false,
        ),
    ],
};

pub static HERO_FORM: EditForm = EditForm {
    title: "mj-hero",
    fields: &[
        f(
            "mode",
            "Mode",
            FieldKind::Enum {
                options: HERO_MODE_OPTIONS,
                default: "fluid-height",
            },
            true,
        ),
        f(
            "background_url",
            "Background URL",
            FieldKind::Url { default: "" },
            false,
        ),
        f(
            "background_color",
            "Background",
            FieldKind::Text { default: "" },
            false,
        ),
        f(
            "background_height",
            "Background height",
            FieldKind::Text { default: "" },
            false,
        ),
        f("width", "Width", FieldKind::Text { default: "" }, false),
        f("height", "Height", FieldKind::Text { default: "" }, false),
    ],
};

pub static TEXT_FORM: EditForm = EditForm {
    title: "mj-text",
    fields: &[
        f(
            "content",
            "Content",
            FieldKind::Textarea {
                rows: 6,
                default: "",
            },
            true,
        ),
        f(
            "align",
            "Align",
            FieldKind::Enum {
                options: ALIGN_OPTIONS,
                default: "",
            },
            false,
        ),
        f(
            "font_size",
            "Font size",
            FieldKind::Text { default: "" },
            false,
        ),
        f(
            "font_family",
            "Font family",
            FieldKind::Text { default: "" },
            false,
        ),
        f("color", "Color", FieldKind::Text { default: "" }, false),
        f("padding", "Padding", FieldKind::Text { default: "" }, false),
    ],
};

pub static BUTTON_FORM: EditForm = EditForm {
    title: "mj-button",
    fields: &[
        f(
            "content",
            "Label",
            FieldKind::Text {
                default: "Read more",
            },
            true,
        ),
        f(
            "href",
            "Href",
            FieldKind::Url {
                default: "https://example.com",
            },
            true,
        ),
        f(
            "background_color",
            "Background",
            FieldKind::Text { default: "" },
            false,
        ),
        f("color", "Color", FieldKind::Text { default: "" }, false),
        f(
            "align",
            "Align",
            FieldKind::Enum {
                options: ALIGN_OPTIONS,
                default: "",
            },
            false,
        ),
        f(
            "font_family",
            "Font family",
            FieldKind::Text { default: "" },
            false,
        ),
        f(
            "border_radius",
            "Border radius",
            FieldKind::Text { default: "" },
            false,
        ),
        f("width", "Width", FieldKind::Text { default: "" }, false),
        f("padding", "Padding", FieldKind::Text { default: "" }, false),
    ],
};

pub static IMAGE_FORM: EditForm = EditForm {
    title: "mj-image",
    fields: &[
        f("src", "Src", FieldKind::Url { default: "" }, true),
        f("alt", "Alt", FieldKind::Text { default: "" }, true),
        f("href", "Href", FieldKind::Url { default: "" }, false),
        f("width", "Width", FieldKind::Text { default: "" }, false),
        f(
            "align",
            "Align",
            FieldKind::Enum {
                options: ALIGN_OPTIONS,
                default: "",
            },
            false,
        ),
        f(
            "fluid_on_mobile",
            "Fluid on mobile",
            FieldKind::Enum {
                options: BOOL_OPTIONS,
                default: "true",
            },
            false,
        ),
        f("padding", "Padding", FieldKind::Text { default: "" }, false),
    ],
};

pub static DIVIDER_FORM: EditForm = EditForm {
    title: "mj-divider",
    fields: &[
        f(
            "border_color",
            "Border color",
            FieldKind::Text { default: "" },
            false,
        ),
        f(
            "border_width",
            "Border width",
            FieldKind::Text { default: "" },
            false,
        ),
        f("padding", "Padding", FieldKind::Text { default: "" }, false),
    ],
};

pub static SPACER_FORM: EditForm = EditForm {
    title: "mj-spacer",
    fields: &[f(
        "height",
        "Height",
        FieldKind::Text { default: "24px" },
        true,
    )],
};

pub static SOCIAL_FORM: EditForm = EditForm {
    title: "mj-social",
    fields: &[
        f(
            "mode",
            "Mode",
            FieldKind::Enum {
                options: SOCIAL_MODE_OPTIONS,
                default: "horizontal",
            },
            true,
        ),
        f(
            "align",
            "Align",
            FieldKind::Enum {
                options: ALIGN_OPTIONS,
                default: "",
            },
            false,
        ),
        f(
            "icon_size",
            "Icon size",
            FieldKind::Text { default: "32px" },
            false,
        ),
        f(
            "elements",
            "Icons",
            FieldKind::SubForm {
                template: &SOCIAL_ITEM_FORM,
                min_items: 0,
                summary_field_id: "name",
            },
            false,
        ),
    ],
};

pub static NAVBAR_FORM: EditForm = EditForm {
    title: "mj-navbar",
    fields: &[
        f(
            "hamburger",
            "Hamburger",
            FieldKind::Enum {
                options: BOOL_OPTIONS,
                default: "false",
            },
            false,
        ),
        f(
            "ico_color",
            "Icon color",
            FieldKind::Text { default: "" },
            false,
        ),
        f(
            "base_url",
            "Base URL",
            FieldKind::Url { default: "" },
            false,
        ),
        f(
            "align",
            "Align",
            FieldKind::Enum {
                options: ALIGN_OPTIONS,
                default: "",
            },
            false,
        ),
        f("padding", "Padding", FieldKind::Text { default: "" }, false),
    ],
};

pub static NAVBAR_LINK_FORM: EditForm = EditForm {
    title: "mj-navbar-link",
    fields: &[
        f(
            "href",
            "Href",
            FieldKind::Url {
                default: "https://example.com",
            },
            true,
        ),
        f(
            "content",
            "Label",
            FieldKind::Text { default: "Link" },
            true,
        ),
        f("color", "Color", FieldKind::Text { default: "" }, false),
        f("padding", "Padding", FieldKind::Text { default: "" }, false),
    ],
};

pub static ACCORDION_FORM: EditForm = EditForm {
    title: "mj-accordion",
    fields: &[
        f("border", "Border", FieldKind::Text { default: "" }, false),
        f("padding", "Padding", FieldKind::Text { default: "" }, false),
    ],
};

pub static ACCORDION_ELEMENT_FORM: EditForm = EditForm {
    title: "mj-accordion-element",
    fields: &[
        f("title", "Title", FieldKind::Text { default: "Title" }, true),
        f(
            "content",
            "Content",
            FieldKind::Textarea {
                rows: 6,
                default: "",
            },
            false,
        ),
        f(
            "background_color",
            "Background",
            FieldKind::Text { default: "" },
            false,
        ),
    ],
};

pub static CAROUSEL_FORM: EditForm = EditForm {
    title: "mj-carousel",
    fields: &[
        f(
            "align",
            "Align",
            FieldKind::Enum {
                options: ALIGN_OPTIONS,
                default: "",
            },
            false,
        ),
        f("padding", "Padding", FieldKind::Text { default: "" }, false),
        f(
            "thumbnails",
            "Thumbnails",
            FieldKind::Enum {
                options: THUMBNAILS_OPTIONS,
                default: "hidden",
            },
            true,
        ),
    ],
};

pub static CAROUSEL_IMAGE_FORM: EditForm = EditForm {
    title: "mj-carousel-image",
    fields: &[
        f("src", "Src", FieldKind::Url { default: "" }, true),
        f("alt", "Alt", FieldKind::Text { default: "" }, true),
        f("href", "Href", FieldKind::Url { default: "" }, false),
        f(
            "thumbnails_src",
            "Thumbnail src",
            FieldKind::Url { default: "" },
            false,
        ),
    ],
};

pub static TABLE_FORM: EditForm = EditForm {
    title: "mj-table",
    fields: &[
        f(
            "content",
            "Table HTML",
            FieldKind::Textarea {
                rows: 8,
                default: "",
            },
            true,
        ),
        f(
            "font_size",
            "Font size",
            FieldKind::Text { default: "" },
            false,
        ),
        f("color", "Color", FieldKind::Text { default: "" }, false),
        f("padding", "Padding", FieldKind::Text { default: "" }, false),
    ],
};

pub static EMAIL_HEADER_FORM: EditForm = EditForm {
    title: "email-header",
    fields: &[
        f(
            "logo_src",
            "Logo src",
            FieldKind::Url { default: "" },
            false,
        ),
        f(
            "logo_alt",
            "Logo alt",
            FieldKind::Text { default: "" },
            false,
        ),
        f(
            "logo_href",
            "Logo href",
            FieldKind::Url { default: "" },
            false,
        ),
        f(
            "logo_width",
            "Logo width",
            FieldKind::Text { default: "160px" },
            false,
        ),
        f(
            "background_color",
            "Background",
            FieldKind::Text { default: "" },
            false,
        ),
    ],
};

pub static EMAIL_HERO_FORM: EditForm = EditForm {
    title: "email-hero",
    fields: &[
        f(
            "image_src",
            "Image src",
            FieldKind::Url { default: "" },
            false,
        ),
        f(
            "image_alt",
            "Image alt",
            FieldKind::Text { default: "" },
            false,
        ),
        f("heading", "Heading", FieldKind::Text { default: "" }, false),
        f(
            "subheading",
            "Subheading",
            FieldKind::Text { default: "" },
            false,
        ),
        f(
            "background_color",
            "Background",
            FieldKind::Text { default: "" },
            false,
        ),
    ],
};

pub static EMAIL_CTA_FORM: EditForm = EditForm {
    title: "email-cta",
    fields: &[
        f("heading", "Heading", FieldKind::Text { default: "" }, false),
        f(
            "copy",
            "Copy",
            FieldKind::Textarea {
                rows: 4,
                default: "",
            },
            false,
        ),
        f(
            "button_label",
            "Button label",
            FieldKind::Text { default: "" },
            false,
        ),
        f(
            "button_href",
            "Button href",
            FieldKind::Url { default: "" },
            false,
        ),
        f(
            "background_color",
            "Background",
            FieldKind::Text { default: "" },
            false,
        ),
    ],
};

pub static EMAIL_ARTICLE_FORM: EditForm = EditForm {
    title: "email-article",
    fields: &[
        f(
            "image_src",
            "Image src",
            FieldKind::Url { default: "" },
            false,
        ),
        f(
            "image_alt",
            "Image alt",
            FieldKind::Text { default: "" },
            false,
        ),
        f("title", "Title", FieldKind::Text { default: "" }, false),
        f(
            "copy",
            "Copy",
            FieldKind::Textarea {
                rows: 4,
                default: "",
            },
            false,
        ),
        f(
            "link_label",
            "Link label",
            FieldKind::Text { default: "" },
            false,
        ),
        f(
            "link_href",
            "Link href",
            FieldKind::Url { default: "" },
            false,
        ),
        f(
            "image_position",
            "Image position",
            FieldKind::Enum {
                options: IMAGE_POS_OPTIONS,
                default: "top",
            },
            false,
        ),
    ],
};

pub static EMAIL_FOOTER_FORM: EditForm = EditForm {
    title: "email-footer",
    fields: &[
        f(
            "company_name",
            "Company",
            FieldKind::Text { default: "" },
            false,
        ),
        f(
            "address_lines",
            "Address (one line per row)",
            FieldKind::Textarea {
                rows: 4,
                default: "",
            },
            true,
        ),
        f(
            "unsubscribe_label",
            "Unsubscribe label",
            FieldKind::Text { default: "" },
            false,
        ),
        f(
            "unsubscribe_href",
            "Unsubscribe href",
            FieldKind::Url { default: "" },
            false,
        ),
        f(
            "copyright",
            "Copyright",
            FieldKind::Text { default: "" },
            false,
        ),
        f(
            "social",
            "Social",
            FieldKind::SubForm {
                template: &SOCIAL_ITEM_FORM,
                min_items: 0,
                summary_field_id: "name",
            },
            false,
        ),
    ],
};
