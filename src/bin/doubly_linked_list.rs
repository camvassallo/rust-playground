use std::collections::linked_list::CursorMut;
use std::collections::LinkedList;
use std::io::Cursor;

fn main() {

}

#[test]
fn browser_history() {

}

struct BrowserHistory {
    // current: CursorMut<'_, String>
    history: LinkedList<String>
}

impl BrowserHistory {
    fn new() -> Self {
        BrowserHistory {
            history: LinkedList::new()
        }
    }

    fn visit(&mut self, site: &str) {
        self.history
    }
}