#[cfg(test)]
mod test_args_global {
    use crate::compiler::semantic::SemanticAnalyzer;
    use crate::compiler::parser::Parser;

    #[test]
    fn test_args_is_predefined() {
        let source = "print(args)";
        let mut parser = Parser::new(source);
        let ast = parser.parse().unwrap();

        let mut analyzer = SemanticAnalyzer::new();
        let result = analyzer.analyze(&ast);

        assert!(result.is_ok(), "args should be predefined as a built-in global");
    }

    #[test]
    fn test_args_can_be_accessed() {
        let source = "val x = args";
        let mut parser = Parser::new(source);
        let ast = parser.parse().unwrap();

        let mut analyzer = SemanticAnalyzer::new();
        let result = analyzer.analyze(&ast);

        assert!(result.is_ok(), "args should be accessible");
    }
}
