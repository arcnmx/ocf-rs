use std::fmt;
use semver::Version;
use self::linux::{LinuxUser, LinuxCapability};

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
    pub linux_user: Option<LinuxUser>,
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
        capabilities: Vec<LinuxCapability>,
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
