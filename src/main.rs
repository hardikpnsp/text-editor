use std::env;

use text_editor::editor::Editor;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("file name not provided");

    Editor::new(filename).run();
}
