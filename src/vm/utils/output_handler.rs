use crate::vm::Value;

pub(crate) trait OutputHandler {
    fn println(&mut self, value: Value);
    fn get_output(&self) -> String;
}

pub(crate) struct ConsoleOutputHandler {}

impl ConsoleOutputHandler {
    pub fn new() -> Self {
        ConsoleOutputHandler {}
    }
}

impl OutputHandler for ConsoleOutputHandler {
    fn println(&mut self, value: Value) {
        println!("{}", value);
    }

    fn get_output(&self) -> String {
        String::new()
    }
}

pub(crate) struct StringBuffer {
    string_buffer: String,
}

impl StringBuffer {
    pub fn new() -> Self {
        StringBuffer {
            string_buffer: String::new(),
        }
    }
}

impl OutputHandler for StringBuffer {
    fn println(&mut self, value: Value) {
        self.string_buffer.push_str(value.to_string().as_str());
    }

    fn get_output(&self) -> String {
        self.string_buffer.clone()
    }
}
