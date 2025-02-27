use serde::{Deserialize, Serialize};

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
    /// Solaris contains platform-specific configuration for Solaris application
    /// containers.
    struct Solaris {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// SMF FMRI which should go "online" before we start the container
        /// process.
        milestone: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Maximum set of privileges any process in this container can obtain.
        limitpriv: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// The maximum amount of shared memory allowed for this container.
        max_shm_memory: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Specification for automatic creation of network resources for this
        /// container.
        anet: Option<Vec<SolarisAnet>>,

        #[serde(default, skip_serializing_if = "Option::is_none", rename = "cappedCPU")]
        /// Set limit on the amount of CPU time that can be used by container.
        capped_cpu: Option<SolarisCappedCPU>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// The physical and swap caps on the memory that can be used by this
        /// container.
        capped_memory: Option<SolarisCappedMemory>,
    }
);

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
    /// SolarisAnet provides the specification for automatic creation of network
    /// resources for this container.
    struct SolarisAnet {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Specify a name for the automatically created VNIC datalink.
        linkname: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Specify the link over which the VNIC will be created.
        lower_link: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// The set of IP addresses that the container can use.
        allowed_address: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Specifies whether allowedAddress limitation is to be applied to the
        /// VNIC.
        configure_allowed_address: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// The value of the optional default router.
        defrouter: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Enable one or more types of link protection.
        link_protection: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Set the VNIC's macAddress.
        mac_address: Option<String>,
    }
);

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
    /// SolarisCappedCPU allows users to set limit on the amount of CPU time
    /// that can be used by container.
    struct SolarisCappedCPU {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// The amount of CPUs.
        ncpus: Option<String>,
    }
);

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
    /// SolarisCappedMemory allows users to set the physical and swap caps on
    /// the memory that can be used by this container.
    struct SolarisCappedMemory {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// The physical caps on the memory.
        physical: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// The swap caps on the memory.
        swap: Option<String>,
    }
);
