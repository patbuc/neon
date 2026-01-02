#[cfg(test)]
mod test_args_global {
    use crate::compiler::parser::Parser;
    use crate::compiler::semantic::SemanticAnalyzer;

    #[test]
    fn test_args_is_predefined() {
        let source = "print(args)";
        let mut parser = Parser::new(source);
        let ast = parser.parse().unwrap();

        let mut analyzer = SemanticAnalyzer::new();
        let result = analyzer.analyze(&ast);

        assert!(
            result.is_ok(),
            "args should be predefined as a built-in global"
        );
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

    #[test]
    fn asdf() {
        let program = r#"
        val test = [
"987654321111111",
"811111111111119",
"234234234234278",
"818181911112111",
]

val f = File("/Users/patricbucher/Develop/aoc-neon/aoc31.txt")
val input = f.readLines()
val lines = input[0].split(",")

fn reduce(line, remove) {
    // Build characters in an array, then join once
    var chars = []
    val n = line.len()
    for (var i = 0; i < n; i++) {
        if (remove.contains(i)) {
            continue
        }
        chars.push(line.charAt(i))
    }
    return chars.join("")
}

var password = 0
for (line in input) {

    var the_line = line
    while (the_line.len() > 12) {
        val remove = []
        for (var j = 0; j < the_line.len() - 1; j++) {
            val a = the_line.charAt(j).toInt()
            val b = the_line.charAt(j + 1).toInt()
            if (a < b) {
                remove.push(j)
                break
            }
        }
        if (remove.size() == 0) {
            remove.push(the_line.len() - 1)
        }
        the_line = reduce(the_line, remove)
    }

    password = password + the_line.toInt()
    // print(the_line)
}

print("Password: " + password.toString())
"#;
        let mut vm = crate::vm::VirtualMachine::new();
        match vm.interpret(program.to_string()) {
            crate::vm::Result::Ok => {
                println!("Output:\n{}", vm.get_output());
            }
            crate::vm::Result::CompileError => {
                let formatted_errors = vm.get_formatted_errors("test.neon");
                panic!("Compilation failed:\n{}", formatted_errors);
            }
            crate::vm::Result::RuntimeError => {
                panic!("Runtime error:\n{}", vm.get_output());
            }
        }
    }
}
