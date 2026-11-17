// Integration test for SigmaOS-file-indexing
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use file_indexing as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
