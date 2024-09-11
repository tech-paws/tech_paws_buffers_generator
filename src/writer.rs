pub struct Writer {
    res: String,
    tab: String,
    const_tabs: usize,
    contains_tabs: bool,
}

impl Writer {
    pub fn new(tab_size: usize) -> Self {
        Writer {
            res: String::with_capacity(10000),
            tab: " ".repeat(tab_size),
            const_tabs: 0,
            contains_tabs: false,
        }
    }

    pub fn push_tab(&mut self) {
        self.const_tabs += 1;
    }

    pub fn pop_tab(&mut self) {
        self.const_tabs -= 1;
    }

    pub fn show(&self) -> &str {
        &self.res
    }

    pub fn write_tabs(&mut self) {
        if !self.contains_tabs {
            self.res += &self.tab.repeat(self.const_tabs);
            self.contains_tabs = true;
        }
    }

    pub fn write(&mut self, data: &str) {
        self.res += data;
        self.contains_tabs = false;
    }

    pub fn writeln(&mut self, data: &str) {
        self.res += &self.tab.repeat(self.const_tabs);
        self.res += data;
        self.res += "\n";
        self.contains_tabs = false;
    }

    pub fn new_line(&mut self) {
        self.res += "\n";
        self.contains_tabs = false;
    }

    pub fn writeln_tab(&mut self, tab: usize, data: &str) {
        self.res += &self.tab.repeat(self.const_tabs + tab);
        self.res += data;
        self.res += "\n";
    }
}

impl Default for Writer {
    fn default() -> Self {
        Self::new(4)
    }
}
