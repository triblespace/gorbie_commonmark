use egui::__run_test_ui;
use gorbie_commonmark_macros::commonmark_str;

// Check that it fails to compile when it is not able to find a file
fn main() {
    let mut cache = gorbie_commonmark_backend::CommonMarkCache::default();
    __run_test_ui(|ui| {
        commonmark_str!(ui, &mut cache, "foo.md");
    });
}
