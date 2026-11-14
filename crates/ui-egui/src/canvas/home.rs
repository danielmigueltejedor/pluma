use super::{PlumaApp, Tokens, Ui, json, vec2};
use nodalia_theme::creator::home;

/// The Home screen (`app.home`; with no document open, unless its preference is off): the creator
/// suite's Home (`nodalia_theme::creator::home`), the same screen as Lienzo's. The rail holds New
/// file, Open…, Home, Learn and the links; the page, Recent Files and a few presets to start from.
pub(super) fn home(app: &mut PlumaApp, ui: &mut Ui, rect: egui::Rect) {
    let t = Tokens::get(ui.ctx());
    let look = t.creator();
    let l = app.session.execute("help.links", &json!({})).unwrap_or_default();
    let (website, github) = (l["website"].as_str().unwrap_or("").to_string(), l["github"].as_str().unwrap_or("").to_string());
    let links = [home::Link { icon: "globe", label: tl!("Website"), url: &website }, home::Link { icon: "code", label: tl!("GitHub"), url: &github }];
    let rail = home::Rail { new_file: tl!("New file"), open: tl!("Open…"), home: tl!("Home"), learn: Some(tl!("Learn")), links: &links };
    let mark = |p: &egui::Painter, r: egui::Rect| crate::brand::paint_mark_on(p, r);
    let mut recent_pick = None;
    let picked = home::view(ui, &look, rect, "Pluma", &mark, &rail, |ui| {
        recent_pick = recent(app, ui, &look);
        ui.add_space(30.0);
        presets(app, ui, &look);
    });
    match picked {
        Some(home::Pick::NewFile) => {
            app.run("file.newDialog", json!({})).ok();
        }
        Some(home::Pick::Open) => {
            app.run("file.open", json!({})).ok();
        }
        // Home is the screen showing; with documents open it goes back to them.
        Some(home::Pick::Home) if !app.session.documents().is_empty() => app.ui.home = None,
        Some(home::Pick::Learn | home::Pick::Link(0)) => app.open_link("help.website"),
        Some(home::Pick::Link(_)) => app.open_link("help.github"),
        _ => {}
    }
    let files: Vec<String> = crate::io::recent_files(app).iter().take(SHOWN).cloned().collect();
    match recent_pick {
        Some(home::RecentPick::Open(i)) => {
            if let Some(path) = files.get(i) {
                // A failure is reported to the user there.
                let _ = crate::io::open_reporting(app, path);
            }
        }
        Some(home::RecentPick::Remove(i)) => {
            if let Some(path) = files.get(i) {
                app.ui.recent_files.retain(|p| p != path);
            }
        }
        Some(home::RecentPick::Clear) => {
            app.run("file.clearRecent", json!({})).ok();
        }
        Some(home::RecentPick::OpenDialog) => {
            app.run("file.open", json!({})).ok();
        }
        None => {}
    }
}

/// Recent files shown on the Home screen (#663): the first of File › Open Recent Files.
const SHOWN: usize = 11;

/// Recent Files, as cards or a list (`ui.home_recent_list`), each its thumbnail, name and folder.
fn recent(app: &mut PlumaApp, ui: &mut Ui, look: &nodalia_theme::creator::Look) -> Option<home::RecentPick> {
    let files: Vec<String> = crate::io::recent_files(app).iter().take(SHOWN).cloned().collect();
    let names: Vec<(String, String)> = files
        .iter()
        .map(|path| {
            let p = std::path::Path::new(path);
            let name = p.file_name().map_or_else(|| path.clone(), |n| n.to_string_lossy().into_owned());
            (name, p.parent().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default())
        })
        .collect();
    let list: Vec<home::Recent> = files
        .iter()
        .zip(&names)
        .map(|(path, (name, folder))| home::Recent { name, folder, path, thumb: super::home_thumbs::get(ui.ctx(), path) })
        .collect();
    let labels = home::RecentLabels {
        heading: tl!("Recent Files"),
        grid: tl!("Grid view"),
        list: tl!("List view"),
        remove: tl!("Remove from Recent"),
        clear: tl!("Clear Recent Files"),
        no_folder: tl!("On your computer"),
        drop: tl!("Drop an SVG, PDF or Pluma file anywhere to open it."),
        drop_icon: "file",
    };
    let mut grid = !app.ui.home_recent_list;
    let pick = home::recents(ui, look, &labels, &list, &mut grid);
    app.ui.home_recent_list = !grid;
    pick
}

/// A few of New Document's presets (`file.newPresets`), as its cards.
fn presets(app: &mut PlumaApp, ui: &mut Ui, look: &nodalia_theme::creator::Look) {
    home::section_heading(ui, look, tl!("Quickly start a new file"));
    ui.add_space(12.0);
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
