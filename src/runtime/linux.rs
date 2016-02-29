use std::collections::BTreeMap;
use serde_value::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdMap {
    #[serde(rename = "hostID")]
    pub host_id: u32,
    #[serde(rename = "containerID")]
    pub container_id: u32,
    pub size: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Runtime {
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    pub namespaces: Vec<Namespace>,
    #[serde(rename = "uidMappings", default, skip_serializing_if="Vec::is_empty")]
    pub uid_mappings: Vec<IdMap>,
    #[serde(rename = "gidMappings", default, skip_serializing_if="Vec::is_empty")]
    pub gid_mappings: Vec<IdMap>,
    #[serde(rename = "rootfsPropagation", default)]
    pub rootfs_propagation: Option<MountPropagation>,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    pub devices: Vec<Device>,

    // Unsupported stubs
    #[serde(default, skip_serializing)]
    pub rlimits: Vec<Value>,
    #[serde(default, skip_serializing_if="BTreeMap::is_empty")]
    pub sysctl: BTreeMap<String, String>,
    #[serde(skip_serializing)]
    pub resources: Value,
    #[serde(rename = "cgroupsPath", skip_serializing)]
    pub cgroups_path: Value,
    #[serde(rename = "apparmorProfile", skip_serializing)]
    pub apparmor_profile: Value,
    #[serde(rename = "selinuxProcessLabel", skip_serializing)]
    pub selinux_process_label: Value,
    #[serde(skip_serializing)]
    pub seccomp: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Namespace {
    #[serde(rename = "type")]
    pub kind: NamespaceKind,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    pub path: String,
    #[serde(rename = "type")]
    pub kind: u8,
    pub major: u64,
    pub minor: u64,
    #[serde(rename = "permissions", default, skip_serializing_if="Option::is_none")]
    pub cgroup_permissions: Option<String>,
    #[serde(rename = "fileMode")]
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
}

string_enum! { NamespaceKind:
    PID => "pid",
    Network => "network",
    Mount => "mount",
    IPC => "ipc",
    UTS => "uts",
    User => "user",
}

string_enum! { MountPropagation:
    Slave => "slave",
    Private => "private",
    Shared => "shared",
}

impl Default for MountPropagation {
    fn default() -> Self {
        MountPropagation::Private
    }
}

impl IdMap {
    pub fn host_id(&self) -> u32 {
        self.host_id
    }

    pub fn container_id(&self) -> u32 {
        self.container_id
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

impl Runtime {
    pub fn namespaces(&self) -> &[Namespace] {
        &self.namespaces
    }

    pub fn uid_mappings(&self) -> &[IdMap] {
        &self.uid_mappings
    }

    pub fn gid_mappings(&self) -> &[IdMap] {
        &self.gid_mappings
    }

    pub fn rootfs_propagation(&self) -> MountPropagation {
        self.rootfs_propagation.unwrap_or_default()
    }

    pub fn devices(&self) -> &[Device] {
        &self.devices
    }
}

impl Namespace {
    pub fn kind(&self) -> NamespaceKind {
        self.kind
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_ref().map(AsRef::as_ref)
    }
}

impl Device {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn kind(&self) -> u8 {
        self.kind
    }

    pub fn major(&self) -> u64 {
        self.major
    }

    pub fn minor(&self) -> u64 {
        self.minor
    }

    pub fn cgroup_permissions(&self) -> Option<&str> {
        self.cgroup_permissions.as_ref().map(AsRef::as_ref)
    }

    pub fn mode(&self) -> u32 {
        self.mode
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn gid(&self) -> u32 {
        self.gid
    }
}
