use std::collections::BTreeMap;
use serde_value::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinuxIdMap {
    #[serde(rename = "hostID")]
    pub host_id: u32,
    #[serde(rename = "containerID")]
    pub container_id: u32,
    pub size: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinuxRuntime {
    #[serde(default, skip_serializing_if_empty)]
    pub namespaces: Vec<LinuxNamespace>,
    #[serde(rename = "uidMappings", default, skip_serializing_if_empty)]
    pub uid_mappings: Vec<LinuxIdMap>,
    #[serde(rename = "gidMappings", default, skip_serializing_if_empty)]
    pub gid_mappings: Vec<LinuxIdMap>,
    #[serde(rename = "rootfsPropagation", default)]
    pub rootfs_propagation: Option<LinuxMountPropagation>,
    #[serde(default, skip_serializing_if_empty)]
    pub devices: Vec<LinuxDevice>,

    // Unsupported stubs
    #[serde(default, skip_serializing)]
    pub rlimits: Vec<Value>,
    #[serde(default, skip_serializing_if_empty)]
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
pub struct LinuxNamespace {
    #[serde(rename = "type")]
    pub kind: LinuxNamespaceKind,
    #[serde(default, skip_serializing_if_none)]
    pub path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinuxDevice {
    pub path: String,
    #[serde(rename = "type")]
    pub kind: u8,
    pub major: u64,
    pub minor: u64,
    #[serde(rename = "permissions", default, skip_serializing_if_none)]
    pub cgroup_permissions: Option<String>,
    #[serde(rename = "fileMode")]
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
}

string_enum! { LinuxNamespaceKind:
    PID => "pid",
    Network => "network",
    Mount => "mount",
    IPC => "ipc",
    UTS => "uts",
    User => "user",
}

string_enum! { LinuxMountPropagation:
    Slave => "slave",
    Private => "private",
    Shared => "shared",
}

impl Default for LinuxMountPropagation {
    fn default() -> Self {
        LinuxMountPropagation::Private
    }
}
