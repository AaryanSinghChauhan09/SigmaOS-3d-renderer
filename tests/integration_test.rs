// Integration test for SigmaOS-3d-renderer
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_3d_renderer as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
