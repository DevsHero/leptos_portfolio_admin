pub const SKILL_LEVELS: [(&str, &str); 3] = [
    ("1", "Basic"),
    ("2", "Middle"),
    ("3", "Expert"),
];
pub const LANGUAGE_LEVELS: [(&str, &str); 4] = [
    ("1", "Basic"),
    ("2", "Intermediate"),
    ("3", "Proficiency"),
    ("4", "Native"),
];
pub const PROFILE_CACHE_KEY: &str = "profile";
pub const CACHE_TTL: u64 = 2592000;
pub const PDF_FILE_NAME: &str = "portfolio.pdf";
pub const PDF_DIR: &str = "./pdf";
pub const PDF_FULL_PATH: &str = "pdf/portfolio.pdf";
