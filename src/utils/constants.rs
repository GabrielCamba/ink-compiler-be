pub const ALLOWED_FEATURES: [&str; 6] = [
    "psp22",
    "psp34",
    "psp37",
    "pausable",
    "ownable",
    "access-control",
];

pub const CARGO_TOML: &str = "[package]\n\
name = \"my_psp22\"\n\
version = \"1.0.0\"\n\
edition = \"2021\"\n\
authors = [\"The best developer ever\"]\n\
\n\
[dependencies]\n\
ink = { version = \"~4.0.0\", default-features = false }\n\
scale = { package = \"parity-scale-codec\", version = \"3\", default-features = false, features = [\"derive\"] }\n\
scale-info = { version = \"2.3\", default-features = false, features = [\"derive\"], optional = true }\n\
# Include brush as a dependency and enable default implementation for PSP22 via brush feature\n\
openbrush = { tag = \"3.0.0\", git = \"https://github.com/727-Ventures/openbrush-contracts\", default-features = false, features = [features_list] }\n\
\n\
[lib]\n\
name = \"my_psp22\"\n\
path = \"lib.rs\"\n\
crate-type = [\n\
    # Used for normal contract Wasm blobs.\n\
    \"cdylib\",\n\
]\n\
\n\
[features]\n\
default = [\"std\"]\n\
std = [\n\
\"ink/std\",\n\
\"scale/std\",\n\
\"scale-info/std\",\n\
\"openbrush/std\",\n\
]\n\
ink-as-dependency = []";
