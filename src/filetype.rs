pub struct FileType {
    name: String,
    hl_opts: HighlightOptions,
}

#[derive(Default, Clone, Copy)]
pub struct HighlightOptions {
    numbers: bool,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighlightOptions::default(),
        }
    }
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn highlight_options(&self) -> HighlightOptions {
        self.hl_opts
    }

    pub fn from(file_name: &str) -> Self {
        if file_name.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighlightOptions { numbers: true },
            };
        }
        Self::default()
    }
}

impl HighlightOptions {
    pub fn numbers(self) -> bool {
        self.numbers
    }
}
