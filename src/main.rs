//! SigmaOS-3d-renderer — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_3d_renderer_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-3d-renderer: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-3d-renderer: ok");
}
