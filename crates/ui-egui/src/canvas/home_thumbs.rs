//! Thumbnails of the Home screen's recent files: a Pluma document is read and drawn on a thread of
//! its own (never the window's), at most [`SIZE`] on its longer side, and kept until the file
//! changes. Other files (SVG, PDF) and the web build show the card's file glyph.

use std::cell::RefCell;
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::{Arc, Mutex, PoisonError};

/// The longer side of a thumbnail, in pixels.
#[cfg(not(target_arch = "wasm32"))]
const SIZE: f64 = 320.0;

/// Files larger than this are not read for a thumbnail.
#[cfg(not(target_arch = "wasm32"))]
const MAX_BYTES: u64 = 64 * 1024 * 1024;

/// A file as it was when its thumbnail was drawn: its path and when it was last changed.
type Key = (String, Option<std::time::SystemTime>);

/// Thumbnails drawn and not uploaded yet (`None`: no thumbnail of that file).
#[cfg(not(target_arch = "wasm32"))]
type Finished = Arc<Mutex<Vec<(Key, Option<pluma_render::Rendered>)>>>;

#[derive(Default)]
struct Store {
    /// `None`: asked for and not back yet, or no thumbnail.
    shown: HashMap<Key, Option<egui::TextureHandle>>,
    #[cfg(not(target_arch = "wasm32"))]
    done: Finished,
}

thread_local! {
    static STORE: RefCell<Store> = RefCell::new(Store::default());
}

fn key(path: &str) -> Key {
    #[cfg(not(target_arch = "wasm32"))]
    let modified = std::fs::metadata(path).and_then(|m| m.modified()).ok();
    #[cfg(target_arch = "wasm32")]
    let modified = None;
    (path.to_string(), modified)
}

/// The thumbnail of `path` (its texture and size) once it is drawn; the first call starts it.
pub(super) fn get(ctx: &egui::Context, path: &str) -> Option<(egui::TextureId, egui::Vec2)> {
    let key = key(path);
    STORE.with(|store| {
        let mut store = store.borrow_mut();
        #[cfg(not(target_arch = "wasm32"))]
        {
            let done: Vec<_> = std::mem::take(&mut *store.done.lock().unwrap_or_else(PoisonError::into_inner));
            for (k, img) in done {
                let tex =
                    img.map(|img| ctx.load_texture(format!("home-thumb {}", k.0), super::raster::color_image(&img), egui::TextureOptions::LINEAR));
                store.shown.insert(k, tex);
            }
        }
        if let Some(tex) = store.shown.get(&key) {
            return tex.as_ref().map(|t| (t.id(), t.size_vec2()));
        }
        // An older thumbnail of the same file is dropped with the new one asked for.
        store.shown.retain(|k, _| k.0 != key.0);
        store.shown.insert(key.clone(), None);
        #[cfg(not(target_arch = "wasm32"))]
        if pluma_format::is_native_name(path) {
            let done = Arc::clone(&store.done);
            let ctx = ctx.clone();
            let spawned = std::thread::Builder::new().name("home-thumb".into()).spawn(move || {
                let path = key.0.clone();
                let img = std::panic::catch_unwind(|| render(&path)).ok().flatten();
                done.lock().unwrap_or_else(PoisonError::into_inner).push((key, img));
                ctx.request_repaint();
            });
            // No thread: the card keeps its glyph.
            drop(spawned);
        }
        None
    })
}

/// Read and draw a Pluma document: its artboards (or its art, without any) on white.
#[cfg(not(target_arch = "wasm32"))]
fn render(path: &str) -> Option<pluma_render::Rendered> {
    if std::fs::metadata(path).ok()?.len() > MAX_BYTES {
        return None;
    }
    let doc = pluma_format::load(&std::fs::read(path).ok()?).ok()?;
    let region = doc.artboards.iter().map(|a| a.rect).reduce(|a, b| a.union(b)).or_else(|| doc.art_bounds())?;
    let longer = region.width().max(region.height());
    if !longer.is_finite() || longer <= 0.0 {
        return None;
    }
    Some(pluma_render::Renderer::new().render_region(&doc, region, SIZE / longer, true))
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn a_file_that_is_not_a_document_has_no_thumbnail() {
        let dir = std::env::temp_dir().join(format!("pluma-home-thumb-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let junk = dir.join("junk.pluma");
        std::fs::write(&junk, b"\x00\xffnot a document").unwrap();
        assert!(render(junk.to_str().unwrap()).is_none());
        assert!(render(dir.join("missing.pluma").to_str().unwrap()).is_none());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_document_is_drawn_at_the_thumbnail_size() {
        let dir = std::env::temp_dir().join(format!("pluma-home-thumb-doc-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("a.pluma");
        let doc = pluma_doc::Document::new(400.0, 200.0);
        std::fs::write(&path, pluma_format::save_file(&doc)).unwrap();
        let img = render(path.to_str().unwrap()).expect("a thumbnail");
        assert!(img.width.max(img.height) <= SIZE as u32 + 1 && img.width.max(img.height) >= SIZE as u32 - 1, "{}×{}", img.width, img.height);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
