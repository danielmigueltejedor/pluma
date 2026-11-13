//! Window chrome: application bar (menus), Control bar, document tabs, status bar.

use std::borrow::Cow;
use std::sync::Arc;

use egui::{CornerRadius, Sense, Stroke, Ui, vec2};
use pluma_doc::NodeKind;
use serde_json::json;

use crate::panels::stroke as stroke_panel;
use crate::state::{ROTATE_VIEW_ANGLES, ZOOM_STOPS, zoom_label};
use crate::theme::{self, Tokens};
use crate::widgets;
use crate::{PlumaApp, icons, menus, titlebar};

mod control_bar;
mod hints;
mod status_bar;
mod tabs;

#[cfg(test)]
use control_bar::ISOLATION_BUTTON;
pub use control_bar::{AnchorControls, CONTROL_BAR_EDGES, anchor_buttons, anchor_controls, control_bar};
#[cfg(test)]
use hints::hint_segments;
pub use hints::{hint_bar, hint_strings};
pub use status_bar::status_bar;
#[cfg(test)]
use status_bar::{StatusReadoutWidths, status_readouts};
#[cfg(test)]
use tabs::{CLOSE_BEFORE_TITLE, first_tab_shown, tab_drop, tab_groups, tab_title};
pub use tabs::{doc_tabs, isolation_bar, step_document};
pub(crate) use tabs::{document_window_title, show_document, window_tab};

/// The application bar's height, in points, in the Modern and Classic schemes.
pub const APP_BAR_HEIGHT: f32 = 32.0;
/// The application bar's height in the Nodalia scheme: room for its 28 pt round controls.
pub const NDL_APP_BAR_HEIGHT: f32 = 44.0;

/// The application bar's height with the scheme in force in `ctx`.
pub fn app_bar_height(ctx: &egui::Context) -> f32 {
    if Tokens::get(ctx).nodalia { NDL_APP_BAR_HEIGHT } else { APP_BAR_HEIGHT }
}

/// Nodalia's main menu: a 28 pt hamburger with the app's own entries (About, Preferences,
/// Keyboard Shortcuts, Quit), each with its icon and shortcut. The work menus stay beside it.
fn main_menu(app: &mut PlumaApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let button = widgets::icon_button(ui, "menu", tl!("Main Menu"), false, 28.0);
    const ENTRIES: [(&str, &str, &str); 4] = [
        ("info", "About Pluma", "help.about"),
        ("settings", "Preferences…", "edit.preferences"),
        ("keyboard", "Keyboard Shortcuts…", "edit.keyboardShortcuts"),
        ("log-out", "Quit", "app.quit"),
    ];
    let mut chosen = None;
    egui::Popup::menu(&button).show(|ui| {
        ui.set_min_width(220.0);
        for (i, (icon, label, id)) in ENTRIES.iter().enumerate() {
            if i == 3 {
                ui.separator();
            }
            let shortcut = menus::shortcut_of(id).map(menus::shown_shortcut).unwrap_or_default();
            let image = egui::Image::new(icons::source(icon)).fit_to_exact_size(vec2(16.0, 16.0)).tint(t.icon);
            if ui.add(egui::Button::image_and_text(image, tl!(label)).shortcut_text(shortcut)).clicked() {
                chosen = Some(*id);
                ui.close();
            }
        }
    });
    if let Some(id) = chosen {
        menus::invoke(app, id, json!({}));
    }
}
/// The brand mark's side.
const MARK: f32 = 18.0;
/// Height of the workspace switcher.
const WIDGET_H: f32 = 22.0;

/// The application bar: brand mark, Home, menus, then search and the workspace switcher
/// at the right. With [`PlumaApp::custom_titlebar`] it is also the window's title bar
/// ([`titlebar`]): the caption buttons take the right end and the rest of the bar drags the window.
/// On macOS, under the system's transparent title bar, its empty space drags and zooms the window
/// too ([`titlebar::drag_area`]). With the system title bar (Windows and Linux, `system_title_bar`) the OS draws its own icon and
/// title, so the in-app bar shows Home, menus and workspace controls only: no brand mark.
pub fn app_bar(app: &mut PlumaApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let custom = app.custom_titlebar;
    let system = system_title_bar(app);
    let left = app.titlebar_inset.clamp(8.0, f32::from(i8::MAX)) as i8;
    let frame = egui::Frame::NONE.fill(t.app_bar).inner_margin(egui::Margin { left, right: if custom { 0 } else { 14 }, top: 0, bottom: 0 });
    // Nodalia's bar is the window's frame, around the islands: no line under it.
    let frame = if t.nodalia { frame } else { frame.stroke(Stroke::new(1.0, t.border)) };
    let bar = egui::Panel::top("app_bar").exact_size(app_bar_height(ui.ctx())).show_separator_line(!t.nodalia).frame(frame).show(ui, |ui| {
        // macOS draws the bar under its transparent title bar, so the bar gets the title bar's
        // clicks: without this a double-click there zooms nothing (#1932).
        if custom || cfg!(target_os = "macos") {
            titlebar::drag_area(ui, ui.max_rect());
        }
        ui.horizontal_centered(|ui| {
            // Nodalia: the app's own menu opens from a hamburger at the start of the bar. Otherwise
            // the brand mark, unless the OS draws its own icon in the system title bar.
            if t.nodalia {
                main_menu(app, ui);
                ui.add_space(2.0);
            } else if !system {
                let (r, _) = ui.allocate_exact_size(vec2(MARK, MARK), Sense::hover());
                crate::brand::paint_mark(ui, r);
                ui.add_space(6.0);
            }
            let on_home = menus::home_showing(app);
            if widgets::icon_button(ui, "house", tl!("Home"), on_home, 20.0).clicked() {
                app.run("app.home", json!({})).ok();
            }
            ui.add_space(2.0);
            // With the macOS menu bar the menus are at the top of the screen instead.
            let menus_end = if app.menus_outside_window() { ui.cursor().min.x } else { menus::menu_bar(app, ui) };
            // The right-side group fills the space after the menus from the right; when it runs
            // short, the search icon goes first (the palette
            // stays under its shortcut), then the workspace switcher narrows.
            let full = ui.max_rect();
            let right_edge = if custom { full.right() - titlebar::width(&t) - 10.0 } else { full.right() };
            let right = egui::Rect::from_min_max(egui::pos2(menus_end + 8.0, full.top()), egui::pos2(right_edge, full.bottom()));
            // In Safe Mode, a label that stays in the bar whatever document is open, and opens
            // Help › Safe Mode; the other controls fit in the room it leaves, and the workspace
            // switcher (also under Window › Workspace) goes when a minimal one doesn't fit.
            let safe = pluma_plugins::safe_mode::active().then(|| egui::RichText::new(tl!("Safe Mode")).color(t.warning).size(12.0));
            let safe_w = safe.as_ref().map_or(0.0, |label| {
                let text = ui.painter().layout_no_wrap(label.text().to_string(), egui::FontId::proportional(12.0), t.warning);
                text.size().x + 2.0 * ui.spacing().button_padding.x + 8.0 + ui.spacing().item_spacing.x
            });
            let room = (right.width() - safe_w).max(0.0);
            let switcher = safe.is_none() || room >= 64.0 + 8.0;
            // Built-in workspace names translate; the user's own stay as typed.
            let ws_name =
                if crate::workspaces::is_builtin(&app.ui.workspace) { tl!(&app.ui.workspace).to_string() } else { app.ui.workspace.clone() };
            let ws = ui.painter().layout_no_wrap(ws_name, egui::FontId::proportional(12.0), t.text);
            let ws_w = (ws.size().x + 36.0).clamp(112.0, 190.0);
            let gap = ui.spacing().item_spacing.x;
            // The search icon, as in Lienzo: shown while a minimal switcher fits beside it.
            let icon = 28.0;
            let search = switcher && room >= 64.0 + 8.0 + gap + icon;
            let ws_w = ws_w.min(room - 8.0 - gap - if search { icon } else { 0.0 }).max(64.0);
            let mut rui = ui.new_child(egui::UiBuilder::new().max_rect(right).layout(egui::Layout::right_to_left(egui::Align::Center)));
            let ui = &mut rui;
            // Workspace switcher: shows the current workspace, opens Window → Workspace.
            if switcher {
                let (wr, wresp) = ui.allocate_exact_size(vec2(ws_w, WIDGET_H), Sense::click());
                ui.painter().rect_filled(wr, CornerRadius::same(4), if wresp.hovered() { t.hover } else { t.panel });
                ui.painter().with_clip_rect(wr.shrink2(vec2(4.0, 0.0))).galley(wr.left_center() + vec2(10.0, -ws.size().y / 2.0), ws, t.text);
                icons::paint(ui, "chevron-down", egui::Rect::from_center_size(wr.right_center() - vec2(12.0, 0.0), vec2(12.0, 12.0)), t.text_dim);
                let wresp = wresp.on_hover_text(tl!("Switch workspace"));
                egui::Popup::menu(&wresp).show(|ui| crate::workspaces::popup(app, ui));
                ui.add_space(8.0);
            }
            if let Some(label) = safe {
                if ui.add(egui::Button::new(label).frame(false)).on_hover_text(tl!("Safe Mode…")).clicked() {
                    app.run("help.safeMode", json!({})).ok();
                }
                ui.add_space(8.0);
            }
            // Search → command palette (its shortcut opens it too).
            if search {
                let tip = match menus::shortcut_of("help.commandPalette") {
                    Some(sc) => format!("{}  ({})", tl!("Search commands and tools"), menus::shown_shortcut(sc)),
                    None => tl!("Search commands and tools").to_string(),
                };
                if widgets::icon_button(ui, "search", &tip, app.ui.palette_open, icon).clicked() {
                    app.ui.palette_open = !app.ui.palette_open;
                    app.ui.palette_query.clear();
                }
            }
            // The title: "Pluma" beside the system's menu bar, and in the Nodalia scheme the
            // window's title (unless the OS draws its own) centred in the bar.
            let shown = if app.menus_outside_window() {
                Some("Pluma".to_string())
            } else if t.nodalia && !system {
                Some(window_title(app))
            } else {
                None
            };
            if let Some(shown) = shown {
                let title = ui.painter().layout_no_wrap(shown, egui::FontId::proportional(13.5), t.text);
                let title_rect = egui::Rect::from_center_size(full.center(), title.size());
                // Keep the title centered only while the interactive groups leave room for it.
                if title_rect.left() >= menus_end + 8.0 && title_rect.right() <= ui.min_rect().left() - 8.0 {
                    ui.painter().galley(title_rect.min, title, t.text);
                }
            }
        });
    });
    if custom {
        titlebar::caption_buttons(app, ui, bar.response.rect);
    }
}

/// Whether the window shows the system's title bar instead of the app drawing its own
/// ([`PlumaApp::desktop_frame`]): Windows with Preferences › User Interface › System Title
/// Bar, and Linux whenever the desktop draws the frame, with or without the preference. The
/// preference is read when the app starts, so changing it later does not change this. macOS
/// always has system decorations, but keeps the in-app brand mark, so it never counts as system
/// mode here; nor does the web build (the browser tab has no document title), nor an embedder
/// without a window of its own.
pub fn system_title_bar(app: &PlumaApp) -> bool {
    app.desktop_frame && !app.custom_titlebar && !cfg!(any(target_os = "macos", target_arch = "wasm32"))
}

/// The OS window title: the active document's name suffixed with the app name (a `*` prefix
/// marks unsaved changes, like the `*` in the document tabs), or just the app name with no
/// document open; in Safe Mode, followed by "(Safe Mode)".
pub fn window_title(app: &PlumaApp) -> String {
    let title = app
        .session
        .active()
        .map(|d| format!("{}{} \u{2014} Pluma", if d.is_dirty() { "*" } else { "" }, d.title()))
        .unwrap_or_else(|| "Pluma".into());
    if pluma_plugins::safe_mode::active() { format!("{title} ({})", tl!("Safe Mode")) } else { title }
}

/// Keep the OS window title (and the taskbar / Alt-Tab entry) on the active file. Sends
/// `ViewportCommand::Title` only when the title changed since the last frame.
pub fn sync_window_title(app: &mut PlumaApp, ctx: &egui::Context) {
    let want = window_title(app);
    if app.last_window_title != want {
        app.last_window_title = want.clone();
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(want));
        // One more frame: a new title makes AppKit lay its title bar out again, putting the window
        // buttons back where it keeps them until the next frame centres them on the bar (#968).
        ctx.request_repaint();
    }
}

/// Full Screen Mode and Presentation Mode fill the screen (#4365: the taskbar and the window's
/// frame stayed): the window goes full screen when one of them starts, and back when a mode with
/// the menu bar returns. A window that was full screen already stays so.
pub fn sync_fullscreen(app: &mut PlumaApp, ctx: &egui::Context) {
    let want = app.ui.screen_mode >= 2;
    let is = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
    if want && !is && !app.fullscreen_for_mode {
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(true));
        app.fullscreen_for_mode = true;
    } else if !want && app.fullscreen_for_mode {
        if is {
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
        }
        app.fullscreen_for_mode = false;
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod visual_regressions {
    use super::*;

    fn scaled_input(size: egui::Vec2, scale: f32) -> egui::RawInput {
        let mut input = egui::RawInput { screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)), ..Default::default() };
        input.viewports.entry(egui::ViewportId::ROOT).or_default().native_pixels_per_point = Some(scale);
        input
    }

    struct NativeMenus;
    impl crate::native_menu::Backend for NativeMenus {
        fn sync(&mut self, _: &crate::native_menu::MenuBar) {}
        fn drain(&mut self) -> Vec<crate::native_menu::Event> {
            Vec::new()
        }
    }

    #[test]
    fn capture_chrome_visual_fixtures() {
        let Some(directory) = std::env::var_os("CRAFT_UI_VISUAL_FIXTURES").map(std::path::PathBuf::from) else {
            return;
        };
        std::fs::create_dir_all(&directory).unwrap();
        for brightness in theme::Brightness::ALL {
            for width in [800.0, 1280.0] {
                for scale in [1.0, 1.5, 2.0] {
                    let mut app = PlumaApp::new(pluma_engine::Session::new(), Default::default());
                    app.services.native_menu = Some(crate::native_menu::NativeMenu::new(Box::new(NativeMenus)));
                    app.ui.status = format!("Saved /Users/example/Documents/{}/illustration.svg", "long-folder-name/".repeat(10));
                    let mut ready = false;
                    let mut harness =
                        egui_kittest::Harness::builder().with_size(vec2(width, 200.0)).with_pixels_per_point(scale).wgpu().build_ui_state(
                            move |ui, app: &mut PlumaApp| {
                                if !ready {
                                    egui_extras::install_image_loaders(ui.ctx());
                                    theme::install_fonts(ui.ctx());
                                    theme::apply(ui.ctx(), brightness);
                                    ready = true;
                                    return;
                                }
                                app_bar(app, ui);
                                status_bar(app, ui);
                            },
                            app,
                        );
                    harness.input_mut().max_texture_side = Some(8192);
                    harness.run_steps(4);
                    let stem = format!("pluma-chrome-{brightness:?}-{width}-{scale}x");
                    harness.render().unwrap().save(directory.join(format!("{stem}.png"))).unwrap();
                    let mut regions = serde_json::Map::new();
                    for id in ["app_bar", "status_bar"] {
                        let rect = egui::containers::panel::PanelState::load(&harness.ctx, egui::Id::unique(id)).unwrap().outer_rect;
                        regions.insert(id.into(), json!([rect.left(), rect.top(), rect.width(), rect.height()]));
                    }
                    let dynamic: Vec<_> = harness
                        .output()
                        .shapes
                        .iter()
                        .filter_map(|shape| match &shape.shape {
                            egui::Shape::Text(text) if text.galley.job.text.contains(" fps") => {
                                let r = text.visual_bounding_rect();
                                Some(json!([r.left(), r.top(), r.width(), r.height()]))
                            }
                            _ => None,
                        })
                        .collect();
                    std::fs::write(
                        directory.join(format!("{stem}.json")),
                        serde_json::to_vec_pretty(&json!({
                            "screen": [width, 200.0], "scale": scale, "regions": regions, "native_dynamic_text_regions": dynamic
                        }))
                        .unwrap(),
                    )
                    .unwrap();
                }
            }
        }
    }

    #[test]
    fn native_title_does_not_overlap_header_controls() {
        for brightness in theme::Brightness::ALL {
            for width in [800.0, 1280.0] {
                for scale in [1.0, 1.5, 2.0] {
                    let ctx = egui::Context::default();
                    egui_extras::install_image_loaders(&ctx);
                    theme::install_fonts(&ctx);
                    theme::apply(&ctx, brightness);
                    let mut app = PlumaApp::new(pluma_engine::Session::new(), Default::default());
                    app.services.native_menu = Some(crate::native_menu::NativeMenu::new(Box::new(NativeMenus)));
                    let mut output = ctx.run_ui(scaled_input(vec2(width, 640.0), scale), |ui| app_bar(&mut app, ui));
                    output.textures_delta.clear();
                    assert_eq!(ctx.content_rect().size(), vec2(width, 640.0));
                    assert_eq!(ctx.pixels_per_point(), scale);
                    let texts: Vec<_> = output
                        .shapes
                        .iter()
                        .filter_map(|shape| match &shape.shape {
                            egui::Shape::Text(text) => Some(text),
                            _ => None,
                        })
                        .collect();
                    let title = texts.iter().find(|text| text.galley.job.text == "Pluma");
                    if width >= 1280.0 {
                        assert!(title.is_some(), "the title remains visible when the header has room");
                    }
                    if let Some(title) = title {
                        for text in texts.iter().filter(|text| text.galley.job.text != "Pluma") {
                            assert!(
                                !title.visual_bounding_rect().intersects(text.visual_bounding_rect()),
                                "{width}: title overlaps {:?}",
                                text.galley.job.text
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn long_status_stays_between_navigation_and_performance_readouts() {
        for brightness in theme::Brightness::ALL {
            for width in [800.0, 1280.0] {
                for scale in [1.0, 1.5, 2.0] {
                    let ctx = egui::Context::default();
                    egui_extras::install_image_loaders(&ctx);
                    theme::install_fonts(&ctx);
                    theme::apply(&ctx, brightness);
                    let mut app = PlumaApp::new(pluma_engine::Session::new(), Default::default());
                    app.ui.status = format!("Saved /Users/example/Documents/{}/illustration.svg", "long-folder-name/".repeat(10));
                    let mut output = ctx.run_ui(scaled_input(vec2(width, 640.0), scale), |ui| status_bar(&mut app, ui));
                    output.textures_delta.clear();
                    assert_eq!(ctx.content_rect().size(), vec2(width, 640.0));
                    assert_eq!(ctx.pixels_per_point(), scale);
                    let texts: Vec<_> = output
                        .shapes
                        .iter()
                        .filter_map(|shape| match &shape.shape {
                            egui::Shape::Text(text) => Some(text),
                            _ => None,
                        })
                        .collect();
                    let status = texts.iter().find(|text| text.galley.job.text.starts_with("Saved ")).expect("status stays visible");
                    assert_eq!(status.galley.rows.len(), 1, "status is one line");
                    assert!(status.galley.elided, "long status retains the full text for the label tooltip");
                    let bounds = status.visual_bounding_rect();
                    assert!(bounds.left() >= 0.0 && bounds.right() <= width, "{width}: {bounds:?}");
                    for text in texts.iter().filter(|text| !text.galley.job.text.starts_with("Saved ")) {
                        assert!(!bounds.intersects(text.visual_bounding_rect()), "{width}: status overlaps {:?}", text.galley.job.text);
                    }
                }
            }
        }
    }
}
