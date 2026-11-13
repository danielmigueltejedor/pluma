//! Icon lookup and drawing. Every icon is Nodalia Design's ([`nodalia_theme::icons`]), drawn in
//! NDL's bold style: SVGs built once from NDL's shapes, rasterized by egui_extras and tinted with
//! theme colours. Pluma's code names an icon by its own name ([`NAMES`] maps it to NDL's).

use std::sync::OnceLock;

use craft_ui::icons::{SvgIconSet, SvgStyle};
use egui::{Color32, ImageSource, Rect, Ui, Vec2};
use nodalia_theme::icons as ndl;

/// Every icon name Pluma uses, sorted, and the NDL icon that draws it.
pub(crate) static NAMES: &[(&str, &str)] = &[
    ("align-center-horizontal", "align-objects-middle"),
    ("align-center-vertical", "align-objects-center"),
    ("align-end-horizontal", "align-objects-bottom"),
    ("align-end-vertical", "align-objects-right"),
    ("align-horizontal-justify-start", "columns"),
    ("align-start-horizontal", "align-objects-top"),
    ("align-start-vertical", "align-objects-left"),
    ("arrow-left-right", "arrows-horizontal"),
    ("arrow-up-down", "arrows-vertical"),
    ("blend", "blend"),
    ("brush", "blob-brush"),
    ("chart-column", "chart"),
    ("chevron-down", "chevron-down"),
    ("chevron-left", "chevron-left"),
    ("chevron-right", "chevron-right"),
    ("chevrons-left", "chevrons-left"),
    ("chevrons-right", "chevrons-right"),
    ("circle", "color-wheel"),
    ("clipboard-paste", "paste"),
    ("cloud", "cloud"),
    ("combine", "merge-cells"),
    ("copy", "copy"),
    ("dc-actions", "actions"),
    ("dc-al-bottom", "align-objects-bottom"),
    ("dc-al-center", "align-objects-both"),
    ("dc-al-hcenter", "align-objects-center"),
    ("dc-al-left", "align-objects-left"),
    ("dc-al-right", "align-objects-right"),
    ("dc-al-top", "align-objects-top"),
    ("dc-al-vcenter", "align-objects-middle"),
    ("dc-align", "align-objects-left"),
    ("dc-alignto-artboard", "align-to-artboard"),
    ("dc-alignto-key", "align-to-key"),
    ("dc-alignto-selection", "align-to-selection"),
    ("dc-anchor", "anchor-point"),
    ("dc-anchor-smooth", "anchor-smooth"),
    ("dc-appearance", "appearance"),
    ("dc-arc", "arc"),
    ("dc-arrow-down", "arrow-down"),
    ("dc-arrow-extend", "arrowhead-extend"),
    ("dc-arrow-tip", "arrowhead-tip"),
    ("dc-arrow-up", "arrow-up"),
    ("dc-artboard-options", "artboard"),
    ("dc-artboards", "artboards"),
    ("dc-blend", "blend"),
    ("dc-bloat", "bloat"),
    ("dc-cap-butt", "cap-butt"),
    ("dc-cap-round", "cap-round"),
    ("dc-cap-square", "cap-square"),
    ("dc-center-hide", "center-hide"),
    ("dc-center-show", "center-show"),
    ("dc-clear", "none"),
    ("dc-color-guide", "color-guide"),
    ("dc-contour", "contour"),
    ("dc-crystallize", "crystallize"),
    ("dc-cube", "wireframe"),
    ("dc-dash-align", "dash-align"),
    ("dc-dash-exact", "dash-exact"),
    ("dc-dimension", "dimension"),
    ("dc-dir-off", "rotate"),
    ("dc-dir-on", "rotate-ccw"),
    ("dc-direct", "select"),
    ("dc-dist-bottom", "distribute-bottom"),
    ("dc-dist-hcenter", "distribute-hcenter"),
    ("dc-dist-hspace", "distribute"),
    ("dc-dist-left", "distribute-left"),
    ("dc-dist-right", "distribute-right"),
    ("dc-dist-top", "distribute-top"),
    ("dc-dist-vcenter", "distribute-vcenter"),
    ("dc-dist-vspace", "distribute-vertical"),
    ("dc-draw-behind", "draw-behind"),
    ("dc-draw-inside", "draw-inside"),
    ("dc-draw-normal", "draw-normal"),
    ("dc-ellipse", "shape-ellipse"),
    ("dc-filter", "filter"),
    ("dc-flare", "flare"),
    ("dc-folder", "folder"),
    ("dc-free-transform", "transform"),
    ("dc-fx", "effects"),
    ("dc-gamut", "warning"),
    ("dc-grad-freeform", "gradient-freeform"),
    ("dc-grad-linear", "gradient"),
    ("dc-grad-radial", "gradient-radial"),
    ("dc-grad-stroke-across", "gradient-stroke-across"),
    ("dc-grad-stroke-along", "gradient-stroke-along"),
    ("dc-grad-stroke-within", "gradient-stroke-within"),
    ("dc-gradient", "gradient"),
    ("dc-graphic-styles", "graphic-styles"),
    ("dc-grid-view", "grid-view"),
    ("dc-group-select", "group-select"),
    ("dc-join", "join"),
    ("dc-join-bevel", "join-bevel"),
    ("dc-join-miter", "join-miter"),
    ("dc-join-round", "join-round"),
    ("dc-knife", "knife"),
    ("dc-line", "line"),
    ("dc-line-cut", "cut-line"),
    ("dc-list-view", "list-view"),
    ("dc-live-bucket", "fill"),
    ("dc-live-select", "live-select"),
    ("dc-mask-none", "none"),
    ("dc-measure", "measure"),
    ("dc-mesh", "mesh-gradient"),
    ("dc-mirror-cut", "cut-mirror"),
    ("dc-new-fill", "fill-add"),
    ("dc-new-item", "new-item"),
    ("dc-new-stroke", "stroke-add"),
    ("dc-objects-on-path", "objects-on-path"),
    ("dc-options", "properties"),
    ("dc-para-bullets", "list"),
    ("dc-para-center", "align-center"),
    ("dc-para-justify-all", "align-justify"),
    ("dc-para-justify-center", "justify-center"),
    ("dc-para-justify-left", "justify-left"),
    ("dc-para-justify-right", "justify-right"),
    ("dc-para-left", "align-left"),
    ("dc-para-ltr", "text-ltr"),
    ("dc-para-numbers", "list-numbers"),
    ("dc-para-right", "align-right"),
    ("dc-para-rtl", "text-rtl"),
    ("dc-path-eraser", "path-eraser"),
    ("dc-pathfinder", "pathfinder"),
    ("dc-perspective", "perspective"),
    ("dc-pf-crop", "pathfinder-crop"),
    ("dc-pf-divide", "pathfinder-divide"),
    ("dc-pf-exclude", "pathfinder-exclude"),
    ("dc-pf-intersect", "pathfinder-intersect"),
    ("dc-pf-merge", "pathfinder-merge"),
    ("dc-pf-minus-back", "pathfinder-minus-back"),
    ("dc-pf-minus-front", "pathfinder-minus-front"),
    ("dc-pf-outline", "pathfinder-outline"),
    ("dc-pf-trim", "pathfinder-trim"),
    ("dc-pf-unite", "pathfinder-unite"),
    ("dc-place-symbol", "symbol"),
    ("dc-polar-grid", "polar-grid"),
    ("dc-polygon", "polygon"),
    ("dc-pucker", "pucker"),
    ("dc-puppet", "puppet-warp"),
    ("dc-rearrange", "rearrange"),
    ("dc-rect-cut", "cut-rect"),
    ("dc-rect-grid", "grid"),
    ("dc-remove-brush", "brush-remove"),
    ("dc-reshape", "reshape"),
    ("dc-reverse", "reverse"),
    ("dc-rotate-view", "rotate-view"),
    ("dc-rounded-rect", "shape-rounded-rectangle"),
    ("dc-rule-evenodd", "fill-rule-evenodd"),
    ("dc-rule-nonzero", "fill-rule-nonzero"),
    ("dc-scallop", "scallop"),
    ("dc-screen-mode", "screen-mode"),
    ("dc-selection", "select"),
    ("dc-shape-builder", "shape-builder"),
    ("dc-shear", "shear"),
    ("dc-smooth", "smooth-path"),
    ("dc-stroke", "stroke-weight"),
    ("dc-stroke-center", "stroke-center"),
    ("dc-stroke-inside", "stroke-inside"),
    ("dc-stroke-outside", "stroke-outside"),
    ("dc-swap", "swap"),
    ("dc-swatch-kinds", "swatches"),
    ("dc-symbol-sprayer", "symbol-sprayer"),
    ("dc-touch-type", "touch-type"),
    ("dc-transform-panel", "transform"),
    ("dc-transparency", "opacity"),
    ("dc-twirl", "twirl"),
    ("dc-type", "text"),
    ("dc-type-area", "text-box"),
    ("dc-type-path", "type-path"),
    ("dc-type-vertical", "type-vertical"),
    ("dc-type-vertical-area", "type-vertical-area"),
    ("dc-type-vertical-path", "type-vertical-path"),
    ("dc-width", "width"),
    ("dc-wrinkle", "wrinkle"),
    ("dc-zoom-large", "mountain"),
    ("dc-zoom-small", "mountain-small"),
    ("ellipsis", "more-horizontal"),
    ("eraser", "eraser"),
    ("external-link", "undock"),
    ("eye", "view"),
    ("eye-off", "eye-off"),
    ("file-plus", "new"),
    ("flip-horizontal-2", "flip-vertical"),
    ("flip-vertical-2", "flip"),
    ("folder-open", "open"),
    ("frame", "clip-mask"),
    ("git-branch", "branch"),
    ("globe", "globe"),
    ("grid-3x3", "grid"),
    ("group", "group"),
    ("hand", "pan"),
    ("hexagon", "polygon"),
    ("history", "history"),
    ("house", "home"),
    ("image", "image"),
    ("info", "info"),
    ("keyboard", "keyboard"),
    ("lasso", "lasso"),
    ("layers", "layers"),
    ("layout-grid", "guides"),
    ("library", "library"),
    ("link", "link"),
    ("link-2-off", "link-off"),
    ("lock", "lock"),
    ("lock-open", "unlock"),
    ("log-out", "log-out"),
    ("map", "navigator"),
    ("maximize-2", "fullscreen"),
    ("menu", "menu"),
    ("minus", "minus"),
    ("monitor", "monitor"),
    ("move", "move"),
    ("move-diagonal-2", "resize"),
    ("paint-bucket", "fill"),
    ("paintbrush", "brush"),
    ("palette", "palette"),
    ("pen-tool", "pen"),
    ("pen-tool-add", "pen-add"),
    ("pen-tool-delete", "pen-remove"),
    ("pencil", "pencil"),
    ("pilcrow", "paragraph"),
    ("pin", "pin"),
    ("pipette", "eyedropper"),
    ("plus", "plus"),
    ("printer", "print"),
    ("redo-2", "redo"),
    ("rotate-ccw", "rotate"),
    ("rotate-cw", "refresh"),
    ("ruler", "ruler"),
    ("save", "save"),
    ("scaling", "scale"),
    ("scissors", "cut"),
    ("search", "search"),
    ("settings", "settings"),
    ("shapes", "shapes"),
    ("share-2", "share"),
    ("slice", "web-slice"),
    ("sparkles", "sparkle"),
    ("spline", "curvature"),
    ("spline-pointer", "code"),
    ("spray-can", "symbol-sprayer"),
    ("square", "shape-rectangle"),
    ("square-dashed", "marquee"),
    ("squares-exclude", "pathfinder-exclude"),
    ("squares-intersect", "pathfinder-intersect"),
    ("squares-subtract", "pathfinder-minus-front"),
    ("squares-unite", "pathfinder-unite"),
    ("star", "star"),
    ("sun", "sun"),
    ("swatch-book", "swatches"),
    ("text-cursor-input", "glyphs"),
    ("tornado", "spiral"),
    ("trash-2", "delete"),
    ("triangle", "triangle"),
    ("type", "text"),
    ("undo-2", "undo"),
    ("ungroup", "ungroup"),
    ("wand-sparkles", "magic-wand"),
    ("waves", "warp"),
    ("x", "close"),
    ("zoom-in", "zoom"),
];

/// The Selection tool's arrow is solid and Direct Selection's hollow, as users know them.
const SOLID: &[&str] = &["dc-selection"];

/// The NDL icon behind Pluma's `name`.
pub fn ndl_name(name: &str) -> Option<&'static str> {
    NAMES.binary_search_by(|(n, _)| (*n).cmp(name)).ok().and_then(|i| NAMES.get(i)).map(|(_, ndl)| *ndl)
}

/// A standalone SVG of `icon` in NDL's bold style. `solid` fills its outline.
fn svg(icon: &ndl::Icon, solid: bool) -> String {
    let mut body = String::new();
    for part in icon.bold_body() {
        if solid && !part.contains("fill=") {
            body.push_str(&part.replacen("<path ", "<path fill=\"currentColor\" ", 1));
        } else {
            body.push_str(part);
        }
    }
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"{}\" stroke-linecap=\"round\" stroke-linejoin=\"round\">{body}</svg>",
        ndl::BOLD_STROKE
    )
}

fn table() -> &'static SvgIconSet {
    static T: OnceLock<SvgIconSet> = OnceLock::new();
    T.get_or_init(|| {
        // Built once for the life of the app; the set keeps borrowed bytes, so they are leaked.
        let icons: Vec<(&'static str, &'static [u8])> = NAMES
            .iter()
            .filter_map(|(name, ndl_name)| {
                let icon = ndl::get(ndl_name)?;
                let bytes: &'static [u8] = Box::leak(svg(icon, SOLID.contains(name)).into_bytes().into_boxed_slice());
                Some((*name, bytes))
            })
            .collect();
        SvgIconSet::new(
            Box::leak(icons.into_boxed_slice()),
            SvgStyle { current_color: "white", stroke_width: None },
            "bytes://ndl-icon/",
            "square-dashed",
        )
    })
}

pub fn exists(name: &str) -> bool {
    table().contains(name)
}

/// Map tool-catalogue icon names to icon names.
pub fn tool_icon(name: &str) -> &'static str {
    match name {
        "tool-selection" => "dc-selection",
        "tool-direct" => "dc-direct",
        "tool-group-select" => "dc-group-select",
        "tool-magic-wand" => "wand-sparkles",
        "tool-lasso" => "lasso",
        "tool-pen" => "pen-tool",
        "tool-pen-add" => "pen-tool-add",
        "tool-pen-delete" => "pen-tool-delete",
        "tool-anchor" => "dc-anchor",
        "tool-curvature" => "spline",
        "tool-type" => "dc-type",
        "tool-type-area" => "dc-type-area",
        "tool-type-path" => "dc-type-path",
        "tool-type-vertical" => "dc-type-vertical",
        "tool-type-vertical-area" => "dc-type-vertical-area",
        "tool-type-vertical-path" => "dc-type-vertical-path",
        "tool-touch-type" => "dc-touch-type",
        "tool-line" => "dc-line",
        "tool-arc" => "dc-arc",
        "tool-spiral" => "tornado",
        "tool-rect-grid" => "dc-rect-grid",
        "tool-polar-grid" => "dc-polar-grid",
        "tool-rect" => "square",
        "tool-rounded-rect" => "dc-rounded-rect",
        "tool-ellipse" => "dc-ellipse",
        "tool-polygon" => "dc-polygon",
        "tool-star" => "star",
        "tool-flare" => "dc-flare",
        "tool-brush" => "paintbrush",
        "tool-blob-brush" => "brush",
        "tool-shaper" => "shapes",
        "tool-pencil" => "pencil",
        "tool-smooth" => "dc-smooth",
        "tool-path-eraser" => "dc-path-eraser",
        "tool-join" => "dc-join",
        "tool-eraser" => "eraser",
        "tool-scissors" => "scissors",
        "tool-knife" => "dc-knife",
        "tool-mirror-cut" => "dc-mirror-cut",
        "tool-line-cut" => "dc-line-cut",
        "tool-rect-cut" => "dc-rect-cut",
        "tool-rotate" => "rotate-ccw",
        "tool-reflect" => "flip-vertical-2",
        "tool-scale" => "scaling",
        "tool-shear" => "dc-shear",
        "tool-reshape" => "dc-reshape",
        "tool-width" => "dc-width",
        "tool-warp" => "waves",
        "tool-twirl" => "dc-twirl",
        "tool-pucker" => "dc-pucker",
        "tool-bloat" => "dc-bloat",
        "tool-scallop" => "dc-scallop",
        "tool-crystallize" => "dc-crystallize",
        "tool-wrinkle" => "dc-wrinkle",
        "tool-free-transform" => "dc-free-transform",
        "tool-puppet" => "dc-puppet",
        "tool-shape-builder" => "dc-shape-builder",
        "tool-bucket" => "dc-live-bucket",
        "tool-live-select" => "dc-live-select",
        "tool-perspective" => "dc-perspective",
        "tool-perspective-select" => "dc-perspective",
        "tool-mesh" => "dc-mesh",
        "tool-gradient" => "dc-gradient",
        "tool-eyedropper" => "pipette",
        "tool-measure" => "dc-measure",
        "tool-dimension" => "dc-dimension",
        "tool-contour" => "dc-contour",
        "tool-blend" => "dc-blend",
        "tool-objects-on-path" => "dc-objects-on-path",
        "tool-symbol" => "dc-symbol-sprayer",
        "tool-graph" => "chart-column",
        "tool-artboard" => "dc-artboard-options",
        "tool-slice" => "slice",
        "tool-hand" => "hand",
        "tool-rotate-view" => "dc-rotate-view",
        "tool-print-tiling" => "printer",
        "tool-zoom" => "zoom-in",
        other => {
            if exists(other) {
                table().canonical_name(other).unwrap_or("square-dashed")
            } else {
                "square-dashed"
            }
        }
    }
}

pub fn source(name: &str) -> ImageSource<'static> {
    table().source(name)
}

/// Paint icon `name` into `rect` tinted with `tint`.
pub fn paint(ui: &Ui, name: &str, rect: Rect, tint: Color32) {
    egui::Image::new(source(name)).tint(tint).fit_to_exact_size(rect.size()).paint_at(ui, rect);
}

/// An icon widget of `size` points.
pub fn icon(ui: &mut Ui, name: &str, size: f32, tint: Color32) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::hover());
    paint(ui, name, rect, tint);
    resp
}

#[cfg(test)]
mod tests {
    use super::*;

    fn svg_of(name: &str) -> String {
        let ImageSource::Bytes { bytes, .. } = source(name) else { panic!("expected SVG bytes") };
        String::from_utf8_lossy(bytes.as_ref()).into_owned()
    }

    /// Every icon Pluma names is drawn by NDL, in its bold style, tinted at draw time.
    #[test]
    fn every_icon_is_an_ndl_icon_in_the_bold_style() {
        assert!(NAMES.windows(2).all(|w| w[0].0 < w[1].0), "NAMES is sorted and unique");
        for (name, ndl_name) in NAMES {
            assert!(ndl::get(ndl_name).is_some(), "{name} maps to {ndl_name}, which NDL does not draw");
            assert!(exists(name), "{name}");
            let ImageSource::Bytes { uri, .. } = source(name) else { panic!("expected SVG bytes") };
            assert_eq!(uri.as_ref(), format!("bytes://ndl-icon/{name}.svg"));
            let svg = svg_of(name);
            assert!(svg.contains(&format!("stroke-width=\"{}\"", ndl::BOLD_STROKE)) && !svg.contains("currentColor"), "{name}");
            assert_eq!(tool_icon(name), *name);
        }
    }

    /// Selection's arrow is solid and Direct Selection's hollow: the same NDL arrow, so the two
    /// read as one family and differ only by the fill.
    #[test]
    fn the_selection_arrow_is_solid_and_direct_selection_hollow() {
        assert_eq!(ndl_name("dc-selection"), ndl_name("dc-direct"));
        assert!(svg_of("dc-selection").contains("<path fill=\"white\""));
        assert!(!svg_of("dc-direct").contains("fill=\"white\""));
    }

    /// Appearance (effects) and Pathfinder both lived in the icon column with the same motif
    /// (storytold/vectorcraft#2681): they stay different icons.
    #[test]
    fn appearance_panel_icon_is_not_the_pathfinder_icon() {
        assert_ne!(ndl_name("dc-appearance"), ndl_name("dc-pathfinder"));
    }

    // Regression test for #479: the Add and Delete Anchor Point tools' icons read as one family:
    // the same pen, with a plus or a minus beside it.
    #[test]
    fn the_add_and_delete_anchor_tool_icons_are_one_pen_with_a_badge() {
        let body = |tool: &str| ndl::get(ndl_name(tool_icon(tool)).unwrap_or_default()).map(|i| i.body).unwrap_or_default();
        let (add, delete) = (body("tool-pen-add"), body("tool-pen-delete"));
        assert!(!delete.is_empty());
        assert_eq!(add[..delete.len() - 1], delete[..delete.len() - 1], "the same pen");
        assert_ne!(add, delete, "plus and minus differ");
    }

    /// The seven type tools each have an icon of their own.
    #[test]
    fn every_type_tool_has_its_own_icon() {
        let tools = ["type", "areaType", "typeOnPath", "verticalType", "verticalAreaType", "verticalTypeOnPath", "touchType"];
        let names: Vec<_> = tools.iter().map(|id| ndl_name(tool_icon(pluma_tools::catalog::tool_info(id).unwrap().icon))).collect();
        let unique: std::collections::BTreeSet<_> = names.iter().collect();
        assert_eq!(unique.len(), tools.len(), "{names:?}");
        assert!(names.iter().all(Option::is_some), "{names:?}");
    }

    #[test]
    fn missing_icons_fall_back_and_tool_aliases_resolve() {
        let ImageSource::Bytes { uri, bytes } = source("missing-icon") else { panic!("expected SVG bytes") };
        let ImageSource::Bytes { bytes: fallback, .. } = source("square-dashed") else { panic!("expected SVG bytes") };
        assert_eq!(uri.as_ref(), "bytes://ndl-icon/missing-icon.svg");
        assert_eq!(bytes.as_ref(), fallback.as_ref());
        assert!(!exists("missing-icon"));
        assert_eq!(tool_icon("missing-icon"), "square-dashed");
        assert_eq!(tool_icon("tool-selection"), "dc-selection");
    }
}
