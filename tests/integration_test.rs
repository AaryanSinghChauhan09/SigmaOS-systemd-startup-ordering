// Integration test for SigmaOS-systemd-startup-ordering
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use systemd_startup_ordering as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
