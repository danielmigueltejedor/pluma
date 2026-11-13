//! Help → About Pluma.

use crate::PlumaApp;
use crate::theme;

/// The About window, while `UiState::about` is set: one page, as Lienzo's, its title band carrying
/// the Nodalia close disc. Contributors opens the compiled-in credits in its place, and Back returns.
pub(super) fn show(app: &mut PlumaApp, ctx: &egui::Context) {
    if !app.ui.about {
        return;
    }
    let t = theme::Tokens::get(ctx);
    let credits_id = egui::Id::unique("about_credits");
    let credits = ctx.data_mut(|d| d.get_temp::<bool>(credits_id)).unwrap_or(false);
    let mut open = true;
    let frame = egui::Frame::window(&ctx.global_style())
        .inner_margin(egui::Margin::same(18))
        .corner_radius(egui::CornerRadius::same(t.card_radius.max(theme::CARD_RADIUS)));
    egui::Window::new(tl!("About Pluma"))
        .id(egui::Id::unique("about-pluma"))
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .pivot(egui::Align2::CENTER_CENTER)
        .default_pos(ctx.content_rect().center())
        .frame(frame)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(tl!("About Pluma")).font(theme::semibold(15.0)).color(t.text));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if super::modal::close_button(ui) {
                        open = false;
                    }
                });
            });
            ui.add_space(6.0);
            if credits {
                ui.set_width(820.0);
                if ui.link(tl!("Back")).clicked() {
                    ui.data_mut(|d| d.insert_temp(credits_id, false));
                }
                ui.add_space(6.0);
                crate::credits::contributors_ui(app, ui);
                ui.add_space(12.0);
                crate::credits::models_ui(ui);
            } else if about_page(app, ui) {
                ui.data_mut(|d| d.insert_temp(credits_id, true));
            }
        });
    // Escape closes it, as it does a dialog.
    if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
        open = false;
    }
    if !open {
        ctx.data_mut(|d| d.remove::<bool>(credits_id));
    }
    app.ui.about = open;
}

/// The About page: mark, name, version, the project's links, licences and the GPU in use. Says
/// whether Contributors was clicked.
fn about_page(app: &mut PlumaApp, ui: &mut egui::Ui) -> bool {
    let t = theme::Tokens::get(ui.ctx());
    let mut credits = false;
    ui.set_width(400.0);
    ui.vertical_centered(|ui| {
        ui.add_space(8.0);
        let (r, _) = ui.allocate_exact_size(egui::vec2(96.0, 96.0), egui::Sense::hover());
        crate::brand::paint_mark(ui, r);
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Pluma").font(theme::semibold(26.0)).color(t.text));
        ui.add_space(2.0);
        ui.label(
            egui::RichText::new(crate::i18n::fmt(
                tl!("Version {version} — open-source vector illustration in pure Rust."),
                &[("version", env!("CARGO_PKG_VERSION"))],
            ))
            .color(t.text_dim),
        );
        ui.add_space(14.0);
        let l = app.session.execute("help.links", &serde_json::json!({})).unwrap_or_default();
        let links = [
            (tl!("Pluma Website"), Some(("help.website", l["website"].as_str().unwrap_or("").to_string()))),
            (tl!("Source code on GitHub"), Some(("help.github", l["github"].as_str().unwrap_or("").to_string()))),
            (tl!("Contributors"), None),
        ];
        // The row is centred by its own measured width.
        let gap = 18.0;
        let widths: Vec<f32> = links
            .iter()
            .map(|(label, _)| ui.painter().layout_no_wrap((*label).to_string(), egui::FontId::proportional(13.0), t.accent).size().x)
            .collect();
        let row = widths.iter().sum::<f32>() + gap * (widths.len().saturating_sub(1)) as f32;
        ui.horizontal(|ui| {
            ui.add_space(((ui.available_width() - row) / 2.0).max(0.0));
            ui.spacing_mut().item_spacing.x = gap;
            for (label, target) in links {
                let r = ui.add(egui::Button::new(egui::RichText::new(label).color(t.accent).size(13.0)).frame(false));
                let r = match &target {
                    Some((_, url)) => r.on_hover_text(url),
                    None => r,
                };
                if r.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                    match target {
                        Some((cmd, _)) => app.open_link(cmd),
                        None => credits = true,
                    }
                }
            }
        });
        ui.add_space(14.0);
        let small = |text: String| egui::RichText::new(text).size(11.0).color(t.text_dim);
        ui.label(small(
            tl!("Part of Nodalia. MIT OR Apache-2.0. Fonts: Source Sans 3, Inter, JetBrains Mono (OFL). Icons: Nodalia Design (MIT).").to_string(),
        ));
        // The GPU the window renders with, for bug reports (Preferences › Performance picks it).
        if let Some(adapter) = &app.graphics_adapter {
            ui.add_space(4.0);
            ui.label(small(crate::i18n::fmt(tl!("Graphics: {adapter}"), &[("adapter", adapter.as_str())])));
        }
    });
    credits
}
