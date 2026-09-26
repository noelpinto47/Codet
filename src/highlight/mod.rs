use std::path::Path;

pub use iced_highlighter::{Highlight, Highlighter, Settings, Theme};

// ── Token detection (file extension → iced_highlighter token) ────────────────
// iced_highlighter matches `token` against file extensions, not language names.

pub fn token_from_path(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("rs")          => "rs",
        Some("js")          => "js",
        Some("ts")          => "ts",
        Some("py")          => "py",
        Some("toml")        => "toml",
        Some("json")        => "json",
        Some("md")          => "md",
        Some("html")        => "html",
        Some("css")         => "css",
        Some("sh")          => "sh",
        Some("yaml")
        | Some("yml")       => "yaml",
        Some("hs")          => "hs",
        Some("cpp")
        | Some("cc")        => "cpp",
        Some("c")           => "c",
        Some("go")          => "go",
        Some("kt")          => "kt",
        Some("swift")       => "swift",
        Some("java")        => "java",
        _                   => "txt",
    }
}

// ── Settings builder ──────────────────────────────────────────────────────────

pub fn settings_for_path(path: &Path) -> Settings {
    Settings {
        theme: Theme::Base16Ocean,
        token: token_from_path(path).to_string(),
    }
}

pub fn default_settings() -> Settings {
    Settings {
        theme: Theme::Base16Ocean,
        token: "txt".to_string(),
    }
}
