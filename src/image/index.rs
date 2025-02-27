use super::{Descriptor, MediaType};
use crate::{error::Result, from_file, from_reader, to_file, to_writer};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Read, Write},
    path::Path,
};

/// The expected schema version; equals 2 for compatibility with older versions of Docker.
pub const SCHEMA_VERSION: u32 = 2;

make_pub!(
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::CopyGetters, getset::Getters),
        builder(
            pattern = "owned",
            setter(into, strip_option),
            build_fn(error = "crate::error::OciSpecError")
        )
    )]
    /// The image index is a higher-level manifest which points to specific
    /// image manifests, ideal for one or more platforms. While the use of
    /// an image index is OPTIONAL for image providers, image consumers
    /// SHOULD be prepared to process them.
    struct ImageIndex {
        /// This REQUIRED property specifies the image manifest schema version.
        /// For this version of the specification, this MUST be 2 to ensure
        /// backward compatibility with older versions of Docker. The
        /// value of this field will not change. This field MAY be
        /// removed in a future version of the specification.
        #[cfg_attr(feature = "builder", getset(get_copy = "pub"))]
        schema_version: u32,
        /// This property is reserved for use, to maintain compatibility. When
        /// used, this field contains the media type of this document,
        /// which differs from the descriptor use of mediaType.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"), builder(default))]
        media_type: Option<MediaType>,
        /// This REQUIRED property contains a list of manifests for specific
        /// platforms. While this property MUST be present, the size of
        /// the array MAY be zero.
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        manifests: Vec<Descriptor>,
        /// This OPTIONAL property contains arbitrary metadata for the image
        /// index. This OPTIONAL property MUST use the annotation rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"), builder(default))]
        annotations: Option<HashMap<String, String>>,
    }
);

impl ImageIndex {
    /// Attempts to load an image index from a file.
    /// # Errors
    /// This function will return an [OciSpecError::Io](crate::OciSpecError::Io)
    /// if the file does not exist or an
    /// [OciSpecError::SerDe](crate::OciSpecError::SerDe) if the image index
    /// cannot be deserialized.
    /// # Example
    /// ``` no_run
    /// use oci_spec::image::ImageIndex;
    ///
    /// let image_index = ImageIndex::from_file("index.json").unwrap();
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<ImageIndex> {
        from_file(path)
    }

    /// Attempts to load an image index from a stream.
    /// # Errors
    /// This function will return an [OciSpecError::SerDe](crate::OciSpecError::SerDe)
    /// if the index cannot be deserialized.
    /// # Example
    /// ``` no_run
    /// use oci_spec::image::ImageIndex;
    /// use std::fs::File;
    ///
    /// let reader = File::open("index.json").unwrap();
    /// let image_index = ImageIndex::from_reader(reader).unwrap();
    /// ```
    pub fn from_reader<R: Read>(reader: R) -> Result<ImageIndex> {
        from_reader(reader)
    }

    /// Attempts to write an image index to a file as JSON. If the file already exists, it
    /// will be overwritten.
    /// # Errors
    /// This function will return an [OciSpecError::SerDe](crate::OciSpecError::SerDe) if
    /// the image index cannot be serialized.
    /// # Example
    /// ``` no_run
    /// use oci_spec::image::ImageIndex;
    ///
    /// let image_index = ImageIndex::from_file("index.json").unwrap();
    /// image_index.to_file("my-index.json").unwrap();
    /// ```
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        to_file(&self, path, false)
    }

    /// Attempts to write an image index to a file as pretty printed JSON. If the file
    /// already exists, it will be overwritten.
    /// # Errors
    /// This function will return an [OciSpecError::SerDe](crate::OciSpecError::SerDe) if
    /// the image index cannot be serialized.
    /// # Example
    /// ``` no_run
    /// use oci_spec::image::ImageIndex;
    ///
    /// let image_index = ImageIndex::from_file("index.json").unwrap();
    /// image_index.to_file_pretty("my-index.json").unwrap();
    /// ```
    pub fn to_file_pretty<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        to_file(&self, path, true)
    }

    /// Attempts to write an image index to a stream as JSON.
    /// # Errors
    /// This function will return an [OciSpecError::SerDe](crate::OciSpecError::SerDe) if
    /// the image index cannot be serialized.
    /// # Example
    /// ``` no_run
    /// use oci_spec::image::ImageIndex;
    ///
    /// let image_index = ImageIndex::from_file("index.json").unwrap();
    /// let mut writer = Vec::new();
    /// image_index.to_writer(&mut writer);
    /// ```
    pub fn to_writer<W: Write>(&self, writer: &mut W) -> Result<()> {
        to_writer(&self, writer, false)
    }

    /// Attempts to write an image index to a stream as pretty printed JSON.
    /// # Errors
    /// This function will return an [OciSpecError::SerDe](crate::OciSpecError::SerDe) if
    /// the image index cannot be serialized.
    /// # Example
    /// ``` no_run
    /// use oci_spec::image::ImageIndex;
    ///
    /// let image_index = ImageIndex::from_file("index.json").unwrap();
    /// let mut writer = Vec::new();
    /// image_index.to_writer_pretty(&mut writer);
    /// ```
    pub fn to_writer_pretty<W: Write>(&self, writer: &mut W) -> Result<()> {
        to_writer(&self, writer, true)
    }
}

impl Default for ImageIndex {
    fn default() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            media_type: Default::default(),
            manifests: Default::default(),
            annotations: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;
    use crate::image::{Arch, Os};
    #[cfg(not(feature = "builder"))]
    use crate::image::{Descriptor, Platform};
    #[cfg(feature = "builder")]
    use crate::image::{DescriptorBuilder, PlatformBuilder};

    #[cfg(feature = "builder")]
    fn create_index() -> ImageIndex {
        let ppc_manifest = DescriptorBuilder::default()
            .media_type(MediaType::ImageManifest)
            .digest("sha256:e692418e4cbaf90ca69d05a66403747baa33ee08806650b51fab815ad7fc331f")
            .size(7143)
            .platform(
                PlatformBuilder::default()
                    .architecture(Arch::PowerPC64le)
                    .os(Os::Linux)
                    .build()
                    .expect("build ppc64le platform"),
            )
            .build()
            .expect("build ppc manifest descriptor");

        let amd64_manifest = DescriptorBuilder::default()
            .media_type(MediaType::ImageManifest)
            .digest("sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270")
            .size(7682)
            .platform(
                PlatformBuilder::default()
                    .architecture(Arch::Amd64)
                    .os(Os::Linux)
                    .build()
                    .expect("build amd64 platform"),
            )
            .build()
            .expect("build amd64 manifest descriptor");

        let index = ImageIndexBuilder::default()
            .schema_version(SCHEMA_VERSION)
            .manifests(vec![ppc_manifest, amd64_manifest])
            .build()
            .expect("build image index");

        index
    }

    #[cfg(not(feature = "builder"))]
    fn create_index() -> ImageIndex {
        let ppc_manifest = {
            let mut r = Descriptor::new(
                MediaType::ImageManifest,
                7143,
                "sha256:e692418e4cbaf90ca69d05a66403747baa33ee08806650b51fab815ad7fc331f",
            );
            r.platform = Some(Platform {
                architecture: Arch::PowerPC64le,
                os: Os::Linux,
                os_version: None,
                os_features: None,
                variant: None,
            });
            r
        };

        let amd64_manifest = Descriptor {
            media_type: MediaType::ImageManifest,
            digest: "sha256:5b0bcabd1ed22e9fb1310cf6c2dec7cdef19f0ad69efa1f392e94a4333501270"
                .to_owned(),
            size: 7682,
            urls: None,
            annotations: None,
            platform: Some(Platform {
                architecture: Arch::Amd64,
                os: Os::Linux,
                os_version: None,
                os_features: None,
                variant: None,
            }),
        };

        let index = ImageIndex {
            schema_version: SCHEMA_VERSION,
            media_type: None,
            manifests: vec![ppc_manifest, amd64_manifest],
            annotations: None,
        };

        index
    }

    fn get_index_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test/data/index.json")
    }

    #[test]
    fn load_index_from_file() {
        // arrange
        let index_path = get_index_path();

        // act
        let actual = ImageIndex::from_file(index_path).expect("from file");

        // assert
        let expected = create_index();
        assert_eq!(actual, expected);
    }

    #[test]
    fn load_index_from_reader() {
        // arrange
        let reader = fs::read(get_index_path()).expect("read index");

        // act
        let actual = ImageIndex::from_reader(&*reader).expect("from reader");

        // assert
        let expected = create_index();
        assert_eq!(actual, expected);
    }

    #[test]
    fn save_index_to_file() {
        // arrange
        let tmp = std::env::temp_dir().join("save_index_to_file");
        fs::create_dir_all(&tmp).expect("create test directory");
        let index = create_index();
        let index_path = tmp.join("index.json");

        // act
        index
            .to_file_pretty(&index_path)
            .expect("write index to file");

        // assert
        let actual = fs::read_to_string(index_path).expect("read actual");
        let expected = fs::read_to_string(get_index_path()).expect("read expected");
        assert_eq!(actual, expected);
    }

    #[test]
    fn save_index_to_writer() {
        // arrange
        let mut actual = Vec::new();
        let index = create_index();

        // act
        index.to_writer_pretty(&mut actual).expect("to writer");

        // assert
        let expected = fs::read(get_index_path()).expect("read expected");
        assert_eq!(actual, expected);
    }
}
