pub struct FileType {
    name: String,
    hl_opts: HighlightOptions,
}

#[derive(Default, Clone, Copy)]
pub struct HighlightOptions {
    numbers: bool,
    strings: bool,
    characters: bool,
    comments: bool,
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

    pub fn highlighting_options(&self) -> HighlightOptions {
        self.hl_opts
    }

    pub fn from(file_name: &str) -> Self {
        if file_name.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighlightOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                },
            };
        }
        Self::default()
    }
}

impl HighlightOptions {
    pub fn numbers(self) -> bool {
        self.numbers
    }

    pub fn strings(self) -> bool {
        self.strings
    }
    pub fn characters(self) -> bool {
        self.characters
    }
    pub fn comments(self) -> bool {
        self.comments
    }
}
