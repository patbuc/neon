use crate::vm::Value;

pub(crate) trait OutputHandler {
    fn println(&self, value: Value);
}

pub(crate) struct ConsoleOutputHandler {}

impl OutputHandler for ConsoleOutputHandler {
    fn println(&self, value: Value) {
        println!("{}", value);
    }
}
