use crate::vm::{InterpretResult, VirtualMachine};

fn run(program: &str) -> VirtualMachine {
    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(InterpretResult::Ok, result);
    vm
}

fn count_for(report: &str, opcode_name: &str) -> u64 {
    report
        .lines()
        .find_map(|line| {
            let mut parts = line.split_whitespace();
            let name = parts.next()?;
            let count = parts.next()?;
            (name == opcode_name).then(|| count.parse().unwrap())
        })
        .unwrap_or_else(|| panic!("opcode {} not found in report:\n{}", opcode_name, report))
}

#[test]
fn for_in_loop_runs_loop_opcode_once_per_iteration() {
    let vm = run(r#"
        for (i in 0..10) {
            val x = i + 1
        }
        "#);

    let report = vm.opcode_stats_report();
    assert_eq!(10, count_for(&report, "Loop"));
    assert_eq!(10, count_for(&report, "Add"));
}

#[test]
fn report_is_sorted_descending_with_tie_break_by_opcode_byte() {
    let vm = run(r#"
        for (i in 0..10) {
            val x = i + 1
        }
        "#);

    let report = vm.opcode_stats_report();
    assert!(!report.is_empty());

    let entries: Vec<(&str, u64)> = report
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let name = parts.next().unwrap();
            let count = parts.next().unwrap().parse().unwrap();
            (name, count)
        })
        .collect();

    let counts: Vec<u64> = entries.iter().map(|(_, count)| *count).collect();
    let mut sorted = counts.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    assert_eq!(sorted, counts);

    // Add, GetLocal, Loop, and IteratorNext all execute 10 times; ties break
    // by ascending opcode byte, so they appear in this order.
    let tied: Vec<&str> = entries
        .iter()
        .filter(|(_, count)| *count == 10)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(vec!["Add", "GetLocal", "Loop", "IteratorNext"], tied);
}
