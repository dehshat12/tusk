#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    // Generic
    Text,

    // Systems / compiled
    Rust,
    C,
    Cpp,
    Header,
    Go,
    Zig,
    Nim,

    // JVM / CLR
    Java,
    Kotlin,
    Scala,
    CSharp,

    // Web / scripting
    JavaScript,
    TypeScript,
    Ruby,
    Python,
    PHP,
    Lua,
    Perl,
    Elixir,
    Shell,

    // Functional
    Haskell,
    OCaml,
    FSharp,
    Erlang,

    // Data / config
    Toml,
    Json,
    Yaml,
    Xml,
    Ini,
    Csv,

    // Build / tooling
    Makefile,
    CMake,
    Gradle,
    Bazel,

    // Docs / markup
    Markdown,
    Html,
    Css,

    // Misc
    Dockerfile,
}

impl FileType {
    pub fn name(&self) -> &'static str {
        match self {
            FileType::Text => "Text",

            FileType::Rust => "Rust",
            FileType::C => "C",
            FileType::Cpp => "C++",
            FileType::Header => "C/C++ Header",
            FileType::Go => "Go",
            FileType::Zig => "Zig",
            FileType::Nim => "Nim",

            FileType::Java => "Java",
            FileType::Kotlin => "Kotlin",
            FileType::Scala => "Scala",
            FileType::CSharp => "C#",

            FileType::JavaScript => "JavaScript",
            FileType::TypeScript => "TypeScript",
            FileType::Ruby => "Ruby",
            FileType::Python => "Python",
            FileType::PHP => "PHP",
            FileType::Lua => "Lua",
            FileType::Perl => "Perl",
            FileType::Elixir => "Elixir",
            FileType::Shell => "Shell",

            FileType::Haskell => "Haskell",
            FileType::OCaml => "OCaml",
            FileType::FSharp => "F#",
            FileType::Erlang => "Erlang",

            FileType::Toml => "TOML",
            FileType::Json => "JSON",
            FileType::Yaml => "YAML",
            FileType::Xml => "XML",
            FileType::Ini => "INI",
            FileType::Csv => "CSV",

            FileType::Makefile => "Makefile",
            FileType::CMake => "CMake",
            FileType::Gradle => "Gradle",
            FileType::Bazel => "Bazel",

            FileType::Markdown => "Markdown",
            FileType::Html => "HTML",
            FileType::Css => "CSS",

            FileType::Dockerfile => "Dockerfile",
        }
    }
}

/// Detect file type from filename (JOE-style)
pub fn detect(path: &str) -> FileType {
    let name = path
        .rsplit('/')
        .next()
        .unwrap_or(path);

    // Exact filename matches
    match name {
        "Makefile" | "makefile" | "GNUmakefile" => return FileType::Makefile,
        "CMakeLists.txt" => return FileType::CMake,
        "Dockerfile" => return FileType::Dockerfile,
        "BUILD" | "BUILD.bazel" => return FileType::Bazel,
        _ => {}
    }

    // Extension-based detection
    match name.rsplit('.').next() {
        Some("rs") => FileType::Rust,
        Some("c") => FileType::C,
        Some("cpp") | Some("cc") | Some("cxx") => FileType::Cpp,
        Some("h") | Some("hpp") => FileType::Header,
        Some("go") => FileType::Go,
        Some("zig") => FileType::Zig,
        Some("nim") => FileType::Nim,

        Some("java") => FileType::Java,
        Some("kt") => FileType::Kotlin,
        Some("scala") => FileType::Scala,
        Some("cs") => FileType::CSharp,

        Some("js") => FileType::JavaScript,
        Some("ts") => FileType::TypeScript,
        Some("rb") => FileType::Ruby,
        Some("py") => FileType::Python,
        Some("php") => FileType::PHP,
        Some("lua") => FileType::Lua,
        Some("pl") => FileType::Perl,
        Some("ex") | Some("exs") => FileType::Elixir,
        Some("sh") => FileType::Shell,

        Some("hs") => FileType::Haskell,
        Some("ml") => FileType::OCaml,
        Some("fs") => FileType::FSharp,
        Some("erl") => FileType::Erlang,

        Some("toml") => FileType::Toml,
        Some("json") => FileType::Json,
        Some("yaml") | Some("yml") => FileType::Yaml,
        Some("xml") => FileType::Xml,
        Some("ini") | Some("conf") => FileType::Ini,
        Some("csv") => FileType::Csv,

        Some("md") => FileType::Markdown,
        Some("html") | Some("htm") => FileType::Html,
        Some("css") => FileType::Css,

        _ => FileType::Text,
    }
}
