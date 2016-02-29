#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub uid: u32,
    pub gid: u32,
    #[serde(default, skip_serializing_if="Vec::is_empty", rename = "additionalGids")]
    pub additional_gids: Vec<u32>,
}

string_enum! { Capability:
    AuditControl => "CAP_AUDIT_CONTROL",
    AuditRead => "CAP_AUDIT_READ",
    AuditWrite => "CAP_AUDIT_WRITE",
    BlockSuspend => "CAP_BLOCK_SUSPEND",
    Chown => "CAP_CHOWN",
    DacOverride => "CAP_DAC_OVERRIDE",
    DacReadSearch => "CAP_DAC_READ_SEARCH",
    FOwner => "CAP_FOWNER",
    FSetID => "CAP_FSETID",
    IpcLock => "CAP_IPC_LOCK",
    IpcOwner => "CAP_IPC_OWNER",
    Kill => "CAP_KILL",
    Lease => "CAP_LEASE",
    LinuxImmutable => "CAP_LINUX_IMMUTABLE",
    MacAdmin => "CAP_MAC_ADMIN",
    MacOverride => "CAP_MAC_OVERRIDE",
    Mknod => "CAP_MKNOD",
    NetAdmin => "CAP_NET_ADMIN",
    NetBindService => "CAP_NET_BIND_SERVICE",
    NetBroadcast => "CAP_NET_BROADCAST",
    NetRaw => "CAP_NET_RAW",
    SetGID => "CAP_SETGID",
    SetTFCap => "CAP_SETFCAP",
    SetPCap => "CAP_SETPCAP",
    SetUID => "CAP_SETUID",
    SysAdmin => "CAP_SYS_ADMIN",
    SysBoot => "CAP_SYS_BOOT",
    SysChroot => "CAP_SYS_CHROOT",
    SysModule => "CAP_SYS_MODULE",
    SysNice => "CAP_SYS_NICE",
    SysPAcct => "CAP_SYS_PACCT",
    SysPTrace => "CAP_SYS_PTRACE",
    SysRawIO => "CAP_SYS_RAWIO",
    SysResource => "CAP_SYS_RESOURCE",
    SysTime => "CAP_SYS_TIME",
    SysTtyConfig => "CAP_SYS_TTY_CONFIG",
    Syslog => "CAP_SYSLOG",
    WakeAlarm => "CAP_WAKE_ALARM",
}

impl Capability {
    pub fn cap_flag(&self) -> usize {
        match *self {
            Capability::AuditControl => 30,
            Capability::AuditRead => 37,
            Capability::AuditWrite => 29,
            Capability::BlockSuspend => 36,
            Capability::Chown => 0,
            Capability::DacOverride => 1,
            Capability::DacReadSearch => 2,
            Capability::FOwner => 3,
            Capability::FSetID => 4,
            Capability::IpcLock => 14,
            Capability::IpcOwner => 15,
            Capability::Kill => 5,
            Capability::Lease => 28,
            Capability::LinuxImmutable => 9,
            Capability::MacAdmin => 33,
            Capability::MacOverride => 32,
            Capability::Mknod => 27,
            Capability::NetAdmin => 12,
            Capability::NetBindService => 10,
            Capability::NetBroadcast => 11,
            Capability::NetRaw => 13,
            Capability::SetGID => 6,
            Capability::SetTFCap => 31,
            Capability::SetPCap => 8,
            Capability::SetUID => 7,
            Capability::SysAdmin => 21,
            Capability::SysBoot => 22,
            Capability::SysChroot => 18,
            Capability::SysModule => 16,
            Capability::SysNice => 23,
            Capability::SysPAcct => 20,
            Capability::SysPTrace => 19,
            Capability::SysRawIO => 17,
            Capability::SysResource => 24,
            Capability::SysTime => 25,
            Capability::SysTtyConfig => 26,
            Capability::Syslog => 34,
            Capability::WakeAlarm => 35,
        }
    }
}

impl User {
    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn gid(&self) -> u32 {
        self.gid
    }

    pub fn additional_gids(&self) -> &[u32] {
        &self.additional_gids
    }
}
