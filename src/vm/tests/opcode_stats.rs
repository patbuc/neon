use crate::common::opcodes::OpCode;
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

fn pair_section(report: &str) -> &str {
    report
        .split("\n\n")
        .nth(1)
        .unwrap_or_else(|| panic!("expected a pair section in report:\n{}", report))
}

fn parse_entries(section: &str) -> Vec<(&str, u64)> {
    section
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let name = parts.next().unwrap();
            let count = parts.next().unwrap().parse().unwrap();
            (name, count)
        })
        .collect()
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
fn for_in_loop_counts_and_orders_opcode_pairs() {
    let vm = run(r#"
        for (i in 0..10) {
            val x = i + 1
        }
        "#);

    let report = vm.opcode_stats_report();
    let section = pair_section(&report);
    assert_eq!(10, count_for(section, "GetLocal->Constant"));
    assert_eq!(10, count_for(section, "Constant->Add"));

    let entries = parse_entries(section);
    let counts: Vec<u64> = entries.iter().map(|(_, count)| *count).collect();
    let mut sorted = counts.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    assert_eq!(sorted, counts);

    // These seven pairs all execute 10 times; ties break by ascending
    // (prev byte, next byte).
    let mut expected = [
        (OpCode::GetLocal, OpCode::Constant),
        (OpCode::Constant, OpCode::Add),
        (OpCode::Add, OpCode::Pop),
        (OpCode::Pop, OpCode::Loop),
        (OpCode::Pop, OpCode::IteratorNext),
        (OpCode::Loop, OpCode::IteratorDone),
        (OpCode::IteratorNext, OpCode::GetLocal),
    ];
    expected.sort_by_key(|(prev, next)| (*prev as u8, *next as u8));
    let expected: Vec<String> = expected
        .iter()
        .map(|(prev, next)| format!("{:?}->{:?}", prev, next))
        .collect();
    let tied: Vec<&str> = entries
        .iter()
        .filter(|(_, count)| *count == 10)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(expected, tied);
}

#[test]
fn opcode_pairs_span_call_and_return() {
    let vm = run(r#"
        fn f(x) {
            return x
        }
        val y = f(1)
        "#);

    let report = vm.opcode_stats_report();
    let section = pair_section(&report);
    assert_eq!(1, count_for(section, "Call->GetLocal"));
    assert_eq!(1, count_for(section, "Return->SetLocal"));
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

    let opcode_section = report.split("\n\n").next().unwrap();
    let entries = parse_entries(opcode_section);

    let counts: Vec<u64> = entries.iter().map(|(_, count)| *count).collect();
    let mut sorted = counts.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    assert_eq!(sorted, counts);

    // Add, GetLocal, Loop, and IteratorNext all execute 10 times; ties break
    // by ascending opcode byte.
    let mut expected = [
        OpCode::Add,
        OpCode::GetLocal,
        OpCode::Loop,
        OpCode::IteratorNext,
    ];
    expected.sort_by_key(|op| *op as u8);
    let expected: Vec<String> = expected.iter().map(|op| format!("{:?}", op)).collect();
    let tied: Vec<&str> = entries
        .iter()
        .filter(|(_, count)| *count == 10)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(expected, tied);
}
