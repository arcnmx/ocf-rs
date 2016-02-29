use std::collections::BTreeMap;
use spec::Env;

pub mod linux;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeSpec {
    pub hooks: Hooks,
    pub mounts: BTreeMap<String, Mount>,
    pub linux: Option<linux::Runtime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Command {
    pub path: String,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    pub env: Vec<Env>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mount {
    #[serde(rename = "type")]
    pub kind: String,
    pub source: String,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
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

impl RuntimeSpec {
    pub fn hooks(&self) -> &Hooks {
        &self.hooks
    }

    pub fn mounts(&self) -> &BTreeMap<String, Mount> {
        &self.mounts
    }

    pub fn linux(&self) -> Option<&linux::Runtime> {
        self.linux.as_ref()
    }
}

impl Command {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }

    pub fn env(&self) -> &[Env] {
        &self.env
    }
}

impl Mount {
    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn options(&self) -> &[String] {
        &self.options
    }
}

impl Hooks {
    pub fn pre_start(&self) -> &[Command] {
        &self.pre_start
    }

    pub fn post_start(&self) -> &[Command] {
        &self.post_start
    }

    pub fn post_stop(&self) -> &[Command] {
        &self.post_stop
    }
}
