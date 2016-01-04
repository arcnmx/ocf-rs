use std::iter;
use std::io;
use serde_json;
use serde::{ser, de};
use semver::Version;
use spec::{Env, Process, Spec, Arch, Platform, Root, MountPoint};
use spec::linux;
use runtime::RuntimeSpec;
use version_req;

impl Spec {
    pub fn from_read<R: io::Read>(r: R) -> serde_json::Result<Self> {
        serde_json::from_reader(r)
    }
}

impl RuntimeSpec {
    pub fn from_read<R: io::Read>(r: R) -> serde_json::Result<Self> {
        serde_json::from_reader(r)
    }
}

impl de::Deserialize for Env {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<Self, D::Error> {
        struct V;

        impl de::Visitor for V {
            type Value = Env;

            fn visit_str<E: de::Error>(&mut self, value: &str) -> Result<Self::Value, E> {
                let mut split = value.splitn(2, '=');
                if let Some((key, value)) = split.next().and_then(|k| split.next().map(|v| (k, v))) {
                    Ok(Env {
                        key: key.to_owned(),
                        value: value.to_owned(),
                    })
                } else {
                    Err(E::syntax("missing '=' in environment variable"))
                }
            }
        }

        d.visit(V)
    }
}

impl de::Deserialize for Process {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct ProcessDeserialize {
            #[serde(default)]
            terminal: Option<bool>,
            #[serde(default)]
            user: Option<linux::User>,
            args: Vec<String>,
            #[serde(default)]
            env: Vec<Env>,
            #[serde(default)]
            cwd: Option<String>,
        }

        ProcessDeserialize::deserialize(d).and_then(|process| {
            let (cmd, args) = try!(process.args.split_first()
                .ok_or_else(|| <D::Error as de::Error>::syntax("args must contain at least one value"))
            );
            Ok(Process {
                terminal: process.terminal,
                linux_user: process.user,
                cmd: cmd.to_owned(),
                args: args.to_owned(),
                env: process.env,
                cwd: process.cwd.and_then(|cwd| if cwd.is_empty() { None } else { Some(cwd) }),
            })
        })
    }
}

impl ser::Serialize for Process {
    fn serialize<S: ser::Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        #[derive(Serialize)]
        struct ProcessSerialize<'a> {
            #[serde(skip_serializing_if_none)]
            terminal: Option<bool>,
            #[serde(skip_serializing_if_none)]
            user: &'a Option<linux::User>,
            args: Vec<&'a str>,
            #[serde(skip_serializing_if_empty)]
            env: &'a Vec<Env>,
            #[serde(skip_serializing_if_none)]
            cwd: &'a Option<String>,
        }

        ser::Serialize::serialize(&ProcessSerialize {
            terminal: self.terminal,
            user: &self.linux_user,
            args: iter::once(&self.cmd[..]).chain(self.args.iter().map(AsRef::as_ref)).collect(),
            env: &self.env,
            cwd: &self.cwd,
        }, s)
    }
}

impl ser::Serialize for Env {
    fn serialize<S: ser::Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        format!("{}={}", self.key, self.value).serialize(s)
    }
}

impl de::Deserialize for Spec {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct PlatformDeserialize {
            os: String,
            arch: Arch,
        }

        #[derive(Deserialize)]
        struct LinuxDeserialize {
            #[serde(default)]
            capabilities: Vec<linux::Capability>,
        }

        #[derive(Deserialize)]
        struct SpecDeserialize {
            version: VersionDeserialize,
            platform: PlatformDeserialize,
            process: Process,
            root: Root,
            #[serde(default)]
            hostname: Option<String>,
            #[serde(default)]
            mounts: Vec<MountPoint>,
            #[serde(default)]
            linux: Option<LinuxDeserialize>,
        }

        SpecDeserialize::deserialize(d).and_then(|spec| {
            let PlatformDeserialize { os, arch } = spec.platform;
            Ok(Spec {
                version: try!(if version_req().matches(&spec.version.0) {
                    Ok(spec.version.0)
                } else {
                    Err(<D::Error as de::Error>::syntax("incompatible version"))
                }),
                platform: try!(match &os[..] {
                    "linux" => spec.linux.map(|linux|
                        Ok(Platform::Linux {
                            arch: arch,
                            capabilities: linux.capabilities,
                        })
                    ),
                    _ => Some(Err(<D::Error as de::Error>::syntax("unrecognized operating system"))),
                }.unwrap_or_else(|| Err(<D::Error as de::Error>::syntax("missing platform configuration")))),
                process: spec.process,
                root: spec.root,
                hostname: spec.hostname,
                mounts: spec.mounts,
            })
        })
    }
}

impl ser::Serialize for Spec {
    fn serialize<S: ser::Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        #[derive(Serialize)]
        struct PlatformSerialize<'a> {
            os: &'a str,
            arch: &'a Arch,
        }

        #[derive(Serialize)]
        struct LinuxSerialize<'a> {
            #[serde(skip_serializing_if_empty)]
            capabilities: &'a Vec<linux::Capability>,
        }

        #[derive(Serialize)]
        struct SpecSerialize<'a> {
            version: VersionSerialize<'a>,
            platform: PlatformSerialize<'a>,
            process: &'a Process,
            root: &'a Root,
            #[serde(skip_serializing_if_none)]
            hostname: &'a Option<String>,
            #[serde(skip_serializing_if_empty)]
            mounts: &'a Vec<MountPoint>,
            #[serde(skip_serializing_if_none)]
            linux: Option<LinuxSerialize<'a>>,
        }

        let (os, arch, linux) = match self.platform {
            Platform::Linux { ref arch, ref capabilities } => (
                "linux",
                arch,
                Some(LinuxSerialize {
                    capabilities: capabilities,
                })
            ),
        };

        ser::Serialize::serialize(&SpecSerialize {
            version: VersionSerialize(&self.version),
            platform: PlatformSerialize {
                arch: arch,
                os: os,
            },
            process: &self.process,
            root: &self.root,
            hostname: &self.hostname,
            mounts: &self.mounts,
            linux: linux,
        }, s)
    }
}

struct VersionDeserialize(Version);

impl de::Deserialize for VersionDeserialize {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<Self, D::Error> {
        use std::error::Error;

        struct V;

        impl de::Visitor for V {
            type Value = VersionDeserialize;

            fn visit_str<E: de::Error>(&mut self, value: &str) -> Result<Self::Value, E> {
                Version::parse(value).map(VersionDeserialize).map_err(|e| E::syntax(e.description()))
            }
        }

        d.visit(V)
    }
}

struct VersionSerialize<'a>(&'a Version);

impl<'a> ser::Serialize for VersionSerialize<'a> {
    fn serialize<S: ser::Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.visit_str(&self.0.to_string())
    }
}
