// SPDX-License-Identifier: MIT
#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/ui/storage_derive/pass/*.rs");
    t.compile_fail("tests/ui/storage_derive/fail/*.rs");

    t.pass("tests/ui/storage_item/pass/*.rs");
}
