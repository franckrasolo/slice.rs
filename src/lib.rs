use glob::Pattern;

pub mod pre_commit;
pub mod pre_push;

pub struct Config {
    ignored_patterns: Vec<Pattern>,
    change_threshold: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ignored_patterns: Config::default_ignored_patterns(),
            change_threshold: 5,
        }
    }
}

impl Config {
    fn default_ignored_patterns() -> Vec<Pattern> {
        let globs = vec![
            "*.adoc",
            "*.asciidoc",
            "*.conf",
            "*.csv",
            "*.dhall",
            "*.diff",
            "*.md",
            "*.nix",
            "*.org",
            "*.patch",
            "*.properties",
            "*.rst",
            "*.sh",
            "*.json",
            "*.lock",
            "*.toml",
            "*.txt",
            "*.yaml",
            "*.yml",
            "*.xml",
            ".*config",
            ".*ignore",
            ".*-version",
            ".gitmodules",
            ".circleci/**",
            ".git/**",
            ".github/**",
            ".gradle/**",
            ".teamcity/**",
            "**/.env",
            "**/Justfile",
            "**/Makefile",
            "**/LICENSE",
            "**/README",
        ];
        globs.iter().map(|glob| Pattern::new(glob).unwrap()).collect()
    }
}

fn zero_oid() -> String {
    "0".repeat(40)
}
