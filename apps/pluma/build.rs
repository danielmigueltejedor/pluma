//! Windows only: embed the app icon and version info (VERSIONINFO) into `pluma.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `PLUMA_REQUIRE_WINRES=1` turns it into
//! an error (for release builds).

fn main() -> Result<(), String> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/pluma.ico");
    println!("cargo:rerun-if-env-changed=PLUMA_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return Ok(());
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/pluma.ico")
        .set("ProductName", "Pluma")
        .set("FileDescription", "Pluma vector illustration editor")
        .set("CompanyName", "Nodalia")
        .set("LegalCopyright", "Copyright (c) the Pluma authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "pluma.exe")
        .set("InternalName", "pluma");
    if let Err(e) = res.compile() {
        if std::env::var_os("PLUMA_REQUIRE_WINRES").is_some() {
            return Err(format!("embedding Windows resources failed: {e}"));
        }
        println!("cargo:warning=pluma.exe built without icon/version resources: {e}");
    }
    Ok(())
}
