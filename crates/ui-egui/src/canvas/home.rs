use super::{CornerRadius, PlumaApp, Tokens, Ui, json, theme, vec2, widgets};

/// The Home screen (`app.home`; with no document open, unless its preference is off). Wide, a rail
/// of actions on the left and the app's name, Recent Files and presets on the right, as Lienzo's
/// Home screen; narrow, one column.
pub(super) fn home(app: &mut PlumaApp, ui: &mut Ui, rect: egui::Rect) {
    let t = Tokens::get(ui.ctx());
    if t.nodalia {
        ui.painter().rect_filled(rect, CornerRadius::same(t.card_radius), t.panel);
    } else {
        ui.painter().rect_filled(rect, 0.0, t.panel_darker);
    }
    let narrow = rect.width() < 640.0;
    let (pad_x, pad_y) = if narrow { (16.0, 24.0) } else { (36.0, 54.0) };
    let inner = rect.shrink2(vec2(pad_x, 0.0));
    if narrow {
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner).layout(egui::Layout::top_down(egui::Align::Min)));
        // Taller than the window (a small window, many recent files): it scrolls. Widgets scrolled
        // out of view are clipped, so they take no pointer.
        egui::ScrollArea::vertical().id_salt("home-scroll").auto_shrink([false, false]).show(&mut child, |ui| {
            ui.add_space(pad_y);
            ui.horizontal(|ui| actions(app, ui));
            ui.add_space(16.0);
            content(app, ui);
        });
        return;
    }
    const RAIL: f32 = 170.0;
    let rail = egui::Rect::from_min_size(egui::pos2(inner.left(), rect.top() + pad_y), vec2(RAIL, (rect.height() - 2.0 * pad_y).max(0.0)));
    let body = egui::Rect::from_min_max(egui::pos2(rail.right() + 36.0, rect.top()), egui::pos2(inner.right(), rect.bottom() - 16.0));
    let mut rail_ui = ui.new_child(egui::UiBuilder::new().max_rect(rail).layout(egui::Layout::top_down(egui::Align::Min)));
    side_rail(app, &mut rail_ui);
    let mut body_ui = ui.new_child(egui::UiBuilder::new().max_rect(body).layout(egui::Layout::top_down(egui::Align::Min)));
    egui::ScrollArea::vertical().id_salt("home-scroll").auto_shrink([false, false]).show(&mut body_ui, |ui| {
        ui.add_space(pad_y);
        content(app, ui);
        ui.add_space(24.0);
    });
}

/// The rail's width for its buttons.
const BUTTON_W: f32 = 130.0;

/// New file and Open…, the screen's two actions.
fn actions(app: &mut PlumaApp, ui: &mut Ui) {
    if new_file_button(ui).clicked() {
        app.run("file.newDialog", json!({})).ok();
    }
    ui.add_space(8.0);
    if open_button(ui).clicked() {
        app.run("file.open", json!({})).ok();
    }
}

/// The left rail: the actions, Home, and the website and GitHub at the foot.
fn side_rail(app: &mut PlumaApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    ui.add_space(8.0);
    actions(app, ui);
    ui.add_space(16.0);
    separator(ui, BUTTON_W + 20.0);
    ui.add_space(14.0);
    // Home is the screen showing: its row is lit, and with documents open it goes back to them.
    let (row, resp) = ui.allocate_exact_size(vec2(width, 32.0), egui::Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, tl!("Home")));
    let (bg, ink) = if t.nodalia { (t.text, t.panel) } else { (t.row_selected, t.text) };
    ui.painter().rect_filled(row, CornerRadius::same(t.radius), bg);
    crate::icons::paint(ui, "house", egui::Rect::from_center_size(egui::pos2(row.left() + 20.0, row.center().y), vec2(16.0, 16.0)), ink);
    ui.painter().text(egui::pos2(row.left() + 34.0, row.center().y), egui::Align2::LEFT_CENTER, tl!("Home"), theme::semibold(13.5), ink);
    if !app.session.documents().is_empty() && resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
        app.ui.home = None;
    }
    // The links sit at the rail's foot.
    ui.add_space((ui.available_height() - 80.0).max(24.0));
    separator(ui, BUTTON_W + 20.0);
    ui.add_space(8.0);
    let l = app.session.execute("help.links", &json!({})).unwrap_or_default();
    for (icon, label, cmd, key) in
        [("globe", tl!("Pluma Website"), "help.website", "website"), ("git-branch", tl!("Source code on GitHub"), "help.github", "github")]
    {
        if rail_link(ui, width, icon, label, l[key].as_str().unwrap_or("")).clicked() {
            app.open_link(cmd);
        }
    }
}

fn separator(ui: &mut Ui, width: f32) {
    let t = Tokens::get(ui.ctx());
    let (r, _) = ui.allocate_exact_size(vec2(width.min(ui.available_width()), 1.0), egui::Sense::hover());
    ui.painter().hline(r.x_range(), r.center().y, egui::Stroke::new(1.0, t.divider));
}

/// New file: a solid pill in the text colour on Nodalia, the accent button elsewhere.
fn new_file_button(ui: &mut Ui) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    if !t.nodalia {
        return widgets::primary_button_width(ui, tl!("New file"), BUTTON_W);
    }
    let (rect, resp) = ui.allocate_exact_size(vec2(BUTTON_W, 32.0), egui::Sense::click());
    let bg = if resp.hovered() { t.text.gamma_multiply(0.88) } else { t.text };
    ui.painter().rect_filled(rect, CornerRadius::same(t.radius), bg);
    ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, tl!("New file"), theme::semibold(13.0), t.panel);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, tl!("New file")));
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Open…: an outlined button.
fn open_button(ui: &mut Ui) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    if !t.nodalia {
        return widgets::secondary_button_width(ui, tl!("Open…"), BUTTON_W);
    }
    let (rect, resp) = ui.allocate_exact_size(vec2(BUTTON_W, 30.0), egui::Sense::click());
    let r = CornerRadius::same(t.radius);
    if resp.hovered() {
        ui.painter().rect_filled(rect, r, t.hover);
    }
    ui.painter().rect_stroke(rect, r, egui::Stroke::new(1.0, t.divider), egui::StrokeKind::Inside);
    ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, tl!("Open…"), theme::semibold(13.0), t.text);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, tl!("Open…")));
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A link at the rail's foot: dim, lit under the pointer, its address in the tooltip.
fn rail_link(ui: &mut Ui, width: f32, icon: &str, label: &str, url: &str) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(width, 28.0), egui::Sense::click());
    if resp.hovered() {
        ui.painter().rect_filled(rect, CornerRadius::same(4), t.hover);
    }
    let ink = if resp.hovered() { t.text } else { t.text_dim };
    crate::icons::paint(ui, icon, egui::Rect::from_center_size(egui::pos2(rect.left() + 20.0, rect.center().y), vec2(14.0, 14.0)), ink);
    ui.painter().text(egui::pos2(rect.left() + 34.0, rect.center().y), egui::Align2::LEFT_CENTER, label, theme::semibold(12.0), ink);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Link, true, label));
    resp.on_hover_text(url).on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// The right column: the app's mark and name, Recent Files and a few presets to start from.
fn content(app: &mut PlumaApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        let (mark, _) = ui.allocate_exact_size(vec2(28.0, 28.0), egui::Sense::hover());
        crate::brand::paint_mark(ui, mark);
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Pluma").font(theme::semibold(24.0)).color(t.text));
    });
    ui.add_space(18.0);
    separator(ui, ui.available_width());
    recent(app, ui);
    ui.add_space(28.0);
    ui.label(egui::RichText::new(tl!("Quickly start a new file")).font(theme::semibold(14.0)).color(t.text));
    ui.add_space(10.0);
    // A few of New Document's presets (`file.newPresets`), as its cards.
    let presets = ["Letter", "A4", "Web 1920×1080", "Phone 390×844", "Postcard", "Social Square Post 1080×1080"];
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(14.0, 14.0);
        let cards: Vec<_> = presets.iter().filter_map(|name| pluma_engine::cmd::newdoc::find(&app.session, name)).collect();
        for s in cards {
            if crate::dialogs::preset_card(ui, &s, false).clicked() {
                app.run("file.new", json!({ "preset": s.name })).ok();
            }
        }
    });
}

/// The Home screen's Recent Files (#663): the first of File › Open Recent Files, each its name and
/// folder; a click opens it. Nothing when there are none.
fn recent(app: &mut PlumaApp, ui: &mut Ui) {
    const SHOWN: usize = 6;
    let files: Vec<String> = crate::io::recent_files(app).iter().take(SHOWN).cloned().collect();
    if files.is_empty() {
        return;
    }
    let t = Tokens::get(ui.ctx());
    ui.add_space(28.0);
    ui.label(egui::RichText::new(tl!("Recent Files")).font(theme::semibold(14.0)).color(t.text));
    ui.add_space(8.0);
    for path in files {
        if recent_row(ui, &path).clicked() {
            // A failure is reported to the user there.
            let _ = crate::io::open_reporting(app, &path);
        }
    }
}

/// One Recent Files row across the Home screen: history icon, name (accent) and folder (dim,
/// truncated). The whole row is one button that lights up under the pointer; its tooltip is the
/// full path.
fn recent_row(ui: &mut Ui, path: &str) -> egui::Response {
    const PAD: f32 = 6.0;
    const ICON: f32 = 16.0;
    let t = Tokens::get(ui.ctx());
    let p = std::path::Path::new(path);
    let name = p.file_name().map_or_else(|| path.to_string(), |n| n.to_string_lossy().into_owned());
    let folder = p.parent().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default();
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 26.0), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        if resp.hovered() {
            ui.painter().rect_filled(rect, CornerRadius::same(4), t.hover);
        }
        let icon = egui::Rect::from_center_size(egui::pos2(rect.left() + PAD + ICON / 2.0, rect.center().y), vec2(ICON, ICON));
        crate::icons::paint(ui, "history", icon, t.icon);
        let x = icon.right() + 8.0;
        let name_galley = egui::WidgetText::from(egui::RichText::new(&name).size(13.0).color(t.accent)).into_galley(
            ui,
            Some(egui::TextWrapMode::Truncate),
            (rect.right() - PAD - x).max(0.0),
            egui::TextStyle::Body,
        );
        let folder_x = x + name_galley.size().x + 10.0;
        ui.painter().galley(egui::pos2(x, rect.center().y - name_galley.size().y / 2.0), name_galley, t.accent);
        let room = rect.right() - PAD - folder_x;
        if room > 0.0 && !folder.is_empty() {
            let folder_galley = egui::WidgetText::from(egui::RichText::new(&folder).size(12.0).color(t.text_dim)).into_galley(
                ui,
                Some(egui::TextWrapMode::Truncate),
                room,
                egui::TextStyle::Body,
            );
            ui.painter().galley(egui::pos2(folder_x, rect.center().y - folder_galley.size().y / 2.0), folder_galley, t.text_dim);
        }
    }
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, &name));
    resp.on_hover_text(path).on_hover_cursor(egui::CursorIcon::PointingHand)
}
