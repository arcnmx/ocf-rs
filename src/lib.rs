extern crate serde;
extern crate serde_value;
extern crate serde_json;
extern crate semver;

use semver::{Version, VersionReq};

#[macro_use]
mod macros;
mod spec;
mod runtime;
mod serialize;
mod fs;

pub use spec::{Spec, Root, Process, MountPoint, Env, Platform, Arch};
pub use spec::linux::{LinuxUser, LinuxCapability};
pub use runtime::{RuntimeSpec, Command, Mount, Hooks};
pub use runtime::linux::{LinuxIdMap, LinuxRuntime, LinuxNamespace, LinuxNamespaceKind, LinuxMountPropagation};
pub use fs::load;

pub fn version() -> Version {
    Version {
        major: 0,
        minor: 2,
        patch: 0,
        pre: Default::default(),
        build: Default::default(),
    }
}

pub fn version_req() -> VersionReq {
    VersionReq::parse("^0.2.0").unwrap()
}
