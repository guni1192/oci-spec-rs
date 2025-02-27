use serde::{Deserialize, Serialize};
use std::path::PathBuf;

make_pub!(
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::CopyGetters, getset::Getters),
        builder(
            default,
            pattern = "owned",
            setter(into, strip_option),
            build_fn(error = "crate::error::OciSpecError")
        )
    )]
    /// Root contains information about the container's root filesystem on the
    /// host.
    struct Root {
        /// Path is the absolute path to the container's root filesystem.
        #[serde(default)]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        path: PathBuf,

        /// Readonly makes the root filesystem for the container readonly before
        /// the process is executed.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get_copy = "pub"))]
        readonly: Option<bool>,
    }
);

/// Default path for container root is "./rootfs" from config.json, with
/// readonly true
impl Default for Root {
    fn default() -> Self {
        Root {
            path: PathBuf::from("rootfs"),
            readonly: true.into(),
        }
    }
}

make_pub!(
    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::Getters),
        builder(
            default,
            pattern = "owned",
            setter(into, strip_option),
            build_fn(error = "crate::error::OciSpecError")
        ),
        getset(get = "pub")
    )]
    /// Mount specifies a mount for a container.
    struct Mount {
        /// Destination is the absolute path where the mount will be placed in
        /// the container.
        destination: PathBuf,

        #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
        /// Type specifies the mount kind.
        typ: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Source specifies the source path of the mount.
        source: Option<PathBuf>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Options are fstab style mount options.
        options: Option<Vec<String>>,
    }
);

/// utility function to generate default config for mounts.
pub fn get_default_mounts() -> Vec<Mount> {
    vec![
        Mount {
            destination: PathBuf::from("/proc"),
            typ: "proc".to_string().into(),
            source: PathBuf::from("proc").into(),
            options: None,
        },
        Mount {
            destination: PathBuf::from("/dev"),
            typ: "tmpfs".to_string().into(),
            source: PathBuf::from("tmpfs").into(),
            options: vec![
                "nosuid".into(),
                "strictatime".into(),
                "mode=755".into(),
                "size=65536k".into(),
            ]
            .into(),
        },
        Mount {
            destination: PathBuf::from("/dev/pts"),
            typ: "devpts".to_string().into(),
            source: PathBuf::from("devpts").into(),
            options: vec![
                "nosuid".into(),
                "noexec".into(),
                "newinstance".into(),
                "ptmxmode=0666".into(),
                "mode=0620".into(),
                "gid=5".into(),
            ]
            .into(),
        },
        Mount {
            destination: PathBuf::from("/dev/shm"),
            typ: "tmpfs".to_string().into(),
            source: PathBuf::from("shm").into(),
            options: vec![
                "nosuid".into(),
                "noexec".into(),
                "nodev".into(),
                "mode=1777".into(),
                "size=65536k".into(),
            ]
            .into(),
        },
        Mount {
            destination: PathBuf::from("/dev/mqueue"),
            typ: "mqueue".to_string().into(),
            source: PathBuf::from("mqueue").into(),
            options: vec!["nosuid".into(), "noexec".into(), "nodev".into()].into(),
        },
        Mount {
            destination: PathBuf::from("/sys"),
            typ: "sysfs".to_string().into(),
            source: PathBuf::from("sysfs").into(),
            options: vec![
                "nosuid".into(),
                "noexec".into(),
                "nodev".into(),
                "ro".into(),
            ]
            .into(),
        },
        Mount {
            destination: PathBuf::from("/sys/fs/cgroup"),
            typ: "cgroup".to_string().into(),
            source: PathBuf::from("cgroup").into(),
            options: vec![
                "nosuid".into(),
                "noexec".into(),
                "nodev".into(),
                "relatime".into(),
                "ro".into(),
            ]
            .into(),
        },
    ]
}
