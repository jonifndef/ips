pub struct ColorTokens {}

impl ColorTokens {
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    //pub const BRIGHT_RED: &str = "\x1b[91m";
    pub const BRIGHT_GREEN: &str = "\x1b[92m";
    //pub const BRIGHT_YELLOW: &str = "\x1b[93m";
    //pub const BRIGHT_BLUE: &str = "\x1b[94m";
    pub const ENDING: &str = "\x1b[0m";
    pub const TOKENS_LEN: usize = 9;
}
