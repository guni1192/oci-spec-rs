//! Error types of the distribution spec.

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use thiserror::Error;

/// The string returned by and ErrorResponse error.
pub const ERR_REGISTRY: &str = "distribution: registry returned error";

/// Unique identifier representing error code.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    /// Blob unknown to registry.
    BlobUnknown,
    /// Blob upload invalid.
    BlobUploadInvalid,
    /// Blob upload unknown to registry.
    BlobUploadUnknown,
    /// Provided digest did not match uploaded content.
    DigestInvalid,
    /// Blob unknown to registry.
    ManifestBlobUnknown,
    /// Manifest invalid.
    ManifestInvalid,
    /// Manifest unknown.
    ManifestUnknown,
    /// Invalid repository name.
    NameInvalid,
    /// Repository name not known to registry.
    NameUnknown,
    /// Provided length did not match content length.
    SizeInvalid,
    /// Authentication required.
    Unauthorized,
    /// Requested access to the resource is denied.
    Denied,
    /// The operation is unsupported.
    Unsupported,
    /// Too many requests.
    #[serde(rename = "TOOMANYREQUESTS")]
    TooManyRequests,
}

make_pub!(
    #[derive(Clone, Debug, Deserialize, Eq, Error, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder),
        builder(
            pattern = "owned",
            setter(into, strip_option),
            build_fn(error = "crate::error::OciSpecError")
        )
    )]
    /// ErrorResponse is returned by a registry on an invalid request.
    struct ErrorResponse {
        /// Available errors within the response.
        errors: Vec<ErrorInfo>,
    }
);

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", ERR_REGISTRY)
    }
}

impl ErrorResponse {
    /// Returns the ErrorInfo slice for the response.
    pub fn detail(&self) -> &[ErrorInfo] {
        &self.errors
    }
}

make_pub!(
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::Getters),
        builder(
            pattern = "owned",
            setter(into, strip_option),
            build_fn(error = "crate::error::OciSpecError")
        ),
        getset(get = "pub")
    )]
    /// Describes a server error returned from a registry.
    struct ErrorInfo {
        /// The code field MUST be a unique identifier, containing only uppercase alphabetic
        /// characters and underscores.
        code: ErrorCode,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", builder(default = "None"))]
        /// The message field is OPTIONAL, and if present, it SHOULD be a human readable string or
        /// MAY be empty.
        message: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", builder(default = "None"))]
        /// The detail field is OPTIONAL and MAY contain arbitrary JSON data providing information
        /// the client can use to resolve the issue.
        detail: Option<String>,
    }
);

#[cfg(test)]
#[cfg(feature = "builder")]
mod tests {
    use super::*;
    use crate::error::Result;

    #[test]
    fn error_response_success() -> Result<()> {
        let response = ErrorResponseBuilder::default().errors(vec![]).build()?;
        assert!(response.detail().is_empty());
        assert_eq!(response.to_string(), ERR_REGISTRY);
        Ok(())
    }

    #[test]
    fn error_response_failure() {
        assert!(ErrorResponseBuilder::default().build().is_err());
    }

    #[test]
    fn error_info_success() -> Result<()> {
        let info = ErrorInfoBuilder::default()
            .code(ErrorCode::BlobUnknown)
            .build()?;
        assert_eq!(info.code(), &ErrorCode::BlobUnknown);
        assert!(info.message().is_none());
        assert!(info.detail().is_none());
        Ok(())
    }

    #[test]
    fn error_info_failure() {
        assert!(ErrorInfoBuilder::default().build().is_err());
    }
}
