use crate::compiler::{CommentKind, Compiler};

#[test]
fn compile_exposes_comments_and_blank_lines_as_trivia() {
    let source = "// hello\nvar a = 1\n\nvar b = 2\n";

    let mut compiler = Compiler::new();
    assert!(compiler.compile(source).is_some());

    let trivia = compiler.get_trivia();
    assert_eq!(trivia.comments.len(), 1);
    assert_eq!(trivia.comments[0].text, "// hello");
    assert_eq!(trivia.comments[0].kind, CommentKind::OwnLine);
    assert_eq!(trivia.blank_lines, vec![3]);
}
