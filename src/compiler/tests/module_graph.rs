use crate::common::errors::CompilationErrorKind;
use crate::compiler::module_graph::{EntryLocation, ModuleGraph};
use std::fs;
use std::path::PathBuf;

pub(super) struct TempDir(pub(super) PathBuf);

impl TempDir {
    pub(super) fn new(name: &str) -> Self {
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

fn dependencies_of_import(dir: &TempDir, import: &str) -> Vec<PathBuf> {
    let a_path = dir.0.join("dir").join("a.n");
    let a_source = format!("import \"{import}\"\n");
    fs::create_dir_all(dir.0.join("dir").join("sub")).expect("Failed to create dirs");
    fs::write(&a_path, &a_source).expect("Failed to write a.n");
    for target in [
        dir.0.join("b.n"),
        dir.0.join("dir").join("b.n"),
        dir.0.join("dir").join("sub").join("b.n"),
        dir.0.join("abs.n"),
    ] {
        fs::write(&target, "val x = 1\n").expect("Failed to write target");
    }

    let graph =
        ModuleGraph::build(&a_source, EntryLocation::File(a_path)).expect("graph should build");
    graph.modules()[1].dependencies.clone()
}

#[test]
fn resolves_dot_slash_b_relative_to_importing_file() {
    let dir = TempDir::new("resolves_dot_slash");
    let deps = dependencies_of_import(&dir, "./b");
    assert_eq!(
        vec![dir.0.join("dir").join("b.n").canonicalize().unwrap()],
        deps
    );
}

#[test]
fn resolves_dot_dot_slash_b_relative_to_importing_file() {
    let dir = TempDir::new("resolves_dot_dot");
    let deps = dependencies_of_import(&dir, "../b");
    assert_eq!(vec![dir.0.join("b.n").canonicalize().unwrap()], deps);
}

#[test]
fn resolves_sub_slash_b_relative_to_importing_file() {
    let dir = TempDir::new("resolves_sub");
    let deps = dependencies_of_import(&dir, "sub/b");
    assert_eq!(
        vec![dir
            .0
            .join("dir")
            .join("sub")
            .join("b.n")
            .canonicalize()
            .unwrap()],
        deps
    );
}

#[test]
fn resolves_absolute_path_import() {
    let dir = TempDir::new("resolves_absolute");
    let abs = dir.0.join("abs");
    let deps = dependencies_of_import(&dir, abs.to_str().unwrap());
    assert_eq!(vec![dir.0.join("abs.n").canonicalize().unwrap()], deps);
}

#[test]
fn does_not_double_n_extension() {
    let dir = TempDir::new("no_double_extension");
    let deps = dependencies_of_import(&dir, "b.n");
    assert_eq!(
        vec![dir.0.join("dir").join("b.n").canonicalize().unwrap()],
        deps
    );
}

#[test]
fn yields_one_module_when_dir_a_imports_both_b_and_dot_slash_b() {
    let dir = TempDir::new("one_module_for_b");
    let a_path = dir.0.join("dir").join("a.n");
    let b_path = dir.0.join("dir").join("b.n");
    let a_source = "import \"b\"\nimport \"./b\"\n";
    fs::create_dir_all(dir.0.join("dir")).expect("Failed to create dir");
    fs::write(&a_path, a_source).expect("Failed to write a.n");
    fs::write(&b_path, "val x = 1\n").expect("Failed to write b.n");

    let graph = ModuleGraph::build(a_source, EntryLocation::File(a_path.clone()))
        .expect("graph should build");

    let paths: Vec<PathBuf> = graph.modules().iter().map(|m| m.path.clone()).collect();
    assert_eq!(2, paths.len());
    assert_eq!(
        1,
        paths
            .iter()
            .filter(|p| **p == b_path.canonicalize().unwrap())
            .count()
    );
    let a_module = graph
        .modules()
        .iter()
        .find(|m| m.path == a_path.canonicalize().unwrap())
        .expect("a.n should be in the graph");
    assert_eq!(vec![b_path.canonicalize().unwrap()], a_module.dependencies);
}

#[test]
fn orders_b_a_main_when_main_imports_a_and_b_and_a_imports_b() {
    let dir = TempDir::new("orders_b_a_main");
    let main_path = dir.0.join("main.n");
    let a_path = dir.0.join("a.n");
    let b_path = dir.0.join("b.n");
    let main_source = "import \"a\"\nimport \"b\"\n";
    fs::write(&main_path, main_source).expect("Failed to write main.n");
    fs::write(&a_path, "import \"b\"\n").expect("Failed to write a.n");
    fs::write(&b_path, "val x = 1\n").expect("Failed to write b.n");

    let graph = ModuleGraph::build(main_source, EntryLocation::File(main_path.clone()))
        .expect("graph should build");

    let paths: Vec<PathBuf> = graph.modules().iter().map(|m| m.path.clone()).collect();
    assert_eq!(
        vec![
            b_path.canonicalize().unwrap(),
            a_path.canonicalize().unwrap(),
            main_path.canonicalize().unwrap()
        ],
        paths
    );
}

#[test]
fn reports_a_b_a_when_a_imports_b_and_b_imports_a() {
    let dir = TempDir::new("reports_cycle");
    let a_path = dir.0.join("a.n");
    let b_path = dir.0.join("b.n");
    let a_source = "import \"b\"\n";
    fs::write(&a_path, a_source).expect("Failed to write a.n");
    fs::write(&b_path, "val x = 1\nimport \"a\"\n").expect("Failed to write b.n");

    let Err(errors) = ModuleGraph::build(a_source, EntryLocation::File(a_path)) else {
        panic!("cycle should be reported");
    };

    assert_eq!(1, errors.len());
    assert!(
        errors[0].message.contains("a.n -> b.n -> a.n"),
        "message was: {}",
        errors[0].message
    );
    assert_eq!(2, errors[0].location.line);
    assert_eq!(Some(b_path.canonicalize().unwrap()), errors[0].file);
}

#[test]
fn reports_an_unknown_module_in_an_imported_file_with_that_files_path() {
    let dir = TempDir::new("unknown_in_imported");
    let a_path = dir.0.join("a.n");
    let b_path = dir.0.join("b.n");
    let a_source = "import \"b\"\n";
    fs::write(&a_path, a_source).expect("Failed to write a.n");
    fs::write(&b_path, "val x = 1\nimport \"missing\"\n").expect("Failed to write b.n");

    let Err(errors) = ModuleGraph::build(a_source, EntryLocation::File(a_path)) else {
        panic!("missing module should be reported");
    };

    assert_eq!(1, errors.len());
    assert_eq!(2, errors[0].location.line);
    assert_eq!(Some(b_path.canonicalize().unwrap()), errors[0].file);
}

#[test]
fn reports_an_unknown_module_in_the_entry_file_without_a_path() {
    let dir = TempDir::new("unknown_in_entry");
    let a_path = dir.0.join("a.n");
    let a_source = "import \"missing\"\n";
    fs::write(&a_path, a_source).expect("Failed to write a.n");

    let Err(errors) = ModuleGraph::build(a_source, EntryLocation::File(a_path)) else {
        panic!("missing module should be reported");
    };

    assert_eq!(1, errors.len());
    assert_eq!(None, errors[0].file);
}

#[test]
fn reports_a_cycle_closed_in_the_entry_file_without_a_path() {
    let dir = TempDir::new("cycle_in_entry");
    let a_path = dir.0.join("a.n");
    let a_source = "import \"a\"\n";
    fs::write(&a_path, a_source).expect("Failed to write a.n");

    let Err(errors) = ModuleGraph::build(a_source, EntryLocation::File(a_path)) else {
        panic!("cycle should be reported");
    };

    assert_eq!(1, errors.len());
    assert_eq!(None, errors[0].file);
}

#[test]
fn reports_a_self_import_as_a_cycle() {
    let dir = TempDir::new("self_import");
    let a_path = dir.0.join("a.n");
    let a_source = "import \"a\"\n";
    fs::write(&a_path, a_source).expect("Failed to write a.n");

    let Err(errors) = ModuleGraph::build(a_source, EntryLocation::File(a_path)) else {
        panic!("self import should be reported");
    };

    assert_eq!(1, errors.len());
    assert!(
        errors[0].message.contains("a.n -> a.n"),
        "message was: {}",
        errors[0].message
    );
}

#[test]
fn reports_a_mid_graph_cycle_without_the_entry_in_the_chain() {
    let dir = TempDir::new("mid_graph_cycle");
    let main_path = dir.0.join("main.n");
    let main_source = "import \"a\"\n";
    fs::write(&main_path, main_source).expect("Failed to write main.n");
    fs::write(dir.0.join("a.n"), "import \"b\"\n").expect("Failed to write a.n");
    fs::write(dir.0.join("b.n"), "import \"a\"\n").expect("Failed to write b.n");

    let Err(errors) = ModuleGraph::build(main_source, EntryLocation::File(main_path)) else {
        panic!("cycle should be reported");
    };

    assert_eq!(1, errors.len());
    assert!(
        errors[0].message.contains("a.n -> b.n -> a.n"),
        "message was: {}",
        errors[0].message
    );
    assert!(
        !errors[0].message.contains("main.n"),
        "message was: {}",
        errors[0].message
    );
}

#[test]
fn treats_dot_slash_std_math_as_a_file_import() {
    let dir = TempDir::new("dot_slash_std_math");
    let a_path = dir.0.join("a.n");
    let a_source = "import \"./std/math\"\n";
    fs::write(&a_path, a_source).expect("Failed to write a.n");

    let Err(errors) = ModuleGraph::build(a_source, EntryLocation::File(a_path)) else {
        panic!("missing file should be reported");
    };

    let tried = dir.0.canonicalize().unwrap().join("std").join("math.n");
    assert_eq!(1, errors.len());
    assert!(
        errors[0].message.contains(tried.to_str().unwrap()),
        "message was: {}",
        errors[0].message
    );
}

#[test]
fn reports_an_unreadable_dependency_at_the_import() {
    let dir = TempDir::new("unreadable_dependency");
    let a_path = dir.0.join("a.n");
    let a_source = "val x = 1\nimport \"b\"\n";
    fs::write(&a_path, a_source).expect("Failed to write a.n");
    fs::create_dir(dir.0.join("b.n")).expect("Failed to create b.n dir");

    let Err(errors) = ModuleGraph::build(a_source, EntryLocation::File(a_path)) else {
        panic!("unreadable module should be reported");
    };

    assert_eq!(1, errors.len());
    assert_eq!(CompilationErrorKind::UnknownModule, errors[0].kind);
    assert!(
        errors[0].message.starts_with("cannot read module 'b' ("),
        "message was: {}",
        errors[0].message
    );
    assert!(
        errors[0]
            .message
            .contains(dir.0.canonicalize().unwrap().join("b.n").to_str().unwrap()),
        "message was: {}",
        errors[0].message
    );
    assert_eq!(2, errors[0].location.line);
    assert_eq!(None, errors[0].file);
}

#[test]
fn reports_the_path_tried_for_import_missing() {
    let dir = TempDir::new("reports_missing");
    let a_path = dir.0.join("a.n");
    let a_source = "val x = 1\nimport \"missing\"\n";
    fs::write(&a_path, a_source).expect("Failed to write a.n");

    let Err(errors) = ModuleGraph::build(a_source, EntryLocation::File(a_path)) else {
        panic!("missing module should be reported");
    };

    let tried = dir.0.canonicalize().unwrap().join("missing.n");
    assert_eq!(1, errors.len());
    assert!(
        errors[0].message.contains(tried.to_str().unwrap()),
        "message was: {}",
        errors[0].message
    );
    assert_eq!(2, errors[0].location.line);
}

#[test]
fn reports_a_parse_error_in_an_imported_file_with_that_files_name_and_line() {
    let dir = TempDir::new("reports_parse_error");
    let a_path = dir.0.join("a.n");
    let b_path = dir.0.join("b.n");
    let a_source = "import \"b\"\n";
    fs::write(&a_path, a_source).expect("Failed to write a.n");
    fs::write(&b_path, "val x = 1\nval = 1\n").expect("Failed to write b.n");

    let Err(errors) = ModuleGraph::build(a_source, EntryLocation::File(a_path)) else {
        panic!("parse error should be reported");
    };

    assert!(!errors.is_empty());
    for error in &errors {
        assert_eq!(Some(b_path.canonicalize().unwrap()), error.file);
        assert_eq!(2, error.location.line);
    }
}

#[test]
fn reports_a_parse_error_in_the_entry_file_without_a_path() {
    let dir = TempDir::new("reports_entry_parse_error");
    let a_path = dir.0.join("a.n");
    let a_source = "val x = 1\nval = 1\n";
    fs::write(&a_path, a_source).expect("Failed to write a.n");

    let Err(errors) = ModuleGraph::build(a_source, EntryLocation::File(a_path.clone())) else {
        panic!("parse error should be reported");
    };

    assert!(!errors.is_empty());
    for error in &errors {
        assert_eq!(None, error.file);
        assert_eq!(2, error.location.line);
    }
}

#[test]
fn marks_std_math_builtin_without_touching_the_disk() {
    let dir = TempDir::new("marks_std_math_builtin");
    let a_path = dir.0.join("a.n");
    let a_source = "import \"std/math\"\n";
    fs::write(&a_path, a_source).expect("Failed to write a.n");

    let graph = ModuleGraph::build(a_source, EntryLocation::File(a_path.clone()))
        .expect("graph should build");

    let modules = graph.modules();
    assert_eq!(2, modules.len());
    assert_eq!(PathBuf::from("std/math"), modules[0].path);
    assert!(modules[0].builtin);
    assert!(modules[0].source.is_empty());
    assert_eq!(a_path.canonicalize().unwrap(), modules[1].path);
    assert!(!modules[1].builtin);
}

#[test]
fn rejects_a_file_import_when_no_entry_path_is_given_with_the_documented_message() {
    let source = "val x = 1\nimport \"b\"\n";

    let Err(errors) = ModuleGraph::build(source, EntryLocation::None) else {
        panic!("file import should be rejected");
    };

    assert_eq!(1, errors.len());
    assert_eq!(
        "file imports are not available in the browser build",
        errors[0].message
    );
    assert_eq!(2, errors[0].location.line);

    let graph = ModuleGraph::build("import \"std/math\"\n", EntryLocation::None)
        .expect("std import should pass through");
    assert!(graph.modules()[0].builtin);
}

#[test]
fn rejects_unknown_std_module_listing_the_builtin_ones() {
    let source = "val x = 1\nimport \"std/nope\"\n";

    let Err(errors) = ModuleGraph::build(source, EntryLocation::Directory(PathBuf::new())) else {
        panic!("unknown std module should be rejected");
    };

    assert_eq!(1, errors.len());
    assert_eq!(CompilationErrorKind::UnknownModule, errors[0].kind);
    assert_eq!(
        "unknown builtin module 'std/nope' (expected one of std/file, std/math, std/pq, std/stdin)",
        errors[0].message
    );
    assert_eq!(2, errors[0].location.line);
}

#[test]
fn builds_a_graph_for_std_math() {
    let graph = ModuleGraph::build(
        "import \"std/math\"\n",
        EntryLocation::Directory(PathBuf::new()),
    );

    assert!(graph.is_ok());
}
