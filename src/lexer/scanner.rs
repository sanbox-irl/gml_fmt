pub struct Scanner {
    input: &'static str,
}

impl Scanner {
    pub fn new(input: &'static str) -> Scanner {
        Scanner { input }
    }
}
