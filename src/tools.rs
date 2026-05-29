use phf::phf_map;

pub const TOOLS: phf::Map<&'static str, &'static str> = phf_map! {
    "python" => "python --version",
    "pip" => "pip --version",
    "node" => "node --version",
    "cargo" => "cargo --version",
    "ruby" => "ruby --version",
};
