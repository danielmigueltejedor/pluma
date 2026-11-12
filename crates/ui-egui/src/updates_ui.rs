//! Software updates: Help ▸ Check for Updates, the check at start, and their windows.
//!
//! The flow and the windows are `nodalia-update`'s, shared with the other Nodalia apps; this file
//! is what is VectorCraft's own: when an update may close the app, and where its keys go.

use serde_json::Value;

use crate::VectorcraftApp;

#[cfg(all(test, not(target_arch = "wasm32")))]
use nodalia_update::egui::Status;
#[cfg(not(target_arch = "wasm32"))]
use nodalia_update::egui::{Action, Updater};

pub(crate) struct Runtime {
    #[cfg(not(target_arch = "wasm32"))]
    updater: Updater,
    /// The desktop app turns the check at start on; tests and the web build leave it off.
    #[cfg(not(target_arch = "wasm32"))]
    startup_check: bool,
    #[cfg(not(target_arch = "wasm32"))]
    check_requested: bool,
    /// The context of the last frame, to wake the UI when a check is queued from a menu callback.
    #[cfg(not(target_arch = "wasm32"))]
    repaint: Option<egui::Context>,
    /// Whether an update window held the input in the last frame.
    #[cfg(not(target_arch = "wasm32"))]
    was_blocking: bool,
    /// The web build has no updater; its window only says so.
    #[cfg(target_arch = "wasm32")]
    pub(crate) manual_window_open: bool,
    #[cfg(target_arch = "wasm32")]
    pub(crate) status_message: Option<String>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            updater: Updater::new(crate::UPDATE_APP).with_accent(ACCENT),
            #[cfg(not(target_arch = "wasm32"))]
            startup_check: false,
            #[cfg(not(target_arch = "wasm32"))]
            check_requested: false,
            #[cfg(not(target_arch = "wasm32"))]
            repaint: None,
            #[cfg(not(target_arch = "wasm32"))]
            was_blocking: false,
            #[cfg(target_arch = "wasm32")]
            manual_window_open: false,
            #[cfg(target_arch = "wasm32")]
            status_message: None,
        }
    }
}

/// The app's accent, for the update windows' main button.
#[cfg(not(target_arch = "wasm32"))]
const ACCENT: [u8; 3] = [0xe8, 0x57, 0x3f];

impl VectorcraftApp {
    /// Whether the app looks for a newer release when it starts (off until the desktop app turns it on).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_startup_update_check(&mut self, on: bool) {
        self.updates_rt.startup_check = on;
    }
}

pub(crate) fn check(app: &mut VectorcraftApp) -> Result<Value, String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.updates_rt.check_requested = true;
        if let Some(ctx) = &app.updates_rt.repaint {
            ctx.request_repaint();
        }
        Ok(Value::Null)
    }
    #[cfg(target_arch = "wasm32")]
    {
        app.updates_rt.manual_window_open = true;
        app.updates_rt.status_message = Some("Update check is not supported in the web browser.".into());
        Ok(Value::Null)
    }
}

pub(crate) fn tick(app: &mut VectorcraftApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let rt = &mut app.updates_rt;
        rt.repaint = Some(ctx.clone());
        if std::mem::take(&mut rt.check_requested) {
            rt.updater.check_now(ctx);
        } else if rt.startup_check {
            rt.updater.check_at_start(ctx);
        }
        // What the update threads sent, so `blocks_input` is current before the shortcuts run.
        let action = rt.updater.poll();
        act(app, ctx, action);
    }
    #[cfg(target_arch = "wasm32")]
    let _ = (app, ctx);
}

/// Whether an update window is open: they are modal, so keys, tool keys and clipboard events stay
/// out of the documents behind them, and the app's other windows take no input.
pub(crate) fn blocks_input(app: &VectorcraftApp) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.updates_rt.updater.is_open()
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = app;
        false
    }
}

/// Whether an update may install and close the app: no document holds unsaved work, no save,
/// export, file opening, plug-in filter or Image Trace is running, no action is playing, no dialog
/// or file dialog is open, and no tool is in the middle of a drag or an edit it computes on
/// another thread.
#[cfg(not(target_arch = "wasm32"))]
fn safe_to_close(app: &VectorcraftApp) -> bool {
    let session = &app.session;
    !crate::unsaved::any_dirty(app)
        && app.background.jobs.is_empty()
        && !crate::opening::long_operation_busy(app)
        && app.ui.action_playing.is_none()
        && app.ui.dialog.is_none()
        && !app.file_dialog_open()
        && !session.in_interaction()
        && !session.tool_busy()
        && !session.tool_editing()
}

/// Carries out what the update windows asked for.
#[cfg(not(target_arch = "wasm32"))]
fn act(app: &mut VectorcraftApp, ctx: &egui::Context, action: Action) {
    match action {
        Action::OpenUrl(url) => app.open_url(&url),
        Action::Exit => {
            // The new version is starting, or a helper waits for this process to end. Close
            // normally so the preferences are saved; nothing holds the close back, since the
            // install only ran once the app was safe to close. Exit anyway if it has not ended soon.
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_secs(5));
                std::process::exit(0);
            });
        }
        Action::None => {}
    }
}

pub(crate) fn show(app: &mut VectorcraftApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // An installation waits for unsaved and running work.
        let safe = safe_to_close(app);
        let language = nodalia_update::Language::from_code(crate::i18n::current().code());
        let action = app.updates_rt.updater.show(ctx, language, safe);
        act(app, ctx, action);
        // A check at startup can open the prompt while a floating document's window covers it.
        let blocking = blocks_input(app);
        if blocking && !app.updates_rt.was_blocking {
            crate::doc_windows::focus_main_window(app, ctx);
        }
        app.updates_rt.was_blocking = blocking;
    }

    #[cfg(target_arch = "wasm32")]
    if app.updates_rt.manual_window_open {
        let mut open = true;
        egui::Window::new(tl!("VectorCraft Updates"))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .pivot(egui::Align2::CENTER_CENTER)
            .default_pos(ctx.content_rect().center())
            .show(ctx, |ui| {
                if let Some(msg) = &app.updates_rt.status_message {
                    ui.label(msg);
                }
                ui.add_space(8.0);
                if ui.button(tl!("OK")).clicked() {
                    app.updates_rt.manual_window_open = false;
                }
            });
        if !open {
            app.updates_rt.manual_window_open = false;
        }
    }
}

/// Opens the update prompt for a test in another module.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn open_prompt_for_test(app: &mut VectorcraftApp) {
    tests::offer(app);
}

/// Opens the window of a running check for a test in another module.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn open_result_window_for_test(app: &mut VectorcraftApp, _ctx: &egui::Context) {
    app.updates_rt.updater.set_status(Status::Checking { user: true });
}

/// Closes the update windows for a test in another module.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn close_dialogs_for_test(app: &mut VectorcraftApp) {
    app.updates_rt.updater.set_status(Status::Idle);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use vectorcraft_engine::Session;
    use vectorcraft_tools::{PointerEvent, PointerKind};

    #[cfg(not(target_arch = "wasm32"))]
    fn release() -> nodalia_update::Release {
        nodalia_update::Release {
            tag: "v99.0.0".into(),
            version: "99.0.0".into(),
            notes: String::new(),
            page: "https://github.com/danielmigueltejedor/pluma/releases/tag/v99.0.0".into(),
            asset: None,
        }
    }

    /// An updater that keeps no record of its checks, showing an update.
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn offer(app: &mut VectorcraftApp) {
        app.updates_rt.updater = Updater::new(crate::UPDATE_APP).with_stamp(None);
        app.updates_rt.updater.offer(release());
    }

    /// An app with one saved document holding a selected rectangle.
    #[cfg(not(target_arch = "wasm32"))]
    fn app_with_saved_object() -> VectorcraftApp {
        let services = crate::Services { write: Some(Box::new(|_: &str, _: &[u8]| Ok(()))), ..Default::default() };
        let mut app = VectorcraftApp::new(Session::new(), services);
        app.run("file.new", json!({"width": 200, "height": 200})).unwrap();
        app.run("shape.rectangle", json!({"x": 10, "y": 10, "width": 50, "height": 50})).unwrap();
        app.run("file.save", json!({"path": "/docs/saved.vectorcraft"})).unwrap();
        assert!(!crate::unsaved::any_dirty(&app));
        app
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn press(key: egui::Key) -> egui::Event {
        egui::Event::Key { key, physical_key: None, pressed: true, repeat: false, modifiers: egui::Modifiers::NONE }
    }

    /// One frame in the app's order: the shortcuts read the input, then the update windows draw.
    #[cfg(not(target_arch = "wasm32"))]
    fn frame(app: &mut VectorcraftApp, ctx: &egui::Context, events: Vec<egui::Event>) {
        let mut output = ctx.run_ui(egui::RawInput { events, ..Default::default() }, |ui| {
            crate::shortcuts::handle(app, ui.ctx());
            show(app, ui.ctx());
        });
        output.textures_delta.clear();
    }

    /// A Copy, the Pen tool's key and Delete, in one frame each: none of them may act.
    #[cfg(not(target_arch = "wasm32"))]
    fn assert_keys_do_nothing(app: &mut VectorcraftApp, ctx: &egui::Context) {
        let tool = app.session.tool_id();
        let mut reached = Vec::new();
        frame(app, ctx, vec![egui::Event::Copy]);
        if !app.session.clipboard.is_empty() {
            reached.push("Copy");
        }
        let tool_key =
            egui::Event::Key { key: egui::Key::P, physical_key: Some(egui::Key::P), pressed: true, repeat: false, modifiers: egui::Modifiers::NONE };
        frame(app, ctx, vec![tool_key, egui::Event::Text("p".into())]);
        if app.session.tool_id() != tool {
            reached.push("the tool key");
        }
        frame(app, ctx, vec![press(egui::Key::Delete)]);
        if app.session.doc().unwrap().selection.is_empty() {
            reached.push("Delete");
        }
        assert_eq!(reached, Vec::<&str>::new(), "these reached the document");
        assert!(!crate::unsaved::any_dirty(app), "a key edited the document");
    }

    #[test]
    fn default_runtime_is_idle() {
        let rt = Runtime::default();
        #[cfg(target_arch = "wasm32")]
        assert!(!rt.manual_window_open);
        #[cfg(not(target_arch = "wasm32"))]
        {
            assert!(!rt.startup_check);
            assert!(!rt.updater.is_open());
            assert_eq!(rt.updater.status(), &Status::Idle);
        }
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn the_releases_are_this_apps_own() {
        assert_eq!(crate::UPDATE_APP.repo, "danielmigueltejedor/pluma");
        assert_eq!(crate::UPDATE_APP.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn startup_prompt_shows_and_closes() {
        let mut app = VectorcraftApp::new(Session::new(), Default::default());
        let ctx = egui::Context::default();
        offer(&mut app);
        frame(&mut app, &ctx, vec![]);
        assert!(blocks_input(&app));
        frame(&mut app, &ctx, vec![press(egui::Key::Escape)]);
        assert!(!blocks_input(&app));
    }

    /// The update prompt, drawn: keys stop at it, Escape closes it, and the next key acts again.
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn keys_do_not_reach_the_document_behind_the_update_prompt() {
        let mut app = app_with_saved_object();
        let ctx = egui::Context::default();
        offer(&mut app);
        frame(&mut app, &ctx, vec![]);
        assert_keys_do_nothing(&mut app, &ctx);
        assert!(blocks_input(&app));

        frame(&mut app, &ctx, vec![press(egui::Key::Escape)]);
        assert!(!blocks_input(&app), "Escape closes the prompt");
        assert!(!app.session.doc().unwrap().selection.is_empty(), "Escape stops at the prompt");
        frame(&mut app, &ctx, vec![press(egui::Key::Delete)]);
        assert!(crate::unsaved::any_dirty(&app), "Delete acts once the prompt has closed");
    }

    /// The window of a check someone asked for, drawn: keys stop at it too.
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn keys_do_not_reach_the_document_behind_the_check_window() {
        let mut app = app_with_saved_object();
        let ctx = egui::Context::default();
        open_result_window_for_test(&mut app, &ctx);
        frame(&mut app, &ctx, vec![]);
        assert_keys_do_nothing(&mut app, &ctx);
    }

    /// A check at start that has not found anything shows nothing and holds nothing back.
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn keys_reach_the_document_during_a_silent_check() {
        let mut app = app_with_saved_object();
        let ctx = egui::Context::default();
        app.updates_rt.updater.set_status(Status::Checking { user: false });
        frame(&mut app, &ctx, vec![]);
        frame(&mut app, &ctx, vec![press(egui::Key::Delete)]);
        assert!(crate::unsaved::any_dirty(&app), "Delete acts during the check");
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn unsaved_changes_dialogs_and_running_work_hold_an_update_back() {
        let mut app = app_with_saved_object();
        assert!(safe_to_close(&app));

        app.run("shape.rectangle", json!({"x": 70, "y": 10, "width": 50, "height": 50})).unwrap();
        assert!(!safe_to_close(&app), "unsaved changes");
        app.run("file.save", json!({"path": "/docs/saved.vectorcraft"})).unwrap();
        assert!(safe_to_close(&app));

        app.ui.dialog = Some(crate::state::Dialog::new("about", json!({})));
        assert!(!safe_to_close(&app), "a dialog");
        app.ui.dialog = None;

        let (shown, _) = crate::tests_picks::off_the_ui_thread(&mut app.services);
        // The command asks again with the answer: until then it reports that it has none.
        assert!(app.run("file.open", json!({})).is_err());
        assert!(app.file_dialog_open());
        assert!(!safe_to_close(&app), "a file dialog");
        shown.answer(&[]);
        crate::picks::poll(&mut app, &egui::Context::default());
        assert!(!app.file_dialog_open());
        assert!(safe_to_close(&app));

        app.services.write_shared = Some(std::sync::Arc::new(|_: &str, _: &[u8]| Ok(())));
        let (release, held) = std::sync::mpsc::channel::<()>();
        let work = move |_: crate::background::Writer| {
            let _ = held.recv();
            Ok(Value::Null)
        };
        // No label: a job the status bar does not list holds the close back too.
        crate::background::run(&mut app, true, String::new(), None, work, |_, r| r).unwrap();
        assert!(!safe_to_close(&app), "a background save");
        let _ = release.send(());
        crate::background::wait_all(&mut app);
        assert!(safe_to_close(&app));

        crate::opening::start(&mut app, "/art/Wait.vectorcraft", vectorcraft_format::save(&vectorcraft_doc::Document::new(10.0, 10.0), false))
            .unwrap();
        assert!(!safe_to_close(&app), "a file opening");
        app.opening.pending.clear();
        assert!(safe_to_close(&app));

        use crate::panels::actions::{Action, ActionSet, PlaybackSpeed, play};
        let step = ("object.move".to_owned(), json!({"dx": 1, "dy": 0}));
        app.ui.action_sets = vec![ActionSet::new("Test", vec![Action::new("Move", vec![step.clone(), step])])];
        app.ui.action_playback.speed = PlaybackSpeed::Step;
        assert_eq!(play(&mut app, 0, 0).unwrap()["pending"], json!(1));
        app.run("file.save", json!({"path": "/docs/saved.vectorcraft"})).unwrap();
        crate::background::wait_all(&mut app);
        assert!(!safe_to_close(&app), "an action playing");
        app.ui.action_playing = None;
        assert!(safe_to_close(&app));
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn a_gesture_or_a_tools_edit_holds_an_update_back() {
        let mut app = app_with_saved_object();
        let view = app.view_info();
        app.session.begin_interaction("Move").unwrap();
        assert!(!safe_to_close(&app), "an interaction");
        app.session.cancel_interaction().unwrap();
        assert!(safe_to_close(&app));

        app.select_tool("selection");
        app.session.pointer(&PointerEvent::new(PointerKind::Down, 150.0, 150.0), view).unwrap();
        app.session.pointer(&PointerEvent::new(PointerKind::Drag, 180.0, 180.0), view).unwrap();
        assert!(app.session.tool_busy() && !app.session.in_interaction());
        assert!(!safe_to_close(&app), "a drag with a tool");
        app.session.pointer(&PointerEvent::new(PointerKind::Up, 180.0, 180.0), view).unwrap();
        assert!(!app.session.tool_busy());
        assert!(safe_to_close(&app));

        // Enough segments for the merge to run on another thread.
        for k in 0..40 {
            app.run("shape.rectangle", json!({"x": 10 * k, "y": 100, "width": 20, "height": 20})).unwrap();
        }
        app.run("file.save", json!({"path": "/docs/saved.vectorcraft"})).unwrap();
        app.run("select.all", json!({})).unwrap();
        app.select_tool("shapeBuilder");
        app.session.pointer(&PointerEvent::new(PointerKind::Move, 15.0, 110.0), view).unwrap();
        app.session.tool_poll(view, true).unwrap();
        app.session.pointer(&PointerEvent::new(PointerKind::Down, 15.0, 110.0), view).unwrap();
        app.session.pointer(&PointerEvent::new(PointerKind::Up, 15.0, 110.0), view).unwrap();
        assert!(app.session.tool_editing() && !crate::unsaved::any_dirty(&app), "the click starts a merge on another thread");
        assert!(!safe_to_close(&app), "a tool's edit");
        crate::unsaved::finish_pending(&mut app);
    }
}
