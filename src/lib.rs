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
pub use runtime::{RuntimeSpec, Command, Mount, Hooks};
pub use fs::load;

pub mod linux {
    pub use super::spec::linux::{User, Capability};
    pub use super::runtime::linux::{IdMap, Runtime, Namespace, NamespaceKind, MountPropagation, Device};
}

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
