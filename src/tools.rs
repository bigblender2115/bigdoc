use phf::phf_map;

pub const TOOLS: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    // languages
    "python" => &["python", "--version"],
    "python3" => &["python3", "--version"],
    "node" => &["node", "--version"],
    "ruby" => &["ruby", "--version"],
    "go" => &["go", "version"],
    "java" => &["java", "--version"],
    "rustc" => &["rustc", "--version"],
    "gcc" => &["gcc", "--version"],
    "clang" => &["clang", "--version"],

    // package managers
    "pip" => &["pip", "--version"],
    "pip3" => &["pip3", "--version"],
    "cargo" => &["cargo", "--version"],
    "npm" => &["npm", "--version"],
    "yarn" => &["yarn", "--version"],
    "pnpm" => &["pnpm", "--version"],
    "bun" => &["bun", "--version"],
    "gem" => &["gem", "--version"],

    // tools
    "git" => &["git", "--version"],
    "docker" => &["docker", "--version"],
    "kubectl" => &["kubectl", "version", "--client"],
    "terraform" => &["terraform", "--version"],
    "make" => &["make", "--version"],
    "cmake" => &["cmake", "--version"],
    "curl" => &["curl", "--version"],
    "wget" => &["wget", "--version"],
};

pub const FIX_COMMANDS: phf::Map<&'static str, &'static str> = phf_map! {
    // languages
    "python" => "https://www.python.org/downloads/",
    "python3" => "https://www.python.org/downloads/",
    "node" => "https://nodejs.org/en/download",
    "ruby" => "https://www.ruby-lang.org/en/documentation/installation/",
    "go" => "https://go.dev/dl/",
    "java" => "https://adoptium.net/",
    "rustc" => "https://rustup.rs/",
    "gcc" => "https://gcc.gnu.org/install/",
    "clang" => "https://releases.llvm.org/",
    // package managers
    "pip" => "python -m ensurepip --upgrade",
    "pip3" => "python3 -m ensurepip --upgrade",
    "cargo" => "https://rustup.rs/",
    "npm" => "https://nodejs.org/en/download",
    "yarn" => "npm install -g yarn",
    "pnpm" => "npm install -g pnpm",
    "bun" => "https://bun.sh/",
    "gem" => "https://www.ruby-lang.org/en/documentation/installation/",
    // tools
    "git" => "https://git-scm.com/downloads",
    "docker" => "https://docs.docker.com/get-docker/",
    "kubectl" => "https://kubernetes.io/docs/tasks/tools/",
    "terraform" => "https://developer.hashicorp.com/terraform/install",
    "make" => "https://www.gnu.org/software/make/",
    "cmake" => "https://cmake.org/download/",
    "curl" => "https://curl.se/download.html",
    "wget" => "https://www.gnu.org/software/wget/",
};