use crate::compiler::module_graph::{EntryLocation, ModuleGraph};
use std::fs;
use std::path::PathBuf;

struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> Self {
        let dir =
            std::env::temp_dir().join(format!("neon_module_graph_{}_{}", name, std::process::id()));
        fs::create_dir_all(&dir).expect("Failed to create temp dir");
        TempDir(dir)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn loads_dir_b_for_import_b_from_dir_a() {
    let dir = TempDir::new("loads_b");
    let a_path = dir.0.join("a.n");
    let b_path = dir.0.join("b.n");
    let a_source = "import \"b\"\n";
    fs::write(&a_path, a_source).expect("Failed to write a.n");
    fs::write(&b_path, "val x = 1\n").expect("Failed to write b.n");

    let graph = ModuleGraph::build(a_source, EntryLocation::File(a_path.clone()))
        .expect("graph should build");

    let paths: Vec<PathBuf> = graph.modules().iter().map(|m| m.path.clone()).collect();
    assert_eq!(
        vec![
            b_path.canonicalize().unwrap(),
            a_path.canonicalize().unwrap()
        ],
        paths
    );
    assert_eq!("val x = 1\n", graph.modules()[0].source);
    assert_eq!(a_source, graph.modules()[1].source);
}
