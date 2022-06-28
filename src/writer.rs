pub struct Writer {
    res: String,
    tab: String,
}

impl Writer {
    pub fn new(tab_size: usize) -> Self {
        Writer {
            res: String::with_capacity(10000),
            tab: " ".repeat(tab_size),
        }
    }

    pub fn show(&self) -> &str {
        &self.res
    }

    pub fn write(&mut self, data: &str) {
        self.res += data;
    }

    pub fn writeln(&mut self, data: &str) {
        self.res += data;
        self.res += "\n";
    }

    pub fn writeln_tab(&mut self, tab: usize, data: &str) {
        self.res += &self.tab.repeat(tab);
        self.res += data;
        self.res += "\n";
    }
}

impl Default for Writer {
    fn default() -> Self {
        Self::new(4)
    }
}
