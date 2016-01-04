use std::fmt;
use semver::Version;

pub mod linux;

#[derive(Clone, Debug)]
pub struct Spec {
    pub version: Version,
    pub platform: Platform,
    pub process: Process,
    pub root: Root,
    pub hostname: Option<String>,
    pub mounts: Vec<MountPoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Root {
    pub path: String,
    #[serde(default, skip_serializing_if_none)]
    pub readonly: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct Process {
    pub terminal: Option<bool>,
    pub linux_user: Option<linux::User>,
    pub cmd: String,
    pub args: Vec<String>,
    pub env: Vec<Env>,
    pub cwd: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MountPoint {
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug)]
pub struct Env {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub enum Platform {
    Linux {
        arch: Arch,
        capabilities: Vec<linux::Capability>,
    },
}

string_enum! { Arch:
    X86 => "386",
    X86_64 => "amd64",
    ARM => "arm",
    ARM64 => "arm64",
    PowerPC64 => "ppc64",
    PowerPC64LE => "ppc64le",
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

impl Spec {
    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn platform(&self) -> &Platform {
        &self.platform
    }

    pub fn process(&self) -> &Process {
        &self.process
    }

    pub fn root(&self) -> &Root {
        &self.root
    }

    pub fn hostname(&self) -> Option<&str> {
        self.hostname.as_ref().map(AsRef::as_ref)
    }

    pub fn mounts(&self) -> &[MountPoint] {
        &self.mounts
    }
}

impl Root {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn readonly(&self) -> bool {
        self.readonly.unwrap_or(false)
    }
}

impl Process {
    pub fn terminal(&self) -> bool {
        self.terminal.unwrap_or(false)
    }

    pub fn linux_user(&self) -> Option<&linux::User> {
        self.linux_user.as_ref()
    }

    pub fn cmd(&self) -> &str {
        &self.cmd
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }

    pub fn env(&self) -> &[Env] {
        &self.env
    }

    pub fn cwd(&self) -> Option<&str> {
        self.cwd.as_ref().map(AsRef::as_ref)
    }
}

impl MountPoint {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Env {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
