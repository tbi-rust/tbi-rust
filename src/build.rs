//! Generates the app icon at build time from `src/assets/tor_logo_tbb.svg`
//! so there's never a manual sips/iconutil step:
//!
//!   - `target/generated-assets/icon.icns` — used by `cargo bundle` (see
//!     `[package.metadata.bundle]` in Cargo.toml) as the .app's Finder/Dock
//!     icon. Written to a fixed path (not OUT_DIR) because Cargo.toml's
//!     `icon = [...]` field is static and can't reference OUT_DIR's hash.
//!   - `$OUT_DIR/icon.png` — used by main.rs at runtime for the window/Dock
//!     icon via `include_bytes!(concat!(env!("OUT_DIR"), "/icon.png"))`.
//!
//! Re-run this by touching the SVG, or just `cargo build` — it only
//! re-renders when the source SVG changes (see the rerun-if-changed below).

use std::env;
use std::fs;
use std::path::PathBuf;

// Rendered once at each of these sizes for a crisp icns (icns wants
// discrete sizes rather than one image scaled down by the OS).
const ICNS_SIZES: &[u32] = &[16, 32, 64, 128, 256, 512, 1024];

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let svg_path = manifest_dir.join("src/assets/tor_logo_tbb.svg");

    println!("cargo:rerun-if-changed={}", svg_path.display());

    let svg_data = fs::read(&svg_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", svg_path.display()));

    let mut opt = usvg::Options::default();
    opt.fontdb_mut().load_system_fonts();
    let tree = usvg::Tree::from_data(&svg_data, &opt).expect("failed to parse tor_logo_tbb.svg");

    let svg_size = tree.size();
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let bundle_dir = manifest_dir.join("target/generated-assets");
    fs::create_dir_all(&bundle_dir).expect("failed to create target/generated-assets");

    let mut icon_family = icns::IconFamily::new();

    for &size in ICNS_SIZES {
        let mut pixmap =
            tiny_skia::Pixmap::new(size, size).expect("failed to allocate pixmap");

        // Uniform scale so the logo isn't stretched if the SVG isn't square.
        let scale = size as f32 / svg_size.width().max(svg_size.height());
        let transform = tiny_skia::Transform::from_scale(scale, scale);
        resvg::render(&tree, transform, &mut pixmap.as_mut());

        let image = icns::Image::from_data(
            icns::PixelFormat::RGBA,
            size,
            size,
            pixmap.data().to_vec(),
        )
        .expect("failed to build icns image");

        // Some sizes (e.g. 64) aren't valid standalone icns entry types on
        // their own — skip anything the format doesn't define rather than
        // failing the whole build over a size icns doesn't use.
        if let Some(icon_type) = icns::IconType::from_pixel_size(size, size) {
            icon_family
                .add_icon_with_type(&image, icon_type)
                .expect("failed to add icon to family");
        }

        // Reuse the largest render as the runtime window/dock icon too.
        if size == *ICNS_SIZES.last().unwrap() {
            let png_path = out_dir.join("icon.png");
            pixmap
                .save_png(&png_path)
                .unwrap_or_else(|e| panic!("failed to write {}: {e}", png_path.display()));
        }
    }

    let icns_path = bundle_dir.join("icon.icns");
    let icns_file =
        fs::File::create(&icns_path).unwrap_or_else(|e| panic!("failed to create {}: {e}", icns_path.display()));
    icon_family
        .write(icns_file)
        .expect("failed to write icon.icns");

    println!("cargo:warning=generated icon -> {}", icns_path.display());
}
