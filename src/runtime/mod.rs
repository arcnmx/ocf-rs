use std::collections::BTreeMap;
use spec::Env;

pub mod linux;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeSpec {
    pub hooks: Hooks,
    pub mounts: BTreeMap<String, Mount>,
    pub linux: Option<linux::LinuxRuntime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Command {
    pub path: String,
    #[serde(default, skip_serializing_if_empty)]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if_empty)]
    pub env: Vec<Env>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mount {
    #[serde(rename = "type")]
    pub kind: String,
    pub source: String,
    #[serde(default, skip_serializing_if_empty)]
    pub options: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hooks {
    #[serde(rename = "prestart")]
    pub pre_start: Vec<Command>,
    #[serde(rename = "poststart")]
    pub post_start: Vec<Command>,
    #[serde(rename = "poststop")]
    pub post_stop: Vec<Command>,
}
