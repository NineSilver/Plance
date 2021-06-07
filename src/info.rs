use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BuildOpts {
    pub compiler: String,
    pub flags: Option<Vec<String>>,
    // TODO
    // pub linker: Option<String>,
    // pub linker_flags: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub project_type: String,
    pub files: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub project_info: ProjectInfo,
    pub build_opts: Option<BuildOpts>,
}
