use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, TreeView, TreeStore, CellRendererText, TreeViewColumn};
use std::env;
use walkdir::WalkDir;

fn main() {
    let application = Application::new(
        Some("com.example.directory-list"),
        Default::default(),
    )
    .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Lista de Diretórios");
        window.set_default_size(600, 400);

        let tree_store = TreeStore::new(&[String::static_type()]);

        let tree_view = TreeView::new_with_model(&tree_store);
        let renderer = CellRendererText::new();
        let column = TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.add_attribute(&renderer, "text", 0);
        tree_view.append_column(&column);

        let args: Vec<String> = env::args().collect();
        let dir = if args.len() > 1 {
            &args[1]
        } else {
            "."
        };

        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path().display().to_string();
            tree_store.insert_with_values(None, None, &[0], &[&path]);
        }

        window.add(&tree_view);
        window.show_all();
    });

    application.run(&[]);
}