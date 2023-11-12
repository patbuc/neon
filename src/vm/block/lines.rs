use crate::vm::block::{Block, Line};

impl Block {
    pub(super) fn add_line(&mut self, offset: usize, line: usize) {
        self.lines.push(Line { offset, line });
    }

    pub(super) fn get_line(&self, offset: usize) -> usize {
        let mut line = 0;
        let mut low = 0;
        let mut high = self.lines.len() - 1;

        while low <= high {
            let mid = (low + high) / 2;
            let l = self.lines.get(mid).unwrap();
            if l.offset > offset {
                high = mid - 1;
            } else {
                line = l.line;
                low = mid + 1;
            }
        }
        line
    }
}
