use serde::{Deserialize, Serialize};
use std::path::PathBuf;

make_pub!(
    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
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
    /// Hooks specifies a command that is run in the container at a particular
    /// event in the lifecycle (setup and teardown) of a container.
    struct Hooks {
        #[deprecated(
            note = "Prestart hooks were deprecated in favor of `createRuntime`, `createContainer` and `startContainer` hooks"
        )]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// The `prestart` hooks MUST be called after the `start` operation is
        /// called but before the user-specified program command is
        /// executed.
        ///
        /// On Linux, for example, they are called after the container
        /// namespaces are created, so they provide an opportunity to
        /// customize the container (e.g. the network namespace could be
        /// specified in this hook).
        ///
        /// The `prestart` hooks' path MUST resolve in the runtime namespace.
        /// The `prestart` hooks MUST be executed in the runtime namespace.
        prestart: Option<Vec<Hook>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// CreateRuntime is a list of hooks to be run after the container has
        /// been created but before `pivot_root` or any equivalent
        /// operation has been called. It is called in the Runtime
        /// Namespace.
        create_runtime: Option<Vec<Hook>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// CreateContainer is a list of hooks to be run after the container has
        /// been created but before `pivot_root` or any equivalent
        /// operation has been called. It is called in the
        /// Container Namespace.
        create_container: Option<Vec<Hook>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// StartContainer is a list of hooks to be run after the start
        /// operation is called but before the container process is
        /// started. It is called in the Container Namespace.
        start_container: Option<Vec<Hook>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Poststart is a list of hooks to be run after the container process
        /// is started. It is called in the Runtime Namespace.
        poststart: Option<Vec<Hook>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Poststop is a list of hooks to be run after the container process
        /// exits. It is called in the Runtime Namespace.
        poststop: Option<Vec<Hook>>,
    }
);

make_pub!(
    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
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
    /// Hook specifies a command that is run at a particular event in the
    /// lifecycle of a container.
    struct Hook {
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// Path to the binary to be executed. Following similar semantics to
        /// [IEEE Std 1003.1-2008 `execv`'s path](https://pubs.opengroup.org/onlinepubs/9699919799/functions/exec.html). This
        /// specification extends the IEEE standard in that path MUST be
        /// absolute.
        path: PathBuf,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// Arguments used for the binary, including the binary name itself.
        /// Following the same semantics as [IEEE Std 1003.1-2008
        /// `execv`'s argv](https://pubs.opengroup.org/onlinepubs/9699919799/functions/exec.html).
        args: Option<Vec<String>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// Additional `key=value` environment variables. Following the same
        /// semantics as [IEEE Std 1003.1-2008's `environ`](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap08.html#tag_08_01).
        env: Option<Vec<String>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get_copy = "pub"))]
        /// Timeout is the number of seconds before aborting the hook. If set,
        /// timeout MUST be greater than zero.
        timeout: Option<i64>,
    }
);
