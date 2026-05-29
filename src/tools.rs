use phf::phf_map;

pub const TOOLS: phf::Map<&'static str, &'static str> = phf_map! {
    // languages
    "python" => "python --version",
    "python3" => "python3 --version",
    "node" => "node --version",
    "ruby" => "ruby --version",
    "go" => "go version",
    "java" => "java --version",
    "rustc" => "rustc --version",
    "gcc" => "gcc --version",
    "clang" => "clang --version",

    // package managers
    "pip" => "pip --version",
    "pip3" => "pip3 --version",
    "cargo" => "cargo --version",
    "npm" => "npm --version",
    "yarn" => "yarn --version",
    "pnpm" => "pnpm --version",
    "bun" => "bun --version",
    "gem" => "gem --version",

    // tools
    "git" => "git --version",
    "docker" => "docker --version",
    "kubectl" => "kubectl version --client",
    "terraform" => "terraform --version",
    "make" => "make --version",
    "cmake" => "cmake --version",
    "curl" => "curl --version",
    "wget" => "wget --version",
};