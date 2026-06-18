#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///`App`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "App",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "bundleId": {
    ///          "type": "string"
    ///        },
    ///        "name": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ResourceLinks"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "apps"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct App {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<AppAttributes>,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ResourceLinks>,
        #[serde(rename = "type")]
        pub type_: AppType,
    }

    ///`AppAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "bundleId": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppAttributes {
        #[serde(
            rename = "bundleId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bundle_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for AppAttributes {
        fn default() -> Self {
            Self {
                bundle_id: Default::default(),
                name: Default::default(),
            }
        }
    }

    ///`AppResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppResponse",
    ///  "type": "object",
    ///  "required": [
    ///    "data",
    ///    "links"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "$ref": "#/components/schemas/App"
    ///    },
    ///    "included": {
    ///      "type": "array",
    ///      "items": {
    ///        "oneOf": [
    ///          {
    ///            "$ref": "#/components/schemas/Build"
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/DocumentLinks"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppResponse {
        pub data: App,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub included: ::std::vec::Vec<Build>,
        pub links: DocumentLinks,
    }

    ///`AppStoreVersion`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppStoreVersion",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "appStoreState": {
    ///          "$ref": "#/components/schemas/AppStoreVersionState"
    ///        },
    ///        "createdDate": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        },
    ///        "earliestReleaseDate": {
    ///          "type": "string",
    ///          "format": "date-time"
    ///        },
    ///        "versionString": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ResourceLinks"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appStoreVersions"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppStoreVersion {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<AppStoreVersionAttributes>,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ResourceLinks>,
        #[serde(rename = "type")]
        pub type_: AppStoreVersionType,
    }

    ///`AppStoreVersionAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "appStoreState": {
    ///      "$ref": "#/components/schemas/AppStoreVersionState"
    ///    },
    ///    "createdDate": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "earliestReleaseDate": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "versionString": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppStoreVersionAttributes {
        #[serde(
            rename = "appStoreState",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub app_store_state: ::std::option::Option<AppStoreVersionState>,
        #[serde(
            rename = "createdDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub created_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "earliestReleaseDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub earliest_release_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "versionString",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub version_string: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for AppStoreVersionAttributes {
        fn default() -> Self {
            Self {
                app_store_state: Default::default(),
                created_date: Default::default(),
                earliest_release_date: Default::default(),
                version_string: Default::default(),
            }
        }
    }

    ///`AppStoreVersionLocalization`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppStoreVersionLocalization",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "description": {
    ///          "type": "string"
    ///        },
    ///        "keywords": {
    ///          "type": "string"
    ///        },
    ///        "locale": {
    ///          "type": "string"
    ///        },
    ///        "marketingUrl": {
    ///          "type": "string",
    ///          "format": "uri"
    ///        },
    ///        "promotionalText": {
    ///          "type": "string"
    ///        },
    ///        "supportUrl": {
    ///          "type": "string",
    ///          "format": "uri"
    ///        },
    ///        "whatsNew": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ResourceLinks"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "appStoreVersionLocalizations"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppStoreVersionLocalization {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<AppStoreVersionLocalizationAttributes>,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ResourceLinks>,
        #[serde(rename = "type")]
        pub type_: AppStoreVersionLocalizationType,
    }

    ///`AppStoreVersionLocalizationAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "keywords": {
    ///      "type": "string"
    ///    },
    ///    "locale": {
    ///      "type": "string"
    ///    },
    ///    "marketingUrl": {
    ///      "type": "string",
    ///      "format": "uri"
    ///    },
    ///    "promotionalText": {
    ///      "type": "string"
    ///    },
    ///    "supportUrl": {
    ///      "type": "string",
    ///      "format": "uri"
    ///    },
    ///    "whatsNew": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppStoreVersionLocalizationAttributes {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub keywords: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub locale: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "marketingUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub marketing_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "promotionalText",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub promotional_text: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "supportUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub support_url: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "whatsNew",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub whats_new: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for AppStoreVersionLocalizationAttributes {
        fn default() -> Self {
            Self {
                description: Default::default(),
                keywords: Default::default(),
                locale: Default::default(),
                marketing_url: Default::default(),
                promotional_text: Default::default(),
                support_url: Default::default(),
                whats_new: Default::default(),
            }
        }
    }

    ///`AppStoreVersionLocalizationType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreVersionLocalizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionLocalizationType {
        #[serde(rename = "appStoreVersionLocalizations")]
        AppStoreVersionLocalizations,
    }

    impl ::std::fmt::Display for AppStoreVersionLocalizationType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreVersionLocalizations => f.write_str("appStoreVersionLocalizations"),
            }
        }
    }

    impl ::std::str::FromStr for AppStoreVersionLocalizationType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreVersionLocalizations" => Ok(Self::AppStoreVersionLocalizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppStoreVersionLocalizationType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppStoreVersionLocalizationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppStoreVersionLocalizationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ACCEPTED",
    ///    "DEVELOPER_REMOVED_FROM_SALE",
    ///    "DEVELOPER_REJECTED",
    ///    "IN_REVIEW",
    ///    "INVALID_BINARY",
    ///    "METADATA_REJECTED",
    ///    "PENDING_APPLE_RELEASE",
    ///    "PENDING_CONTRACT",
    ///    "PENDING_DEVELOPER_RELEASE",
    ///    "PREPARE_FOR_SUBMISSION",
    ///    "PREORDER_READY_FOR_SALE",
    ///    "PROCESSING_FOR_APP_STORE",
    ///    "READY_FOR_REVIEW",
    ///    "READY_FOR_SALE",
    ///    "REJECTED",
    ///    "REMOVED_FROM_SALE",
    ///    "WAITING_FOR_EXPORT_COMPLIANCE",
    ///    "WAITING_FOR_REVIEW",
    ///    "REPLACED_WITH_NEW_VERSION",
    ///    "NOT_APPLICABLE"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionState {
        #[serde(rename = "ACCEPTED")]
        Accepted,
        #[serde(rename = "DEVELOPER_REMOVED_FROM_SALE")]
        DeveloperRemovedFromSale,
        #[serde(rename = "DEVELOPER_REJECTED")]
        DeveloperRejected,
        #[serde(rename = "IN_REVIEW")]
        InReview,
        #[serde(rename = "INVALID_BINARY")]
        InvalidBinary,
        #[serde(rename = "METADATA_REJECTED")]
        MetadataRejected,
        #[serde(rename = "PENDING_APPLE_RELEASE")]
        PendingAppleRelease,
        #[serde(rename = "PENDING_CONTRACT")]
        PendingContract,
        #[serde(rename = "PENDING_DEVELOPER_RELEASE")]
        PendingDeveloperRelease,
        #[serde(rename = "PREPARE_FOR_SUBMISSION")]
        PrepareForSubmission,
        #[serde(rename = "PREORDER_READY_FOR_SALE")]
        PreorderReadyForSale,
        #[serde(rename = "PROCESSING_FOR_APP_STORE")]
        ProcessingForAppStore,
        #[serde(rename = "READY_FOR_REVIEW")]
        ReadyForReview,
        #[serde(rename = "READY_FOR_SALE")]
        ReadyForSale,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "REMOVED_FROM_SALE")]
        RemovedFromSale,
        #[serde(rename = "WAITING_FOR_EXPORT_COMPLIANCE")]
        WaitingForExportCompliance,
        #[serde(rename = "WAITING_FOR_REVIEW")]
        WaitingForReview,
        #[serde(rename = "REPLACED_WITH_NEW_VERSION")]
        ReplacedWithNewVersion,
        #[serde(rename = "NOT_APPLICABLE")]
        NotApplicable,
    }

    impl ::std::fmt::Display for AppStoreVersionState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Accepted => f.write_str("ACCEPTED"),
                Self::DeveloperRemovedFromSale => f.write_str("DEVELOPER_REMOVED_FROM_SALE"),
                Self::DeveloperRejected => f.write_str("DEVELOPER_REJECTED"),
                Self::InReview => f.write_str("IN_REVIEW"),
                Self::InvalidBinary => f.write_str("INVALID_BINARY"),
                Self::MetadataRejected => f.write_str("METADATA_REJECTED"),
                Self::PendingAppleRelease => f.write_str("PENDING_APPLE_RELEASE"),
                Self::PendingContract => f.write_str("PENDING_CONTRACT"),
                Self::PendingDeveloperRelease => f.write_str("PENDING_DEVELOPER_RELEASE"),
                Self::PrepareForSubmission => f.write_str("PREPARE_FOR_SUBMISSION"),
                Self::PreorderReadyForSale => f.write_str("PREORDER_READY_FOR_SALE"),
                Self::ProcessingForAppStore => f.write_str("PROCESSING_FOR_APP_STORE"),
                Self::ReadyForReview => f.write_str("READY_FOR_REVIEW"),
                Self::ReadyForSale => f.write_str("READY_FOR_SALE"),
                Self::Rejected => f.write_str("REJECTED"),
                Self::RemovedFromSale => f.write_str("REMOVED_FROM_SALE"),
                Self::WaitingForExportCompliance => f.write_str("WAITING_FOR_EXPORT_COMPLIANCE"),
                Self::WaitingForReview => f.write_str("WAITING_FOR_REVIEW"),
                Self::ReplacedWithNewVersion => f.write_str("REPLACED_WITH_NEW_VERSION"),
                Self::NotApplicable => f.write_str("NOT_APPLICABLE"),
            }
        }
    }

    impl ::std::str::FromStr for AppStoreVersionState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ACCEPTED" => Ok(Self::Accepted),
                "DEVELOPER_REMOVED_FROM_SALE" => Ok(Self::DeveloperRemovedFromSale),
                "DEVELOPER_REJECTED" => Ok(Self::DeveloperRejected),
                "IN_REVIEW" => Ok(Self::InReview),
                "INVALID_BINARY" => Ok(Self::InvalidBinary),
                "METADATA_REJECTED" => Ok(Self::MetadataRejected),
                "PENDING_APPLE_RELEASE" => Ok(Self::PendingAppleRelease),
                "PENDING_CONTRACT" => Ok(Self::PendingContract),
                "PENDING_DEVELOPER_RELEASE" => Ok(Self::PendingDeveloperRelease),
                "PREPARE_FOR_SUBMISSION" => Ok(Self::PrepareForSubmission),
                "PREORDER_READY_FOR_SALE" => Ok(Self::PreorderReadyForSale),
                "PROCESSING_FOR_APP_STORE" => Ok(Self::ProcessingForAppStore),
                "READY_FOR_REVIEW" => Ok(Self::ReadyForReview),
                "READY_FOR_SALE" => Ok(Self::ReadyForSale),
                "REJECTED" => Ok(Self::Rejected),
                "REMOVED_FROM_SALE" => Ok(Self::RemovedFromSale),
                "WAITING_FOR_EXPORT_COMPLIANCE" => Ok(Self::WaitingForExportCompliance),
                "WAITING_FOR_REVIEW" => Ok(Self::WaitingForReview),
                "REPLACED_WITH_NEW_VERSION" => Ok(Self::ReplacedWithNewVersion),
                "NOT_APPLICABLE" => Ok(Self::NotApplicable),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppStoreVersionState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppStoreVersionState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppStoreVersionState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreVersions"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppStoreVersionType {
        #[serde(rename = "appStoreVersions")]
        AppStoreVersions,
    }

    impl ::std::fmt::Display for AppStoreVersionType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreVersions => f.write_str("appStoreVersions"),
            }
        }
    }

    impl ::std::str::FromStr for AppStoreVersionType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreVersions" => Ok(Self::AppStoreVersions),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppStoreVersionType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppStoreVersionType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppStoreVersionType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppStoreVersionsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppStoreVersionsResponse",
    ///  "type": "object",
    ///  "required": [
    ///    "data",
    ///    "links"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AppStoreVersion"
    ///      }
    ///    },
    ///    "included": {
    ///      "type": "array",
    ///      "items": {
    ///        "oneOf": [
    ///          {
    ///            "$ref": "#/components/schemas/AppStoreVersionLocalization"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/Build"
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/PagedDocumentLinks"
    ///    },
    ///    "meta": {
    ///      "$ref": "#/components/schemas/PagingInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppStoreVersionsResponse {
        pub data: ::std::vec::Vec<AppStoreVersion>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub included: ::std::vec::Vec<AppStoreVersionsResponseIncludedItem>,
        pub links: PagedDocumentLinks,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub meta: ::std::option::Option<PagingInformation>,
    }

    ///`AppStoreVersionsResponseIncludedItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AppStoreVersionLocalization"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Build"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum AppStoreVersionsResponseIncludedItem {
        AppStoreVersionLocalization(AppStoreVersionLocalization),
        Build(Build),
    }

    impl ::std::convert::From<AppStoreVersionLocalization> for AppStoreVersionsResponseIncludedItem {
        fn from(value: AppStoreVersionLocalization) -> Self {
            Self::AppStoreVersionLocalization(value)
        }
    }

    impl ::std::convert::From<Build> for AppStoreVersionsResponseIncludedItem {
        fn from(value: Build) -> Self {
            Self::Build(value)
        }
    }

    ///`AppType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "apps"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppType {
        #[serde(rename = "apps")]
        Apps,
    }

    impl ::std::fmt::Display for AppType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Apps => f.write_str("apps"),
            }
        }
    }

    impl ::std::str::FromStr for AppType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "apps" => Ok(Self::Apps),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "sourceFileChecksum",
    ///    "versions"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem {
        #[serde(rename = "sourceFileChecksum")]
        SourceFileChecksum,
        #[serde(rename = "versions")]
        Versions,
    }

    impl ::std::fmt::Display
        for AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::SourceFileChecksum => f.write_str("sourceFileChecksum"),
                Self::Versions => f.write_str("versions"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "sourceFileChecksum" => Ok(Self::SourceFileChecksum),
                "versions" => Ok(Self::Versions),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "action",
    ///    "appClip",
    ///    "releaseWithAppStoreVersion",
    ///    "appClipDefaultExperienceLocalizations",
    ///    "appClipAppStoreReviewDetail"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem {
        #[serde(rename = "action")]
        Action,
        #[serde(rename = "appClip")]
        AppClip,
        #[serde(rename = "releaseWithAppStoreVersion")]
        ReleaseWithAppStoreVersion,
        #[serde(rename = "appClipDefaultExperienceLocalizations")]
        AppClipDefaultExperienceLocalizations,
        #[serde(rename = "appClipAppStoreReviewDetail")]
        AppClipAppStoreReviewDetail,
    }

    impl ::std::fmt::Display
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Action => f.write_str("action"),
                Self::AppClip => f.write_str("appClip"),
                Self::ReleaseWithAppStoreVersion => f.write_str("releaseWithAppStoreVersion"),
                Self::AppClipDefaultExperienceLocalizations => {
                    f.write_str("appClipDefaultExperienceLocalizations")
                }
                Self::AppClipAppStoreReviewDetail => f.write_str("appClipAppStoreReviewDetail"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "action" => Ok(Self::Action),
                "appClip" => Ok(Self::AppClip),
                "releaseWithAppStoreVersion" => Ok(Self::ReleaseWithAppStoreVersion),
                "appClipDefaultExperienceLocalizations" => {
                    Ok(Self::AppClipDefaultExperienceLocalizations)
                }
                "appClipAppStoreReviewDetail" => Ok(Self::AppClipAppStoreReviewDetail),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "contactFirstName",
    ///    "contactLastName",
    ///    "contactPhone",
    ///    "contactEmail",
    ///    "demoAccountName",
    ///    "demoAccountPassword",
    ///    "demoAccountRequired",
    ///    "notes",
    ///    "appStoreVersion",
    ///    "appStoreReviewAttachments"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem {
        #[serde(rename = "contactFirstName")]
        ContactFirstName,
        #[serde(rename = "contactLastName")]
        ContactLastName,
        #[serde(rename = "contactPhone")]
        ContactPhone,
        #[serde(rename = "contactEmail")]
        ContactEmail,
        #[serde(rename = "demoAccountName")]
        DemoAccountName,
        #[serde(rename = "demoAccountPassword")]
        DemoAccountPassword,
        #[serde(rename = "demoAccountRequired")]
        DemoAccountRequired,
        #[serde(rename = "notes")]
        Notes,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "appStoreReviewAttachments")]
        AppStoreReviewAttachments,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ContactFirstName => f.write_str("contactFirstName"),
                Self::ContactLastName => f.write_str("contactLastName"),
                Self::ContactPhone => f.write_str("contactPhone"),
                Self::ContactEmail => f.write_str("contactEmail"),
                Self::DemoAccountName => f.write_str("demoAccountName"),
                Self::DemoAccountPassword => f.write_str("demoAccountPassword"),
                Self::DemoAccountRequired => f.write_str("demoAccountRequired"),
                Self::Notes => f.write_str("notes"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
                Self::AppStoreReviewAttachments => f.write_str("appStoreReviewAttachments"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "contactFirstName" => Ok(Self::ContactFirstName),
                "contactLastName" => Ok(Self::ContactLastName),
                "contactPhone" => Ok(Self::ContactPhone),
                "contactEmail" => Ok(Self::ContactEmail),
                "demoAccountName" => Ok(Self::DemoAccountName),
                "demoAccountPassword" => Ok(Self::DemoAccountPassword),
                "demoAccountRequired" => Ok(Self::DemoAccountRequired),
                "notes" => Ok(Self::Notes),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                "appStoreReviewAttachments" => Ok(Self::AppStoreReviewAttachments),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "trafficProportion",
    ///    "state",
    ///    "reviewRequired",
    ///    "startDate",
    ///    "endDate",
    ///    "appStoreVersion",
    ///    "appStoreVersionExperimentTreatments",
    ///    "platform",
    ///    "app",
    ///    "latestControlVersion",
    ///    "controlVersions"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "trafficProportion")]
        TrafficProportion,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "reviewRequired")]
        ReviewRequired,
        #[serde(rename = "startDate")]
        StartDate,
        #[serde(rename = "endDate")]
        EndDate,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "appStoreVersionExperimentTreatments")]
        AppStoreVersionExperimentTreatments,
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "latestControlVersion")]
        LatestControlVersion,
        #[serde(rename = "controlVersions")]
        ControlVersions,
    }

    impl ::std::fmt::Display
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::TrafficProportion => f.write_str("trafficProportion"),
                Self::State => f.write_str("state"),
                Self::ReviewRequired => f.write_str("reviewRequired"),
                Self::StartDate => f.write_str("startDate"),
                Self::EndDate => f.write_str("endDate"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
                Self::AppStoreVersionExperimentTreatments => {
                    f.write_str("appStoreVersionExperimentTreatments")
                }
                Self::Platform => f.write_str("platform"),
                Self::App => f.write_str("app"),
                Self::LatestControlVersion => f.write_str("latestControlVersion"),
                Self::ControlVersions => f.write_str("controlVersions"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "trafficProportion" => Ok(Self::TrafficProportion),
                "state" => Ok(Self::State),
                "reviewRequired" => Ok(Self::ReviewRequired),
                "startDate" => Ok(Self::StartDate),
                "endDate" => Ok(Self::EndDate),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                "appStoreVersionExperimentTreatments" => {
                    Ok(Self::AppStoreVersionExperimentTreatments)
                }
                "platform" => Ok(Self::Platform),
                "app" => Ok(Self::App),
                "latestControlVersion" => Ok(Self::LatestControlVersion),
                "controlVersions" => Ok(Self::ControlVersions),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "description",
    ///    "locale",
    ///    "keywords",
    ///    "marketingUrl",
    ///    "promotionalText",
    ///    "supportUrl",
    ///    "whatsNew",
    ///    "appStoreVersion",
    ///    "appScreenshotSets",
    ///    "appPreviewSets",
    ///    "searchKeywords"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem {
        #[serde(rename = "description")]
        Description,
        #[serde(rename = "locale")]
        Locale,
        #[serde(rename = "keywords")]
        Keywords,
        #[serde(rename = "marketingUrl")]
        MarketingUrl,
        #[serde(rename = "promotionalText")]
        PromotionalText,
        #[serde(rename = "supportUrl")]
        SupportUrl,
        #[serde(rename = "whatsNew")]
        WhatsNew,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "appScreenshotSets")]
        AppScreenshotSets,
        #[serde(rename = "appPreviewSets")]
        AppPreviewSets,
        #[serde(rename = "searchKeywords")]
        SearchKeywords,
    }

    impl ::std::fmt::Display
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Description => f.write_str("description"),
                Self::Locale => f.write_str("locale"),
                Self::Keywords => f.write_str("keywords"),
                Self::MarketingUrl => f.write_str("marketingUrl"),
                Self::PromotionalText => f.write_str("promotionalText"),
                Self::SupportUrl => f.write_str("supportUrl"),
                Self::WhatsNew => f.write_str("whatsNew"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
                Self::AppScreenshotSets => f.write_str("appScreenshotSets"),
                Self::AppPreviewSets => f.write_str("appPreviewSets"),
                Self::SearchKeywords => f.write_str("searchKeywords"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "description" => Ok(Self::Description),
                "locale" => Ok(Self::Locale),
                "keywords" => Ok(Self::Keywords),
                "marketingUrl" => Ok(Self::MarketingUrl),
                "promotionalText" => Ok(Self::PromotionalText),
                "supportUrl" => Ok(Self::SupportUrl),
                "whatsNew" => Ok(Self::WhatsNew),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                "appScreenshotSets" => Ok(Self::AppScreenshotSets),
                "appPreviewSets" => Ok(Self::AppPreviewSets),
                "searchKeywords" => Ok(Self::SearchKeywords),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "phasedReleaseState",
    ///    "startDate",
    ///    "totalPauseDuration",
    ///    "currentDayNumber"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem {
        #[serde(rename = "phasedReleaseState")]
        PhasedReleaseState,
        #[serde(rename = "startDate")]
        StartDate,
        #[serde(rename = "totalPauseDuration")]
        TotalPauseDuration,
        #[serde(rename = "currentDayNumber")]
        CurrentDayNumber,
    }

    impl ::std::fmt::Display
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::PhasedReleaseState => f.write_str("phasedReleaseState"),
                Self::StartDate => f.write_str("startDate"),
                Self::TotalPauseDuration => f.write_str("totalPauseDuration"),
                Self::CurrentDayNumber => f.write_str("currentDayNumber"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "phasedReleaseState" => Ok(Self::PhasedReleaseState),
                "startDate" => Ok(Self::StartDate),
                "totalPauseDuration" => Ok(Self::TotalPauseDuration),
                "currentDayNumber" => Ok(Self::CurrentDayNumber),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreVersion"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem {
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
    }

    impl ::std::fmt::Display
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
            }
        }
    }

    impl ::std::str::FromStr
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem
    {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "versionString",
    ///    "appStoreState",
    ///    "appVersionState",
    ///    "copyright",
    ///    "reviewType",
    ///    "releaseType",
    ///    "earliestReleaseDate",
    ///    "usesIdfa",
    ///    "downloadable",
    ///    "createdDate",
    ///    "app",
    ///    "appStoreVersionLocalizations",
    ///    "build",
    ///    "appStoreVersionPhasedRelease",
    ///    "gameCenterAppVersion",
    ///    "routingAppCoverage",
    ///    "appStoreReviewDetail",
    ///    "appStoreVersionSubmission",
    ///    "appClipDefaultExperience",
    ///    "appStoreVersionExperiments",
    ///    "appStoreVersionExperimentsV2",
    ///    "customerReviews",
    ///    "alternativeDistributionPackage"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "versionString")]
        VersionString,
        #[serde(rename = "appStoreState")]
        AppStoreState,
        #[serde(rename = "appVersionState")]
        AppVersionState,
        #[serde(rename = "copyright")]
        Copyright,
        #[serde(rename = "reviewType")]
        ReviewType,
        #[serde(rename = "releaseType")]
        ReleaseType,
        #[serde(rename = "earliestReleaseDate")]
        EarliestReleaseDate,
        #[serde(rename = "usesIdfa")]
        UsesIdfa,
        #[serde(rename = "downloadable")]
        Downloadable,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appStoreVersionLocalizations")]
        AppStoreVersionLocalizations,
        #[serde(rename = "build")]
        Build,
        #[serde(rename = "appStoreVersionPhasedRelease")]
        AppStoreVersionPhasedRelease,
        #[serde(rename = "gameCenterAppVersion")]
        GameCenterAppVersion,
        #[serde(rename = "routingAppCoverage")]
        RoutingAppCoverage,
        #[serde(rename = "appStoreReviewDetail")]
        AppStoreReviewDetail,
        #[serde(rename = "appStoreVersionSubmission")]
        AppStoreVersionSubmission,
        #[serde(rename = "appClipDefaultExperience")]
        AppClipDefaultExperience,
        #[serde(rename = "appStoreVersionExperiments")]
        AppStoreVersionExperiments,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "customerReviews")]
        CustomerReviews,
        #[serde(rename = "alternativeDistributionPackage")]
        AlternativeDistributionPackage,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::VersionString => f.write_str("versionString"),
                Self::AppStoreState => f.write_str("appStoreState"),
                Self::AppVersionState => f.write_str("appVersionState"),
                Self::Copyright => f.write_str("copyright"),
                Self::ReviewType => f.write_str("reviewType"),
                Self::ReleaseType => f.write_str("releaseType"),
                Self::EarliestReleaseDate => f.write_str("earliestReleaseDate"),
                Self::UsesIdfa => f.write_str("usesIdfa"),
                Self::Downloadable => f.write_str("downloadable"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::App => f.write_str("app"),
                Self::AppStoreVersionLocalizations => f.write_str("appStoreVersionLocalizations"),
                Self::Build => f.write_str("build"),
                Self::AppStoreVersionPhasedRelease => f.write_str("appStoreVersionPhasedRelease"),
                Self::GameCenterAppVersion => f.write_str("gameCenterAppVersion"),
                Self::RoutingAppCoverage => f.write_str("routingAppCoverage"),
                Self::AppStoreReviewDetail => f.write_str("appStoreReviewDetail"),
                Self::AppStoreVersionSubmission => f.write_str("appStoreVersionSubmission"),
                Self::AppClipDefaultExperience => f.write_str("appClipDefaultExperience"),
                Self::AppStoreVersionExperiments => f.write_str("appStoreVersionExperiments"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::CustomerReviews => f.write_str("customerReviews"),
                Self::AlternativeDistributionPackage => {
                    f.write_str("alternativeDistributionPackage")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "versionString" => Ok(Self::VersionString),
                "appStoreState" => Ok(Self::AppStoreState),
                "appVersionState" => Ok(Self::AppVersionState),
                "copyright" => Ok(Self::Copyright),
                "reviewType" => Ok(Self::ReviewType),
                "releaseType" => Ok(Self::ReleaseType),
                "earliestReleaseDate" => Ok(Self::EarliestReleaseDate),
                "usesIdfa" => Ok(Self::UsesIdfa),
                "downloadable" => Ok(Self::Downloadable),
                "createdDate" => Ok(Self::CreatedDate),
                "app" => Ok(Self::App),
                "appStoreVersionLocalizations" => Ok(Self::AppStoreVersionLocalizations),
                "build" => Ok(Self::Build),
                "appStoreVersionPhasedRelease" => Ok(Self::AppStoreVersionPhasedRelease),
                "gameCenterAppVersion" => Ok(Self::GameCenterAppVersion),
                "routingAppCoverage" => Ok(Self::RoutingAppCoverage),
                "appStoreReviewDetail" => Ok(Self::AppStoreReviewDetail),
                "appStoreVersionSubmission" => Ok(Self::AppStoreVersionSubmission),
                "appClipDefaultExperience" => Ok(Self::AppClipDefaultExperience),
                "appStoreVersionExperiments" => Ok(Self::AppStoreVersionExperiments),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "customerReviews" => Ok(Self::CustomerReviews),
                "alternativeDistributionPackage" => Ok(Self::AlternativeDistributionPackage),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "accessibilityUrl",
    ///    "name",
    ///    "bundleId",
    ///    "sku",
    ///    "primaryLocale",
    ///    "isOrEverWasMadeForKids",
    ///    "subscriptionStatusUrl",
    ///    "subscriptionStatusUrlVersion",
    ///    "subscriptionStatusUrlForSandbox",
    ///    "subscriptionStatusUrlVersionForSandbox",
    ///    "contentRightsDeclaration",
    ///    "streamlinedPurchasingEnabled",
    ///    "accessibilityDeclarations",
    ///    "appEncryptionDeclarations",
    ///    "appStoreIcon",
    ///    "ciProduct",
    ///    "betaTesters",
    ///    "betaGroups",
    ///    "appStoreVersions",
    ///    "appTags",
    ///    "preReleaseVersions",
    ///    "betaAppLocalizations",
    ///    "builds",
    ///    "betaLicenseAgreement",
    ///    "betaAppReviewDetail",
    ///    "appInfos",
    ///    "appClips",
    ///    "appPricePoints",
    ///    "endUserLicenseAgreement",
    ///    "appPriceSchedule",
    ///    "appAvailabilityV2",
    ///    "inAppPurchases",
    ///    "subscriptionGroups",
    ///    "gameCenterEnabledVersions",
    ///    "perfPowerMetrics",
    ///    "appCustomProductPages",
    ///    "inAppPurchasesV2",
    ///    "promotedPurchases",
    ///    "appEvents",
    ///    "reviewSubmissions",
    ///    "subscriptionGracePeriod",
    ///    "customerReviews",
    ///    "customerReviewSummarizations",
    ///    "gameCenterDetail",
    ///    "appStoreVersionExperimentsV2",
    ///    "alternativeDistributionKey",
    ///    "analyticsReportRequests",
    ///    "marketplaceSearchDetail",
    ///    "buildUploads",
    ///    "backgroundAssets",
    ///    "betaFeedbackScreenshotSubmissions",
    ///    "betaFeedbackCrashSubmissions",
    ///    "searchKeywords",
    ///    "webhooks",
    ///    "androidToIosAppMappingDetails"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem {
        #[serde(rename = "accessibilityUrl")]
        AccessibilityUrl,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "sku")]
        Sku,
        #[serde(rename = "primaryLocale")]
        PrimaryLocale,
        #[serde(rename = "isOrEverWasMadeForKids")]
        IsOrEverWasMadeForKids,
        #[serde(rename = "subscriptionStatusUrl")]
        SubscriptionStatusUrl,
        #[serde(rename = "subscriptionStatusUrlVersion")]
        SubscriptionStatusUrlVersion,
        #[serde(rename = "subscriptionStatusUrlForSandbox")]
        SubscriptionStatusUrlForSandbox,
        #[serde(rename = "subscriptionStatusUrlVersionForSandbox")]
        SubscriptionStatusUrlVersionForSandbox,
        #[serde(rename = "contentRightsDeclaration")]
        ContentRightsDeclaration,
        #[serde(rename = "streamlinedPurchasingEnabled")]
        StreamlinedPurchasingEnabled,
        #[serde(rename = "accessibilityDeclarations")]
        AccessibilityDeclarations,
        #[serde(rename = "appEncryptionDeclarations")]
        AppEncryptionDeclarations,
        #[serde(rename = "appStoreIcon")]
        AppStoreIcon,
        #[serde(rename = "ciProduct")]
        CiProduct,
        #[serde(rename = "betaTesters")]
        BetaTesters,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "appStoreVersions")]
        AppStoreVersions,
        #[serde(rename = "appTags")]
        AppTags,
        #[serde(rename = "preReleaseVersions")]
        PreReleaseVersions,
        #[serde(rename = "betaAppLocalizations")]
        BetaAppLocalizations,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "betaLicenseAgreement")]
        BetaLicenseAgreement,
        #[serde(rename = "betaAppReviewDetail")]
        BetaAppReviewDetail,
        #[serde(rename = "appInfos")]
        AppInfos,
        #[serde(rename = "appClips")]
        AppClips,
        #[serde(rename = "appPricePoints")]
        AppPricePoints,
        #[serde(rename = "endUserLicenseAgreement")]
        EndUserLicenseAgreement,
        #[serde(rename = "appPriceSchedule")]
        AppPriceSchedule,
        #[serde(rename = "appAvailabilityV2")]
        AppAvailabilityV2,
        #[serde(rename = "inAppPurchases")]
        InAppPurchases,
        #[serde(rename = "subscriptionGroups")]
        SubscriptionGroups,
        #[serde(rename = "gameCenterEnabledVersions")]
        GameCenterEnabledVersions,
        #[serde(rename = "perfPowerMetrics")]
        PerfPowerMetrics,
        #[serde(rename = "appCustomProductPages")]
        AppCustomProductPages,
        #[serde(rename = "inAppPurchasesV2")]
        InAppPurchasesV2,
        #[serde(rename = "promotedPurchases")]
        PromotedPurchases,
        #[serde(rename = "appEvents")]
        AppEvents,
        #[serde(rename = "reviewSubmissions")]
        ReviewSubmissions,
        #[serde(rename = "subscriptionGracePeriod")]
        SubscriptionGracePeriod,
        #[serde(rename = "customerReviews")]
        CustomerReviews,
        #[serde(rename = "customerReviewSummarizations")]
        CustomerReviewSummarizations,
        #[serde(rename = "gameCenterDetail")]
        GameCenterDetail,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "alternativeDistributionKey")]
        AlternativeDistributionKey,
        #[serde(rename = "analyticsReportRequests")]
        AnalyticsReportRequests,
        #[serde(rename = "marketplaceSearchDetail")]
        MarketplaceSearchDetail,
        #[serde(rename = "buildUploads")]
        BuildUploads,
        #[serde(rename = "backgroundAssets")]
        BackgroundAssets,
        #[serde(rename = "betaFeedbackScreenshotSubmissions")]
        BetaFeedbackScreenshotSubmissions,
        #[serde(rename = "betaFeedbackCrashSubmissions")]
        BetaFeedbackCrashSubmissions,
        #[serde(rename = "searchKeywords")]
        SearchKeywords,
        #[serde(rename = "webhooks")]
        Webhooks,
        #[serde(rename = "androidToIosAppMappingDetails")]
        AndroidToIosAppMappingDetails,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AccessibilityUrl => f.write_str("accessibilityUrl"),
                Self::Name => f.write_str("name"),
                Self::BundleId => f.write_str("bundleId"),
                Self::Sku => f.write_str("sku"),
                Self::PrimaryLocale => f.write_str("primaryLocale"),
                Self::IsOrEverWasMadeForKids => f.write_str("isOrEverWasMadeForKids"),
                Self::SubscriptionStatusUrl => f.write_str("subscriptionStatusUrl"),
                Self::SubscriptionStatusUrlVersion => f.write_str("subscriptionStatusUrlVersion"),
                Self::SubscriptionStatusUrlForSandbox => {
                    f.write_str("subscriptionStatusUrlForSandbox")
                }
                Self::SubscriptionStatusUrlVersionForSandbox => {
                    f.write_str("subscriptionStatusUrlVersionForSandbox")
                }
                Self::ContentRightsDeclaration => f.write_str("contentRightsDeclaration"),
                Self::StreamlinedPurchasingEnabled => f.write_str("streamlinedPurchasingEnabled"),
                Self::AccessibilityDeclarations => f.write_str("accessibilityDeclarations"),
                Self::AppEncryptionDeclarations => f.write_str("appEncryptionDeclarations"),
                Self::AppStoreIcon => f.write_str("appStoreIcon"),
                Self::CiProduct => f.write_str("ciProduct"),
                Self::BetaTesters => f.write_str("betaTesters"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::AppStoreVersions => f.write_str("appStoreVersions"),
                Self::AppTags => f.write_str("appTags"),
                Self::PreReleaseVersions => f.write_str("preReleaseVersions"),
                Self::BetaAppLocalizations => f.write_str("betaAppLocalizations"),
                Self::Builds => f.write_str("builds"),
                Self::BetaLicenseAgreement => f.write_str("betaLicenseAgreement"),
                Self::BetaAppReviewDetail => f.write_str("betaAppReviewDetail"),
                Self::AppInfos => f.write_str("appInfos"),
                Self::AppClips => f.write_str("appClips"),
                Self::AppPricePoints => f.write_str("appPricePoints"),
                Self::EndUserLicenseAgreement => f.write_str("endUserLicenseAgreement"),
                Self::AppPriceSchedule => f.write_str("appPriceSchedule"),
                Self::AppAvailabilityV2 => f.write_str("appAvailabilityV2"),
                Self::InAppPurchases => f.write_str("inAppPurchases"),
                Self::SubscriptionGroups => f.write_str("subscriptionGroups"),
                Self::GameCenterEnabledVersions => f.write_str("gameCenterEnabledVersions"),
                Self::PerfPowerMetrics => f.write_str("perfPowerMetrics"),
                Self::AppCustomProductPages => f.write_str("appCustomProductPages"),
                Self::InAppPurchasesV2 => f.write_str("inAppPurchasesV2"),
                Self::PromotedPurchases => f.write_str("promotedPurchases"),
                Self::AppEvents => f.write_str("appEvents"),
                Self::ReviewSubmissions => f.write_str("reviewSubmissions"),
                Self::SubscriptionGracePeriod => f.write_str("subscriptionGracePeriod"),
                Self::CustomerReviews => f.write_str("customerReviews"),
                Self::CustomerReviewSummarizations => f.write_str("customerReviewSummarizations"),
                Self::GameCenterDetail => f.write_str("gameCenterDetail"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::AlternativeDistributionKey => f.write_str("alternativeDistributionKey"),
                Self::AnalyticsReportRequests => f.write_str("analyticsReportRequests"),
                Self::MarketplaceSearchDetail => f.write_str("marketplaceSearchDetail"),
                Self::BuildUploads => f.write_str("buildUploads"),
                Self::BackgroundAssets => f.write_str("backgroundAssets"),
                Self::BetaFeedbackScreenshotSubmissions => {
                    f.write_str("betaFeedbackScreenshotSubmissions")
                }
                Self::BetaFeedbackCrashSubmissions => f.write_str("betaFeedbackCrashSubmissions"),
                Self::SearchKeywords => f.write_str("searchKeywords"),
                Self::Webhooks => f.write_str("webhooks"),
                Self::AndroidToIosAppMappingDetails => f.write_str("androidToIosAppMappingDetails"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "accessibilityUrl" => Ok(Self::AccessibilityUrl),
                "name" => Ok(Self::Name),
                "bundleId" => Ok(Self::BundleId),
                "sku" => Ok(Self::Sku),
                "primaryLocale" => Ok(Self::PrimaryLocale),
                "isOrEverWasMadeForKids" => Ok(Self::IsOrEverWasMadeForKids),
                "subscriptionStatusUrl" => Ok(Self::SubscriptionStatusUrl),
                "subscriptionStatusUrlVersion" => Ok(Self::SubscriptionStatusUrlVersion),
                "subscriptionStatusUrlForSandbox" => Ok(Self::SubscriptionStatusUrlForSandbox),
                "subscriptionStatusUrlVersionForSandbox" => {
                    Ok(Self::SubscriptionStatusUrlVersionForSandbox)
                }
                "contentRightsDeclaration" => Ok(Self::ContentRightsDeclaration),
                "streamlinedPurchasingEnabled" => Ok(Self::StreamlinedPurchasingEnabled),
                "accessibilityDeclarations" => Ok(Self::AccessibilityDeclarations),
                "appEncryptionDeclarations" => Ok(Self::AppEncryptionDeclarations),
                "appStoreIcon" => Ok(Self::AppStoreIcon),
                "ciProduct" => Ok(Self::CiProduct),
                "betaTesters" => Ok(Self::BetaTesters),
                "betaGroups" => Ok(Self::BetaGroups),
                "appStoreVersions" => Ok(Self::AppStoreVersions),
                "appTags" => Ok(Self::AppTags),
                "preReleaseVersions" => Ok(Self::PreReleaseVersions),
                "betaAppLocalizations" => Ok(Self::BetaAppLocalizations),
                "builds" => Ok(Self::Builds),
                "betaLicenseAgreement" => Ok(Self::BetaLicenseAgreement),
                "betaAppReviewDetail" => Ok(Self::BetaAppReviewDetail),
                "appInfos" => Ok(Self::AppInfos),
                "appClips" => Ok(Self::AppClips),
                "appPricePoints" => Ok(Self::AppPricePoints),
                "endUserLicenseAgreement" => Ok(Self::EndUserLicenseAgreement),
                "appPriceSchedule" => Ok(Self::AppPriceSchedule),
                "appAvailabilityV2" => Ok(Self::AppAvailabilityV2),
                "inAppPurchases" => Ok(Self::InAppPurchases),
                "subscriptionGroups" => Ok(Self::SubscriptionGroups),
                "gameCenterEnabledVersions" => Ok(Self::GameCenterEnabledVersions),
                "perfPowerMetrics" => Ok(Self::PerfPowerMetrics),
                "appCustomProductPages" => Ok(Self::AppCustomProductPages),
                "inAppPurchasesV2" => Ok(Self::InAppPurchasesV2),
                "promotedPurchases" => Ok(Self::PromotedPurchases),
                "appEvents" => Ok(Self::AppEvents),
                "reviewSubmissions" => Ok(Self::ReviewSubmissions),
                "subscriptionGracePeriod" => Ok(Self::SubscriptionGracePeriod),
                "customerReviews" => Ok(Self::CustomerReviews),
                "customerReviewSummarizations" => Ok(Self::CustomerReviewSummarizations),
                "gameCenterDetail" => Ok(Self::GameCenterDetail),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "alternativeDistributionKey" => Ok(Self::AlternativeDistributionKey),
                "analyticsReportRequests" => Ok(Self::AnalyticsReportRequests),
                "marketplaceSearchDetail" => Ok(Self::MarketplaceSearchDetail),
                "buildUploads" => Ok(Self::BuildUploads),
                "backgroundAssets" => Ok(Self::BackgroundAssets),
                "betaFeedbackScreenshotSubmissions" => Ok(Self::BetaFeedbackScreenshotSubmissions),
                "betaFeedbackCrashSubmissions" => Ok(Self::BetaFeedbackCrashSubmissions),
                "searchKeywords" => Ok(Self::SearchKeywords),
                "webhooks" => Ok(Self::Webhooks),
                "androidToIosAppMappingDetails" => Ok(Self::AndroidToIosAppMappingDetails),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "version",
    ///    "uploadedDate",
    ///    "expirationDate",
    ///    "expired",
    ///    "minOsVersion",
    ///    "lsMinimumSystemVersion",
    ///    "computedMinMacOsVersion",
    ///    "computedMinVisionOsVersion",
    ///    "iconAssetToken",
    ///    "processingState",
    ///    "buildAudienceType",
    ///    "usesNonExemptEncryption",
    ///    "preReleaseVersion",
    ///    "individualTesters",
    ///    "betaGroups",
    ///    "betaBuildLocalizations",
    ///    "appEncryptionDeclaration",
    ///    "betaAppReviewSubmission",
    ///    "app",
    ///    "buildBetaDetail",
    ///    "appStoreVersion",
    ///    "icons",
    ///    "buildBundles",
    ///    "buildUpload",
    ///    "perfPowerMetrics",
    ///    "diagnosticSignatures"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem {
        #[serde(rename = "version")]
        Version,
        #[serde(rename = "uploadedDate")]
        UploadedDate,
        #[serde(rename = "expirationDate")]
        ExpirationDate,
        #[serde(rename = "expired")]
        Expired,
        #[serde(rename = "minOsVersion")]
        MinOsVersion,
        #[serde(rename = "lsMinimumSystemVersion")]
        LsMinimumSystemVersion,
        #[serde(rename = "computedMinMacOsVersion")]
        ComputedMinMacOsVersion,
        #[serde(rename = "computedMinVisionOsVersion")]
        ComputedMinVisionOsVersion,
        #[serde(rename = "iconAssetToken")]
        IconAssetToken,
        #[serde(rename = "processingState")]
        ProcessingState,
        #[serde(rename = "buildAudienceType")]
        BuildAudienceType,
        #[serde(rename = "usesNonExemptEncryption")]
        UsesNonExemptEncryption,
        #[serde(rename = "preReleaseVersion")]
        PreReleaseVersion,
        #[serde(rename = "individualTesters")]
        IndividualTesters,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "betaBuildLocalizations")]
        BetaBuildLocalizations,
        #[serde(rename = "appEncryptionDeclaration")]
        AppEncryptionDeclaration,
        #[serde(rename = "betaAppReviewSubmission")]
        BetaAppReviewSubmission,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "buildBetaDetail")]
        BuildBetaDetail,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "icons")]
        Icons,
        #[serde(rename = "buildBundles")]
        BuildBundles,
        #[serde(rename = "buildUpload")]
        BuildUpload,
        #[serde(rename = "perfPowerMetrics")]
        PerfPowerMetrics,
        #[serde(rename = "diagnosticSignatures")]
        DiagnosticSignatures,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Version => f.write_str("version"),
                Self::UploadedDate => f.write_str("uploadedDate"),
                Self::ExpirationDate => f.write_str("expirationDate"),
                Self::Expired => f.write_str("expired"),
                Self::MinOsVersion => f.write_str("minOsVersion"),
                Self::LsMinimumSystemVersion => f.write_str("lsMinimumSystemVersion"),
                Self::ComputedMinMacOsVersion => f.write_str("computedMinMacOsVersion"),
                Self::ComputedMinVisionOsVersion => f.write_str("computedMinVisionOsVersion"),
                Self::IconAssetToken => f.write_str("iconAssetToken"),
                Self::ProcessingState => f.write_str("processingState"),
                Self::BuildAudienceType => f.write_str("buildAudienceType"),
                Self::UsesNonExemptEncryption => f.write_str("usesNonExemptEncryption"),
                Self::PreReleaseVersion => f.write_str("preReleaseVersion"),
                Self::IndividualTesters => f.write_str("individualTesters"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::BetaBuildLocalizations => f.write_str("betaBuildLocalizations"),
                Self::AppEncryptionDeclaration => f.write_str("appEncryptionDeclaration"),
                Self::BetaAppReviewSubmission => f.write_str("betaAppReviewSubmission"),
                Self::App => f.write_str("app"),
                Self::BuildBetaDetail => f.write_str("buildBetaDetail"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
                Self::Icons => f.write_str("icons"),
                Self::BuildBundles => f.write_str("buildBundles"),
                Self::BuildUpload => f.write_str("buildUpload"),
                Self::PerfPowerMetrics => f.write_str("perfPowerMetrics"),
                Self::DiagnosticSignatures => f.write_str("diagnosticSignatures"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "version" => Ok(Self::Version),
                "uploadedDate" => Ok(Self::UploadedDate),
                "expirationDate" => Ok(Self::ExpirationDate),
                "expired" => Ok(Self::Expired),
                "minOsVersion" => Ok(Self::MinOsVersion),
                "lsMinimumSystemVersion" => Ok(Self::LsMinimumSystemVersion),
                "computedMinMacOsVersion" => Ok(Self::ComputedMinMacOsVersion),
                "computedMinVisionOsVersion" => Ok(Self::ComputedMinVisionOsVersion),
                "iconAssetToken" => Ok(Self::IconAssetToken),
                "processingState" => Ok(Self::ProcessingState),
                "buildAudienceType" => Ok(Self::BuildAudienceType),
                "usesNonExemptEncryption" => Ok(Self::UsesNonExemptEncryption),
                "preReleaseVersion" => Ok(Self::PreReleaseVersion),
                "individualTesters" => Ok(Self::IndividualTesters),
                "betaGroups" => Ok(Self::BetaGroups),
                "betaBuildLocalizations" => Ok(Self::BetaBuildLocalizations),
                "appEncryptionDeclaration" => Ok(Self::AppEncryptionDeclaration),
                "betaAppReviewSubmission" => Ok(Self::BetaAppReviewSubmission),
                "app" => Ok(Self::App),
                "buildBetaDetail" => Ok(Self::BuildBetaDetail),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                "icons" => Ok(Self::Icons),
                "buildBundles" => Ok(Self::BuildBundles),
                "buildUpload" => Ok(Self::BuildUpload),
                "perfPowerMetrics" => Ok(Self::PerfPowerMetrics),
                "diagnosticSignatures" => Ok(Self::DiagnosticSignatures),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "enabled",
    ///    "compatibilityVersions",
    ///    "appStoreVersion"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "compatibilityVersions")]
        CompatibilityVersions,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Enabled => f.write_str("enabled"),
                Self::CompatibilityVersions => f.write_str("compatibilityVersions"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "enabled" => Ok(Self::Enabled),
                "compatibilityVersions" => Ok(Self::CompatibilityVersions),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "fileSize",
    ///    "fileName",
    ///    "sourceFileChecksum",
    ///    "uploadOperations",
    ///    "assetDeliveryState",
    ///    "appStoreVersion"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem {
        #[serde(rename = "fileSize")]
        FileSize,
        #[serde(rename = "fileName")]
        FileName,
        #[serde(rename = "sourceFileChecksum")]
        SourceFileChecksum,
        #[serde(rename = "uploadOperations")]
        UploadOperations,
        #[serde(rename = "assetDeliveryState")]
        AssetDeliveryState,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::FileSize => f.write_str("fileSize"),
                Self::FileName => f.write_str("fileName"),
                Self::SourceFileChecksum => f.write_str("sourceFileChecksum"),
                Self::UploadOperations => f.write_str("uploadOperations"),
                Self::AssetDeliveryState => f.write_str("assetDeliveryState"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "fileSize" => Ok(Self::FileSize),
                "fileName" => Ok(Self::FileName),
                "sourceFileChecksum" => Ok(Self::SourceFileChecksum),
                "uploadOperations" => Ok(Self::UploadOperations),
                "assetDeliveryState" => Ok(Self::AssetDeliveryState),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ACCEPTED",
    ///    "DEVELOPER_REMOVED_FROM_SALE",
    ///    "DEVELOPER_REJECTED",
    ///    "IN_REVIEW",
    ///    "INVALID_BINARY",
    ///    "METADATA_REJECTED",
    ///    "PENDING_APPLE_RELEASE",
    ///    "PENDING_CONTRACT",
    ///    "PENDING_DEVELOPER_RELEASE",
    ///    "PREPARE_FOR_SUBMISSION",
    ///    "PREORDER_READY_FOR_SALE",
    ///    "PROCESSING_FOR_APP_STORE",
    ///    "READY_FOR_REVIEW",
    ///    "READY_FOR_SALE",
    ///    "REJECTED",
    ///    "REMOVED_FROM_SALE",
    ///    "WAITING_FOR_EXPORT_COMPLIANCE",
    ///    "WAITING_FOR_REVIEW",
    ///    "REPLACED_WITH_NEW_VERSION",
    ///    "NOT_APPLICABLE"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem {
        #[serde(rename = "ACCEPTED")]
        Accepted,
        #[serde(rename = "DEVELOPER_REMOVED_FROM_SALE")]
        DeveloperRemovedFromSale,
        #[serde(rename = "DEVELOPER_REJECTED")]
        DeveloperRejected,
        #[serde(rename = "IN_REVIEW")]
        InReview,
        #[serde(rename = "INVALID_BINARY")]
        InvalidBinary,
        #[serde(rename = "METADATA_REJECTED")]
        MetadataRejected,
        #[serde(rename = "PENDING_APPLE_RELEASE")]
        PendingAppleRelease,
        #[serde(rename = "PENDING_CONTRACT")]
        PendingContract,
        #[serde(rename = "PENDING_DEVELOPER_RELEASE")]
        PendingDeveloperRelease,
        #[serde(rename = "PREPARE_FOR_SUBMISSION")]
        PrepareForSubmission,
        #[serde(rename = "PREORDER_READY_FOR_SALE")]
        PreorderReadyForSale,
        #[serde(rename = "PROCESSING_FOR_APP_STORE")]
        ProcessingForAppStore,
        #[serde(rename = "READY_FOR_REVIEW")]
        ReadyForReview,
        #[serde(rename = "READY_FOR_SALE")]
        ReadyForSale,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "REMOVED_FROM_SALE")]
        RemovedFromSale,
        #[serde(rename = "WAITING_FOR_EXPORT_COMPLIANCE")]
        WaitingForExportCompliance,
        #[serde(rename = "WAITING_FOR_REVIEW")]
        WaitingForReview,
        #[serde(rename = "REPLACED_WITH_NEW_VERSION")]
        ReplacedWithNewVersion,
        #[serde(rename = "NOT_APPLICABLE")]
        NotApplicable,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Accepted => f.write_str("ACCEPTED"),
                Self::DeveloperRemovedFromSale => f.write_str("DEVELOPER_REMOVED_FROM_SALE"),
                Self::DeveloperRejected => f.write_str("DEVELOPER_REJECTED"),
                Self::InReview => f.write_str("IN_REVIEW"),
                Self::InvalidBinary => f.write_str("INVALID_BINARY"),
                Self::MetadataRejected => f.write_str("METADATA_REJECTED"),
                Self::PendingAppleRelease => f.write_str("PENDING_APPLE_RELEASE"),
                Self::PendingContract => f.write_str("PENDING_CONTRACT"),
                Self::PendingDeveloperRelease => f.write_str("PENDING_DEVELOPER_RELEASE"),
                Self::PrepareForSubmission => f.write_str("PREPARE_FOR_SUBMISSION"),
                Self::PreorderReadyForSale => f.write_str("PREORDER_READY_FOR_SALE"),
                Self::ProcessingForAppStore => f.write_str("PROCESSING_FOR_APP_STORE"),
                Self::ReadyForReview => f.write_str("READY_FOR_REVIEW"),
                Self::ReadyForSale => f.write_str("READY_FOR_SALE"),
                Self::Rejected => f.write_str("REJECTED"),
                Self::RemovedFromSale => f.write_str("REMOVED_FROM_SALE"),
                Self::WaitingForExportCompliance => f.write_str("WAITING_FOR_EXPORT_COMPLIANCE"),
                Self::WaitingForReview => f.write_str("WAITING_FOR_REVIEW"),
                Self::ReplacedWithNewVersion => f.write_str("REPLACED_WITH_NEW_VERSION"),
                Self::NotApplicable => f.write_str("NOT_APPLICABLE"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ACCEPTED" => Ok(Self::Accepted),
                "DEVELOPER_REMOVED_FROM_SALE" => Ok(Self::DeveloperRemovedFromSale),
                "DEVELOPER_REJECTED" => Ok(Self::DeveloperRejected),
                "IN_REVIEW" => Ok(Self::InReview),
                "INVALID_BINARY" => Ok(Self::InvalidBinary),
                "METADATA_REJECTED" => Ok(Self::MetadataRejected),
                "PENDING_APPLE_RELEASE" => Ok(Self::PendingAppleRelease),
                "PENDING_CONTRACT" => Ok(Self::PendingContract),
                "PENDING_DEVELOPER_RELEASE" => Ok(Self::PendingDeveloperRelease),
                "PREPARE_FOR_SUBMISSION" => Ok(Self::PrepareForSubmission),
                "PREORDER_READY_FOR_SALE" => Ok(Self::PreorderReadyForSale),
                "PROCESSING_FOR_APP_STORE" => Ok(Self::ProcessingForAppStore),
                "READY_FOR_REVIEW" => Ok(Self::ReadyForReview),
                "READY_FOR_SALE" => Ok(Self::ReadyForSale),
                "REJECTED" => Ok(Self::Rejected),
                "REMOVED_FROM_SALE" => Ok(Self::RemovedFromSale),
                "WAITING_FOR_EXPORT_COMPLIANCE" => Ok(Self::WaitingForExportCompliance),
                "WAITING_FOR_REVIEW" => Ok(Self::WaitingForReview),
                "REPLACED_WITH_NEW_VERSION" => Ok(Self::ReplacedWithNewVersion),
                "NOT_APPLICABLE" => Ok(Self::NotApplicable),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ACCEPTED",
    ///    "DEVELOPER_REJECTED",
    ///    "IN_REVIEW",
    ///    "INVALID_BINARY",
    ///    "METADATA_REJECTED",
    ///    "PENDING_APPLE_RELEASE",
    ///    "PENDING_DEVELOPER_RELEASE",
    ///    "PREPARE_FOR_SUBMISSION",
    ///    "PROCESSING_FOR_DISTRIBUTION",
    ///    "READY_FOR_DISTRIBUTION",
    ///    "READY_FOR_REVIEW",
    ///    "REJECTED",
    ///    "REPLACED_WITH_NEW_VERSION",
    ///    "WAITING_FOR_EXPORT_COMPLIANCE",
    ///    "WAITING_FOR_REVIEW"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem {
        #[serde(rename = "ACCEPTED")]
        Accepted,
        #[serde(rename = "DEVELOPER_REJECTED")]
        DeveloperRejected,
        #[serde(rename = "IN_REVIEW")]
        InReview,
        #[serde(rename = "INVALID_BINARY")]
        InvalidBinary,
        #[serde(rename = "METADATA_REJECTED")]
        MetadataRejected,
        #[serde(rename = "PENDING_APPLE_RELEASE")]
        PendingAppleRelease,
        #[serde(rename = "PENDING_DEVELOPER_RELEASE")]
        PendingDeveloperRelease,
        #[serde(rename = "PREPARE_FOR_SUBMISSION")]
        PrepareForSubmission,
        #[serde(rename = "PROCESSING_FOR_DISTRIBUTION")]
        ProcessingForDistribution,
        #[serde(rename = "READY_FOR_DISTRIBUTION")]
        ReadyForDistribution,
        #[serde(rename = "READY_FOR_REVIEW")]
        ReadyForReview,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "REPLACED_WITH_NEW_VERSION")]
        ReplacedWithNewVersion,
        #[serde(rename = "WAITING_FOR_EXPORT_COMPLIANCE")]
        WaitingForExportCompliance,
        #[serde(rename = "WAITING_FOR_REVIEW")]
        WaitingForReview,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Accepted => f.write_str("ACCEPTED"),
                Self::DeveloperRejected => f.write_str("DEVELOPER_REJECTED"),
                Self::InReview => f.write_str("IN_REVIEW"),
                Self::InvalidBinary => f.write_str("INVALID_BINARY"),
                Self::MetadataRejected => f.write_str("METADATA_REJECTED"),
                Self::PendingAppleRelease => f.write_str("PENDING_APPLE_RELEASE"),
                Self::PendingDeveloperRelease => f.write_str("PENDING_DEVELOPER_RELEASE"),
                Self::PrepareForSubmission => f.write_str("PREPARE_FOR_SUBMISSION"),
                Self::ProcessingForDistribution => f.write_str("PROCESSING_FOR_DISTRIBUTION"),
                Self::ReadyForDistribution => f.write_str("READY_FOR_DISTRIBUTION"),
                Self::ReadyForReview => f.write_str("READY_FOR_REVIEW"),
                Self::Rejected => f.write_str("REJECTED"),
                Self::ReplacedWithNewVersion => f.write_str("REPLACED_WITH_NEW_VERSION"),
                Self::WaitingForExportCompliance => f.write_str("WAITING_FOR_EXPORT_COMPLIANCE"),
                Self::WaitingForReview => f.write_str("WAITING_FOR_REVIEW"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ACCEPTED" => Ok(Self::Accepted),
                "DEVELOPER_REJECTED" => Ok(Self::DeveloperRejected),
                "IN_REVIEW" => Ok(Self::InReview),
                "INVALID_BINARY" => Ok(Self::InvalidBinary),
                "METADATA_REJECTED" => Ok(Self::MetadataRejected),
                "PENDING_APPLE_RELEASE" => Ok(Self::PendingAppleRelease),
                "PENDING_DEVELOPER_RELEASE" => Ok(Self::PendingDeveloperRelease),
                "PREPARE_FOR_SUBMISSION" => Ok(Self::PrepareForSubmission),
                "PROCESSING_FOR_DISTRIBUTION" => Ok(Self::ProcessingForDistribution),
                "READY_FOR_DISTRIBUTION" => Ok(Self::ReadyForDistribution),
                "READY_FOR_REVIEW" => Ok(Self::ReadyForReview),
                "REJECTED" => Ok(Self::Rejected),
                "REPLACED_WITH_NEW_VERSION" => Ok(Self::ReplacedWithNewVersion),
                "WAITING_FOR_EXPORT_COMPLIANCE" => Ok(Self::WaitingForExportCompliance),
                "WAITING_FOR_REVIEW" => Ok(Self::WaitingForReview),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str>
        for AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "IOS",
    ///    "MAC_OS",
    ///    "TV_OS",
    ///    "VISION_OS"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem {
        #[serde(rename = "IOS")]
        Ios,
        #[serde(rename = "MAC_OS")]
        MacOs,
        #[serde(rename = "TV_OS")]
        TvOs,
        #[serde(rename = "VISION_OS")]
        VisionOs,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ios => f.write_str("IOS"),
                Self::MacOs => f.write_str("MAC_OS"),
                Self::TvOs => f.write_str("TV_OS"),
                Self::VisionOs => f.write_str("VISION_OS"),
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "IOS" => Ok(Self::Ios),
                "MAC_OS" => Ok(Self::MacOs),
                "TV_OS" => Ok(Self::TvOs),
                "VISION_OS" => Ok(Self::VisionOs),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsAppStoreVersionsGetToManyRelatedIncludeItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "app",
    ///    "appStoreVersionLocalizations",
    ///    "build",
    ///    "appStoreVersionPhasedRelease",
    ///    "gameCenterAppVersion",
    ///    "routingAppCoverage",
    ///    "appStoreReviewDetail",
    ///    "appStoreVersionSubmission",
    ///    "appClipDefaultExperience",
    ///    "appStoreVersionExperiments",
    ///    "appStoreVersionExperimentsV2",
    ///    "alternativeDistributionPackage"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsAppStoreVersionsGetToManyRelatedIncludeItem {
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appStoreVersionLocalizations")]
        AppStoreVersionLocalizations,
        #[serde(rename = "build")]
        Build,
        #[serde(rename = "appStoreVersionPhasedRelease")]
        AppStoreVersionPhasedRelease,
        #[serde(rename = "gameCenterAppVersion")]
        GameCenterAppVersion,
        #[serde(rename = "routingAppCoverage")]
        RoutingAppCoverage,
        #[serde(rename = "appStoreReviewDetail")]
        AppStoreReviewDetail,
        #[serde(rename = "appStoreVersionSubmission")]
        AppStoreVersionSubmission,
        #[serde(rename = "appClipDefaultExperience")]
        AppClipDefaultExperience,
        #[serde(rename = "appStoreVersionExperiments")]
        AppStoreVersionExperiments,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "alternativeDistributionPackage")]
        AlternativeDistributionPackage,
    }

    impl ::std::fmt::Display for AppsAppStoreVersionsGetToManyRelatedIncludeItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::App => f.write_str("app"),
                Self::AppStoreVersionLocalizations => f.write_str("appStoreVersionLocalizations"),
                Self::Build => f.write_str("build"),
                Self::AppStoreVersionPhasedRelease => f.write_str("appStoreVersionPhasedRelease"),
                Self::GameCenterAppVersion => f.write_str("gameCenterAppVersion"),
                Self::RoutingAppCoverage => f.write_str("routingAppCoverage"),
                Self::AppStoreReviewDetail => f.write_str("appStoreReviewDetail"),
                Self::AppStoreVersionSubmission => f.write_str("appStoreVersionSubmission"),
                Self::AppClipDefaultExperience => f.write_str("appClipDefaultExperience"),
                Self::AppStoreVersionExperiments => f.write_str("appStoreVersionExperiments"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::AlternativeDistributionPackage => {
                    f.write_str("alternativeDistributionPackage")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsAppStoreVersionsGetToManyRelatedIncludeItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "app" => Ok(Self::App),
                "appStoreVersionLocalizations" => Ok(Self::AppStoreVersionLocalizations),
                "build" => Ok(Self::Build),
                "appStoreVersionPhasedRelease" => Ok(Self::AppStoreVersionPhasedRelease),
                "gameCenterAppVersion" => Ok(Self::GameCenterAppVersion),
                "routingAppCoverage" => Ok(Self::RoutingAppCoverage),
                "appStoreReviewDetail" => Ok(Self::AppStoreReviewDetail),
                "appStoreVersionSubmission" => Ok(Self::AppStoreVersionSubmission),
                "appClipDefaultExperience" => Ok(Self::AppClipDefaultExperience),
                "appStoreVersionExperiments" => Ok(Self::AppStoreVersionExperiments),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "alternativeDistributionPackage" => Ok(Self::AlternativeDistributionPackage),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsAppStoreVersionsGetToManyRelatedIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedIncludeItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsAppStoreVersionsGetToManyRelatedIncludeItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "packageName",
    ///    "appSigningKeyPublicCertificateSha256Fingerprints"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem {
        #[serde(rename = "packageName")]
        PackageName,
        #[serde(rename = "appSigningKeyPublicCertificateSha256Fingerprints")]
        AppSigningKeyPublicCertificateSha256Fingerprints,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::PackageName => f.write_str("packageName"),
                Self::AppSigningKeyPublicCertificateSha256Fingerprints => {
                    f.write_str("appSigningKeyPublicCertificateSha256Fingerprints")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "packageName" => Ok(Self::PackageName),
                "appSigningKeyPublicCertificateSha256Fingerprints" => {
                    Ok(Self::AppSigningKeyPublicCertificateSha256Fingerprints)
                }
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppClipsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "bundleId",
    ///    "app",
    ///    "appClipDefaultExperiences",
    ///    "appClipAdvancedExperiences"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppClipsItem {
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appClipDefaultExperiences")]
        AppClipDefaultExperiences,
        #[serde(rename = "appClipAdvancedExperiences")]
        AppClipAdvancedExperiences,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppClipsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::BundleId => f.write_str("bundleId"),
                Self::App => f.write_str("app"),
                Self::AppClipDefaultExperiences => f.write_str("appClipDefaultExperiences"),
                Self::AppClipAdvancedExperiences => f.write_str("appClipAdvancedExperiences"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppClipsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "bundleId" => Ok(Self::BundleId),
                "app" => Ok(Self::App),
                "appClipDefaultExperiences" => Ok(Self::AppClipDefaultExperiences),
                "appClipAdvancedExperiences" => Ok(Self::AppClipAdvancedExperiences),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppClipsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsAppClipsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsAppClipsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppCustomProductPagesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "url",
    ///    "visible",
    ///    "app",
    ///    "appCustomProductPageVersions"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppCustomProductPagesItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "url")]
        Url,
        #[serde(rename = "visible")]
        Visible,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appCustomProductPageVersions")]
        AppCustomProductPageVersions,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppCustomProductPagesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::Url => f.write_str("url"),
                Self::Visible => f.write_str("visible"),
                Self::App => f.write_str("app"),
                Self::AppCustomProductPageVersions => f.write_str("appCustomProductPageVersions"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppCustomProductPagesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "url" => Ok(Self::Url),
                "visible" => Ok(Self::Visible),
                "app" => Ok(Self::App),
                "appCustomProductPageVersions" => Ok(Self::AppCustomProductPageVersions),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppCustomProductPagesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsAppCustomProductPagesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsAppCustomProductPagesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppEncryptionDeclarationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appDescription",
    ///    "createdDate",
    ///    "usesEncryption",
    ///    "exempt",
    ///    "containsProprietaryCryptography",
    ///    "containsThirdPartyCryptography",
    ///    "availableOnFrenchStore",
    ///    "platform",
    ///    "uploadedDate",
    ///    "documentUrl",
    ///    "documentName",
    ///    "documentType",
    ///    "appEncryptionDeclarationState",
    ///    "codeValue",
    ///    "app",
    ///    "builds",
    ///    "appEncryptionDeclarationDocument"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppEncryptionDeclarationsItem {
        #[serde(rename = "appDescription")]
        AppDescription,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "usesEncryption")]
        UsesEncryption,
        #[serde(rename = "exempt")]
        Exempt,
        #[serde(rename = "containsProprietaryCryptography")]
        ContainsProprietaryCryptography,
        #[serde(rename = "containsThirdPartyCryptography")]
        ContainsThirdPartyCryptography,
        #[serde(rename = "availableOnFrenchStore")]
        AvailableOnFrenchStore,
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "uploadedDate")]
        UploadedDate,
        #[serde(rename = "documentUrl")]
        DocumentUrl,
        #[serde(rename = "documentName")]
        DocumentName,
        #[serde(rename = "documentType")]
        DocumentType,
        #[serde(rename = "appEncryptionDeclarationState")]
        AppEncryptionDeclarationState,
        #[serde(rename = "codeValue")]
        CodeValue,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "appEncryptionDeclarationDocument")]
        AppEncryptionDeclarationDocument,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppEncryptionDeclarationsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppDescription => f.write_str("appDescription"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::UsesEncryption => f.write_str("usesEncryption"),
                Self::Exempt => f.write_str("exempt"),
                Self::ContainsProprietaryCryptography => {
                    f.write_str("containsProprietaryCryptography")
                }
                Self::ContainsThirdPartyCryptography => {
                    f.write_str("containsThirdPartyCryptography")
                }
                Self::AvailableOnFrenchStore => f.write_str("availableOnFrenchStore"),
                Self::Platform => f.write_str("platform"),
                Self::UploadedDate => f.write_str("uploadedDate"),
                Self::DocumentUrl => f.write_str("documentUrl"),
                Self::DocumentName => f.write_str("documentName"),
                Self::DocumentType => f.write_str("documentType"),
                Self::AppEncryptionDeclarationState => f.write_str("appEncryptionDeclarationState"),
                Self::CodeValue => f.write_str("codeValue"),
                Self::App => f.write_str("app"),
                Self::Builds => f.write_str("builds"),
                Self::AppEncryptionDeclarationDocument => {
                    f.write_str("appEncryptionDeclarationDocument")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppEncryptionDeclarationsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appDescription" => Ok(Self::AppDescription),
                "createdDate" => Ok(Self::CreatedDate),
                "usesEncryption" => Ok(Self::UsesEncryption),
                "exempt" => Ok(Self::Exempt),
                "containsProprietaryCryptography" => Ok(Self::ContainsProprietaryCryptography),
                "containsThirdPartyCryptography" => Ok(Self::ContainsThirdPartyCryptography),
                "availableOnFrenchStore" => Ok(Self::AvailableOnFrenchStore),
                "platform" => Ok(Self::Platform),
                "uploadedDate" => Ok(Self::UploadedDate),
                "documentUrl" => Ok(Self::DocumentUrl),
                "documentName" => Ok(Self::DocumentName),
                "documentType" => Ok(Self::DocumentType),
                "appEncryptionDeclarationState" => Ok(Self::AppEncryptionDeclarationState),
                "codeValue" => Ok(Self::CodeValue),
                "app" => Ok(Self::App),
                "builds" => Ok(Self::Builds),
                "appEncryptionDeclarationDocument" => Ok(Self::AppEncryptionDeclarationDocument),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppEncryptionDeclarationsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsAppEncryptionDeclarationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsAppEncryptionDeclarationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppEventsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "referenceName",
    ///    "badge",
    ///    "eventState",
    ///    "deepLink",
    ///    "purchaseRequirement",
    ///    "primaryLocale",
    ///    "priority",
    ///    "purpose",
    ///    "territorySchedules",
    ///    "archivedTerritorySchedules",
    ///    "localizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppEventsItem {
        #[serde(rename = "referenceName")]
        ReferenceName,
        #[serde(rename = "badge")]
        Badge,
        #[serde(rename = "eventState")]
        EventState,
        #[serde(rename = "deepLink")]
        DeepLink,
        #[serde(rename = "purchaseRequirement")]
        PurchaseRequirement,
        #[serde(rename = "primaryLocale")]
        PrimaryLocale,
        #[serde(rename = "priority")]
        Priority,
        #[serde(rename = "purpose")]
        Purpose,
        #[serde(rename = "territorySchedules")]
        TerritorySchedules,
        #[serde(rename = "archivedTerritorySchedules")]
        ArchivedTerritorySchedules,
        #[serde(rename = "localizations")]
        Localizations,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppEventsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReferenceName => f.write_str("referenceName"),
                Self::Badge => f.write_str("badge"),
                Self::EventState => f.write_str("eventState"),
                Self::DeepLink => f.write_str("deepLink"),
                Self::PurchaseRequirement => f.write_str("purchaseRequirement"),
                Self::PrimaryLocale => f.write_str("primaryLocale"),
                Self::Priority => f.write_str("priority"),
                Self::Purpose => f.write_str("purpose"),
                Self::TerritorySchedules => f.write_str("territorySchedules"),
                Self::ArchivedTerritorySchedules => f.write_str("archivedTerritorySchedules"),
                Self::Localizations => f.write_str("localizations"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppEventsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "referenceName" => Ok(Self::ReferenceName),
                "badge" => Ok(Self::Badge),
                "eventState" => Ok(Self::EventState),
                "deepLink" => Ok(Self::DeepLink),
                "purchaseRequirement" => Ok(Self::PurchaseRequirement),
                "primaryLocale" => Ok(Self::PrimaryLocale),
                "priority" => Ok(Self::Priority),
                "purpose" => Ok(Self::Purpose),
                "territorySchedules" => Ok(Self::TerritorySchedules),
                "archivedTerritorySchedules" => Ok(Self::ArchivedTerritorySchedules),
                "localizations" => Ok(Self::Localizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsAppEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsAppEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppInfosItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreState",
    ///    "state",
    ///    "appStoreAgeRating",
    ///    "australiaAgeRating",
    ///    "brazilAgeRating",
    ///    "brazilAgeRatingV2",
    ///    "franceAgeRating",
    ///    "koreaAgeRating",
    ///    "app",
    ///    "ageRatingDeclaration",
    ///    "appInfoLocalizations",
    ///    "primaryCategory",
    ///    "primarySubcategoryOne",
    ///    "primarySubcategoryTwo",
    ///    "secondaryCategory",
    ///    "secondarySubcategoryOne",
    ///    "secondarySubcategoryTwo",
    ///    "territoryAgeRatings"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppInfosItem {
        #[serde(rename = "appStoreState")]
        AppStoreState,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "appStoreAgeRating")]
        AppStoreAgeRating,
        #[serde(rename = "australiaAgeRating")]
        AustraliaAgeRating,
        #[serde(rename = "brazilAgeRating")]
        BrazilAgeRating,
        #[serde(rename = "brazilAgeRatingV2")]
        BrazilAgeRatingV2,
        #[serde(rename = "franceAgeRating")]
        FranceAgeRating,
        #[serde(rename = "koreaAgeRating")]
        KoreaAgeRating,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "ageRatingDeclaration")]
        AgeRatingDeclaration,
        #[serde(rename = "appInfoLocalizations")]
        AppInfoLocalizations,
        #[serde(rename = "primaryCategory")]
        PrimaryCategory,
        #[serde(rename = "primarySubcategoryOne")]
        PrimarySubcategoryOne,
        #[serde(rename = "primarySubcategoryTwo")]
        PrimarySubcategoryTwo,
        #[serde(rename = "secondaryCategory")]
        SecondaryCategory,
        #[serde(rename = "secondarySubcategoryOne")]
        SecondarySubcategoryOne,
        #[serde(rename = "secondarySubcategoryTwo")]
        SecondarySubcategoryTwo,
        #[serde(rename = "territoryAgeRatings")]
        TerritoryAgeRatings,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppInfosItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreState => f.write_str("appStoreState"),
                Self::State => f.write_str("state"),
                Self::AppStoreAgeRating => f.write_str("appStoreAgeRating"),
                Self::AustraliaAgeRating => f.write_str("australiaAgeRating"),
                Self::BrazilAgeRating => f.write_str("brazilAgeRating"),
                Self::BrazilAgeRatingV2 => f.write_str("brazilAgeRatingV2"),
                Self::FranceAgeRating => f.write_str("franceAgeRating"),
                Self::KoreaAgeRating => f.write_str("koreaAgeRating"),
                Self::App => f.write_str("app"),
                Self::AgeRatingDeclaration => f.write_str("ageRatingDeclaration"),
                Self::AppInfoLocalizations => f.write_str("appInfoLocalizations"),
                Self::PrimaryCategory => f.write_str("primaryCategory"),
                Self::PrimarySubcategoryOne => f.write_str("primarySubcategoryOne"),
                Self::PrimarySubcategoryTwo => f.write_str("primarySubcategoryTwo"),
                Self::SecondaryCategory => f.write_str("secondaryCategory"),
                Self::SecondarySubcategoryOne => f.write_str("secondarySubcategoryOne"),
                Self::SecondarySubcategoryTwo => f.write_str("secondarySubcategoryTwo"),
                Self::TerritoryAgeRatings => f.write_str("territoryAgeRatings"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppInfosItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreState" => Ok(Self::AppStoreState),
                "state" => Ok(Self::State),
                "appStoreAgeRating" => Ok(Self::AppStoreAgeRating),
                "australiaAgeRating" => Ok(Self::AustraliaAgeRating),
                "brazilAgeRating" => Ok(Self::BrazilAgeRating),
                "brazilAgeRatingV2" => Ok(Self::BrazilAgeRatingV2),
                "franceAgeRating" => Ok(Self::FranceAgeRating),
                "koreaAgeRating" => Ok(Self::KoreaAgeRating),
                "app" => Ok(Self::App),
                "ageRatingDeclaration" => Ok(Self::AgeRatingDeclaration),
                "appInfoLocalizations" => Ok(Self::AppInfoLocalizations),
                "primaryCategory" => Ok(Self::PrimaryCategory),
                "primarySubcategoryOne" => Ok(Self::PrimarySubcategoryOne),
                "primarySubcategoryTwo" => Ok(Self::PrimarySubcategoryTwo),
                "secondaryCategory" => Ok(Self::SecondaryCategory),
                "secondarySubcategoryOne" => Ok(Self::SecondarySubcategoryOne),
                "secondarySubcategoryTwo" => Ok(Self::SecondarySubcategoryTwo),
                "territoryAgeRatings" => Ok(Self::TerritoryAgeRatings),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppInfosItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsAppInfosItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsAppInfosItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppStoreVersionExperimentsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "platform",
    ///    "trafficProportion",
    ///    "state",
    ///    "reviewRequired",
    ///    "startDate",
    ///    "endDate",
    ///    "app",
    ///    "latestControlVersion",
    ///    "controlVersions",
    ///    "appStoreVersionExperimentTreatments"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppStoreVersionExperimentsItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "trafficProportion")]
        TrafficProportion,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "reviewRequired")]
        ReviewRequired,
        #[serde(rename = "startDate")]
        StartDate,
        #[serde(rename = "endDate")]
        EndDate,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "latestControlVersion")]
        LatestControlVersion,
        #[serde(rename = "controlVersions")]
        ControlVersions,
        #[serde(rename = "appStoreVersionExperimentTreatments")]
        AppStoreVersionExperimentTreatments,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppStoreVersionExperimentsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::Platform => f.write_str("platform"),
                Self::TrafficProportion => f.write_str("trafficProportion"),
                Self::State => f.write_str("state"),
                Self::ReviewRequired => f.write_str("reviewRequired"),
                Self::StartDate => f.write_str("startDate"),
                Self::EndDate => f.write_str("endDate"),
                Self::App => f.write_str("app"),
                Self::LatestControlVersion => f.write_str("latestControlVersion"),
                Self::ControlVersions => f.write_str("controlVersions"),
                Self::AppStoreVersionExperimentTreatments => {
                    f.write_str("appStoreVersionExperimentTreatments")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppStoreVersionExperimentsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "platform" => Ok(Self::Platform),
                "trafficProportion" => Ok(Self::TrafficProportion),
                "state" => Ok(Self::State),
                "reviewRequired" => Ok(Self::ReviewRequired),
                "startDate" => Ok(Self::StartDate),
                "endDate" => Ok(Self::EndDate),
                "app" => Ok(Self::App),
                "latestControlVersion" => Ok(Self::LatestControlVersion),
                "controlVersions" => Ok(Self::ControlVersions),
                "appStoreVersionExperimentTreatments" => {
                    Ok(Self::AppStoreVersionExperimentTreatments)
                }
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppStoreVersionExperimentsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsAppStoreVersionExperimentsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsAppStoreVersionExperimentsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppStoreVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "versionString",
    ///    "appStoreState",
    ///    "appVersionState",
    ///    "copyright",
    ///    "reviewType",
    ///    "releaseType",
    ///    "earliestReleaseDate",
    ///    "usesIdfa",
    ///    "downloadable",
    ///    "createdDate",
    ///    "app",
    ///    "appStoreVersionLocalizations",
    ///    "build",
    ///    "appStoreVersionPhasedRelease",
    ///    "gameCenterAppVersion",
    ///    "routingAppCoverage",
    ///    "appStoreReviewDetail",
    ///    "appStoreVersionSubmission",
    ///    "appClipDefaultExperience",
    ///    "appStoreVersionExperiments",
    ///    "appStoreVersionExperimentsV2",
    ///    "customerReviews",
    ///    "alternativeDistributionPackage"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppStoreVersionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "versionString")]
        VersionString,
        #[serde(rename = "appStoreState")]
        AppStoreState,
        #[serde(rename = "appVersionState")]
        AppVersionState,
        #[serde(rename = "copyright")]
        Copyright,
        #[serde(rename = "reviewType")]
        ReviewType,
        #[serde(rename = "releaseType")]
        ReleaseType,
        #[serde(rename = "earliestReleaseDate")]
        EarliestReleaseDate,
        #[serde(rename = "usesIdfa")]
        UsesIdfa,
        #[serde(rename = "downloadable")]
        Downloadable,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appStoreVersionLocalizations")]
        AppStoreVersionLocalizations,
        #[serde(rename = "build")]
        Build,
        #[serde(rename = "appStoreVersionPhasedRelease")]
        AppStoreVersionPhasedRelease,
        #[serde(rename = "gameCenterAppVersion")]
        GameCenterAppVersion,
        #[serde(rename = "routingAppCoverage")]
        RoutingAppCoverage,
        #[serde(rename = "appStoreReviewDetail")]
        AppStoreReviewDetail,
        #[serde(rename = "appStoreVersionSubmission")]
        AppStoreVersionSubmission,
        #[serde(rename = "appClipDefaultExperience")]
        AppClipDefaultExperience,
        #[serde(rename = "appStoreVersionExperiments")]
        AppStoreVersionExperiments,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "customerReviews")]
        CustomerReviews,
        #[serde(rename = "alternativeDistributionPackage")]
        AlternativeDistributionPackage,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppStoreVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::VersionString => f.write_str("versionString"),
                Self::AppStoreState => f.write_str("appStoreState"),
                Self::AppVersionState => f.write_str("appVersionState"),
                Self::Copyright => f.write_str("copyright"),
                Self::ReviewType => f.write_str("reviewType"),
                Self::ReleaseType => f.write_str("releaseType"),
                Self::EarliestReleaseDate => f.write_str("earliestReleaseDate"),
                Self::UsesIdfa => f.write_str("usesIdfa"),
                Self::Downloadable => f.write_str("downloadable"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::App => f.write_str("app"),
                Self::AppStoreVersionLocalizations => f.write_str("appStoreVersionLocalizations"),
                Self::Build => f.write_str("build"),
                Self::AppStoreVersionPhasedRelease => f.write_str("appStoreVersionPhasedRelease"),
                Self::GameCenterAppVersion => f.write_str("gameCenterAppVersion"),
                Self::RoutingAppCoverage => f.write_str("routingAppCoverage"),
                Self::AppStoreReviewDetail => f.write_str("appStoreReviewDetail"),
                Self::AppStoreVersionSubmission => f.write_str("appStoreVersionSubmission"),
                Self::AppClipDefaultExperience => f.write_str("appClipDefaultExperience"),
                Self::AppStoreVersionExperiments => f.write_str("appStoreVersionExperiments"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::CustomerReviews => f.write_str("customerReviews"),
                Self::AlternativeDistributionPackage => {
                    f.write_str("alternativeDistributionPackage")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppStoreVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "versionString" => Ok(Self::VersionString),
                "appStoreState" => Ok(Self::AppStoreState),
                "appVersionState" => Ok(Self::AppVersionState),
                "copyright" => Ok(Self::Copyright),
                "reviewType" => Ok(Self::ReviewType),
                "releaseType" => Ok(Self::ReleaseType),
                "earliestReleaseDate" => Ok(Self::EarliestReleaseDate),
                "usesIdfa" => Ok(Self::UsesIdfa),
                "downloadable" => Ok(Self::Downloadable),
                "createdDate" => Ok(Self::CreatedDate),
                "app" => Ok(Self::App),
                "appStoreVersionLocalizations" => Ok(Self::AppStoreVersionLocalizations),
                "build" => Ok(Self::Build),
                "appStoreVersionPhasedRelease" => Ok(Self::AppStoreVersionPhasedRelease),
                "gameCenterAppVersion" => Ok(Self::GameCenterAppVersion),
                "routingAppCoverage" => Ok(Self::RoutingAppCoverage),
                "appStoreReviewDetail" => Ok(Self::AppStoreReviewDetail),
                "appStoreVersionSubmission" => Ok(Self::AppStoreVersionSubmission),
                "appClipDefaultExperience" => Ok(Self::AppClipDefaultExperience),
                "appStoreVersionExperiments" => Ok(Self::AppStoreVersionExperiments),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "customerReviews" => Ok(Self::CustomerReviews),
                "alternativeDistributionPackage" => Ok(Self::AlternativeDistributionPackage),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppStoreVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsAppStoreVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsAppsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "accessibilityUrl",
    ///    "name",
    ///    "bundleId",
    ///    "sku",
    ///    "primaryLocale",
    ///    "isOrEverWasMadeForKids",
    ///    "subscriptionStatusUrl",
    ///    "subscriptionStatusUrlVersion",
    ///    "subscriptionStatusUrlForSandbox",
    ///    "subscriptionStatusUrlVersionForSandbox",
    ///    "contentRightsDeclaration",
    ///    "streamlinedPurchasingEnabled",
    ///    "accessibilityDeclarations",
    ///    "appEncryptionDeclarations",
    ///    "appStoreIcon",
    ///    "ciProduct",
    ///    "betaTesters",
    ///    "betaGroups",
    ///    "appStoreVersions",
    ///    "appTags",
    ///    "preReleaseVersions",
    ///    "betaAppLocalizations",
    ///    "builds",
    ///    "betaLicenseAgreement",
    ///    "betaAppReviewDetail",
    ///    "appInfos",
    ///    "appClips",
    ///    "appPricePoints",
    ///    "endUserLicenseAgreement",
    ///    "appPriceSchedule",
    ///    "appAvailabilityV2",
    ///    "inAppPurchases",
    ///    "subscriptionGroups",
    ///    "gameCenterEnabledVersions",
    ///    "perfPowerMetrics",
    ///    "appCustomProductPages",
    ///    "inAppPurchasesV2",
    ///    "promotedPurchases",
    ///    "appEvents",
    ///    "reviewSubmissions",
    ///    "subscriptionGracePeriod",
    ///    "customerReviews",
    ///    "customerReviewSummarizations",
    ///    "gameCenterDetail",
    ///    "appStoreVersionExperimentsV2",
    ///    "alternativeDistributionKey",
    ///    "analyticsReportRequests",
    ///    "marketplaceSearchDetail",
    ///    "buildUploads",
    ///    "backgroundAssets",
    ///    "betaFeedbackScreenshotSubmissions",
    ///    "betaFeedbackCrashSubmissions",
    ///    "searchKeywords",
    ///    "webhooks",
    ///    "androidToIosAppMappingDetails"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsAppsItem {
        #[serde(rename = "accessibilityUrl")]
        AccessibilityUrl,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "sku")]
        Sku,
        #[serde(rename = "primaryLocale")]
        PrimaryLocale,
        #[serde(rename = "isOrEverWasMadeForKids")]
        IsOrEverWasMadeForKids,
        #[serde(rename = "subscriptionStatusUrl")]
        SubscriptionStatusUrl,
        #[serde(rename = "subscriptionStatusUrlVersion")]
        SubscriptionStatusUrlVersion,
        #[serde(rename = "subscriptionStatusUrlForSandbox")]
        SubscriptionStatusUrlForSandbox,
        #[serde(rename = "subscriptionStatusUrlVersionForSandbox")]
        SubscriptionStatusUrlVersionForSandbox,
        #[serde(rename = "contentRightsDeclaration")]
        ContentRightsDeclaration,
        #[serde(rename = "streamlinedPurchasingEnabled")]
        StreamlinedPurchasingEnabled,
        #[serde(rename = "accessibilityDeclarations")]
        AccessibilityDeclarations,
        #[serde(rename = "appEncryptionDeclarations")]
        AppEncryptionDeclarations,
        #[serde(rename = "appStoreIcon")]
        AppStoreIcon,
        #[serde(rename = "ciProduct")]
        CiProduct,
        #[serde(rename = "betaTesters")]
        BetaTesters,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "appStoreVersions")]
        AppStoreVersions,
        #[serde(rename = "appTags")]
        AppTags,
        #[serde(rename = "preReleaseVersions")]
        PreReleaseVersions,
        #[serde(rename = "betaAppLocalizations")]
        BetaAppLocalizations,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "betaLicenseAgreement")]
        BetaLicenseAgreement,
        #[serde(rename = "betaAppReviewDetail")]
        BetaAppReviewDetail,
        #[serde(rename = "appInfos")]
        AppInfos,
        #[serde(rename = "appClips")]
        AppClips,
        #[serde(rename = "appPricePoints")]
        AppPricePoints,
        #[serde(rename = "endUserLicenseAgreement")]
        EndUserLicenseAgreement,
        #[serde(rename = "appPriceSchedule")]
        AppPriceSchedule,
        #[serde(rename = "appAvailabilityV2")]
        AppAvailabilityV2,
        #[serde(rename = "inAppPurchases")]
        InAppPurchases,
        #[serde(rename = "subscriptionGroups")]
        SubscriptionGroups,
        #[serde(rename = "gameCenterEnabledVersions")]
        GameCenterEnabledVersions,
        #[serde(rename = "perfPowerMetrics")]
        PerfPowerMetrics,
        #[serde(rename = "appCustomProductPages")]
        AppCustomProductPages,
        #[serde(rename = "inAppPurchasesV2")]
        InAppPurchasesV2,
        #[serde(rename = "promotedPurchases")]
        PromotedPurchases,
        #[serde(rename = "appEvents")]
        AppEvents,
        #[serde(rename = "reviewSubmissions")]
        ReviewSubmissions,
        #[serde(rename = "subscriptionGracePeriod")]
        SubscriptionGracePeriod,
        #[serde(rename = "customerReviews")]
        CustomerReviews,
        #[serde(rename = "customerReviewSummarizations")]
        CustomerReviewSummarizations,
        #[serde(rename = "gameCenterDetail")]
        GameCenterDetail,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "alternativeDistributionKey")]
        AlternativeDistributionKey,
        #[serde(rename = "analyticsReportRequests")]
        AnalyticsReportRequests,
        #[serde(rename = "marketplaceSearchDetail")]
        MarketplaceSearchDetail,
        #[serde(rename = "buildUploads")]
        BuildUploads,
        #[serde(rename = "backgroundAssets")]
        BackgroundAssets,
        #[serde(rename = "betaFeedbackScreenshotSubmissions")]
        BetaFeedbackScreenshotSubmissions,
        #[serde(rename = "betaFeedbackCrashSubmissions")]
        BetaFeedbackCrashSubmissions,
        #[serde(rename = "searchKeywords")]
        SearchKeywords,
        #[serde(rename = "webhooks")]
        Webhooks,
        #[serde(rename = "androidToIosAppMappingDetails")]
        AndroidToIosAppMappingDetails,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsAppsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AccessibilityUrl => f.write_str("accessibilityUrl"),
                Self::Name => f.write_str("name"),
                Self::BundleId => f.write_str("bundleId"),
                Self::Sku => f.write_str("sku"),
                Self::PrimaryLocale => f.write_str("primaryLocale"),
                Self::IsOrEverWasMadeForKids => f.write_str("isOrEverWasMadeForKids"),
                Self::SubscriptionStatusUrl => f.write_str("subscriptionStatusUrl"),
                Self::SubscriptionStatusUrlVersion => f.write_str("subscriptionStatusUrlVersion"),
                Self::SubscriptionStatusUrlForSandbox => {
                    f.write_str("subscriptionStatusUrlForSandbox")
                }
                Self::SubscriptionStatusUrlVersionForSandbox => {
                    f.write_str("subscriptionStatusUrlVersionForSandbox")
                }
                Self::ContentRightsDeclaration => f.write_str("contentRightsDeclaration"),
                Self::StreamlinedPurchasingEnabled => f.write_str("streamlinedPurchasingEnabled"),
                Self::AccessibilityDeclarations => f.write_str("accessibilityDeclarations"),
                Self::AppEncryptionDeclarations => f.write_str("appEncryptionDeclarations"),
                Self::AppStoreIcon => f.write_str("appStoreIcon"),
                Self::CiProduct => f.write_str("ciProduct"),
                Self::BetaTesters => f.write_str("betaTesters"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::AppStoreVersions => f.write_str("appStoreVersions"),
                Self::AppTags => f.write_str("appTags"),
                Self::PreReleaseVersions => f.write_str("preReleaseVersions"),
                Self::BetaAppLocalizations => f.write_str("betaAppLocalizations"),
                Self::Builds => f.write_str("builds"),
                Self::BetaLicenseAgreement => f.write_str("betaLicenseAgreement"),
                Self::BetaAppReviewDetail => f.write_str("betaAppReviewDetail"),
                Self::AppInfos => f.write_str("appInfos"),
                Self::AppClips => f.write_str("appClips"),
                Self::AppPricePoints => f.write_str("appPricePoints"),
                Self::EndUserLicenseAgreement => f.write_str("endUserLicenseAgreement"),
                Self::AppPriceSchedule => f.write_str("appPriceSchedule"),
                Self::AppAvailabilityV2 => f.write_str("appAvailabilityV2"),
                Self::InAppPurchases => f.write_str("inAppPurchases"),
                Self::SubscriptionGroups => f.write_str("subscriptionGroups"),
                Self::GameCenterEnabledVersions => f.write_str("gameCenterEnabledVersions"),
                Self::PerfPowerMetrics => f.write_str("perfPowerMetrics"),
                Self::AppCustomProductPages => f.write_str("appCustomProductPages"),
                Self::InAppPurchasesV2 => f.write_str("inAppPurchasesV2"),
                Self::PromotedPurchases => f.write_str("promotedPurchases"),
                Self::AppEvents => f.write_str("appEvents"),
                Self::ReviewSubmissions => f.write_str("reviewSubmissions"),
                Self::SubscriptionGracePeriod => f.write_str("subscriptionGracePeriod"),
                Self::CustomerReviews => f.write_str("customerReviews"),
                Self::CustomerReviewSummarizations => f.write_str("customerReviewSummarizations"),
                Self::GameCenterDetail => f.write_str("gameCenterDetail"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::AlternativeDistributionKey => f.write_str("alternativeDistributionKey"),
                Self::AnalyticsReportRequests => f.write_str("analyticsReportRequests"),
                Self::MarketplaceSearchDetail => f.write_str("marketplaceSearchDetail"),
                Self::BuildUploads => f.write_str("buildUploads"),
                Self::BackgroundAssets => f.write_str("backgroundAssets"),
                Self::BetaFeedbackScreenshotSubmissions => {
                    f.write_str("betaFeedbackScreenshotSubmissions")
                }
                Self::BetaFeedbackCrashSubmissions => f.write_str("betaFeedbackCrashSubmissions"),
                Self::SearchKeywords => f.write_str("searchKeywords"),
                Self::Webhooks => f.write_str("webhooks"),
                Self::AndroidToIosAppMappingDetails => f.write_str("androidToIosAppMappingDetails"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsAppsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "accessibilityUrl" => Ok(Self::AccessibilityUrl),
                "name" => Ok(Self::Name),
                "bundleId" => Ok(Self::BundleId),
                "sku" => Ok(Self::Sku),
                "primaryLocale" => Ok(Self::PrimaryLocale),
                "isOrEverWasMadeForKids" => Ok(Self::IsOrEverWasMadeForKids),
                "subscriptionStatusUrl" => Ok(Self::SubscriptionStatusUrl),
                "subscriptionStatusUrlVersion" => Ok(Self::SubscriptionStatusUrlVersion),
                "subscriptionStatusUrlForSandbox" => Ok(Self::SubscriptionStatusUrlForSandbox),
                "subscriptionStatusUrlVersionForSandbox" => {
                    Ok(Self::SubscriptionStatusUrlVersionForSandbox)
                }
                "contentRightsDeclaration" => Ok(Self::ContentRightsDeclaration),
                "streamlinedPurchasingEnabled" => Ok(Self::StreamlinedPurchasingEnabled),
                "accessibilityDeclarations" => Ok(Self::AccessibilityDeclarations),
                "appEncryptionDeclarations" => Ok(Self::AppEncryptionDeclarations),
                "appStoreIcon" => Ok(Self::AppStoreIcon),
                "ciProduct" => Ok(Self::CiProduct),
                "betaTesters" => Ok(Self::BetaTesters),
                "betaGroups" => Ok(Self::BetaGroups),
                "appStoreVersions" => Ok(Self::AppStoreVersions),
                "appTags" => Ok(Self::AppTags),
                "preReleaseVersions" => Ok(Self::PreReleaseVersions),
                "betaAppLocalizations" => Ok(Self::BetaAppLocalizations),
                "builds" => Ok(Self::Builds),
                "betaLicenseAgreement" => Ok(Self::BetaLicenseAgreement),
                "betaAppReviewDetail" => Ok(Self::BetaAppReviewDetail),
                "appInfos" => Ok(Self::AppInfos),
                "appClips" => Ok(Self::AppClips),
                "appPricePoints" => Ok(Self::AppPricePoints),
                "endUserLicenseAgreement" => Ok(Self::EndUserLicenseAgreement),
                "appPriceSchedule" => Ok(Self::AppPriceSchedule),
                "appAvailabilityV2" => Ok(Self::AppAvailabilityV2),
                "inAppPurchases" => Ok(Self::InAppPurchases),
                "subscriptionGroups" => Ok(Self::SubscriptionGroups),
                "gameCenterEnabledVersions" => Ok(Self::GameCenterEnabledVersions),
                "perfPowerMetrics" => Ok(Self::PerfPowerMetrics),
                "appCustomProductPages" => Ok(Self::AppCustomProductPages),
                "inAppPurchasesV2" => Ok(Self::InAppPurchasesV2),
                "promotedPurchases" => Ok(Self::PromotedPurchases),
                "appEvents" => Ok(Self::AppEvents),
                "reviewSubmissions" => Ok(Self::ReviewSubmissions),
                "subscriptionGracePeriod" => Ok(Self::SubscriptionGracePeriod),
                "customerReviews" => Ok(Self::CustomerReviews),
                "customerReviewSummarizations" => Ok(Self::CustomerReviewSummarizations),
                "gameCenterDetail" => Ok(Self::GameCenterDetail),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "alternativeDistributionKey" => Ok(Self::AlternativeDistributionKey),
                "analyticsReportRequests" => Ok(Self::AnalyticsReportRequests),
                "marketplaceSearchDetail" => Ok(Self::MarketplaceSearchDetail),
                "buildUploads" => Ok(Self::BuildUploads),
                "backgroundAssets" => Ok(Self::BackgroundAssets),
                "betaFeedbackScreenshotSubmissions" => Ok(Self::BetaFeedbackScreenshotSubmissions),
                "betaFeedbackCrashSubmissions" => Ok(Self::BetaFeedbackCrashSubmissions),
                "searchKeywords" => Ok(Self::SearchKeywords),
                "webhooks" => Ok(Self::Webhooks),
                "androidToIosAppMappingDetails" => Ok(Self::AndroidToIosAppMappingDetails),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsBetaAppLocalizationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "feedbackEmail",
    ///    "marketingUrl",
    ///    "privacyPolicyUrl",
    ///    "tvOsPrivacyPolicy",
    ///    "description",
    ///    "locale",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsBetaAppLocalizationsItem {
        #[serde(rename = "feedbackEmail")]
        FeedbackEmail,
        #[serde(rename = "marketingUrl")]
        MarketingUrl,
        #[serde(rename = "privacyPolicyUrl")]
        PrivacyPolicyUrl,
        #[serde(rename = "tvOsPrivacyPolicy")]
        TvOsPrivacyPolicy,
        #[serde(rename = "description")]
        Description,
        #[serde(rename = "locale")]
        Locale,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsBetaAppLocalizationsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::FeedbackEmail => f.write_str("feedbackEmail"),
                Self::MarketingUrl => f.write_str("marketingUrl"),
                Self::PrivacyPolicyUrl => f.write_str("privacyPolicyUrl"),
                Self::TvOsPrivacyPolicy => f.write_str("tvOsPrivacyPolicy"),
                Self::Description => f.write_str("description"),
                Self::Locale => f.write_str("locale"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsBetaAppLocalizationsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "feedbackEmail" => Ok(Self::FeedbackEmail),
                "marketingUrl" => Ok(Self::MarketingUrl),
                "privacyPolicyUrl" => Ok(Self::PrivacyPolicyUrl),
                "tvOsPrivacyPolicy" => Ok(Self::TvOsPrivacyPolicy),
                "description" => Ok(Self::Description),
                "locale" => Ok(Self::Locale),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsBetaAppLocalizationsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsBetaAppLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsBetaAppLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsBetaAppReviewDetailsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "contactFirstName",
    ///    "contactLastName",
    ///    "contactPhone",
    ///    "contactEmail",
    ///    "demoAccountName",
    ///    "demoAccountPassword",
    ///    "demoAccountRequired",
    ///    "notes",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsBetaAppReviewDetailsItem {
        #[serde(rename = "contactFirstName")]
        ContactFirstName,
        #[serde(rename = "contactLastName")]
        ContactLastName,
        #[serde(rename = "contactPhone")]
        ContactPhone,
        #[serde(rename = "contactEmail")]
        ContactEmail,
        #[serde(rename = "demoAccountName")]
        DemoAccountName,
        #[serde(rename = "demoAccountPassword")]
        DemoAccountPassword,
        #[serde(rename = "demoAccountRequired")]
        DemoAccountRequired,
        #[serde(rename = "notes")]
        Notes,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsBetaAppReviewDetailsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ContactFirstName => f.write_str("contactFirstName"),
                Self::ContactLastName => f.write_str("contactLastName"),
                Self::ContactPhone => f.write_str("contactPhone"),
                Self::ContactEmail => f.write_str("contactEmail"),
                Self::DemoAccountName => f.write_str("demoAccountName"),
                Self::DemoAccountPassword => f.write_str("demoAccountPassword"),
                Self::DemoAccountRequired => f.write_str("demoAccountRequired"),
                Self::Notes => f.write_str("notes"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsBetaAppReviewDetailsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "contactFirstName" => Ok(Self::ContactFirstName),
                "contactLastName" => Ok(Self::ContactLastName),
                "contactPhone" => Ok(Self::ContactPhone),
                "contactEmail" => Ok(Self::ContactEmail),
                "demoAccountName" => Ok(Self::DemoAccountName),
                "demoAccountPassword" => Ok(Self::DemoAccountPassword),
                "demoAccountRequired" => Ok(Self::DemoAccountRequired),
                "notes" => Ok(Self::Notes),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsBetaAppReviewDetailsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsBetaAppReviewDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsBetaAppReviewDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsBetaGroupsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "createdDate",
    ///    "isInternalGroup",
    ///    "hasAccessToAllBuilds",
    ///    "publicLinkEnabled",
    ///    "publicLinkId",
    ///    "publicLinkLimitEnabled",
    ///    "publicLinkLimit",
    ///    "publicLink",
    ///    "feedbackEnabled",
    ///    "iosBuildsAvailableForAppleSiliconMac",
    ///    "iosBuildsAvailableForAppleVision",
    ///    "app",
    ///    "builds",
    ///    "betaTesters",
    ///    "betaRecruitmentCriteria",
    ///    "betaRecruitmentCriterionCompatibleBuildCheck"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsBetaGroupsItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "isInternalGroup")]
        IsInternalGroup,
        #[serde(rename = "hasAccessToAllBuilds")]
        HasAccessToAllBuilds,
        #[serde(rename = "publicLinkEnabled")]
        PublicLinkEnabled,
        #[serde(rename = "publicLinkId")]
        PublicLinkId,
        #[serde(rename = "publicLinkLimitEnabled")]
        PublicLinkLimitEnabled,
        #[serde(rename = "publicLinkLimit")]
        PublicLinkLimit,
        #[serde(rename = "publicLink")]
        PublicLink,
        #[serde(rename = "feedbackEnabled")]
        FeedbackEnabled,
        #[serde(rename = "iosBuildsAvailableForAppleSiliconMac")]
        IosBuildsAvailableForAppleSiliconMac,
        #[serde(rename = "iosBuildsAvailableForAppleVision")]
        IosBuildsAvailableForAppleVision,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "betaTesters")]
        BetaTesters,
        #[serde(rename = "betaRecruitmentCriteria")]
        BetaRecruitmentCriteria,
        #[serde(rename = "betaRecruitmentCriterionCompatibleBuildCheck")]
        BetaRecruitmentCriterionCompatibleBuildCheck,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsBetaGroupsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::IsInternalGroup => f.write_str("isInternalGroup"),
                Self::HasAccessToAllBuilds => f.write_str("hasAccessToAllBuilds"),
                Self::PublicLinkEnabled => f.write_str("publicLinkEnabled"),
                Self::PublicLinkId => f.write_str("publicLinkId"),
                Self::PublicLinkLimitEnabled => f.write_str("publicLinkLimitEnabled"),
                Self::PublicLinkLimit => f.write_str("publicLinkLimit"),
                Self::PublicLink => f.write_str("publicLink"),
                Self::FeedbackEnabled => f.write_str("feedbackEnabled"),
                Self::IosBuildsAvailableForAppleSiliconMac => {
                    f.write_str("iosBuildsAvailableForAppleSiliconMac")
                }
                Self::IosBuildsAvailableForAppleVision => {
                    f.write_str("iosBuildsAvailableForAppleVision")
                }
                Self::App => f.write_str("app"),
                Self::Builds => f.write_str("builds"),
                Self::BetaTesters => f.write_str("betaTesters"),
                Self::BetaRecruitmentCriteria => f.write_str("betaRecruitmentCriteria"),
                Self::BetaRecruitmentCriterionCompatibleBuildCheck => {
                    f.write_str("betaRecruitmentCriterionCompatibleBuildCheck")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsBetaGroupsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "createdDate" => Ok(Self::CreatedDate),
                "isInternalGroup" => Ok(Self::IsInternalGroup),
                "hasAccessToAllBuilds" => Ok(Self::HasAccessToAllBuilds),
                "publicLinkEnabled" => Ok(Self::PublicLinkEnabled),
                "publicLinkId" => Ok(Self::PublicLinkId),
                "publicLinkLimitEnabled" => Ok(Self::PublicLinkLimitEnabled),
                "publicLinkLimit" => Ok(Self::PublicLinkLimit),
                "publicLink" => Ok(Self::PublicLink),
                "feedbackEnabled" => Ok(Self::FeedbackEnabled),
                "iosBuildsAvailableForAppleSiliconMac" => {
                    Ok(Self::IosBuildsAvailableForAppleSiliconMac)
                }
                "iosBuildsAvailableForAppleVision" => Ok(Self::IosBuildsAvailableForAppleVision),
                "app" => Ok(Self::App),
                "builds" => Ok(Self::Builds),
                "betaTesters" => Ok(Self::BetaTesters),
                "betaRecruitmentCriteria" => Ok(Self::BetaRecruitmentCriteria),
                "betaRecruitmentCriterionCompatibleBuildCheck" => {
                    Ok(Self::BetaRecruitmentCriterionCompatibleBuildCheck)
                }
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsBetaGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsBetaGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsBetaGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsBetaLicenseAgreementsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "agreementText",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsBetaLicenseAgreementsItem {
        #[serde(rename = "agreementText")]
        AgreementText,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsBetaLicenseAgreementsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AgreementText => f.write_str("agreementText"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsBetaLicenseAgreementsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "agreementText" => Ok(Self::AgreementText),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsBetaLicenseAgreementsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsBetaLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsBetaLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsBuildIconsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "iconAsset",
    ///    "iconType",
    ///    "masked",
    ///    "name"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsBuildIconsItem {
        #[serde(rename = "iconAsset")]
        IconAsset,
        #[serde(rename = "iconType")]
        IconType,
        #[serde(rename = "masked")]
        Masked,
        #[serde(rename = "name")]
        Name,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsBuildIconsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::IconAsset => f.write_str("iconAsset"),
                Self::IconType => f.write_str("iconType"),
                Self::Masked => f.write_str("masked"),
                Self::Name => f.write_str("name"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsBuildIconsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "iconAsset" => Ok(Self::IconAsset),
                "iconType" => Ok(Self::IconType),
                "masked" => Ok(Self::Masked),
                "name" => Ok(Self::Name),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsBuildIconsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsBuildIconsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsBuildIconsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsBuildsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "version",
    ///    "uploadedDate",
    ///    "expirationDate",
    ///    "expired",
    ///    "minOsVersion",
    ///    "lsMinimumSystemVersion",
    ///    "computedMinMacOsVersion",
    ///    "computedMinVisionOsVersion",
    ///    "iconAssetToken",
    ///    "processingState",
    ///    "buildAudienceType",
    ///    "usesNonExemptEncryption",
    ///    "preReleaseVersion",
    ///    "individualTesters",
    ///    "betaGroups",
    ///    "betaBuildLocalizations",
    ///    "appEncryptionDeclaration",
    ///    "betaAppReviewSubmission",
    ///    "app",
    ///    "buildBetaDetail",
    ///    "appStoreVersion",
    ///    "icons",
    ///    "buildBundles",
    ///    "buildUpload",
    ///    "perfPowerMetrics",
    ///    "diagnosticSignatures"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsBuildsItem {
        #[serde(rename = "version")]
        Version,
        #[serde(rename = "uploadedDate")]
        UploadedDate,
        #[serde(rename = "expirationDate")]
        ExpirationDate,
        #[serde(rename = "expired")]
        Expired,
        #[serde(rename = "minOsVersion")]
        MinOsVersion,
        #[serde(rename = "lsMinimumSystemVersion")]
        LsMinimumSystemVersion,
        #[serde(rename = "computedMinMacOsVersion")]
        ComputedMinMacOsVersion,
        #[serde(rename = "computedMinVisionOsVersion")]
        ComputedMinVisionOsVersion,
        #[serde(rename = "iconAssetToken")]
        IconAssetToken,
        #[serde(rename = "processingState")]
        ProcessingState,
        #[serde(rename = "buildAudienceType")]
        BuildAudienceType,
        #[serde(rename = "usesNonExemptEncryption")]
        UsesNonExemptEncryption,
        #[serde(rename = "preReleaseVersion")]
        PreReleaseVersion,
        #[serde(rename = "individualTesters")]
        IndividualTesters,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "betaBuildLocalizations")]
        BetaBuildLocalizations,
        #[serde(rename = "appEncryptionDeclaration")]
        AppEncryptionDeclaration,
        #[serde(rename = "betaAppReviewSubmission")]
        BetaAppReviewSubmission,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "buildBetaDetail")]
        BuildBetaDetail,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "icons")]
        Icons,
        #[serde(rename = "buildBundles")]
        BuildBundles,
        #[serde(rename = "buildUpload")]
        BuildUpload,
        #[serde(rename = "perfPowerMetrics")]
        PerfPowerMetrics,
        #[serde(rename = "diagnosticSignatures")]
        DiagnosticSignatures,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsBuildsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Version => f.write_str("version"),
                Self::UploadedDate => f.write_str("uploadedDate"),
                Self::ExpirationDate => f.write_str("expirationDate"),
                Self::Expired => f.write_str("expired"),
                Self::MinOsVersion => f.write_str("minOsVersion"),
                Self::LsMinimumSystemVersion => f.write_str("lsMinimumSystemVersion"),
                Self::ComputedMinMacOsVersion => f.write_str("computedMinMacOsVersion"),
                Self::ComputedMinVisionOsVersion => f.write_str("computedMinVisionOsVersion"),
                Self::IconAssetToken => f.write_str("iconAssetToken"),
                Self::ProcessingState => f.write_str("processingState"),
                Self::BuildAudienceType => f.write_str("buildAudienceType"),
                Self::UsesNonExemptEncryption => f.write_str("usesNonExemptEncryption"),
                Self::PreReleaseVersion => f.write_str("preReleaseVersion"),
                Self::IndividualTesters => f.write_str("individualTesters"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::BetaBuildLocalizations => f.write_str("betaBuildLocalizations"),
                Self::AppEncryptionDeclaration => f.write_str("appEncryptionDeclaration"),
                Self::BetaAppReviewSubmission => f.write_str("betaAppReviewSubmission"),
                Self::App => f.write_str("app"),
                Self::BuildBetaDetail => f.write_str("buildBetaDetail"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
                Self::Icons => f.write_str("icons"),
                Self::BuildBundles => f.write_str("buildBundles"),
                Self::BuildUpload => f.write_str("buildUpload"),
                Self::PerfPowerMetrics => f.write_str("perfPowerMetrics"),
                Self::DiagnosticSignatures => f.write_str("diagnosticSignatures"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsBuildsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "version" => Ok(Self::Version),
                "uploadedDate" => Ok(Self::UploadedDate),
                "expirationDate" => Ok(Self::ExpirationDate),
                "expired" => Ok(Self::Expired),
                "minOsVersion" => Ok(Self::MinOsVersion),
                "lsMinimumSystemVersion" => Ok(Self::LsMinimumSystemVersion),
                "computedMinMacOsVersion" => Ok(Self::ComputedMinMacOsVersion),
                "computedMinVisionOsVersion" => Ok(Self::ComputedMinVisionOsVersion),
                "iconAssetToken" => Ok(Self::IconAssetToken),
                "processingState" => Ok(Self::ProcessingState),
                "buildAudienceType" => Ok(Self::BuildAudienceType),
                "usesNonExemptEncryption" => Ok(Self::UsesNonExemptEncryption),
                "preReleaseVersion" => Ok(Self::PreReleaseVersion),
                "individualTesters" => Ok(Self::IndividualTesters),
                "betaGroups" => Ok(Self::BetaGroups),
                "betaBuildLocalizations" => Ok(Self::BetaBuildLocalizations),
                "appEncryptionDeclaration" => Ok(Self::AppEncryptionDeclaration),
                "betaAppReviewSubmission" => Ok(Self::BetaAppReviewSubmission),
                "app" => Ok(Self::App),
                "buildBetaDetail" => Ok(Self::BuildBetaDetail),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                "icons" => Ok(Self::Icons),
                "buildBundles" => Ok(Self::BuildBundles),
                "buildUpload" => Ok(Self::BuildUpload),
                "perfPowerMetrics" => Ok(Self::PerfPowerMetrics),
                "diagnosticSignatures" => Ok(Self::DiagnosticSignatures),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsBuildsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsBuildsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsBuildsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsCiProductsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "createdDate",
    ///    "productType",
    ///    "app",
    ///    "bundleId",
    ///    "workflows",
    ///    "primaryRepositories",
    ///    "additionalRepositories",
    ///    "buildRuns"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsCiProductsItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "productType")]
        ProductType,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "workflows")]
        Workflows,
        #[serde(rename = "primaryRepositories")]
        PrimaryRepositories,
        #[serde(rename = "additionalRepositories")]
        AdditionalRepositories,
        #[serde(rename = "buildRuns")]
        BuildRuns,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsCiProductsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::ProductType => f.write_str("productType"),
                Self::App => f.write_str("app"),
                Self::BundleId => f.write_str("bundleId"),
                Self::Workflows => f.write_str("workflows"),
                Self::PrimaryRepositories => f.write_str("primaryRepositories"),
                Self::AdditionalRepositories => f.write_str("additionalRepositories"),
                Self::BuildRuns => f.write_str("buildRuns"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsCiProductsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "createdDate" => Ok(Self::CreatedDate),
                "productType" => Ok(Self::ProductType),
                "app" => Ok(Self::App),
                "bundleId" => Ok(Self::BundleId),
                "workflows" => Ok(Self::Workflows),
                "primaryRepositories" => Ok(Self::PrimaryRepositories),
                "additionalRepositories" => Ok(Self::AdditionalRepositories),
                "buildRuns" => Ok(Self::BuildRuns),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsCiProductsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsCiProductsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsCiProductsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsEndUserLicenseAgreementsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "agreementText",
    ///    "app",
    ///    "territories"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsEndUserLicenseAgreementsItem {
        #[serde(rename = "agreementText")]
        AgreementText,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "territories")]
        Territories,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsEndUserLicenseAgreementsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AgreementText => f.write_str("agreementText"),
                Self::App => f.write_str("app"),
                Self::Territories => f.write_str("territories"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsEndUserLicenseAgreementsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "agreementText" => Ok(Self::AgreementText),
                "app" => Ok(Self::App),
                "territories" => Ok(Self::Territories),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsEndUserLicenseAgreementsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsEndUserLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsEndUserLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsGameCenterDetailsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "arcadeEnabled",
    ///    "challengeEnabled",
    ///    "app",
    ///    "gameCenterAppVersions",
    ///    "gameCenterGroup",
    ///    "gameCenterLeaderboards",
    ///    "gameCenterLeaderboardsV2",
    ///    "gameCenterLeaderboardSets",
    ///    "gameCenterLeaderboardSetsV2",
    ///    "gameCenterAchievements",
    ///    "gameCenterAchievementsV2",
    ///    "gameCenterActivities",
    ///    "gameCenterChallenges",
    ///    "defaultLeaderboard",
    ///    "defaultLeaderboardV2",
    ///    "defaultGroupLeaderboard",
    ///    "defaultGroupLeaderboardV2",
    ///    "achievementReleases",
    ///    "activityReleases",
    ///    "challengeReleases",
    ///    "leaderboardReleases",
    ///    "leaderboardSetReleases",
    ///    "challengesMinimumPlatformVersions"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsGameCenterDetailsItem {
        #[serde(rename = "arcadeEnabled")]
        ArcadeEnabled,
        #[serde(rename = "challengeEnabled")]
        ChallengeEnabled,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "gameCenterAppVersions")]
        GameCenterAppVersions,
        #[serde(rename = "gameCenterGroup")]
        GameCenterGroup,
        #[serde(rename = "gameCenterLeaderboards")]
        GameCenterLeaderboards,
        #[serde(rename = "gameCenterLeaderboardsV2")]
        GameCenterLeaderboardsV2,
        #[serde(rename = "gameCenterLeaderboardSets")]
        GameCenterLeaderboardSets,
        #[serde(rename = "gameCenterLeaderboardSetsV2")]
        GameCenterLeaderboardSetsV2,
        #[serde(rename = "gameCenterAchievements")]
        GameCenterAchievements,
        #[serde(rename = "gameCenterAchievementsV2")]
        GameCenterAchievementsV2,
        #[serde(rename = "gameCenterActivities")]
        GameCenterActivities,
        #[serde(rename = "gameCenterChallenges")]
        GameCenterChallenges,
        #[serde(rename = "defaultLeaderboard")]
        DefaultLeaderboard,
        #[serde(rename = "defaultLeaderboardV2")]
        DefaultLeaderboardV2,
        #[serde(rename = "defaultGroupLeaderboard")]
        DefaultGroupLeaderboard,
        #[serde(rename = "defaultGroupLeaderboardV2")]
        DefaultGroupLeaderboardV2,
        #[serde(rename = "achievementReleases")]
        AchievementReleases,
        #[serde(rename = "activityReleases")]
        ActivityReleases,
        #[serde(rename = "challengeReleases")]
        ChallengeReleases,
        #[serde(rename = "leaderboardReleases")]
        LeaderboardReleases,
        #[serde(rename = "leaderboardSetReleases")]
        LeaderboardSetReleases,
        #[serde(rename = "challengesMinimumPlatformVersions")]
        ChallengesMinimumPlatformVersions,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsGameCenterDetailsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ArcadeEnabled => f.write_str("arcadeEnabled"),
                Self::ChallengeEnabled => f.write_str("challengeEnabled"),
                Self::App => f.write_str("app"),
                Self::GameCenterAppVersions => f.write_str("gameCenterAppVersions"),
                Self::GameCenterGroup => f.write_str("gameCenterGroup"),
                Self::GameCenterLeaderboards => f.write_str("gameCenterLeaderboards"),
                Self::GameCenterLeaderboardsV2 => f.write_str("gameCenterLeaderboardsV2"),
                Self::GameCenterLeaderboardSets => f.write_str("gameCenterLeaderboardSets"),
                Self::GameCenterLeaderboardSetsV2 => f.write_str("gameCenterLeaderboardSetsV2"),
                Self::GameCenterAchievements => f.write_str("gameCenterAchievements"),
                Self::GameCenterAchievementsV2 => f.write_str("gameCenterAchievementsV2"),
                Self::GameCenterActivities => f.write_str("gameCenterActivities"),
                Self::GameCenterChallenges => f.write_str("gameCenterChallenges"),
                Self::DefaultLeaderboard => f.write_str("defaultLeaderboard"),
                Self::DefaultLeaderboardV2 => f.write_str("defaultLeaderboardV2"),
                Self::DefaultGroupLeaderboard => f.write_str("defaultGroupLeaderboard"),
                Self::DefaultGroupLeaderboardV2 => f.write_str("defaultGroupLeaderboardV2"),
                Self::AchievementReleases => f.write_str("achievementReleases"),
                Self::ActivityReleases => f.write_str("activityReleases"),
                Self::ChallengeReleases => f.write_str("challengeReleases"),
                Self::LeaderboardReleases => f.write_str("leaderboardReleases"),
                Self::LeaderboardSetReleases => f.write_str("leaderboardSetReleases"),
                Self::ChallengesMinimumPlatformVersions => {
                    f.write_str("challengesMinimumPlatformVersions")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsGameCenterDetailsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "arcadeEnabled" => Ok(Self::ArcadeEnabled),
                "challengeEnabled" => Ok(Self::ChallengeEnabled),
                "app" => Ok(Self::App),
                "gameCenterAppVersions" => Ok(Self::GameCenterAppVersions),
                "gameCenterGroup" => Ok(Self::GameCenterGroup),
                "gameCenterLeaderboards" => Ok(Self::GameCenterLeaderboards),
                "gameCenterLeaderboardsV2" => Ok(Self::GameCenterLeaderboardsV2),
                "gameCenterLeaderboardSets" => Ok(Self::GameCenterLeaderboardSets),
                "gameCenterLeaderboardSetsV2" => Ok(Self::GameCenterLeaderboardSetsV2),
                "gameCenterAchievements" => Ok(Self::GameCenterAchievements),
                "gameCenterAchievementsV2" => Ok(Self::GameCenterAchievementsV2),
                "gameCenterActivities" => Ok(Self::GameCenterActivities),
                "gameCenterChallenges" => Ok(Self::GameCenterChallenges),
                "defaultLeaderboard" => Ok(Self::DefaultLeaderboard),
                "defaultLeaderboardV2" => Ok(Self::DefaultLeaderboardV2),
                "defaultGroupLeaderboard" => Ok(Self::DefaultGroupLeaderboard),
                "defaultGroupLeaderboardV2" => Ok(Self::DefaultGroupLeaderboardV2),
                "achievementReleases" => Ok(Self::AchievementReleases),
                "activityReleases" => Ok(Self::ActivityReleases),
                "challengeReleases" => Ok(Self::ChallengeReleases),
                "leaderboardReleases" => Ok(Self::LeaderboardReleases),
                "leaderboardSetReleases" => Ok(Self::LeaderboardSetReleases),
                "challengesMinimumPlatformVersions" => Ok(Self::ChallengesMinimumPlatformVersions),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsGameCenterDetailsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsGameCenterDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsGameCenterDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsGameCenterEnabledVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "versionString",
    ///    "iconAsset",
    ///    "compatibleVersions",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsGameCenterEnabledVersionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "versionString")]
        VersionString,
        #[serde(rename = "iconAsset")]
        IconAsset,
        #[serde(rename = "compatibleVersions")]
        CompatibleVersions,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsGameCenterEnabledVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::VersionString => f.write_str("versionString"),
                Self::IconAsset => f.write_str("iconAsset"),
                Self::CompatibleVersions => f.write_str("compatibleVersions"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsGameCenterEnabledVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "versionString" => Ok(Self::VersionString),
                "iconAsset" => Ok(Self::IconAsset),
                "compatibleVersions" => Ok(Self::CompatibleVersions),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsGameCenterEnabledVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsGameCenterEnabledVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsGameCenterEnabledVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsInAppPurchasesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "referenceName",
    ///    "productId",
    ///    "inAppPurchaseType",
    ///    "state",
    ///    "apps",
    ///    "name",
    ///    "reviewNote",
    ///    "familySharable",
    ///    "contentHosting",
    ///    "inAppPurchaseLocalizations",
    ///    "pricePoints",
    ///    "content",
    ///    "appStoreReviewScreenshot",
    ///    "promotedPurchase",
    ///    "iapPriceSchedule",
    ///    "inAppPurchaseAvailability",
    ///    "images",
    ///    "offerCodes"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsInAppPurchasesItem {
        #[serde(rename = "referenceName")]
        ReferenceName,
        #[serde(rename = "productId")]
        ProductId,
        #[serde(rename = "inAppPurchaseType")]
        InAppPurchaseType,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "apps")]
        Apps,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "reviewNote")]
        ReviewNote,
        #[serde(rename = "familySharable")]
        FamilySharable,
        #[serde(rename = "contentHosting")]
        ContentHosting,
        #[serde(rename = "inAppPurchaseLocalizations")]
        InAppPurchaseLocalizations,
        #[serde(rename = "pricePoints")]
        PricePoints,
        #[serde(rename = "content")]
        Content,
        #[serde(rename = "appStoreReviewScreenshot")]
        AppStoreReviewScreenshot,
        #[serde(rename = "promotedPurchase")]
        PromotedPurchase,
        #[serde(rename = "iapPriceSchedule")]
        IapPriceSchedule,
        #[serde(rename = "inAppPurchaseAvailability")]
        InAppPurchaseAvailability,
        #[serde(rename = "images")]
        Images,
        #[serde(rename = "offerCodes")]
        OfferCodes,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsInAppPurchasesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReferenceName => f.write_str("referenceName"),
                Self::ProductId => f.write_str("productId"),
                Self::InAppPurchaseType => f.write_str("inAppPurchaseType"),
                Self::State => f.write_str("state"),
                Self::Apps => f.write_str("apps"),
                Self::Name => f.write_str("name"),
                Self::ReviewNote => f.write_str("reviewNote"),
                Self::FamilySharable => f.write_str("familySharable"),
                Self::ContentHosting => f.write_str("contentHosting"),
                Self::InAppPurchaseLocalizations => f.write_str("inAppPurchaseLocalizations"),
                Self::PricePoints => f.write_str("pricePoints"),
                Self::Content => f.write_str("content"),
                Self::AppStoreReviewScreenshot => f.write_str("appStoreReviewScreenshot"),
                Self::PromotedPurchase => f.write_str("promotedPurchase"),
                Self::IapPriceSchedule => f.write_str("iapPriceSchedule"),
                Self::InAppPurchaseAvailability => f.write_str("inAppPurchaseAvailability"),
                Self::Images => f.write_str("images"),
                Self::OfferCodes => f.write_str("offerCodes"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsInAppPurchasesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "referenceName" => Ok(Self::ReferenceName),
                "productId" => Ok(Self::ProductId),
                "inAppPurchaseType" => Ok(Self::InAppPurchaseType),
                "state" => Ok(Self::State),
                "apps" => Ok(Self::Apps),
                "name" => Ok(Self::Name),
                "reviewNote" => Ok(Self::ReviewNote),
                "familySharable" => Ok(Self::FamilySharable),
                "contentHosting" => Ok(Self::ContentHosting),
                "inAppPurchaseLocalizations" => Ok(Self::InAppPurchaseLocalizations),
                "pricePoints" => Ok(Self::PricePoints),
                "content" => Ok(Self::Content),
                "appStoreReviewScreenshot" => Ok(Self::AppStoreReviewScreenshot),
                "promotedPurchase" => Ok(Self::PromotedPurchase),
                "iapPriceSchedule" => Ok(Self::IapPriceSchedule),
                "inAppPurchaseAvailability" => Ok(Self::InAppPurchaseAvailability),
                "images" => Ok(Self::Images),
                "offerCodes" => Ok(Self::OfferCodes),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsInAppPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionFieldsInAppPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionFieldsInAppPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsPreReleaseVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "version",
    ///    "platform",
    ///    "builds",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsPreReleaseVersionsItem {
        #[serde(rename = "version")]
        Version,
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsPreReleaseVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Version => f.write_str("version"),
                Self::Platform => f.write_str("platform"),
                Self::Builds => f.write_str("builds"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsPreReleaseVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "version" => Ok(Self::Version),
                "platform" => Ok(Self::Platform),
                "builds" => Ok(Self::Builds),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsPreReleaseVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsPreReleaseVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsPreReleaseVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsPromotedPurchasesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "visibleForAllUsers",
    ///    "enabled",
    ///    "state",
    ///    "inAppPurchaseV2",
    ///    "subscription"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsPromotedPurchasesItem {
        #[serde(rename = "visibleForAllUsers")]
        VisibleForAllUsers,
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "inAppPurchaseV2")]
        InAppPurchaseV2,
        #[serde(rename = "subscription")]
        Subscription,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsPromotedPurchasesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::VisibleForAllUsers => f.write_str("visibleForAllUsers"),
                Self::Enabled => f.write_str("enabled"),
                Self::State => f.write_str("state"),
                Self::InAppPurchaseV2 => f.write_str("inAppPurchaseV2"),
                Self::Subscription => f.write_str("subscription"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsPromotedPurchasesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "visibleForAllUsers" => Ok(Self::VisibleForAllUsers),
                "enabled" => Ok(Self::Enabled),
                "state" => Ok(Self::State),
                "inAppPurchaseV2" => Ok(Self::InAppPurchaseV2),
                "subscription" => Ok(Self::Subscription),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsPromotedPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsPromotedPurchasesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsPromotedPurchasesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsReviewSubmissionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "submittedDate",
    ///    "state",
    ///    "app",
    ///    "items",
    ///    "appStoreVersionForReview",
    ///    "submittedByActor",
    ///    "lastUpdatedByActor"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsReviewSubmissionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "submittedDate")]
        SubmittedDate,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "items")]
        Items,
        #[serde(rename = "appStoreVersionForReview")]
        AppStoreVersionForReview,
        #[serde(rename = "submittedByActor")]
        SubmittedByActor,
        #[serde(rename = "lastUpdatedByActor")]
        LastUpdatedByActor,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsReviewSubmissionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::SubmittedDate => f.write_str("submittedDate"),
                Self::State => f.write_str("state"),
                Self::App => f.write_str("app"),
                Self::Items => f.write_str("items"),
                Self::AppStoreVersionForReview => f.write_str("appStoreVersionForReview"),
                Self::SubmittedByActor => f.write_str("submittedByActor"),
                Self::LastUpdatedByActor => f.write_str("lastUpdatedByActor"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsReviewSubmissionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "submittedDate" => Ok(Self::SubmittedDate),
                "state" => Ok(Self::State),
                "app" => Ok(Self::App),
                "items" => Ok(Self::Items),
                "appStoreVersionForReview" => Ok(Self::AppStoreVersionForReview),
                "submittedByActor" => Ok(Self::SubmittedByActor),
                "lastUpdatedByActor" => Ok(Self::LastUpdatedByActor),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsReviewSubmissionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsReviewSubmissionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsReviewSubmissionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsSubscriptionGracePeriodsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "optIn",
    ///    "sandboxOptIn",
    ///    "duration",
    ///    "renewalType"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsSubscriptionGracePeriodsItem {
        #[serde(rename = "optIn")]
        OptIn,
        #[serde(rename = "sandboxOptIn")]
        SandboxOptIn,
        #[serde(rename = "duration")]
        Duration,
        #[serde(rename = "renewalType")]
        RenewalType,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsSubscriptionGracePeriodsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::OptIn => f.write_str("optIn"),
                Self::SandboxOptIn => f.write_str("sandboxOptIn"),
                Self::Duration => f.write_str("duration"),
                Self::RenewalType => f.write_str("renewalType"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsSubscriptionGracePeriodsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "optIn" => Ok(Self::OptIn),
                "sandboxOptIn" => Ok(Self::SandboxOptIn),
                "duration" => Ok(Self::Duration),
                "renewalType" => Ok(Self::RenewalType),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsSubscriptionGracePeriodsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsSubscriptionGracePeriodsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsSubscriptionGracePeriodsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFieldsSubscriptionGroupsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "referenceName",
    ///    "subscriptions",
    ///    "subscriptionGroupLocalizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFieldsSubscriptionGroupsItem {
        #[serde(rename = "referenceName")]
        ReferenceName,
        #[serde(rename = "subscriptions")]
        Subscriptions,
        #[serde(rename = "subscriptionGroupLocalizations")]
        SubscriptionGroupLocalizations,
    }

    impl ::std::fmt::Display for AppsGetCollectionFieldsSubscriptionGroupsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReferenceName => f.write_str("referenceName"),
                Self::Subscriptions => f.write_str("subscriptions"),
                Self::SubscriptionGroupLocalizations => {
                    f.write_str("subscriptionGroupLocalizations")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFieldsSubscriptionGroupsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "referenceName" => Ok(Self::ReferenceName),
                "subscriptions" => Ok(Self::Subscriptions),
                "subscriptionGroupLocalizations" => Ok(Self::SubscriptionGroupLocalizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFieldsSubscriptionGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFieldsSubscriptionGroupsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFieldsSubscriptionGroupsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ACCEPTED",
    ///    "DEVELOPER_REMOVED_FROM_SALE",
    ///    "DEVELOPER_REJECTED",
    ///    "IN_REVIEW",
    ///    "INVALID_BINARY",
    ///    "METADATA_REJECTED",
    ///    "PENDING_APPLE_RELEASE",
    ///    "PENDING_CONTRACT",
    ///    "PENDING_DEVELOPER_RELEASE",
    ///    "PREPARE_FOR_SUBMISSION",
    ///    "PREORDER_READY_FOR_SALE",
    ///    "PROCESSING_FOR_APP_STORE",
    ///    "READY_FOR_REVIEW",
    ///    "READY_FOR_SALE",
    ///    "REJECTED",
    ///    "REMOVED_FROM_SALE",
    ///    "WAITING_FOR_EXPORT_COMPLIANCE",
    ///    "WAITING_FOR_REVIEW",
    ///    "REPLACED_WITH_NEW_VERSION",
    ///    "NOT_APPLICABLE"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem {
        #[serde(rename = "ACCEPTED")]
        Accepted,
        #[serde(rename = "DEVELOPER_REMOVED_FROM_SALE")]
        DeveloperRemovedFromSale,
        #[serde(rename = "DEVELOPER_REJECTED")]
        DeveloperRejected,
        #[serde(rename = "IN_REVIEW")]
        InReview,
        #[serde(rename = "INVALID_BINARY")]
        InvalidBinary,
        #[serde(rename = "METADATA_REJECTED")]
        MetadataRejected,
        #[serde(rename = "PENDING_APPLE_RELEASE")]
        PendingAppleRelease,
        #[serde(rename = "PENDING_CONTRACT")]
        PendingContract,
        #[serde(rename = "PENDING_DEVELOPER_RELEASE")]
        PendingDeveloperRelease,
        #[serde(rename = "PREPARE_FOR_SUBMISSION")]
        PrepareForSubmission,
        #[serde(rename = "PREORDER_READY_FOR_SALE")]
        PreorderReadyForSale,
        #[serde(rename = "PROCESSING_FOR_APP_STORE")]
        ProcessingForAppStore,
        #[serde(rename = "READY_FOR_REVIEW")]
        ReadyForReview,
        #[serde(rename = "READY_FOR_SALE")]
        ReadyForSale,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "REMOVED_FROM_SALE")]
        RemovedFromSale,
        #[serde(rename = "WAITING_FOR_EXPORT_COMPLIANCE")]
        WaitingForExportCompliance,
        #[serde(rename = "WAITING_FOR_REVIEW")]
        WaitingForReview,
        #[serde(rename = "REPLACED_WITH_NEW_VERSION")]
        ReplacedWithNewVersion,
        #[serde(rename = "NOT_APPLICABLE")]
        NotApplicable,
    }

    impl ::std::fmt::Display for AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Accepted => f.write_str("ACCEPTED"),
                Self::DeveloperRemovedFromSale => f.write_str("DEVELOPER_REMOVED_FROM_SALE"),
                Self::DeveloperRejected => f.write_str("DEVELOPER_REJECTED"),
                Self::InReview => f.write_str("IN_REVIEW"),
                Self::InvalidBinary => f.write_str("INVALID_BINARY"),
                Self::MetadataRejected => f.write_str("METADATA_REJECTED"),
                Self::PendingAppleRelease => f.write_str("PENDING_APPLE_RELEASE"),
                Self::PendingContract => f.write_str("PENDING_CONTRACT"),
                Self::PendingDeveloperRelease => f.write_str("PENDING_DEVELOPER_RELEASE"),
                Self::PrepareForSubmission => f.write_str("PREPARE_FOR_SUBMISSION"),
                Self::PreorderReadyForSale => f.write_str("PREORDER_READY_FOR_SALE"),
                Self::ProcessingForAppStore => f.write_str("PROCESSING_FOR_APP_STORE"),
                Self::ReadyForReview => f.write_str("READY_FOR_REVIEW"),
                Self::ReadyForSale => f.write_str("READY_FOR_SALE"),
                Self::Rejected => f.write_str("REJECTED"),
                Self::RemovedFromSale => f.write_str("REMOVED_FROM_SALE"),
                Self::WaitingForExportCompliance => f.write_str("WAITING_FOR_EXPORT_COMPLIANCE"),
                Self::WaitingForReview => f.write_str("WAITING_FOR_REVIEW"),
                Self::ReplacedWithNewVersion => f.write_str("REPLACED_WITH_NEW_VERSION"),
                Self::NotApplicable => f.write_str("NOT_APPLICABLE"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ACCEPTED" => Ok(Self::Accepted),
                "DEVELOPER_REMOVED_FROM_SALE" => Ok(Self::DeveloperRemovedFromSale),
                "DEVELOPER_REJECTED" => Ok(Self::DeveloperRejected),
                "IN_REVIEW" => Ok(Self::InReview),
                "INVALID_BINARY" => Ok(Self::InvalidBinary),
                "METADATA_REJECTED" => Ok(Self::MetadataRejected),
                "PENDING_APPLE_RELEASE" => Ok(Self::PendingAppleRelease),
                "PENDING_CONTRACT" => Ok(Self::PendingContract),
                "PENDING_DEVELOPER_RELEASE" => Ok(Self::PendingDeveloperRelease),
                "PREPARE_FOR_SUBMISSION" => Ok(Self::PrepareForSubmission),
                "PREORDER_READY_FOR_SALE" => Ok(Self::PreorderReadyForSale),
                "PROCESSING_FOR_APP_STORE" => Ok(Self::ProcessingForAppStore),
                "READY_FOR_REVIEW" => Ok(Self::ReadyForReview),
                "READY_FOR_SALE" => Ok(Self::ReadyForSale),
                "REJECTED" => Ok(Self::Rejected),
                "REMOVED_FROM_SALE" => Ok(Self::RemovedFromSale),
                "WAITING_FOR_EXPORT_COMPLIANCE" => Ok(Self::WaitingForExportCompliance),
                "WAITING_FOR_REVIEW" => Ok(Self::WaitingForReview),
                "REPLACED_WITH_NEW_VERSION" => Ok(Self::ReplacedWithNewVersion),
                "NOT_APPLICABLE" => Ok(Self::NotApplicable),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ACCEPTED",
    ///    "DEVELOPER_REJECTED",
    ///    "IN_REVIEW",
    ///    "INVALID_BINARY",
    ///    "METADATA_REJECTED",
    ///    "PENDING_APPLE_RELEASE",
    ///    "PENDING_DEVELOPER_RELEASE",
    ///    "PREPARE_FOR_SUBMISSION",
    ///    "PROCESSING_FOR_DISTRIBUTION",
    ///    "READY_FOR_DISTRIBUTION",
    ///    "READY_FOR_REVIEW",
    ///    "REJECTED",
    ///    "REPLACED_WITH_NEW_VERSION",
    ///    "WAITING_FOR_EXPORT_COMPLIANCE",
    ///    "WAITING_FOR_REVIEW"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem {
        #[serde(rename = "ACCEPTED")]
        Accepted,
        #[serde(rename = "DEVELOPER_REJECTED")]
        DeveloperRejected,
        #[serde(rename = "IN_REVIEW")]
        InReview,
        #[serde(rename = "INVALID_BINARY")]
        InvalidBinary,
        #[serde(rename = "METADATA_REJECTED")]
        MetadataRejected,
        #[serde(rename = "PENDING_APPLE_RELEASE")]
        PendingAppleRelease,
        #[serde(rename = "PENDING_DEVELOPER_RELEASE")]
        PendingDeveloperRelease,
        #[serde(rename = "PREPARE_FOR_SUBMISSION")]
        PrepareForSubmission,
        #[serde(rename = "PROCESSING_FOR_DISTRIBUTION")]
        ProcessingForDistribution,
        #[serde(rename = "READY_FOR_DISTRIBUTION")]
        ReadyForDistribution,
        #[serde(rename = "READY_FOR_REVIEW")]
        ReadyForReview,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "REPLACED_WITH_NEW_VERSION")]
        ReplacedWithNewVersion,
        #[serde(rename = "WAITING_FOR_EXPORT_COMPLIANCE")]
        WaitingForExportCompliance,
        #[serde(rename = "WAITING_FOR_REVIEW")]
        WaitingForReview,
    }

    impl ::std::fmt::Display for AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Accepted => f.write_str("ACCEPTED"),
                Self::DeveloperRejected => f.write_str("DEVELOPER_REJECTED"),
                Self::InReview => f.write_str("IN_REVIEW"),
                Self::InvalidBinary => f.write_str("INVALID_BINARY"),
                Self::MetadataRejected => f.write_str("METADATA_REJECTED"),
                Self::PendingAppleRelease => f.write_str("PENDING_APPLE_RELEASE"),
                Self::PendingDeveloperRelease => f.write_str("PENDING_DEVELOPER_RELEASE"),
                Self::PrepareForSubmission => f.write_str("PREPARE_FOR_SUBMISSION"),
                Self::ProcessingForDistribution => f.write_str("PROCESSING_FOR_DISTRIBUTION"),
                Self::ReadyForDistribution => f.write_str("READY_FOR_DISTRIBUTION"),
                Self::ReadyForReview => f.write_str("READY_FOR_REVIEW"),
                Self::Rejected => f.write_str("REJECTED"),
                Self::ReplacedWithNewVersion => f.write_str("REPLACED_WITH_NEW_VERSION"),
                Self::WaitingForExportCompliance => f.write_str("WAITING_FOR_EXPORT_COMPLIANCE"),
                Self::WaitingForReview => f.write_str("WAITING_FOR_REVIEW"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ACCEPTED" => Ok(Self::Accepted),
                "DEVELOPER_REJECTED" => Ok(Self::DeveloperRejected),
                "IN_REVIEW" => Ok(Self::InReview),
                "INVALID_BINARY" => Ok(Self::InvalidBinary),
                "METADATA_REJECTED" => Ok(Self::MetadataRejected),
                "PENDING_APPLE_RELEASE" => Ok(Self::PendingAppleRelease),
                "PENDING_DEVELOPER_RELEASE" => Ok(Self::PendingDeveloperRelease),
                "PREPARE_FOR_SUBMISSION" => Ok(Self::PrepareForSubmission),
                "PROCESSING_FOR_DISTRIBUTION" => Ok(Self::ProcessingForDistribution),
                "READY_FOR_DISTRIBUTION" => Ok(Self::ReadyForDistribution),
                "READY_FOR_REVIEW" => Ok(Self::ReadyForReview),
                "REJECTED" => Ok(Self::Rejected),
                "REPLACED_WITH_NEW_VERSION" => Ok(Self::ReplacedWithNewVersion),
                "WAITING_FOR_EXPORT_COMPLIANCE" => Ok(Self::WaitingForExportCompliance),
                "WAITING_FOR_REVIEW" => Ok(Self::WaitingForReview),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFilterAppStoreVersionsPlatformItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "IOS",
    ///    "MAC_OS",
    ///    "TV_OS",
    ///    "VISION_OS"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFilterAppStoreVersionsPlatformItem {
        #[serde(rename = "IOS")]
        Ios,
        #[serde(rename = "MAC_OS")]
        MacOs,
        #[serde(rename = "TV_OS")]
        TvOs,
        #[serde(rename = "VISION_OS")]
        VisionOs,
    }

    impl ::std::fmt::Display for AppsGetCollectionFilterAppStoreVersionsPlatformItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ios => f.write_str("IOS"),
                Self::MacOs => f.write_str("MAC_OS"),
                Self::TvOs => f.write_str("TV_OS"),
                Self::VisionOs => f.write_str("VISION_OS"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFilterAppStoreVersionsPlatformItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "IOS" => Ok(Self::Ios),
                "MAC_OS" => Ok(Self::MacOs),
                "TV_OS" => Ok(Self::TvOs),
                "VISION_OS" => Ok(Self::VisionOs),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFilterAppStoreVersionsPlatformItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFilterAppStoreVersionsPlatformItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFilterAppStoreVersionsPlatformItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFilterReviewSubmissionsPlatformItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "IOS",
    ///    "MAC_OS",
    ///    "TV_OS",
    ///    "VISION_OS"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFilterReviewSubmissionsPlatformItem {
        #[serde(rename = "IOS")]
        Ios,
        #[serde(rename = "MAC_OS")]
        MacOs,
        #[serde(rename = "TV_OS")]
        TvOs,
        #[serde(rename = "VISION_OS")]
        VisionOs,
    }

    impl ::std::fmt::Display for AppsGetCollectionFilterReviewSubmissionsPlatformItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ios => f.write_str("IOS"),
                Self::MacOs => f.write_str("MAC_OS"),
                Self::TvOs => f.write_str("TV_OS"),
                Self::VisionOs => f.write_str("VISION_OS"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFilterReviewSubmissionsPlatformItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "IOS" => Ok(Self::Ios),
                "MAC_OS" => Ok(Self::MacOs),
                "TV_OS" => Ok(Self::TvOs),
                "VISION_OS" => Ok(Self::VisionOs),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFilterReviewSubmissionsPlatformItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFilterReviewSubmissionsPlatformItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFilterReviewSubmissionsPlatformItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionFilterReviewSubmissionsStateItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "READY_FOR_REVIEW",
    ///    "WAITING_FOR_REVIEW",
    ///    "IN_REVIEW",
    ///    "UNRESOLVED_ISSUES",
    ///    "CANCELING",
    ///    "COMPLETING",
    ///    "COMPLETE"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionFilterReviewSubmissionsStateItem {
        #[serde(rename = "READY_FOR_REVIEW")]
        ReadyForReview,
        #[serde(rename = "WAITING_FOR_REVIEW")]
        WaitingForReview,
        #[serde(rename = "IN_REVIEW")]
        InReview,
        #[serde(rename = "UNRESOLVED_ISSUES")]
        UnresolvedIssues,
        #[serde(rename = "CANCELING")]
        Canceling,
        #[serde(rename = "COMPLETING")]
        Completing,
        #[serde(rename = "COMPLETE")]
        Complete,
    }

    impl ::std::fmt::Display for AppsGetCollectionFilterReviewSubmissionsStateItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReadyForReview => f.write_str("READY_FOR_REVIEW"),
                Self::WaitingForReview => f.write_str("WAITING_FOR_REVIEW"),
                Self::InReview => f.write_str("IN_REVIEW"),
                Self::UnresolvedIssues => f.write_str("UNRESOLVED_ISSUES"),
                Self::Canceling => f.write_str("CANCELING"),
                Self::Completing => f.write_str("COMPLETING"),
                Self::Complete => f.write_str("COMPLETE"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionFilterReviewSubmissionsStateItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "READY_FOR_REVIEW" => Ok(Self::ReadyForReview),
                "WAITING_FOR_REVIEW" => Ok(Self::WaitingForReview),
                "IN_REVIEW" => Ok(Self::InReview),
                "UNRESOLVED_ISSUES" => Ok(Self::UnresolvedIssues),
                "CANCELING" => Ok(Self::Canceling),
                "COMPLETING" => Ok(Self::Completing),
                "COMPLETE" => Ok(Self::Complete),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionFilterReviewSubmissionsStateItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetCollectionFilterReviewSubmissionsStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetCollectionFilterReviewSubmissionsStateItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionIncludeItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appEncryptionDeclarations",
    ///    "appStoreIcon",
    ///    "ciProduct",
    ///    "betaGroups",
    ///    "appStoreVersions",
    ///    "preReleaseVersions",
    ///    "betaAppLocalizations",
    ///    "builds",
    ///    "betaLicenseAgreement",
    ///    "betaAppReviewDetail",
    ///    "appInfos",
    ///    "appClips",
    ///    "endUserLicenseAgreement",
    ///    "inAppPurchases",
    ///    "subscriptionGroups",
    ///    "gameCenterEnabledVersions",
    ///    "appCustomProductPages",
    ///    "inAppPurchasesV2",
    ///    "promotedPurchases",
    ///    "appEvents",
    ///    "reviewSubmissions",
    ///    "subscriptionGracePeriod",
    ///    "gameCenterDetail",
    ///    "appStoreVersionExperimentsV2",
    ///    "androidToIosAppMappingDetails"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionIncludeItem {
        #[serde(rename = "appEncryptionDeclarations")]
        AppEncryptionDeclarations,
        #[serde(rename = "appStoreIcon")]
        AppStoreIcon,
        #[serde(rename = "ciProduct")]
        CiProduct,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "appStoreVersions")]
        AppStoreVersions,
        #[serde(rename = "preReleaseVersions")]
        PreReleaseVersions,
        #[serde(rename = "betaAppLocalizations")]
        BetaAppLocalizations,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "betaLicenseAgreement")]
        BetaLicenseAgreement,
        #[serde(rename = "betaAppReviewDetail")]
        BetaAppReviewDetail,
        #[serde(rename = "appInfos")]
        AppInfos,
        #[serde(rename = "appClips")]
        AppClips,
        #[serde(rename = "endUserLicenseAgreement")]
        EndUserLicenseAgreement,
        #[serde(rename = "inAppPurchases")]
        InAppPurchases,
        #[serde(rename = "subscriptionGroups")]
        SubscriptionGroups,
        #[serde(rename = "gameCenterEnabledVersions")]
        GameCenterEnabledVersions,
        #[serde(rename = "appCustomProductPages")]
        AppCustomProductPages,
        #[serde(rename = "inAppPurchasesV2")]
        InAppPurchasesV2,
        #[serde(rename = "promotedPurchases")]
        PromotedPurchases,
        #[serde(rename = "appEvents")]
        AppEvents,
        #[serde(rename = "reviewSubmissions")]
        ReviewSubmissions,
        #[serde(rename = "subscriptionGracePeriod")]
        SubscriptionGracePeriod,
        #[serde(rename = "gameCenterDetail")]
        GameCenterDetail,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "androidToIosAppMappingDetails")]
        AndroidToIosAppMappingDetails,
    }

    impl ::std::fmt::Display for AppsGetCollectionIncludeItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppEncryptionDeclarations => f.write_str("appEncryptionDeclarations"),
                Self::AppStoreIcon => f.write_str("appStoreIcon"),
                Self::CiProduct => f.write_str("ciProduct"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::AppStoreVersions => f.write_str("appStoreVersions"),
                Self::PreReleaseVersions => f.write_str("preReleaseVersions"),
                Self::BetaAppLocalizations => f.write_str("betaAppLocalizations"),
                Self::Builds => f.write_str("builds"),
                Self::BetaLicenseAgreement => f.write_str("betaLicenseAgreement"),
                Self::BetaAppReviewDetail => f.write_str("betaAppReviewDetail"),
                Self::AppInfos => f.write_str("appInfos"),
                Self::AppClips => f.write_str("appClips"),
                Self::EndUserLicenseAgreement => f.write_str("endUserLicenseAgreement"),
                Self::InAppPurchases => f.write_str("inAppPurchases"),
                Self::SubscriptionGroups => f.write_str("subscriptionGroups"),
                Self::GameCenterEnabledVersions => f.write_str("gameCenterEnabledVersions"),
                Self::AppCustomProductPages => f.write_str("appCustomProductPages"),
                Self::InAppPurchasesV2 => f.write_str("inAppPurchasesV2"),
                Self::PromotedPurchases => f.write_str("promotedPurchases"),
                Self::AppEvents => f.write_str("appEvents"),
                Self::ReviewSubmissions => f.write_str("reviewSubmissions"),
                Self::SubscriptionGracePeriod => f.write_str("subscriptionGracePeriod"),
                Self::GameCenterDetail => f.write_str("gameCenterDetail"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::AndroidToIosAppMappingDetails => f.write_str("androidToIosAppMappingDetails"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionIncludeItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appEncryptionDeclarations" => Ok(Self::AppEncryptionDeclarations),
                "appStoreIcon" => Ok(Self::AppStoreIcon),
                "ciProduct" => Ok(Self::CiProduct),
                "betaGroups" => Ok(Self::BetaGroups),
                "appStoreVersions" => Ok(Self::AppStoreVersions),
                "preReleaseVersions" => Ok(Self::PreReleaseVersions),
                "betaAppLocalizations" => Ok(Self::BetaAppLocalizations),
                "builds" => Ok(Self::Builds),
                "betaLicenseAgreement" => Ok(Self::BetaLicenseAgreement),
                "betaAppReviewDetail" => Ok(Self::BetaAppReviewDetail),
                "appInfos" => Ok(Self::AppInfos),
                "appClips" => Ok(Self::AppClips),
                "endUserLicenseAgreement" => Ok(Self::EndUserLicenseAgreement),
                "inAppPurchases" => Ok(Self::InAppPurchases),
                "subscriptionGroups" => Ok(Self::SubscriptionGroups),
                "gameCenterEnabledVersions" => Ok(Self::GameCenterEnabledVersions),
                "appCustomProductPages" => Ok(Self::AppCustomProductPages),
                "inAppPurchasesV2" => Ok(Self::InAppPurchasesV2),
                "promotedPurchases" => Ok(Self::PromotedPurchases),
                "appEvents" => Ok(Self::AppEvents),
                "reviewSubmissions" => Ok(Self::ReviewSubmissions),
                "subscriptionGracePeriod" => Ok(Self::SubscriptionGracePeriod),
                "gameCenterDetail" => Ok(Self::GameCenterDetail),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "androidToIosAppMappingDetails" => Ok(Self::AndroidToIosAppMappingDetails),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetCollectionSortItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "-name",
    ///    "bundleId",
    ///    "-bundleId",
    ///    "sku",
    ///    "-sku"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetCollectionSortItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "-name")]
        Xname,
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "-bundleId")]
        XbundleId,
        #[serde(rename = "sku")]
        Sku,
        #[serde(rename = "-sku")]
        Xsku,
    }

    impl ::std::fmt::Display for AppsGetCollectionSortItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::Xname => f.write_str("-name"),
                Self::BundleId => f.write_str("bundleId"),
                Self::XbundleId => f.write_str("-bundleId"),
                Self::Sku => f.write_str("sku"),
                Self::Xsku => f.write_str("-sku"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetCollectionSortItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "-name" => Ok(Self::Xname),
                "bundleId" => Ok(Self::BundleId),
                "-bundleId" => Ok(Self::XbundleId),
                "sku" => Ok(Self::Sku),
                "-sku" => Ok(Self::Xsku),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetCollectionSortItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetCollectionSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetCollectionSortItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "packageName",
    ///    "appSigningKeyPublicCertificateSha256Fingerprints"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem {
        #[serde(rename = "packageName")]
        PackageName,
        #[serde(rename = "appSigningKeyPublicCertificateSha256Fingerprints")]
        AppSigningKeyPublicCertificateSha256Fingerprints,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::PackageName => f.write_str("packageName"),
                Self::AppSigningKeyPublicCertificateSha256Fingerprints => {
                    f.write_str("appSigningKeyPublicCertificateSha256Fingerprints")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "packageName" => Ok(Self::PackageName),
                "appSigningKeyPublicCertificateSha256Fingerprints" => {
                    Ok(Self::AppSigningKeyPublicCertificateSha256Fingerprints)
                }
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppClipsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "bundleId",
    ///    "app",
    ///    "appClipDefaultExperiences",
    ///    "appClipAdvancedExperiences"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppClipsItem {
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appClipDefaultExperiences")]
        AppClipDefaultExperiences,
        #[serde(rename = "appClipAdvancedExperiences")]
        AppClipAdvancedExperiences,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppClipsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::BundleId => f.write_str("bundleId"),
                Self::App => f.write_str("app"),
                Self::AppClipDefaultExperiences => f.write_str("appClipDefaultExperiences"),
                Self::AppClipAdvancedExperiences => f.write_str("appClipAdvancedExperiences"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppClipsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "bundleId" => Ok(Self::BundleId),
                "app" => Ok(Self::App),
                "appClipDefaultExperiences" => Ok(Self::AppClipDefaultExperiences),
                "appClipAdvancedExperiences" => Ok(Self::AppClipAdvancedExperiences),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppClipsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsAppClipsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsAppClipsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppCustomProductPagesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "url",
    ///    "visible",
    ///    "app",
    ///    "appCustomProductPageVersions"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppCustomProductPagesItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "url")]
        Url,
        #[serde(rename = "visible")]
        Visible,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appCustomProductPageVersions")]
        AppCustomProductPageVersions,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppCustomProductPagesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::Url => f.write_str("url"),
                Self::Visible => f.write_str("visible"),
                Self::App => f.write_str("app"),
                Self::AppCustomProductPageVersions => f.write_str("appCustomProductPageVersions"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppCustomProductPagesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "url" => Ok(Self::Url),
                "visible" => Ok(Self::Visible),
                "app" => Ok(Self::App),
                "appCustomProductPageVersions" => Ok(Self::AppCustomProductPageVersions),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppCustomProductPagesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsAppCustomProductPagesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsAppCustomProductPagesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppEncryptionDeclarationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appDescription",
    ///    "createdDate",
    ///    "usesEncryption",
    ///    "exempt",
    ///    "containsProprietaryCryptography",
    ///    "containsThirdPartyCryptography",
    ///    "availableOnFrenchStore",
    ///    "platform",
    ///    "uploadedDate",
    ///    "documentUrl",
    ///    "documentName",
    ///    "documentType",
    ///    "appEncryptionDeclarationState",
    ///    "codeValue",
    ///    "app",
    ///    "builds",
    ///    "appEncryptionDeclarationDocument"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppEncryptionDeclarationsItem {
        #[serde(rename = "appDescription")]
        AppDescription,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "usesEncryption")]
        UsesEncryption,
        #[serde(rename = "exempt")]
        Exempt,
        #[serde(rename = "containsProprietaryCryptography")]
        ContainsProprietaryCryptography,
        #[serde(rename = "containsThirdPartyCryptography")]
        ContainsThirdPartyCryptography,
        #[serde(rename = "availableOnFrenchStore")]
        AvailableOnFrenchStore,
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "uploadedDate")]
        UploadedDate,
        #[serde(rename = "documentUrl")]
        DocumentUrl,
        #[serde(rename = "documentName")]
        DocumentName,
        #[serde(rename = "documentType")]
        DocumentType,
        #[serde(rename = "appEncryptionDeclarationState")]
        AppEncryptionDeclarationState,
        #[serde(rename = "codeValue")]
        CodeValue,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "appEncryptionDeclarationDocument")]
        AppEncryptionDeclarationDocument,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppEncryptionDeclarationsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppDescription => f.write_str("appDescription"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::UsesEncryption => f.write_str("usesEncryption"),
                Self::Exempt => f.write_str("exempt"),
                Self::ContainsProprietaryCryptography => {
                    f.write_str("containsProprietaryCryptography")
                }
                Self::ContainsThirdPartyCryptography => {
                    f.write_str("containsThirdPartyCryptography")
                }
                Self::AvailableOnFrenchStore => f.write_str("availableOnFrenchStore"),
                Self::Platform => f.write_str("platform"),
                Self::UploadedDate => f.write_str("uploadedDate"),
                Self::DocumentUrl => f.write_str("documentUrl"),
                Self::DocumentName => f.write_str("documentName"),
                Self::DocumentType => f.write_str("documentType"),
                Self::AppEncryptionDeclarationState => f.write_str("appEncryptionDeclarationState"),
                Self::CodeValue => f.write_str("codeValue"),
                Self::App => f.write_str("app"),
                Self::Builds => f.write_str("builds"),
                Self::AppEncryptionDeclarationDocument => {
                    f.write_str("appEncryptionDeclarationDocument")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppEncryptionDeclarationsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appDescription" => Ok(Self::AppDescription),
                "createdDate" => Ok(Self::CreatedDate),
                "usesEncryption" => Ok(Self::UsesEncryption),
                "exempt" => Ok(Self::Exempt),
                "containsProprietaryCryptography" => Ok(Self::ContainsProprietaryCryptography),
                "containsThirdPartyCryptography" => Ok(Self::ContainsThirdPartyCryptography),
                "availableOnFrenchStore" => Ok(Self::AvailableOnFrenchStore),
                "platform" => Ok(Self::Platform),
                "uploadedDate" => Ok(Self::UploadedDate),
                "documentUrl" => Ok(Self::DocumentUrl),
                "documentName" => Ok(Self::DocumentName),
                "documentType" => Ok(Self::DocumentType),
                "appEncryptionDeclarationState" => Ok(Self::AppEncryptionDeclarationState),
                "codeValue" => Ok(Self::CodeValue),
                "app" => Ok(Self::App),
                "builds" => Ok(Self::Builds),
                "appEncryptionDeclarationDocument" => Ok(Self::AppEncryptionDeclarationDocument),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppEncryptionDeclarationsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsAppEncryptionDeclarationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsAppEncryptionDeclarationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppEventsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "referenceName",
    ///    "badge",
    ///    "eventState",
    ///    "deepLink",
    ///    "purchaseRequirement",
    ///    "primaryLocale",
    ///    "priority",
    ///    "purpose",
    ///    "territorySchedules",
    ///    "archivedTerritorySchedules",
    ///    "localizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppEventsItem {
        #[serde(rename = "referenceName")]
        ReferenceName,
        #[serde(rename = "badge")]
        Badge,
        #[serde(rename = "eventState")]
        EventState,
        #[serde(rename = "deepLink")]
        DeepLink,
        #[serde(rename = "purchaseRequirement")]
        PurchaseRequirement,
        #[serde(rename = "primaryLocale")]
        PrimaryLocale,
        #[serde(rename = "priority")]
        Priority,
        #[serde(rename = "purpose")]
        Purpose,
        #[serde(rename = "territorySchedules")]
        TerritorySchedules,
        #[serde(rename = "archivedTerritorySchedules")]
        ArchivedTerritorySchedules,
        #[serde(rename = "localizations")]
        Localizations,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppEventsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReferenceName => f.write_str("referenceName"),
                Self::Badge => f.write_str("badge"),
                Self::EventState => f.write_str("eventState"),
                Self::DeepLink => f.write_str("deepLink"),
                Self::PurchaseRequirement => f.write_str("purchaseRequirement"),
                Self::PrimaryLocale => f.write_str("primaryLocale"),
                Self::Priority => f.write_str("priority"),
                Self::Purpose => f.write_str("purpose"),
                Self::TerritorySchedules => f.write_str("territorySchedules"),
                Self::ArchivedTerritorySchedules => f.write_str("archivedTerritorySchedules"),
                Self::Localizations => f.write_str("localizations"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppEventsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "referenceName" => Ok(Self::ReferenceName),
                "badge" => Ok(Self::Badge),
                "eventState" => Ok(Self::EventState),
                "deepLink" => Ok(Self::DeepLink),
                "purchaseRequirement" => Ok(Self::PurchaseRequirement),
                "primaryLocale" => Ok(Self::PrimaryLocale),
                "priority" => Ok(Self::Priority),
                "purpose" => Ok(Self::Purpose),
                "territorySchedules" => Ok(Self::TerritorySchedules),
                "archivedTerritorySchedules" => Ok(Self::ArchivedTerritorySchedules),
                "localizations" => Ok(Self::Localizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsAppEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsAppEventsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppInfosItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appStoreState",
    ///    "state",
    ///    "appStoreAgeRating",
    ///    "australiaAgeRating",
    ///    "brazilAgeRating",
    ///    "brazilAgeRatingV2",
    ///    "franceAgeRating",
    ///    "koreaAgeRating",
    ///    "app",
    ///    "ageRatingDeclaration",
    ///    "appInfoLocalizations",
    ///    "primaryCategory",
    ///    "primarySubcategoryOne",
    ///    "primarySubcategoryTwo",
    ///    "secondaryCategory",
    ///    "secondarySubcategoryOne",
    ///    "secondarySubcategoryTwo",
    ///    "territoryAgeRatings"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppInfosItem {
        #[serde(rename = "appStoreState")]
        AppStoreState,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "appStoreAgeRating")]
        AppStoreAgeRating,
        #[serde(rename = "australiaAgeRating")]
        AustraliaAgeRating,
        #[serde(rename = "brazilAgeRating")]
        BrazilAgeRating,
        #[serde(rename = "brazilAgeRatingV2")]
        BrazilAgeRatingV2,
        #[serde(rename = "franceAgeRating")]
        FranceAgeRating,
        #[serde(rename = "koreaAgeRating")]
        KoreaAgeRating,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "ageRatingDeclaration")]
        AgeRatingDeclaration,
        #[serde(rename = "appInfoLocalizations")]
        AppInfoLocalizations,
        #[serde(rename = "primaryCategory")]
        PrimaryCategory,
        #[serde(rename = "primarySubcategoryOne")]
        PrimarySubcategoryOne,
        #[serde(rename = "primarySubcategoryTwo")]
        PrimarySubcategoryTwo,
        #[serde(rename = "secondaryCategory")]
        SecondaryCategory,
        #[serde(rename = "secondarySubcategoryOne")]
        SecondarySubcategoryOne,
        #[serde(rename = "secondarySubcategoryTwo")]
        SecondarySubcategoryTwo,
        #[serde(rename = "territoryAgeRatings")]
        TerritoryAgeRatings,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppInfosItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppStoreState => f.write_str("appStoreState"),
                Self::State => f.write_str("state"),
                Self::AppStoreAgeRating => f.write_str("appStoreAgeRating"),
                Self::AustraliaAgeRating => f.write_str("australiaAgeRating"),
                Self::BrazilAgeRating => f.write_str("brazilAgeRating"),
                Self::BrazilAgeRatingV2 => f.write_str("brazilAgeRatingV2"),
                Self::FranceAgeRating => f.write_str("franceAgeRating"),
                Self::KoreaAgeRating => f.write_str("koreaAgeRating"),
                Self::App => f.write_str("app"),
                Self::AgeRatingDeclaration => f.write_str("ageRatingDeclaration"),
                Self::AppInfoLocalizations => f.write_str("appInfoLocalizations"),
                Self::PrimaryCategory => f.write_str("primaryCategory"),
                Self::PrimarySubcategoryOne => f.write_str("primarySubcategoryOne"),
                Self::PrimarySubcategoryTwo => f.write_str("primarySubcategoryTwo"),
                Self::SecondaryCategory => f.write_str("secondaryCategory"),
                Self::SecondarySubcategoryOne => f.write_str("secondarySubcategoryOne"),
                Self::SecondarySubcategoryTwo => f.write_str("secondarySubcategoryTwo"),
                Self::TerritoryAgeRatings => f.write_str("territoryAgeRatings"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppInfosItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appStoreState" => Ok(Self::AppStoreState),
                "state" => Ok(Self::State),
                "appStoreAgeRating" => Ok(Self::AppStoreAgeRating),
                "australiaAgeRating" => Ok(Self::AustraliaAgeRating),
                "brazilAgeRating" => Ok(Self::BrazilAgeRating),
                "brazilAgeRatingV2" => Ok(Self::BrazilAgeRatingV2),
                "franceAgeRating" => Ok(Self::FranceAgeRating),
                "koreaAgeRating" => Ok(Self::KoreaAgeRating),
                "app" => Ok(Self::App),
                "ageRatingDeclaration" => Ok(Self::AgeRatingDeclaration),
                "appInfoLocalizations" => Ok(Self::AppInfoLocalizations),
                "primaryCategory" => Ok(Self::PrimaryCategory),
                "primarySubcategoryOne" => Ok(Self::PrimarySubcategoryOne),
                "primarySubcategoryTwo" => Ok(Self::PrimarySubcategoryTwo),
                "secondaryCategory" => Ok(Self::SecondaryCategory),
                "secondarySubcategoryOne" => Ok(Self::SecondarySubcategoryOne),
                "secondarySubcategoryTwo" => Ok(Self::SecondarySubcategoryTwo),
                "territoryAgeRatings" => Ok(Self::TerritoryAgeRatings),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppInfosItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsAppInfosItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsAppInfosItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppStoreVersionExperimentsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "platform",
    ///    "trafficProportion",
    ///    "state",
    ///    "reviewRequired",
    ///    "startDate",
    ///    "endDate",
    ///    "app",
    ///    "latestControlVersion",
    ///    "controlVersions",
    ///    "appStoreVersionExperimentTreatments"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppStoreVersionExperimentsItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "trafficProportion")]
        TrafficProportion,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "reviewRequired")]
        ReviewRequired,
        #[serde(rename = "startDate")]
        StartDate,
        #[serde(rename = "endDate")]
        EndDate,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "latestControlVersion")]
        LatestControlVersion,
        #[serde(rename = "controlVersions")]
        ControlVersions,
        #[serde(rename = "appStoreVersionExperimentTreatments")]
        AppStoreVersionExperimentTreatments,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppStoreVersionExperimentsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::Platform => f.write_str("platform"),
                Self::TrafficProportion => f.write_str("trafficProportion"),
                Self::State => f.write_str("state"),
                Self::ReviewRequired => f.write_str("reviewRequired"),
                Self::StartDate => f.write_str("startDate"),
                Self::EndDate => f.write_str("endDate"),
                Self::App => f.write_str("app"),
                Self::LatestControlVersion => f.write_str("latestControlVersion"),
                Self::ControlVersions => f.write_str("controlVersions"),
                Self::AppStoreVersionExperimentTreatments => {
                    f.write_str("appStoreVersionExperimentTreatments")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppStoreVersionExperimentsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "platform" => Ok(Self::Platform),
                "trafficProportion" => Ok(Self::TrafficProportion),
                "state" => Ok(Self::State),
                "reviewRequired" => Ok(Self::ReviewRequired),
                "startDate" => Ok(Self::StartDate),
                "endDate" => Ok(Self::EndDate),
                "app" => Ok(Self::App),
                "latestControlVersion" => Ok(Self::LatestControlVersion),
                "controlVersions" => Ok(Self::ControlVersions),
                "appStoreVersionExperimentTreatments" => {
                    Ok(Self::AppStoreVersionExperimentTreatments)
                }
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppStoreVersionExperimentsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsAppStoreVersionExperimentsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsAppStoreVersionExperimentsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppStoreVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "versionString",
    ///    "appStoreState",
    ///    "appVersionState",
    ///    "copyright",
    ///    "reviewType",
    ///    "releaseType",
    ///    "earliestReleaseDate",
    ///    "usesIdfa",
    ///    "downloadable",
    ///    "createdDate",
    ///    "app",
    ///    "appStoreVersionLocalizations",
    ///    "build",
    ///    "appStoreVersionPhasedRelease",
    ///    "gameCenterAppVersion",
    ///    "routingAppCoverage",
    ///    "appStoreReviewDetail",
    ///    "appStoreVersionSubmission",
    ///    "appClipDefaultExperience",
    ///    "appStoreVersionExperiments",
    ///    "appStoreVersionExperimentsV2",
    ///    "customerReviews",
    ///    "alternativeDistributionPackage"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppStoreVersionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "versionString")]
        VersionString,
        #[serde(rename = "appStoreState")]
        AppStoreState,
        #[serde(rename = "appVersionState")]
        AppVersionState,
        #[serde(rename = "copyright")]
        Copyright,
        #[serde(rename = "reviewType")]
        ReviewType,
        #[serde(rename = "releaseType")]
        ReleaseType,
        #[serde(rename = "earliestReleaseDate")]
        EarliestReleaseDate,
        #[serde(rename = "usesIdfa")]
        UsesIdfa,
        #[serde(rename = "downloadable")]
        Downloadable,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "appStoreVersionLocalizations")]
        AppStoreVersionLocalizations,
        #[serde(rename = "build")]
        Build,
        #[serde(rename = "appStoreVersionPhasedRelease")]
        AppStoreVersionPhasedRelease,
        #[serde(rename = "gameCenterAppVersion")]
        GameCenterAppVersion,
        #[serde(rename = "routingAppCoverage")]
        RoutingAppCoverage,
        #[serde(rename = "appStoreReviewDetail")]
        AppStoreReviewDetail,
        #[serde(rename = "appStoreVersionSubmission")]
        AppStoreVersionSubmission,
        #[serde(rename = "appClipDefaultExperience")]
        AppClipDefaultExperience,
        #[serde(rename = "appStoreVersionExperiments")]
        AppStoreVersionExperiments,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "customerReviews")]
        CustomerReviews,
        #[serde(rename = "alternativeDistributionPackage")]
        AlternativeDistributionPackage,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppStoreVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::VersionString => f.write_str("versionString"),
                Self::AppStoreState => f.write_str("appStoreState"),
                Self::AppVersionState => f.write_str("appVersionState"),
                Self::Copyright => f.write_str("copyright"),
                Self::ReviewType => f.write_str("reviewType"),
                Self::ReleaseType => f.write_str("releaseType"),
                Self::EarliestReleaseDate => f.write_str("earliestReleaseDate"),
                Self::UsesIdfa => f.write_str("usesIdfa"),
                Self::Downloadable => f.write_str("downloadable"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::App => f.write_str("app"),
                Self::AppStoreVersionLocalizations => f.write_str("appStoreVersionLocalizations"),
                Self::Build => f.write_str("build"),
                Self::AppStoreVersionPhasedRelease => f.write_str("appStoreVersionPhasedRelease"),
                Self::GameCenterAppVersion => f.write_str("gameCenterAppVersion"),
                Self::RoutingAppCoverage => f.write_str("routingAppCoverage"),
                Self::AppStoreReviewDetail => f.write_str("appStoreReviewDetail"),
                Self::AppStoreVersionSubmission => f.write_str("appStoreVersionSubmission"),
                Self::AppClipDefaultExperience => f.write_str("appClipDefaultExperience"),
                Self::AppStoreVersionExperiments => f.write_str("appStoreVersionExperiments"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::CustomerReviews => f.write_str("customerReviews"),
                Self::AlternativeDistributionPackage => {
                    f.write_str("alternativeDistributionPackage")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppStoreVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "versionString" => Ok(Self::VersionString),
                "appStoreState" => Ok(Self::AppStoreState),
                "appVersionState" => Ok(Self::AppVersionState),
                "copyright" => Ok(Self::Copyright),
                "reviewType" => Ok(Self::ReviewType),
                "releaseType" => Ok(Self::ReleaseType),
                "earliestReleaseDate" => Ok(Self::EarliestReleaseDate),
                "usesIdfa" => Ok(Self::UsesIdfa),
                "downloadable" => Ok(Self::Downloadable),
                "createdDate" => Ok(Self::CreatedDate),
                "app" => Ok(Self::App),
                "appStoreVersionLocalizations" => Ok(Self::AppStoreVersionLocalizations),
                "build" => Ok(Self::Build),
                "appStoreVersionPhasedRelease" => Ok(Self::AppStoreVersionPhasedRelease),
                "gameCenterAppVersion" => Ok(Self::GameCenterAppVersion),
                "routingAppCoverage" => Ok(Self::RoutingAppCoverage),
                "appStoreReviewDetail" => Ok(Self::AppStoreReviewDetail),
                "appStoreVersionSubmission" => Ok(Self::AppStoreVersionSubmission),
                "appClipDefaultExperience" => Ok(Self::AppClipDefaultExperience),
                "appStoreVersionExperiments" => Ok(Self::AppStoreVersionExperiments),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "customerReviews" => Ok(Self::CustomerReviews),
                "alternativeDistributionPackage" => Ok(Self::AlternativeDistributionPackage),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppStoreVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsAppStoreVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsAppStoreVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsAppsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "accessibilityUrl",
    ///    "name",
    ///    "bundleId",
    ///    "sku",
    ///    "primaryLocale",
    ///    "isOrEverWasMadeForKids",
    ///    "subscriptionStatusUrl",
    ///    "subscriptionStatusUrlVersion",
    ///    "subscriptionStatusUrlForSandbox",
    ///    "subscriptionStatusUrlVersionForSandbox",
    ///    "contentRightsDeclaration",
    ///    "streamlinedPurchasingEnabled",
    ///    "accessibilityDeclarations",
    ///    "appEncryptionDeclarations",
    ///    "appStoreIcon",
    ///    "ciProduct",
    ///    "betaTesters",
    ///    "betaGroups",
    ///    "appStoreVersions",
    ///    "appTags",
    ///    "preReleaseVersions",
    ///    "betaAppLocalizations",
    ///    "builds",
    ///    "betaLicenseAgreement",
    ///    "betaAppReviewDetail",
    ///    "appInfos",
    ///    "appClips",
    ///    "appPricePoints",
    ///    "endUserLicenseAgreement",
    ///    "appPriceSchedule",
    ///    "appAvailabilityV2",
    ///    "inAppPurchases",
    ///    "subscriptionGroups",
    ///    "gameCenterEnabledVersions",
    ///    "perfPowerMetrics",
    ///    "appCustomProductPages",
    ///    "inAppPurchasesV2",
    ///    "promotedPurchases",
    ///    "appEvents",
    ///    "reviewSubmissions",
    ///    "subscriptionGracePeriod",
    ///    "customerReviews",
    ///    "customerReviewSummarizations",
    ///    "gameCenterDetail",
    ///    "appStoreVersionExperimentsV2",
    ///    "alternativeDistributionKey",
    ///    "analyticsReportRequests",
    ///    "marketplaceSearchDetail",
    ///    "buildUploads",
    ///    "backgroundAssets",
    ///    "betaFeedbackScreenshotSubmissions",
    ///    "betaFeedbackCrashSubmissions",
    ///    "searchKeywords",
    ///    "webhooks",
    ///    "androidToIosAppMappingDetails"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsAppsItem {
        #[serde(rename = "accessibilityUrl")]
        AccessibilityUrl,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "sku")]
        Sku,
        #[serde(rename = "primaryLocale")]
        PrimaryLocale,
        #[serde(rename = "isOrEverWasMadeForKids")]
        IsOrEverWasMadeForKids,
        #[serde(rename = "subscriptionStatusUrl")]
        SubscriptionStatusUrl,
        #[serde(rename = "subscriptionStatusUrlVersion")]
        SubscriptionStatusUrlVersion,
        #[serde(rename = "subscriptionStatusUrlForSandbox")]
        SubscriptionStatusUrlForSandbox,
        #[serde(rename = "subscriptionStatusUrlVersionForSandbox")]
        SubscriptionStatusUrlVersionForSandbox,
        #[serde(rename = "contentRightsDeclaration")]
        ContentRightsDeclaration,
        #[serde(rename = "streamlinedPurchasingEnabled")]
        StreamlinedPurchasingEnabled,
        #[serde(rename = "accessibilityDeclarations")]
        AccessibilityDeclarations,
        #[serde(rename = "appEncryptionDeclarations")]
        AppEncryptionDeclarations,
        #[serde(rename = "appStoreIcon")]
        AppStoreIcon,
        #[serde(rename = "ciProduct")]
        CiProduct,
        #[serde(rename = "betaTesters")]
        BetaTesters,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "appStoreVersions")]
        AppStoreVersions,
        #[serde(rename = "appTags")]
        AppTags,
        #[serde(rename = "preReleaseVersions")]
        PreReleaseVersions,
        #[serde(rename = "betaAppLocalizations")]
        BetaAppLocalizations,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "betaLicenseAgreement")]
        BetaLicenseAgreement,
        #[serde(rename = "betaAppReviewDetail")]
        BetaAppReviewDetail,
        #[serde(rename = "appInfos")]
        AppInfos,
        #[serde(rename = "appClips")]
        AppClips,
        #[serde(rename = "appPricePoints")]
        AppPricePoints,
        #[serde(rename = "endUserLicenseAgreement")]
        EndUserLicenseAgreement,
        #[serde(rename = "appPriceSchedule")]
        AppPriceSchedule,
        #[serde(rename = "appAvailabilityV2")]
        AppAvailabilityV2,
        #[serde(rename = "inAppPurchases")]
        InAppPurchases,
        #[serde(rename = "subscriptionGroups")]
        SubscriptionGroups,
        #[serde(rename = "gameCenterEnabledVersions")]
        GameCenterEnabledVersions,
        #[serde(rename = "perfPowerMetrics")]
        PerfPowerMetrics,
        #[serde(rename = "appCustomProductPages")]
        AppCustomProductPages,
        #[serde(rename = "inAppPurchasesV2")]
        InAppPurchasesV2,
        #[serde(rename = "promotedPurchases")]
        PromotedPurchases,
        #[serde(rename = "appEvents")]
        AppEvents,
        #[serde(rename = "reviewSubmissions")]
        ReviewSubmissions,
        #[serde(rename = "subscriptionGracePeriod")]
        SubscriptionGracePeriod,
        #[serde(rename = "customerReviews")]
        CustomerReviews,
        #[serde(rename = "customerReviewSummarizations")]
        CustomerReviewSummarizations,
        #[serde(rename = "gameCenterDetail")]
        GameCenterDetail,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "alternativeDistributionKey")]
        AlternativeDistributionKey,
        #[serde(rename = "analyticsReportRequests")]
        AnalyticsReportRequests,
        #[serde(rename = "marketplaceSearchDetail")]
        MarketplaceSearchDetail,
        #[serde(rename = "buildUploads")]
        BuildUploads,
        #[serde(rename = "backgroundAssets")]
        BackgroundAssets,
        #[serde(rename = "betaFeedbackScreenshotSubmissions")]
        BetaFeedbackScreenshotSubmissions,
        #[serde(rename = "betaFeedbackCrashSubmissions")]
        BetaFeedbackCrashSubmissions,
        #[serde(rename = "searchKeywords")]
        SearchKeywords,
        #[serde(rename = "webhooks")]
        Webhooks,
        #[serde(rename = "androidToIosAppMappingDetails")]
        AndroidToIosAppMappingDetails,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsAppsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AccessibilityUrl => f.write_str("accessibilityUrl"),
                Self::Name => f.write_str("name"),
                Self::BundleId => f.write_str("bundleId"),
                Self::Sku => f.write_str("sku"),
                Self::PrimaryLocale => f.write_str("primaryLocale"),
                Self::IsOrEverWasMadeForKids => f.write_str("isOrEverWasMadeForKids"),
                Self::SubscriptionStatusUrl => f.write_str("subscriptionStatusUrl"),
                Self::SubscriptionStatusUrlVersion => f.write_str("subscriptionStatusUrlVersion"),
                Self::SubscriptionStatusUrlForSandbox => {
                    f.write_str("subscriptionStatusUrlForSandbox")
                }
                Self::SubscriptionStatusUrlVersionForSandbox => {
                    f.write_str("subscriptionStatusUrlVersionForSandbox")
                }
                Self::ContentRightsDeclaration => f.write_str("contentRightsDeclaration"),
                Self::StreamlinedPurchasingEnabled => f.write_str("streamlinedPurchasingEnabled"),
                Self::AccessibilityDeclarations => f.write_str("accessibilityDeclarations"),
                Self::AppEncryptionDeclarations => f.write_str("appEncryptionDeclarations"),
                Self::AppStoreIcon => f.write_str("appStoreIcon"),
                Self::CiProduct => f.write_str("ciProduct"),
                Self::BetaTesters => f.write_str("betaTesters"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::AppStoreVersions => f.write_str("appStoreVersions"),
                Self::AppTags => f.write_str("appTags"),
                Self::PreReleaseVersions => f.write_str("preReleaseVersions"),
                Self::BetaAppLocalizations => f.write_str("betaAppLocalizations"),
                Self::Builds => f.write_str("builds"),
                Self::BetaLicenseAgreement => f.write_str("betaLicenseAgreement"),
                Self::BetaAppReviewDetail => f.write_str("betaAppReviewDetail"),
                Self::AppInfos => f.write_str("appInfos"),
                Self::AppClips => f.write_str("appClips"),
                Self::AppPricePoints => f.write_str("appPricePoints"),
                Self::EndUserLicenseAgreement => f.write_str("endUserLicenseAgreement"),
                Self::AppPriceSchedule => f.write_str("appPriceSchedule"),
                Self::AppAvailabilityV2 => f.write_str("appAvailabilityV2"),
                Self::InAppPurchases => f.write_str("inAppPurchases"),
                Self::SubscriptionGroups => f.write_str("subscriptionGroups"),
                Self::GameCenterEnabledVersions => f.write_str("gameCenterEnabledVersions"),
                Self::PerfPowerMetrics => f.write_str("perfPowerMetrics"),
                Self::AppCustomProductPages => f.write_str("appCustomProductPages"),
                Self::InAppPurchasesV2 => f.write_str("inAppPurchasesV2"),
                Self::PromotedPurchases => f.write_str("promotedPurchases"),
                Self::AppEvents => f.write_str("appEvents"),
                Self::ReviewSubmissions => f.write_str("reviewSubmissions"),
                Self::SubscriptionGracePeriod => f.write_str("subscriptionGracePeriod"),
                Self::CustomerReviews => f.write_str("customerReviews"),
                Self::CustomerReviewSummarizations => f.write_str("customerReviewSummarizations"),
                Self::GameCenterDetail => f.write_str("gameCenterDetail"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::AlternativeDistributionKey => f.write_str("alternativeDistributionKey"),
                Self::AnalyticsReportRequests => f.write_str("analyticsReportRequests"),
                Self::MarketplaceSearchDetail => f.write_str("marketplaceSearchDetail"),
                Self::BuildUploads => f.write_str("buildUploads"),
                Self::BackgroundAssets => f.write_str("backgroundAssets"),
                Self::BetaFeedbackScreenshotSubmissions => {
                    f.write_str("betaFeedbackScreenshotSubmissions")
                }
                Self::BetaFeedbackCrashSubmissions => f.write_str("betaFeedbackCrashSubmissions"),
                Self::SearchKeywords => f.write_str("searchKeywords"),
                Self::Webhooks => f.write_str("webhooks"),
                Self::AndroidToIosAppMappingDetails => f.write_str("androidToIosAppMappingDetails"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsAppsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "accessibilityUrl" => Ok(Self::AccessibilityUrl),
                "name" => Ok(Self::Name),
                "bundleId" => Ok(Self::BundleId),
                "sku" => Ok(Self::Sku),
                "primaryLocale" => Ok(Self::PrimaryLocale),
                "isOrEverWasMadeForKids" => Ok(Self::IsOrEverWasMadeForKids),
                "subscriptionStatusUrl" => Ok(Self::SubscriptionStatusUrl),
                "subscriptionStatusUrlVersion" => Ok(Self::SubscriptionStatusUrlVersion),
                "subscriptionStatusUrlForSandbox" => Ok(Self::SubscriptionStatusUrlForSandbox),
                "subscriptionStatusUrlVersionForSandbox" => {
                    Ok(Self::SubscriptionStatusUrlVersionForSandbox)
                }
                "contentRightsDeclaration" => Ok(Self::ContentRightsDeclaration),
                "streamlinedPurchasingEnabled" => Ok(Self::StreamlinedPurchasingEnabled),
                "accessibilityDeclarations" => Ok(Self::AccessibilityDeclarations),
                "appEncryptionDeclarations" => Ok(Self::AppEncryptionDeclarations),
                "appStoreIcon" => Ok(Self::AppStoreIcon),
                "ciProduct" => Ok(Self::CiProduct),
                "betaTesters" => Ok(Self::BetaTesters),
                "betaGroups" => Ok(Self::BetaGroups),
                "appStoreVersions" => Ok(Self::AppStoreVersions),
                "appTags" => Ok(Self::AppTags),
                "preReleaseVersions" => Ok(Self::PreReleaseVersions),
                "betaAppLocalizations" => Ok(Self::BetaAppLocalizations),
                "builds" => Ok(Self::Builds),
                "betaLicenseAgreement" => Ok(Self::BetaLicenseAgreement),
                "betaAppReviewDetail" => Ok(Self::BetaAppReviewDetail),
                "appInfos" => Ok(Self::AppInfos),
                "appClips" => Ok(Self::AppClips),
                "appPricePoints" => Ok(Self::AppPricePoints),
                "endUserLicenseAgreement" => Ok(Self::EndUserLicenseAgreement),
                "appPriceSchedule" => Ok(Self::AppPriceSchedule),
                "appAvailabilityV2" => Ok(Self::AppAvailabilityV2),
                "inAppPurchases" => Ok(Self::InAppPurchases),
                "subscriptionGroups" => Ok(Self::SubscriptionGroups),
                "gameCenterEnabledVersions" => Ok(Self::GameCenterEnabledVersions),
                "perfPowerMetrics" => Ok(Self::PerfPowerMetrics),
                "appCustomProductPages" => Ok(Self::AppCustomProductPages),
                "inAppPurchasesV2" => Ok(Self::InAppPurchasesV2),
                "promotedPurchases" => Ok(Self::PromotedPurchases),
                "appEvents" => Ok(Self::AppEvents),
                "reviewSubmissions" => Ok(Self::ReviewSubmissions),
                "subscriptionGracePeriod" => Ok(Self::SubscriptionGracePeriod),
                "customerReviews" => Ok(Self::CustomerReviews),
                "customerReviewSummarizations" => Ok(Self::CustomerReviewSummarizations),
                "gameCenterDetail" => Ok(Self::GameCenterDetail),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "alternativeDistributionKey" => Ok(Self::AlternativeDistributionKey),
                "analyticsReportRequests" => Ok(Self::AnalyticsReportRequests),
                "marketplaceSearchDetail" => Ok(Self::MarketplaceSearchDetail),
                "buildUploads" => Ok(Self::BuildUploads),
                "backgroundAssets" => Ok(Self::BackgroundAssets),
                "betaFeedbackScreenshotSubmissions" => Ok(Self::BetaFeedbackScreenshotSubmissions),
                "betaFeedbackCrashSubmissions" => Ok(Self::BetaFeedbackCrashSubmissions),
                "searchKeywords" => Ok(Self::SearchKeywords),
                "webhooks" => Ok(Self::Webhooks),
                "androidToIosAppMappingDetails" => Ok(Self::AndroidToIosAppMappingDetails),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsAppsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsBetaAppLocalizationsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "feedbackEmail",
    ///    "marketingUrl",
    ///    "privacyPolicyUrl",
    ///    "tvOsPrivacyPolicy",
    ///    "description",
    ///    "locale",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsBetaAppLocalizationsItem {
        #[serde(rename = "feedbackEmail")]
        FeedbackEmail,
        #[serde(rename = "marketingUrl")]
        MarketingUrl,
        #[serde(rename = "privacyPolicyUrl")]
        PrivacyPolicyUrl,
        #[serde(rename = "tvOsPrivacyPolicy")]
        TvOsPrivacyPolicy,
        #[serde(rename = "description")]
        Description,
        #[serde(rename = "locale")]
        Locale,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsBetaAppLocalizationsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::FeedbackEmail => f.write_str("feedbackEmail"),
                Self::MarketingUrl => f.write_str("marketingUrl"),
                Self::PrivacyPolicyUrl => f.write_str("privacyPolicyUrl"),
                Self::TvOsPrivacyPolicy => f.write_str("tvOsPrivacyPolicy"),
                Self::Description => f.write_str("description"),
                Self::Locale => f.write_str("locale"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsBetaAppLocalizationsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "feedbackEmail" => Ok(Self::FeedbackEmail),
                "marketingUrl" => Ok(Self::MarketingUrl),
                "privacyPolicyUrl" => Ok(Self::PrivacyPolicyUrl),
                "tvOsPrivacyPolicy" => Ok(Self::TvOsPrivacyPolicy),
                "description" => Ok(Self::Description),
                "locale" => Ok(Self::Locale),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsBetaAppLocalizationsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsBetaAppLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsBetaAppLocalizationsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsBetaAppReviewDetailsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "contactFirstName",
    ///    "contactLastName",
    ///    "contactPhone",
    ///    "contactEmail",
    ///    "demoAccountName",
    ///    "demoAccountPassword",
    ///    "demoAccountRequired",
    ///    "notes",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsBetaAppReviewDetailsItem {
        #[serde(rename = "contactFirstName")]
        ContactFirstName,
        #[serde(rename = "contactLastName")]
        ContactLastName,
        #[serde(rename = "contactPhone")]
        ContactPhone,
        #[serde(rename = "contactEmail")]
        ContactEmail,
        #[serde(rename = "demoAccountName")]
        DemoAccountName,
        #[serde(rename = "demoAccountPassword")]
        DemoAccountPassword,
        #[serde(rename = "demoAccountRequired")]
        DemoAccountRequired,
        #[serde(rename = "notes")]
        Notes,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsBetaAppReviewDetailsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ContactFirstName => f.write_str("contactFirstName"),
                Self::ContactLastName => f.write_str("contactLastName"),
                Self::ContactPhone => f.write_str("contactPhone"),
                Self::ContactEmail => f.write_str("contactEmail"),
                Self::DemoAccountName => f.write_str("demoAccountName"),
                Self::DemoAccountPassword => f.write_str("demoAccountPassword"),
                Self::DemoAccountRequired => f.write_str("demoAccountRequired"),
                Self::Notes => f.write_str("notes"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsBetaAppReviewDetailsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "contactFirstName" => Ok(Self::ContactFirstName),
                "contactLastName" => Ok(Self::ContactLastName),
                "contactPhone" => Ok(Self::ContactPhone),
                "contactEmail" => Ok(Self::ContactEmail),
                "demoAccountName" => Ok(Self::DemoAccountName),
                "demoAccountPassword" => Ok(Self::DemoAccountPassword),
                "demoAccountRequired" => Ok(Self::DemoAccountRequired),
                "notes" => Ok(Self::Notes),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsBetaAppReviewDetailsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsBetaAppReviewDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsBetaAppReviewDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsBetaGroupsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "createdDate",
    ///    "isInternalGroup",
    ///    "hasAccessToAllBuilds",
    ///    "publicLinkEnabled",
    ///    "publicLinkId",
    ///    "publicLinkLimitEnabled",
    ///    "publicLinkLimit",
    ///    "publicLink",
    ///    "feedbackEnabled",
    ///    "iosBuildsAvailableForAppleSiliconMac",
    ///    "iosBuildsAvailableForAppleVision",
    ///    "app",
    ///    "builds",
    ///    "betaTesters",
    ///    "betaRecruitmentCriteria",
    ///    "betaRecruitmentCriterionCompatibleBuildCheck"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsBetaGroupsItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "isInternalGroup")]
        IsInternalGroup,
        #[serde(rename = "hasAccessToAllBuilds")]
        HasAccessToAllBuilds,
        #[serde(rename = "publicLinkEnabled")]
        PublicLinkEnabled,
        #[serde(rename = "publicLinkId")]
        PublicLinkId,
        #[serde(rename = "publicLinkLimitEnabled")]
        PublicLinkLimitEnabled,
        #[serde(rename = "publicLinkLimit")]
        PublicLinkLimit,
        #[serde(rename = "publicLink")]
        PublicLink,
        #[serde(rename = "feedbackEnabled")]
        FeedbackEnabled,
        #[serde(rename = "iosBuildsAvailableForAppleSiliconMac")]
        IosBuildsAvailableForAppleSiliconMac,
        #[serde(rename = "iosBuildsAvailableForAppleVision")]
        IosBuildsAvailableForAppleVision,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "betaTesters")]
        BetaTesters,
        #[serde(rename = "betaRecruitmentCriteria")]
        BetaRecruitmentCriteria,
        #[serde(rename = "betaRecruitmentCriterionCompatibleBuildCheck")]
        BetaRecruitmentCriterionCompatibleBuildCheck,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsBetaGroupsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::IsInternalGroup => f.write_str("isInternalGroup"),
                Self::HasAccessToAllBuilds => f.write_str("hasAccessToAllBuilds"),
                Self::PublicLinkEnabled => f.write_str("publicLinkEnabled"),
                Self::PublicLinkId => f.write_str("publicLinkId"),
                Self::PublicLinkLimitEnabled => f.write_str("publicLinkLimitEnabled"),
                Self::PublicLinkLimit => f.write_str("publicLinkLimit"),
                Self::PublicLink => f.write_str("publicLink"),
                Self::FeedbackEnabled => f.write_str("feedbackEnabled"),
                Self::IosBuildsAvailableForAppleSiliconMac => {
                    f.write_str("iosBuildsAvailableForAppleSiliconMac")
                }
                Self::IosBuildsAvailableForAppleVision => {
                    f.write_str("iosBuildsAvailableForAppleVision")
                }
                Self::App => f.write_str("app"),
                Self::Builds => f.write_str("builds"),
                Self::BetaTesters => f.write_str("betaTesters"),
                Self::BetaRecruitmentCriteria => f.write_str("betaRecruitmentCriteria"),
                Self::BetaRecruitmentCriterionCompatibleBuildCheck => {
                    f.write_str("betaRecruitmentCriterionCompatibleBuildCheck")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsBetaGroupsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "createdDate" => Ok(Self::CreatedDate),
                "isInternalGroup" => Ok(Self::IsInternalGroup),
                "hasAccessToAllBuilds" => Ok(Self::HasAccessToAllBuilds),
                "publicLinkEnabled" => Ok(Self::PublicLinkEnabled),
                "publicLinkId" => Ok(Self::PublicLinkId),
                "publicLinkLimitEnabled" => Ok(Self::PublicLinkLimitEnabled),
                "publicLinkLimit" => Ok(Self::PublicLinkLimit),
                "publicLink" => Ok(Self::PublicLink),
                "feedbackEnabled" => Ok(Self::FeedbackEnabled),
                "iosBuildsAvailableForAppleSiliconMac" => {
                    Ok(Self::IosBuildsAvailableForAppleSiliconMac)
                }
                "iosBuildsAvailableForAppleVision" => Ok(Self::IosBuildsAvailableForAppleVision),
                "app" => Ok(Self::App),
                "builds" => Ok(Self::Builds),
                "betaTesters" => Ok(Self::BetaTesters),
                "betaRecruitmentCriteria" => Ok(Self::BetaRecruitmentCriteria),
                "betaRecruitmentCriterionCompatibleBuildCheck" => {
                    Ok(Self::BetaRecruitmentCriterionCompatibleBuildCheck)
                }
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsBetaGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsBetaGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsBetaGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsBetaLicenseAgreementsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "agreementText",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsBetaLicenseAgreementsItem {
        #[serde(rename = "agreementText")]
        AgreementText,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsBetaLicenseAgreementsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AgreementText => f.write_str("agreementText"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsBetaLicenseAgreementsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "agreementText" => Ok(Self::AgreementText),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsBetaLicenseAgreementsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsBetaLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsBetaLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsBuildIconsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "iconAsset",
    ///    "iconType",
    ///    "masked",
    ///    "name"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsBuildIconsItem {
        #[serde(rename = "iconAsset")]
        IconAsset,
        #[serde(rename = "iconType")]
        IconType,
        #[serde(rename = "masked")]
        Masked,
        #[serde(rename = "name")]
        Name,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsBuildIconsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::IconAsset => f.write_str("iconAsset"),
                Self::IconType => f.write_str("iconType"),
                Self::Masked => f.write_str("masked"),
                Self::Name => f.write_str("name"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsBuildIconsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "iconAsset" => Ok(Self::IconAsset),
                "iconType" => Ok(Self::IconType),
                "masked" => Ok(Self::Masked),
                "name" => Ok(Self::Name),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsBuildIconsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsBuildIconsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsBuildIconsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsBuildsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "version",
    ///    "uploadedDate",
    ///    "expirationDate",
    ///    "expired",
    ///    "minOsVersion",
    ///    "lsMinimumSystemVersion",
    ///    "computedMinMacOsVersion",
    ///    "computedMinVisionOsVersion",
    ///    "iconAssetToken",
    ///    "processingState",
    ///    "buildAudienceType",
    ///    "usesNonExemptEncryption",
    ///    "preReleaseVersion",
    ///    "individualTesters",
    ///    "betaGroups",
    ///    "betaBuildLocalizations",
    ///    "appEncryptionDeclaration",
    ///    "betaAppReviewSubmission",
    ///    "app",
    ///    "buildBetaDetail",
    ///    "appStoreVersion",
    ///    "icons",
    ///    "buildBundles",
    ///    "buildUpload",
    ///    "perfPowerMetrics",
    ///    "diagnosticSignatures"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsBuildsItem {
        #[serde(rename = "version")]
        Version,
        #[serde(rename = "uploadedDate")]
        UploadedDate,
        #[serde(rename = "expirationDate")]
        ExpirationDate,
        #[serde(rename = "expired")]
        Expired,
        #[serde(rename = "minOsVersion")]
        MinOsVersion,
        #[serde(rename = "lsMinimumSystemVersion")]
        LsMinimumSystemVersion,
        #[serde(rename = "computedMinMacOsVersion")]
        ComputedMinMacOsVersion,
        #[serde(rename = "computedMinVisionOsVersion")]
        ComputedMinVisionOsVersion,
        #[serde(rename = "iconAssetToken")]
        IconAssetToken,
        #[serde(rename = "processingState")]
        ProcessingState,
        #[serde(rename = "buildAudienceType")]
        BuildAudienceType,
        #[serde(rename = "usesNonExemptEncryption")]
        UsesNonExemptEncryption,
        #[serde(rename = "preReleaseVersion")]
        PreReleaseVersion,
        #[serde(rename = "individualTesters")]
        IndividualTesters,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "betaBuildLocalizations")]
        BetaBuildLocalizations,
        #[serde(rename = "appEncryptionDeclaration")]
        AppEncryptionDeclaration,
        #[serde(rename = "betaAppReviewSubmission")]
        BetaAppReviewSubmission,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "buildBetaDetail")]
        BuildBetaDetail,
        #[serde(rename = "appStoreVersion")]
        AppStoreVersion,
        #[serde(rename = "icons")]
        Icons,
        #[serde(rename = "buildBundles")]
        BuildBundles,
        #[serde(rename = "buildUpload")]
        BuildUpload,
        #[serde(rename = "perfPowerMetrics")]
        PerfPowerMetrics,
        #[serde(rename = "diagnosticSignatures")]
        DiagnosticSignatures,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsBuildsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Version => f.write_str("version"),
                Self::UploadedDate => f.write_str("uploadedDate"),
                Self::ExpirationDate => f.write_str("expirationDate"),
                Self::Expired => f.write_str("expired"),
                Self::MinOsVersion => f.write_str("minOsVersion"),
                Self::LsMinimumSystemVersion => f.write_str("lsMinimumSystemVersion"),
                Self::ComputedMinMacOsVersion => f.write_str("computedMinMacOsVersion"),
                Self::ComputedMinVisionOsVersion => f.write_str("computedMinVisionOsVersion"),
                Self::IconAssetToken => f.write_str("iconAssetToken"),
                Self::ProcessingState => f.write_str("processingState"),
                Self::BuildAudienceType => f.write_str("buildAudienceType"),
                Self::UsesNonExemptEncryption => f.write_str("usesNonExemptEncryption"),
                Self::PreReleaseVersion => f.write_str("preReleaseVersion"),
                Self::IndividualTesters => f.write_str("individualTesters"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::BetaBuildLocalizations => f.write_str("betaBuildLocalizations"),
                Self::AppEncryptionDeclaration => f.write_str("appEncryptionDeclaration"),
                Self::BetaAppReviewSubmission => f.write_str("betaAppReviewSubmission"),
                Self::App => f.write_str("app"),
                Self::BuildBetaDetail => f.write_str("buildBetaDetail"),
                Self::AppStoreVersion => f.write_str("appStoreVersion"),
                Self::Icons => f.write_str("icons"),
                Self::BuildBundles => f.write_str("buildBundles"),
                Self::BuildUpload => f.write_str("buildUpload"),
                Self::PerfPowerMetrics => f.write_str("perfPowerMetrics"),
                Self::DiagnosticSignatures => f.write_str("diagnosticSignatures"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsBuildsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "version" => Ok(Self::Version),
                "uploadedDate" => Ok(Self::UploadedDate),
                "expirationDate" => Ok(Self::ExpirationDate),
                "expired" => Ok(Self::Expired),
                "minOsVersion" => Ok(Self::MinOsVersion),
                "lsMinimumSystemVersion" => Ok(Self::LsMinimumSystemVersion),
                "computedMinMacOsVersion" => Ok(Self::ComputedMinMacOsVersion),
                "computedMinVisionOsVersion" => Ok(Self::ComputedMinVisionOsVersion),
                "iconAssetToken" => Ok(Self::IconAssetToken),
                "processingState" => Ok(Self::ProcessingState),
                "buildAudienceType" => Ok(Self::BuildAudienceType),
                "usesNonExemptEncryption" => Ok(Self::UsesNonExemptEncryption),
                "preReleaseVersion" => Ok(Self::PreReleaseVersion),
                "individualTesters" => Ok(Self::IndividualTesters),
                "betaGroups" => Ok(Self::BetaGroups),
                "betaBuildLocalizations" => Ok(Self::BetaBuildLocalizations),
                "appEncryptionDeclaration" => Ok(Self::AppEncryptionDeclaration),
                "betaAppReviewSubmission" => Ok(Self::BetaAppReviewSubmission),
                "app" => Ok(Self::App),
                "buildBetaDetail" => Ok(Self::BuildBetaDetail),
                "appStoreVersion" => Ok(Self::AppStoreVersion),
                "icons" => Ok(Self::Icons),
                "buildBundles" => Ok(Self::BuildBundles),
                "buildUpload" => Ok(Self::BuildUpload),
                "perfPowerMetrics" => Ok(Self::PerfPowerMetrics),
                "diagnosticSignatures" => Ok(Self::DiagnosticSignatures),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsBuildsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsBuildsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsBuildsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsCiProductsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "name",
    ///    "createdDate",
    ///    "productType",
    ///    "app",
    ///    "bundleId",
    ///    "workflows",
    ///    "primaryRepositories",
    ///    "additionalRepositories",
    ///    "buildRuns"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsCiProductsItem {
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "createdDate")]
        CreatedDate,
        #[serde(rename = "productType")]
        ProductType,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "bundleId")]
        BundleId,
        #[serde(rename = "workflows")]
        Workflows,
        #[serde(rename = "primaryRepositories")]
        PrimaryRepositories,
        #[serde(rename = "additionalRepositories")]
        AdditionalRepositories,
        #[serde(rename = "buildRuns")]
        BuildRuns,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsCiProductsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Name => f.write_str("name"),
                Self::CreatedDate => f.write_str("createdDate"),
                Self::ProductType => f.write_str("productType"),
                Self::App => f.write_str("app"),
                Self::BundleId => f.write_str("bundleId"),
                Self::Workflows => f.write_str("workflows"),
                Self::PrimaryRepositories => f.write_str("primaryRepositories"),
                Self::AdditionalRepositories => f.write_str("additionalRepositories"),
                Self::BuildRuns => f.write_str("buildRuns"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsCiProductsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name" => Ok(Self::Name),
                "createdDate" => Ok(Self::CreatedDate),
                "productType" => Ok(Self::ProductType),
                "app" => Ok(Self::App),
                "bundleId" => Ok(Self::BundleId),
                "workflows" => Ok(Self::Workflows),
                "primaryRepositories" => Ok(Self::PrimaryRepositories),
                "additionalRepositories" => Ok(Self::AdditionalRepositories),
                "buildRuns" => Ok(Self::BuildRuns),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsCiProductsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsCiProductsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsCiProductsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsEndUserLicenseAgreementsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "agreementText",
    ///    "app",
    ///    "territories"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsEndUserLicenseAgreementsItem {
        #[serde(rename = "agreementText")]
        AgreementText,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "territories")]
        Territories,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsEndUserLicenseAgreementsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AgreementText => f.write_str("agreementText"),
                Self::App => f.write_str("app"),
                Self::Territories => f.write_str("territories"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsEndUserLicenseAgreementsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "agreementText" => Ok(Self::AgreementText),
                "app" => Ok(Self::App),
                "territories" => Ok(Self::Territories),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsEndUserLicenseAgreementsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsEndUserLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsEndUserLicenseAgreementsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsGameCenterDetailsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "arcadeEnabled",
    ///    "challengeEnabled",
    ///    "app",
    ///    "gameCenterAppVersions",
    ///    "gameCenterGroup",
    ///    "gameCenterLeaderboards",
    ///    "gameCenterLeaderboardsV2",
    ///    "gameCenterLeaderboardSets",
    ///    "gameCenterLeaderboardSetsV2",
    ///    "gameCenterAchievements",
    ///    "gameCenterAchievementsV2",
    ///    "gameCenterActivities",
    ///    "gameCenterChallenges",
    ///    "defaultLeaderboard",
    ///    "defaultLeaderboardV2",
    ///    "defaultGroupLeaderboard",
    ///    "defaultGroupLeaderboardV2",
    ///    "achievementReleases",
    ///    "activityReleases",
    ///    "challengeReleases",
    ///    "leaderboardReleases",
    ///    "leaderboardSetReleases",
    ///    "challengesMinimumPlatformVersions"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsGameCenterDetailsItem {
        #[serde(rename = "arcadeEnabled")]
        ArcadeEnabled,
        #[serde(rename = "challengeEnabled")]
        ChallengeEnabled,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "gameCenterAppVersions")]
        GameCenterAppVersions,
        #[serde(rename = "gameCenterGroup")]
        GameCenterGroup,
        #[serde(rename = "gameCenterLeaderboards")]
        GameCenterLeaderboards,
        #[serde(rename = "gameCenterLeaderboardsV2")]
        GameCenterLeaderboardsV2,
        #[serde(rename = "gameCenterLeaderboardSets")]
        GameCenterLeaderboardSets,
        #[serde(rename = "gameCenterLeaderboardSetsV2")]
        GameCenterLeaderboardSetsV2,
        #[serde(rename = "gameCenterAchievements")]
        GameCenterAchievements,
        #[serde(rename = "gameCenterAchievementsV2")]
        GameCenterAchievementsV2,
        #[serde(rename = "gameCenterActivities")]
        GameCenterActivities,
        #[serde(rename = "gameCenterChallenges")]
        GameCenterChallenges,
        #[serde(rename = "defaultLeaderboard")]
        DefaultLeaderboard,
        #[serde(rename = "defaultLeaderboardV2")]
        DefaultLeaderboardV2,
        #[serde(rename = "defaultGroupLeaderboard")]
        DefaultGroupLeaderboard,
        #[serde(rename = "defaultGroupLeaderboardV2")]
        DefaultGroupLeaderboardV2,
        #[serde(rename = "achievementReleases")]
        AchievementReleases,
        #[serde(rename = "activityReleases")]
        ActivityReleases,
        #[serde(rename = "challengeReleases")]
        ChallengeReleases,
        #[serde(rename = "leaderboardReleases")]
        LeaderboardReleases,
        #[serde(rename = "leaderboardSetReleases")]
        LeaderboardSetReleases,
        #[serde(rename = "challengesMinimumPlatformVersions")]
        ChallengesMinimumPlatformVersions,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsGameCenterDetailsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ArcadeEnabled => f.write_str("arcadeEnabled"),
                Self::ChallengeEnabled => f.write_str("challengeEnabled"),
                Self::App => f.write_str("app"),
                Self::GameCenterAppVersions => f.write_str("gameCenterAppVersions"),
                Self::GameCenterGroup => f.write_str("gameCenterGroup"),
                Self::GameCenterLeaderboards => f.write_str("gameCenterLeaderboards"),
                Self::GameCenterLeaderboardsV2 => f.write_str("gameCenterLeaderboardsV2"),
                Self::GameCenterLeaderboardSets => f.write_str("gameCenterLeaderboardSets"),
                Self::GameCenterLeaderboardSetsV2 => f.write_str("gameCenterLeaderboardSetsV2"),
                Self::GameCenterAchievements => f.write_str("gameCenterAchievements"),
                Self::GameCenterAchievementsV2 => f.write_str("gameCenterAchievementsV2"),
                Self::GameCenterActivities => f.write_str("gameCenterActivities"),
                Self::GameCenterChallenges => f.write_str("gameCenterChallenges"),
                Self::DefaultLeaderboard => f.write_str("defaultLeaderboard"),
                Self::DefaultLeaderboardV2 => f.write_str("defaultLeaderboardV2"),
                Self::DefaultGroupLeaderboard => f.write_str("defaultGroupLeaderboard"),
                Self::DefaultGroupLeaderboardV2 => f.write_str("defaultGroupLeaderboardV2"),
                Self::AchievementReleases => f.write_str("achievementReleases"),
                Self::ActivityReleases => f.write_str("activityReleases"),
                Self::ChallengeReleases => f.write_str("challengeReleases"),
                Self::LeaderboardReleases => f.write_str("leaderboardReleases"),
                Self::LeaderboardSetReleases => f.write_str("leaderboardSetReleases"),
                Self::ChallengesMinimumPlatformVersions => {
                    f.write_str("challengesMinimumPlatformVersions")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsGameCenterDetailsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "arcadeEnabled" => Ok(Self::ArcadeEnabled),
                "challengeEnabled" => Ok(Self::ChallengeEnabled),
                "app" => Ok(Self::App),
                "gameCenterAppVersions" => Ok(Self::GameCenterAppVersions),
                "gameCenterGroup" => Ok(Self::GameCenterGroup),
                "gameCenterLeaderboards" => Ok(Self::GameCenterLeaderboards),
                "gameCenterLeaderboardsV2" => Ok(Self::GameCenterLeaderboardsV2),
                "gameCenterLeaderboardSets" => Ok(Self::GameCenterLeaderboardSets),
                "gameCenterLeaderboardSetsV2" => Ok(Self::GameCenterLeaderboardSetsV2),
                "gameCenterAchievements" => Ok(Self::GameCenterAchievements),
                "gameCenterAchievementsV2" => Ok(Self::GameCenterAchievementsV2),
                "gameCenterActivities" => Ok(Self::GameCenterActivities),
                "gameCenterChallenges" => Ok(Self::GameCenterChallenges),
                "defaultLeaderboard" => Ok(Self::DefaultLeaderboard),
                "defaultLeaderboardV2" => Ok(Self::DefaultLeaderboardV2),
                "defaultGroupLeaderboard" => Ok(Self::DefaultGroupLeaderboard),
                "defaultGroupLeaderboardV2" => Ok(Self::DefaultGroupLeaderboardV2),
                "achievementReleases" => Ok(Self::AchievementReleases),
                "activityReleases" => Ok(Self::ActivityReleases),
                "challengeReleases" => Ok(Self::ChallengeReleases),
                "leaderboardReleases" => Ok(Self::LeaderboardReleases),
                "leaderboardSetReleases" => Ok(Self::LeaderboardSetReleases),
                "challengesMinimumPlatformVersions" => Ok(Self::ChallengesMinimumPlatformVersions),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsGameCenterDetailsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsGameCenterDetailsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsGameCenterDetailsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsGameCenterEnabledVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "versionString",
    ///    "iconAsset",
    ///    "compatibleVersions",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsGameCenterEnabledVersionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "versionString")]
        VersionString,
        #[serde(rename = "iconAsset")]
        IconAsset,
        #[serde(rename = "compatibleVersions")]
        CompatibleVersions,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsGameCenterEnabledVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::VersionString => f.write_str("versionString"),
                Self::IconAsset => f.write_str("iconAsset"),
                Self::CompatibleVersions => f.write_str("compatibleVersions"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsGameCenterEnabledVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "versionString" => Ok(Self::VersionString),
                "iconAsset" => Ok(Self::IconAsset),
                "compatibleVersions" => Ok(Self::CompatibleVersions),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsGameCenterEnabledVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsGameCenterEnabledVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsGameCenterEnabledVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsInAppPurchasesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "referenceName",
    ///    "productId",
    ///    "inAppPurchaseType",
    ///    "state",
    ///    "apps",
    ///    "name",
    ///    "reviewNote",
    ///    "familySharable",
    ///    "contentHosting",
    ///    "inAppPurchaseLocalizations",
    ///    "pricePoints",
    ///    "content",
    ///    "appStoreReviewScreenshot",
    ///    "promotedPurchase",
    ///    "iapPriceSchedule",
    ///    "inAppPurchaseAvailability",
    ///    "images",
    ///    "offerCodes"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsInAppPurchasesItem {
        #[serde(rename = "referenceName")]
        ReferenceName,
        #[serde(rename = "productId")]
        ProductId,
        #[serde(rename = "inAppPurchaseType")]
        InAppPurchaseType,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "apps")]
        Apps,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "reviewNote")]
        ReviewNote,
        #[serde(rename = "familySharable")]
        FamilySharable,
        #[serde(rename = "contentHosting")]
        ContentHosting,
        #[serde(rename = "inAppPurchaseLocalizations")]
        InAppPurchaseLocalizations,
        #[serde(rename = "pricePoints")]
        PricePoints,
        #[serde(rename = "content")]
        Content,
        #[serde(rename = "appStoreReviewScreenshot")]
        AppStoreReviewScreenshot,
        #[serde(rename = "promotedPurchase")]
        PromotedPurchase,
        #[serde(rename = "iapPriceSchedule")]
        IapPriceSchedule,
        #[serde(rename = "inAppPurchaseAvailability")]
        InAppPurchaseAvailability,
        #[serde(rename = "images")]
        Images,
        #[serde(rename = "offerCodes")]
        OfferCodes,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsInAppPurchasesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReferenceName => f.write_str("referenceName"),
                Self::ProductId => f.write_str("productId"),
                Self::InAppPurchaseType => f.write_str("inAppPurchaseType"),
                Self::State => f.write_str("state"),
                Self::Apps => f.write_str("apps"),
                Self::Name => f.write_str("name"),
                Self::ReviewNote => f.write_str("reviewNote"),
                Self::FamilySharable => f.write_str("familySharable"),
                Self::ContentHosting => f.write_str("contentHosting"),
                Self::InAppPurchaseLocalizations => f.write_str("inAppPurchaseLocalizations"),
                Self::PricePoints => f.write_str("pricePoints"),
                Self::Content => f.write_str("content"),
                Self::AppStoreReviewScreenshot => f.write_str("appStoreReviewScreenshot"),
                Self::PromotedPurchase => f.write_str("promotedPurchase"),
                Self::IapPriceSchedule => f.write_str("iapPriceSchedule"),
                Self::InAppPurchaseAvailability => f.write_str("inAppPurchaseAvailability"),
                Self::Images => f.write_str("images"),
                Self::OfferCodes => f.write_str("offerCodes"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsInAppPurchasesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "referenceName" => Ok(Self::ReferenceName),
                "productId" => Ok(Self::ProductId),
                "inAppPurchaseType" => Ok(Self::InAppPurchaseType),
                "state" => Ok(Self::State),
                "apps" => Ok(Self::Apps),
                "name" => Ok(Self::Name),
                "reviewNote" => Ok(Self::ReviewNote),
                "familySharable" => Ok(Self::FamilySharable),
                "contentHosting" => Ok(Self::ContentHosting),
                "inAppPurchaseLocalizations" => Ok(Self::InAppPurchaseLocalizations),
                "pricePoints" => Ok(Self::PricePoints),
                "content" => Ok(Self::Content),
                "appStoreReviewScreenshot" => Ok(Self::AppStoreReviewScreenshot),
                "promotedPurchase" => Ok(Self::PromotedPurchase),
                "iapPriceSchedule" => Ok(Self::IapPriceSchedule),
                "inAppPurchaseAvailability" => Ok(Self::InAppPurchaseAvailability),
                "images" => Ok(Self::Images),
                "offerCodes" => Ok(Self::OfferCodes),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsInAppPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceFieldsInAppPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsInAppPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsPreReleaseVersionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "version",
    ///    "platform",
    ///    "builds",
    ///    "app"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsPreReleaseVersionsItem {
        #[serde(rename = "version")]
        Version,
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "app")]
        App,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsPreReleaseVersionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Version => f.write_str("version"),
                Self::Platform => f.write_str("platform"),
                Self::Builds => f.write_str("builds"),
                Self::App => f.write_str("app"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsPreReleaseVersionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "version" => Ok(Self::Version),
                "platform" => Ok(Self::Platform),
                "builds" => Ok(Self::Builds),
                "app" => Ok(Self::App),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsPreReleaseVersionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsPreReleaseVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsPreReleaseVersionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsPromotedPurchasesItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "visibleForAllUsers",
    ///    "enabled",
    ///    "state",
    ///    "inAppPurchaseV2",
    ///    "subscription"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsPromotedPurchasesItem {
        #[serde(rename = "visibleForAllUsers")]
        VisibleForAllUsers,
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "inAppPurchaseV2")]
        InAppPurchaseV2,
        #[serde(rename = "subscription")]
        Subscription,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsPromotedPurchasesItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::VisibleForAllUsers => f.write_str("visibleForAllUsers"),
                Self::Enabled => f.write_str("enabled"),
                Self::State => f.write_str("state"),
                Self::InAppPurchaseV2 => f.write_str("inAppPurchaseV2"),
                Self::Subscription => f.write_str("subscription"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsPromotedPurchasesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "visibleForAllUsers" => Ok(Self::VisibleForAllUsers),
                "enabled" => Ok(Self::Enabled),
                "state" => Ok(Self::State),
                "inAppPurchaseV2" => Ok(Self::InAppPurchaseV2),
                "subscription" => Ok(Self::Subscription),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsPromotedPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsPromotedPurchasesItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsPromotedPurchasesItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsReviewSubmissionsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "platform",
    ///    "submittedDate",
    ///    "state",
    ///    "app",
    ///    "items",
    ///    "appStoreVersionForReview",
    ///    "submittedByActor",
    ///    "lastUpdatedByActor"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsReviewSubmissionsItem {
        #[serde(rename = "platform")]
        Platform,
        #[serde(rename = "submittedDate")]
        SubmittedDate,
        #[serde(rename = "state")]
        State,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "items")]
        Items,
        #[serde(rename = "appStoreVersionForReview")]
        AppStoreVersionForReview,
        #[serde(rename = "submittedByActor")]
        SubmittedByActor,
        #[serde(rename = "lastUpdatedByActor")]
        LastUpdatedByActor,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsReviewSubmissionsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Platform => f.write_str("platform"),
                Self::SubmittedDate => f.write_str("submittedDate"),
                Self::State => f.write_str("state"),
                Self::App => f.write_str("app"),
                Self::Items => f.write_str("items"),
                Self::AppStoreVersionForReview => f.write_str("appStoreVersionForReview"),
                Self::SubmittedByActor => f.write_str("submittedByActor"),
                Self::LastUpdatedByActor => f.write_str("lastUpdatedByActor"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsReviewSubmissionsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "platform" => Ok(Self::Platform),
                "submittedDate" => Ok(Self::SubmittedDate),
                "state" => Ok(Self::State),
                "app" => Ok(Self::App),
                "items" => Ok(Self::Items),
                "appStoreVersionForReview" => Ok(Self::AppStoreVersionForReview),
                "submittedByActor" => Ok(Self::SubmittedByActor),
                "lastUpdatedByActor" => Ok(Self::LastUpdatedByActor),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsReviewSubmissionsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsReviewSubmissionsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceFieldsReviewSubmissionsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsSubscriptionGracePeriodsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "optIn",
    ///    "sandboxOptIn",
    ///    "duration",
    ///    "renewalType"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsSubscriptionGracePeriodsItem {
        #[serde(rename = "optIn")]
        OptIn,
        #[serde(rename = "sandboxOptIn")]
        SandboxOptIn,
        #[serde(rename = "duration")]
        Duration,
        #[serde(rename = "renewalType")]
        RenewalType,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsSubscriptionGracePeriodsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::OptIn => f.write_str("optIn"),
                Self::SandboxOptIn => f.write_str("sandboxOptIn"),
                Self::Duration => f.write_str("duration"),
                Self::RenewalType => f.write_str("renewalType"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsSubscriptionGracePeriodsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "optIn" => Ok(Self::OptIn),
                "sandboxOptIn" => Ok(Self::SandboxOptIn),
                "duration" => Ok(Self::Duration),
                "renewalType" => Ok(Self::RenewalType),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsSubscriptionGracePeriodsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsSubscriptionGracePeriodsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsSubscriptionGracePeriodsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceFieldsSubscriptionGroupsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "referenceName",
    ///    "subscriptions",
    ///    "subscriptionGroupLocalizations"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceFieldsSubscriptionGroupsItem {
        #[serde(rename = "referenceName")]
        ReferenceName,
        #[serde(rename = "subscriptions")]
        Subscriptions,
        #[serde(rename = "subscriptionGroupLocalizations")]
        SubscriptionGroupLocalizations,
    }

    impl ::std::fmt::Display for AppsGetInstanceFieldsSubscriptionGroupsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReferenceName => f.write_str("referenceName"),
                Self::Subscriptions => f.write_str("subscriptions"),
                Self::SubscriptionGroupLocalizations => {
                    f.write_str("subscriptionGroupLocalizations")
                }
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceFieldsSubscriptionGroupsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "referenceName" => Ok(Self::ReferenceName),
                "subscriptions" => Ok(Self::Subscriptions),
                "subscriptionGroupLocalizations" => Ok(Self::SubscriptionGroupLocalizations),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceFieldsSubscriptionGroupsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AppsGetInstanceFieldsSubscriptionGroupsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for AppsGetInstanceFieldsSubscriptionGroupsItem
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsGetInstanceIncludeItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "appEncryptionDeclarations",
    ///    "appStoreIcon",
    ///    "ciProduct",
    ///    "betaGroups",
    ///    "appStoreVersions",
    ///    "preReleaseVersions",
    ///    "betaAppLocalizations",
    ///    "builds",
    ///    "betaLicenseAgreement",
    ///    "betaAppReviewDetail",
    ///    "appInfos",
    ///    "appClips",
    ///    "endUserLicenseAgreement",
    ///    "inAppPurchases",
    ///    "subscriptionGroups",
    ///    "gameCenterEnabledVersions",
    ///    "appCustomProductPages",
    ///    "inAppPurchasesV2",
    ///    "promotedPurchases",
    ///    "appEvents",
    ///    "reviewSubmissions",
    ///    "subscriptionGracePeriod",
    ///    "gameCenterDetail",
    ///    "appStoreVersionExperimentsV2",
    ///    "androidToIosAppMappingDetails"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AppsGetInstanceIncludeItem {
        #[serde(rename = "appEncryptionDeclarations")]
        AppEncryptionDeclarations,
        #[serde(rename = "appStoreIcon")]
        AppStoreIcon,
        #[serde(rename = "ciProduct")]
        CiProduct,
        #[serde(rename = "betaGroups")]
        BetaGroups,
        #[serde(rename = "appStoreVersions")]
        AppStoreVersions,
        #[serde(rename = "preReleaseVersions")]
        PreReleaseVersions,
        #[serde(rename = "betaAppLocalizations")]
        BetaAppLocalizations,
        #[serde(rename = "builds")]
        Builds,
        #[serde(rename = "betaLicenseAgreement")]
        BetaLicenseAgreement,
        #[serde(rename = "betaAppReviewDetail")]
        BetaAppReviewDetail,
        #[serde(rename = "appInfos")]
        AppInfos,
        #[serde(rename = "appClips")]
        AppClips,
        #[serde(rename = "endUserLicenseAgreement")]
        EndUserLicenseAgreement,
        #[serde(rename = "inAppPurchases")]
        InAppPurchases,
        #[serde(rename = "subscriptionGroups")]
        SubscriptionGroups,
        #[serde(rename = "gameCenterEnabledVersions")]
        GameCenterEnabledVersions,
        #[serde(rename = "appCustomProductPages")]
        AppCustomProductPages,
        #[serde(rename = "inAppPurchasesV2")]
        InAppPurchasesV2,
        #[serde(rename = "promotedPurchases")]
        PromotedPurchases,
        #[serde(rename = "appEvents")]
        AppEvents,
        #[serde(rename = "reviewSubmissions")]
        ReviewSubmissions,
        #[serde(rename = "subscriptionGracePeriod")]
        SubscriptionGracePeriod,
        #[serde(rename = "gameCenterDetail")]
        GameCenterDetail,
        #[serde(rename = "appStoreVersionExperimentsV2")]
        AppStoreVersionExperimentsV2,
        #[serde(rename = "androidToIosAppMappingDetails")]
        AndroidToIosAppMappingDetails,
    }

    impl ::std::fmt::Display for AppsGetInstanceIncludeItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AppEncryptionDeclarations => f.write_str("appEncryptionDeclarations"),
                Self::AppStoreIcon => f.write_str("appStoreIcon"),
                Self::CiProduct => f.write_str("ciProduct"),
                Self::BetaGroups => f.write_str("betaGroups"),
                Self::AppStoreVersions => f.write_str("appStoreVersions"),
                Self::PreReleaseVersions => f.write_str("preReleaseVersions"),
                Self::BetaAppLocalizations => f.write_str("betaAppLocalizations"),
                Self::Builds => f.write_str("builds"),
                Self::BetaLicenseAgreement => f.write_str("betaLicenseAgreement"),
                Self::BetaAppReviewDetail => f.write_str("betaAppReviewDetail"),
                Self::AppInfos => f.write_str("appInfos"),
                Self::AppClips => f.write_str("appClips"),
                Self::EndUserLicenseAgreement => f.write_str("endUserLicenseAgreement"),
                Self::InAppPurchases => f.write_str("inAppPurchases"),
                Self::SubscriptionGroups => f.write_str("subscriptionGroups"),
                Self::GameCenterEnabledVersions => f.write_str("gameCenterEnabledVersions"),
                Self::AppCustomProductPages => f.write_str("appCustomProductPages"),
                Self::InAppPurchasesV2 => f.write_str("inAppPurchasesV2"),
                Self::PromotedPurchases => f.write_str("promotedPurchases"),
                Self::AppEvents => f.write_str("appEvents"),
                Self::ReviewSubmissions => f.write_str("reviewSubmissions"),
                Self::SubscriptionGracePeriod => f.write_str("subscriptionGracePeriod"),
                Self::GameCenterDetail => f.write_str("gameCenterDetail"),
                Self::AppStoreVersionExperimentsV2 => f.write_str("appStoreVersionExperimentsV2"),
                Self::AndroidToIosAppMappingDetails => f.write_str("androidToIosAppMappingDetails"),
            }
        }
    }

    impl ::std::str::FromStr for AppsGetInstanceIncludeItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "appEncryptionDeclarations" => Ok(Self::AppEncryptionDeclarations),
                "appStoreIcon" => Ok(Self::AppStoreIcon),
                "ciProduct" => Ok(Self::CiProduct),
                "betaGroups" => Ok(Self::BetaGroups),
                "appStoreVersions" => Ok(Self::AppStoreVersions),
                "preReleaseVersions" => Ok(Self::PreReleaseVersions),
                "betaAppLocalizations" => Ok(Self::BetaAppLocalizations),
                "builds" => Ok(Self::Builds),
                "betaLicenseAgreement" => Ok(Self::BetaLicenseAgreement),
                "betaAppReviewDetail" => Ok(Self::BetaAppReviewDetail),
                "appInfos" => Ok(Self::AppInfos),
                "appClips" => Ok(Self::AppClips),
                "endUserLicenseAgreement" => Ok(Self::EndUserLicenseAgreement),
                "inAppPurchases" => Ok(Self::InAppPurchases),
                "subscriptionGroups" => Ok(Self::SubscriptionGroups),
                "gameCenterEnabledVersions" => Ok(Self::GameCenterEnabledVersions),
                "appCustomProductPages" => Ok(Self::AppCustomProductPages),
                "inAppPurchasesV2" => Ok(Self::InAppPurchasesV2),
                "promotedPurchases" => Ok(Self::PromotedPurchases),
                "appEvents" => Ok(Self::AppEvents),
                "reviewSubmissions" => Ok(Self::ReviewSubmissions),
                "subscriptionGracePeriod" => Ok(Self::SubscriptionGracePeriod),
                "gameCenterDetail" => Ok(Self::GameCenterDetail),
                "appStoreVersionExperimentsV2" => Ok(Self::AppStoreVersionExperimentsV2),
                "androidToIosAppMappingDetails" => Ok(Self::AndroidToIosAppMappingDetails),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AppsGetInstanceIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AppsGetInstanceIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AppsGetInstanceIncludeItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AppsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "AppsResponse",
    ///  "type": "object",
    ///  "required": [
    ///    "data",
    ///    "links"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/App"
    ///      }
    ///    },
    ///    "included": {
    ///      "type": "array",
    ///      "items": {
    ///        "oneOf": [
    ///          {
    ///            "$ref": "#/components/schemas/Build"
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/PagedDocumentLinks"
    ///    },
    ///    "meta": {
    ///      "$ref": "#/components/schemas/PagingInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AppsResponse {
        pub data: ::std::vec::Vec<App>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub included: ::std::vec::Vec<Build>,
        pub links: PagedDocumentLinks,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub meta: ::std::option::Option<PagingInformation>,
    }

    ///`Build`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "Build",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "properties": {
    ///        "version": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ResourceLinks"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "builds"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Build {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attributes: ::std::option::Option<BuildAttributes>,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ResourceLinks>,
        #[serde(rename = "type")]
        pub type_: BuildType,
    }

    ///`BuildAttributes`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "version": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BuildAttributes {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for BuildAttributes {
        fn default() -> Self {
            Self {
                version: Default::default(),
            }
        }
    }

    ///`BuildType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "builds"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum BuildType {
        #[serde(rename = "builds")]
        Builds,
    }

    impl ::std::fmt::Display for BuildType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Builds => f.write_str("builds"),
            }
        }
    }

    impl ::std::str::FromStr for BuildType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "builds" => Ok(Self::Builds),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for BuildType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for BuildType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for BuildType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`DocumentLinks`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "self"
    ///  ],
    ///  "properties": {
    ///    "self": {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DocumentLinks {
        #[serde(rename = "self")]
        pub self_: ::std::string::String,
    }

    ///`ErrorLinks`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "about": {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    },
    ///    "associated": {
    ///      "oneOf": [
    ///        {
    ///          "type": "string",
    ///          "format": "uri-reference"
    ///        },
    ///        {
    ///          "type": "object",
    ///          "properties": {
    ///            "href": {
    ///              "type": "string",
    ///              "format": "uri-reference"
    ///            },
    ///            "meta": {
    ///              "type": "object",
    ///              "properties": {
    ///                "source": {
    ///                  "type": "string"
    ///                }
    ///              }
    ///            }
    ///          }
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorLinks {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub about: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub associated: ::std::option::Option<ErrorLinksAssociated>,
    }

    impl ::std::default::Default for ErrorLinks {
        fn default() -> Self {
            Self {
                about: Default::default(),
                associated: Default::default(),
            }
        }
    }

    ///`ErrorLinksAssociated`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "properties": {
    ///        "href": {
    ///          "type": "string",
    ///          "format": "uri-reference"
    ///        },
    ///        "meta": {
    ///          "type": "object",
    ///          "properties": {
    ///            "source": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum ErrorLinksAssociated {
        UriReference(::std::string::String),
        Object {
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            href: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            meta: ::std::option::Option<ErrorLinksAssociatedObjectMeta>,
        },
    }

    ///`ErrorLinksAssociatedObjectMeta`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "source": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorLinksAssociatedObjectMeta {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ErrorLinksAssociatedObjectMeta {
        fn default() -> Self {
            Self {
                source: Default::default(),
            }
        }
    }

    ///`ErrorResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "errors": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "code",
    ///          "detail",
    ///          "status",
    ///          "title"
    ///        ],
    ///        "properties": {
    ///          "code": {
    ///            "type": "string"
    ///          },
    ///          "detail": {
    ///            "type": "string"
    ///          },
    ///          "id": {
    ///            "type": "string"
    ///          },
    ///          "links": {
    ///            "$ref": "#/components/schemas/ErrorLinks"
    ///          },
    ///          "meta": {
    ///            "type": "object",
    ///            "additionalProperties": {}
    ///          },
    ///          "source": {
    ///            "oneOf": [
    ///              {
    ///                "$ref": "#/components/schemas/ErrorSourcePointer"
    ///              },
    ///              {
    ///                "$ref": "#/components/schemas/ErrorSourceParameter"
    ///              }
    ///            ]
    ///          },
    ///          "status": {
    ///            "type": "string"
    ///          },
    ///          "title": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub errors: ::std::vec::Vec<ErrorResponseErrorsItem>,
    }

    impl ::std::default::Default for ErrorResponse {
        fn default() -> Self {
            Self {
                errors: Default::default(),
            }
        }
    }

    ///`ErrorResponseErrorsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "detail",
    ///    "status",
    ///    "title"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "detail": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "links": {
    ///      "$ref": "#/components/schemas/ErrorLinks"
    ///    },
    ///    "meta": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "source": {
    ///      "oneOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ErrorSourcePointer"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/ErrorSourceParameter"
    ///        }
    ///      ]
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorResponseErrorsItem {
        pub code: ::std::string::String,
        pub detail: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub links: ::std::option::Option<ErrorLinks>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<ErrorResponseErrorsItemSource>,
        pub status: ::std::string::String,
        pub title: ::std::string::String,
    }

    ///`ErrorResponseErrorsItemSource`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/ErrorSourcePointer"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/ErrorSourceParameter"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum ErrorResponseErrorsItemSource {
        Pointer(ErrorSourcePointer),
        Parameter(ErrorSourceParameter),
    }

    impl ::std::convert::From<ErrorSourcePointer> for ErrorResponseErrorsItemSource {
        fn from(value: ErrorSourcePointer) -> Self {
            Self::Pointer(value)
        }
    }

    impl ::std::convert::From<ErrorSourceParameter> for ErrorResponseErrorsItemSource {
        fn from(value: ErrorSourceParameter) -> Self {
            Self::Parameter(value)
        }
    }

    ///`ErrorSourceParameter`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "Parameter",
    ///  "type": "object",
    ///  "required": [
    ///    "parameter"
    ///  ],
    ///  "properties": {
    ///    "parameter": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorSourceParameter {
        pub parameter: ::std::string::String,
    }

    ///`ErrorSourcePointer`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonPointer",
    ///  "type": "object",
    ///  "required": [
    ///    "pointer"
    ///  ],
    ///  "properties": {
    ///    "pointer": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorSourcePointer {
        pub pointer: ::std::string::String,
    }

    ///`PagedDocumentLinks`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "self"
    ///  ],
    ///  "properties": {
    ///    "first": {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    },
    ///    "next": {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    },
    ///    "self": {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PagedDocumentLinks {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub first: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next: ::std::option::Option<::std::string::String>,
        #[serde(rename = "self")]
        pub self_: ::std::string::String,
    }

    ///`PagingInformation`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "paging"
    ///  ],
    ///  "properties": {
    ///    "paging": {
    ///      "type": "object",
    ///      "required": [
    ///        "limit"
    ///      ],
    ///      "properties": {
    ///        "limit": {
    ///          "type": "integer"
    ///        },
    ///        "nextCursor": {
    ///          "type": "string"
    ///        },
    ///        "total": {
    ///          "type": "integer"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PagingInformation {
        pub paging: PagingInformationPaging,
    }

    ///`PagingInformationPaging`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "limit"
    ///  ],
    ///  "properties": {
    ///    "limit": {
    ///      "type": "integer"
    ///    },
    ///    "nextCursor": {
    ///      "type": "string"
    ///    },
    ///    "total": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PagingInformationPaging {
        pub limit: i64,
        #[serde(
            rename = "nextCursor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_cursor: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total: ::std::option::Option<i64>,
    }

    ///`ResourceLinks`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "self": {
    ///      "type": "string",
    ///      "format": "uri-reference"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ResourceLinks {
        #[serde(
            rename = "self",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub self_: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ResourceLinks {
        fn default() -> Self {
            Self {
                self_: Default::default(),
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for App Store Connect API
///
///Version: 4.4
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "4.4"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    ///Sends a `GET` request to `/v1/apps`
    ///
    ///Arguments:
    /// - `exists_game_center_enabled_versions`: filter by existence or
    ///   non-existence of related 'gameCenterEnabledVersions'
    /// - `fields_android_to_ios_app_mapping_details`: the fields to include for
    ///   returned resources of type androidToIosAppMappingDetails
    /// - `fields_app_clips`: the fields to include for returned resources of
    ///   type appClips
    /// - `fields_app_custom_product_pages`: the fields to include for returned
    ///   resources of type appCustomProductPages
    /// - `fields_app_encryption_declarations`: the fields to include for
    ///   returned resources of type appEncryptionDeclarations
    /// - `fields_app_events`: the fields to include for returned resources of
    ///   type appEvents
    /// - `fields_app_infos`: the fields to include for returned resources of
    ///   type appInfos
    /// - `fields_app_store_version_experiments`: the fields to include for
    ///   returned resources of type appStoreVersionExperiments
    /// - `fields_app_store_versions`: the fields to include for returned
    ///   resources of type appStoreVersions
    /// - `fields_apps`: the fields to include for returned resources of type
    ///   apps
    /// - `fields_beta_app_localizations`: the fields to include for returned
    ///   resources of type betaAppLocalizations
    /// - `fields_beta_app_review_details`: the fields to include for returned
    ///   resources of type betaAppReviewDetails
    /// - `fields_beta_groups`: the fields to include for returned resources of
    ///   type betaGroups
    /// - `fields_beta_license_agreements`: the fields to include for returned
    ///   resources of type betaLicenseAgreements
    /// - `fields_build_icons`: the fields to include for returned resources of
    ///   type buildIcons
    /// - `fields_builds`: the fields to include for returned resources of type
    ///   builds
    /// - `fields_ci_products`: the fields to include for returned resources of
    ///   type ciProducts
    /// - `fields_end_user_license_agreements`: the fields to include for
    ///   returned resources of type endUserLicenseAgreements
    /// - `fields_game_center_details`: the fields to include for returned
    ///   resources of type gameCenterDetails
    /// - `fields_game_center_enabled_versions`: the fields to include for
    ///   returned resources of type gameCenterEnabledVersions
    /// - `fields_in_app_purchases`: the fields to include for returned
    ///   resources of type inAppPurchases
    /// - `fields_pre_release_versions`: the fields to include for returned
    ///   resources of type preReleaseVersions
    /// - `fields_promoted_purchases`: the fields to include for returned
    ///   resources of type promotedPurchases
    /// - `fields_review_submissions`: the fields to include for returned
    ///   resources of type reviewSubmissions
    /// - `fields_subscription_grace_periods`: the fields to include for
    ///   returned resources of type subscriptionGracePeriods
    /// - `fields_subscription_groups`: the fields to include for returned
    ///   resources of type subscriptionGroups
    /// - `filter_app_store_versions_app_store_state`: filter by attribute
    ///   'appStoreVersions.appStoreState'
    /// - `filter_app_store_versions_app_version_state`: filter by attribute
    ///   'appStoreVersions.appVersionState'
    /// - `filter_app_store_versions_platform`: filter by attribute
    ///   'appStoreVersions.platform'
    /// - `filter_app_store_versions`: filter by id(s) of related
    ///   'appStoreVersions'
    /// - `filter_bundle_id`: filter by attribute 'bundleId'
    /// - `filter_id`: filter by id(s)
    /// - `filter_name`: filter by attribute 'name'
    /// - `filter_review_submissions_platform`: filter by attribute
    ///   'reviewSubmissions.platform'
    /// - `filter_review_submissions_state`: filter by attribute
    ///   'reviewSubmissions.state'
    /// - `filter_sku`: filter by attribute 'sku'
    /// - `include`: comma-separated list of relationships to include
    /// - `limit`: maximum resources per page
    /// - `limit_android_to_ios_app_mapping_details`: maximum number of related
    ///   androidToIosAppMappingDetails returned (when they are included)
    /// - `limit_app_clips`: maximum number of related appClips returned (when
    ///   they are included)
    /// - `limit_app_custom_product_pages`: maximum number of related
    ///   appCustomProductPages returned (when they are included)
    /// - `limit_app_encryption_declarations`: maximum number of related
    ///   appEncryptionDeclarations returned (when they are included)
    /// - `limit_app_events`: maximum number of related appEvents returned (when
    ///   they are included)
    /// - `limit_app_infos`: maximum number of related appInfos returned (when
    ///   they are included)
    /// - `limit_app_store_version_experiments_v2`: maximum number of related
    ///   appStoreVersionExperimentsV2 returned (when they are included)
    /// - `limit_app_store_versions`: maximum number of related appStoreVersions
    ///   returned (when they are included)
    /// - `limit_beta_app_localizations`: maximum number of related
    ///   betaAppLocalizations returned (when they are included)
    /// - `limit_beta_groups`: maximum number of related betaGroups returned
    ///   (when they are included)
    /// - `limit_builds`: maximum number of related builds returned (when they
    ///   are included)
    /// - `limit_game_center_enabled_versions`: maximum number of related
    ///   gameCenterEnabledVersions returned (when they are included)
    /// - `limit_in_app_purchases_v2`: maximum number of related
    ///   inAppPurchasesV2 returned (when they are included)
    /// - `limit_in_app_purchases`: maximum number of related inAppPurchases
    ///   returned (when they are included)
    /// - `limit_pre_release_versions`: maximum number of related
    ///   preReleaseVersions returned (when they are included)
    /// - `limit_promoted_purchases`: maximum number of related
    ///   promotedPurchases returned (when they are included)
    /// - `limit_review_submissions`: maximum number of related
    ///   reviewSubmissions returned (when they are included)
    /// - `limit_subscription_groups`: maximum number of related
    ///   subscriptionGroups returned (when they are included)
    /// - `sort`: comma-separated list of sort expressions; resources will be
    ///   sorted as specified
    pub async fn apps_get_collection<'a>(
        &'a self,
        exists_game_center_enabled_versions: Option<bool>,
        fields_android_to_ios_app_mapping_details: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsAndroidToIosAppMappingDetailsItem>,
        >,
        fields_app_clips: Option<&'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppClipsItem>>,
        fields_app_custom_product_pages: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppCustomProductPagesItem>,
        >,
        fields_app_encryption_declarations: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppEncryptionDeclarationsItem>,
        >,
        fields_app_events: Option<&'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppEventsItem>>,
        fields_app_infos: Option<&'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppInfosItem>>,
        fields_app_store_version_experiments: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppStoreVersionExperimentsItem>,
        >,
        fields_app_store_versions: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppStoreVersionsItem>,
        >,
        fields_apps: Option<&'a ::std::vec::Vec<types::AppsGetCollectionFieldsAppsItem>>,
        fields_beta_app_localizations: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsBetaAppLocalizationsItem>,
        >,
        fields_beta_app_review_details: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsBetaAppReviewDetailsItem>,
        >,
        fields_beta_groups: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsBetaGroupsItem>,
        >,
        fields_beta_license_agreements: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsBetaLicenseAgreementsItem>,
        >,
        fields_build_icons: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsBuildIconsItem>,
        >,
        fields_builds: Option<&'a ::std::vec::Vec<types::AppsGetCollectionFieldsBuildsItem>>,
        fields_ci_products: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsCiProductsItem>,
        >,
        fields_end_user_license_agreements: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsEndUserLicenseAgreementsItem>,
        >,
        fields_game_center_details: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsGameCenterDetailsItem>,
        >,
        fields_game_center_enabled_versions: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsGameCenterEnabledVersionsItem>,
        >,
        fields_in_app_purchases: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsInAppPurchasesItem>,
        >,
        fields_pre_release_versions: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsPreReleaseVersionsItem>,
        >,
        fields_promoted_purchases: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsPromotedPurchasesItem>,
        >,
        fields_review_submissions: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsReviewSubmissionsItem>,
        >,
        fields_subscription_grace_periods: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsSubscriptionGracePeriodsItem>,
        >,
        fields_subscription_groups: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFieldsSubscriptionGroupsItem>,
        >,
        filter_app_store_versions_app_store_state: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFilterAppStoreVersionsAppStoreStateItem>,
        >,
        filter_app_store_versions_app_version_state: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFilterAppStoreVersionsAppVersionStateItem>,
        >,
        filter_app_store_versions_platform: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFilterAppStoreVersionsPlatformItem>,
        >,
        filter_app_store_versions: Option<&'a ::std::vec::Vec<::std::string::String>>,
        filter_bundle_id: Option<&'a ::std::vec::Vec<::std::string::String>>,
        filter_id: Option<&'a ::std::vec::Vec<::std::string::String>>,
        filter_name: Option<&'a ::std::vec::Vec<::std::string::String>>,
        filter_review_submissions_platform: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFilterReviewSubmissionsPlatformItem>,
        >,
        filter_review_submissions_state: Option<
            &'a ::std::vec::Vec<types::AppsGetCollectionFilterReviewSubmissionsStateItem>,
        >,
        filter_sku: Option<&'a ::std::vec::Vec<::std::string::String>>,
        include: Option<&'a ::std::vec::Vec<types::AppsGetCollectionIncludeItem>>,
        limit: Option<i64>,
        limit_android_to_ios_app_mapping_details: Option<i64>,
        limit_app_clips: Option<i64>,
        limit_app_custom_product_pages: Option<i64>,
        limit_app_encryption_declarations: Option<i64>,
        limit_app_events: Option<i64>,
        limit_app_infos: Option<i64>,
        limit_app_store_version_experiments_v2: Option<i64>,
        limit_app_store_versions: Option<i64>,
        limit_beta_app_localizations: Option<i64>,
        limit_beta_groups: Option<i64>,
        limit_builds: Option<i64>,
        limit_game_center_enabled_versions: Option<i64>,
        limit_in_app_purchases_v2: Option<i64>,
        limit_in_app_purchases: Option<i64>,
        limit_pre_release_versions: Option<i64>,
        limit_promoted_purchases: Option<i64>,
        limit_review_submissions: Option<i64>,
        limit_subscription_groups: Option<i64>,
        sort: Option<&'a ::std::vec::Vec<types::AppsGetCollectionSortItem>>,
    ) -> Result<ResponseValue<types::AppsResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/v1/apps", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "exists[gameCenterEnabledVersions]",
                &exists_game_center_enabled_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[androidToIosAppMappingDetails]",
                &fields_android_to_ios_app_mapping_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appClips]",
                &fields_app_clips,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appCustomProductPages]",
                &fields_app_custom_product_pages,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appEncryptionDeclarations]",
                &fields_app_encryption_declarations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appEvents]",
                &fields_app_events,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appInfos]",
                &fields_app_infos,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersionExperiments]",
                &fields_app_store_version_experiments,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersions]",
                &fields_app_store_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[apps]",
                &fields_apps,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaAppLocalizations]",
                &fields_beta_app_localizations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaAppReviewDetails]",
                &fields_beta_app_review_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaGroups]",
                &fields_beta_groups,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaLicenseAgreements]",
                &fields_beta_license_agreements,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[buildIcons]",
                &fields_build_icons,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[builds]",
                &fields_builds,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[ciProducts]",
                &fields_ci_products,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[endUserLicenseAgreements]",
                &fields_end_user_license_agreements,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[gameCenterDetails]",
                &fields_game_center_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[gameCenterEnabledVersions]",
                &fields_game_center_enabled_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[inAppPurchases]",
                &fields_in_app_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[preReleaseVersions]",
                &fields_pre_release_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[promotedPurchases]",
                &fields_promoted_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[reviewSubmissions]",
                &fields_review_submissions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[subscriptionGracePeriods]",
                &fields_subscription_grace_periods,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[subscriptionGroups]",
                &fields_subscription_groups,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[appStoreVersions.appStoreState]",
                &filter_app_store_versions_app_store_state,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[appStoreVersions.appVersionState]",
                &filter_app_store_versions_app_version_state,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[appStoreVersions.platform]",
                &filter_app_store_versions_platform,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[appStoreVersions]",
                &filter_app_store_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[bundleId]",
                &filter_bundle_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[id]",
                &filter_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[name]",
                &filter_name,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[reviewSubmissions.platform]",
                &filter_review_submissions_platform,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[reviewSubmissions.state]",
                &filter_review_submissions_state,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[sku]",
                &filter_sku,
            ))
            .query(&progenitor_client::QueryParam::new("include", &include))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new(
                "limit[androidToIosAppMappingDetails]",
                &limit_android_to_ios_app_mapping_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appClips]",
                &limit_app_clips,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appCustomProductPages]",
                &limit_app_custom_product_pages,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appEncryptionDeclarations]",
                &limit_app_encryption_declarations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appEvents]",
                &limit_app_events,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appInfos]",
                &limit_app_infos,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appStoreVersionExperimentsV2]",
                &limit_app_store_version_experiments_v2,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appStoreVersions]",
                &limit_app_store_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[betaAppLocalizations]",
                &limit_beta_app_localizations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[betaGroups]",
                &limit_beta_groups,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[builds]",
                &limit_builds,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[gameCenterEnabledVersions]",
                &limit_game_center_enabled_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[inAppPurchasesV2]",
                &limit_in_app_purchases_v2,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[inAppPurchases]",
                &limit_in_app_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[preReleaseVersions]",
                &limit_pre_release_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[promotedPurchases]",
                &limit_promoted_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[reviewSubmissions]",
                &limit_review_submissions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[subscriptionGroups]",
                &limit_subscription_groups,
            ))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "apps_get_collection",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            429u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/apps/{id}`
    ///
    ///Arguments:
    /// - `id`: the id of the requested resource
    /// - `fields_android_to_ios_app_mapping_details`: the fields to include for
    ///   returned resources of type androidToIosAppMappingDetails
    /// - `fields_app_clips`: the fields to include for returned resources of
    ///   type appClips
    /// - `fields_app_custom_product_pages`: the fields to include for returned
    ///   resources of type appCustomProductPages
    /// - `fields_app_encryption_declarations`: the fields to include for
    ///   returned resources of type appEncryptionDeclarations
    /// - `fields_app_events`: the fields to include for returned resources of
    ///   type appEvents
    /// - `fields_app_infos`: the fields to include for returned resources of
    ///   type appInfos
    /// - `fields_app_store_version_experiments`: the fields to include for
    ///   returned resources of type appStoreVersionExperiments
    /// - `fields_app_store_versions`: the fields to include for returned
    ///   resources of type appStoreVersions
    /// - `fields_apps`: the fields to include for returned resources of type
    ///   apps
    /// - `fields_beta_app_localizations`: the fields to include for returned
    ///   resources of type betaAppLocalizations
    /// - `fields_beta_app_review_details`: the fields to include for returned
    ///   resources of type betaAppReviewDetails
    /// - `fields_beta_groups`: the fields to include for returned resources of
    ///   type betaGroups
    /// - `fields_beta_license_agreements`: the fields to include for returned
    ///   resources of type betaLicenseAgreements
    /// - `fields_build_icons`: the fields to include for returned resources of
    ///   type buildIcons
    /// - `fields_builds`: the fields to include for returned resources of type
    ///   builds
    /// - `fields_ci_products`: the fields to include for returned resources of
    ///   type ciProducts
    /// - `fields_end_user_license_agreements`: the fields to include for
    ///   returned resources of type endUserLicenseAgreements
    /// - `fields_game_center_details`: the fields to include for returned
    ///   resources of type gameCenterDetails
    /// - `fields_game_center_enabled_versions`: the fields to include for
    ///   returned resources of type gameCenterEnabledVersions
    /// - `fields_in_app_purchases`: the fields to include for returned
    ///   resources of type inAppPurchases
    /// - `fields_pre_release_versions`: the fields to include for returned
    ///   resources of type preReleaseVersions
    /// - `fields_promoted_purchases`: the fields to include for returned
    ///   resources of type promotedPurchases
    /// - `fields_review_submissions`: the fields to include for returned
    ///   resources of type reviewSubmissions
    /// - `fields_subscription_grace_periods`: the fields to include for
    ///   returned resources of type subscriptionGracePeriods
    /// - `fields_subscription_groups`: the fields to include for returned
    ///   resources of type subscriptionGroups
    /// - `include`: comma-separated list of relationships to include
    /// - `limit_android_to_ios_app_mapping_details`: maximum number of related
    ///   androidToIosAppMappingDetails returned (when they are included)
    /// - `limit_app_clips`: maximum number of related appClips returned (when
    ///   they are included)
    /// - `limit_app_custom_product_pages`: maximum number of related
    ///   appCustomProductPages returned (when they are included)
    /// - `limit_app_encryption_declarations`: maximum number of related
    ///   appEncryptionDeclarations returned (when they are included)
    /// - `limit_app_events`: maximum number of related appEvents returned (when
    ///   they are included)
    /// - `limit_app_infos`: maximum number of related appInfos returned (when
    ///   they are included)
    /// - `limit_app_store_version_experiments_v2`: maximum number of related
    ///   appStoreVersionExperimentsV2 returned (when they are included)
    /// - `limit_app_store_versions`: maximum number of related appStoreVersions
    ///   returned (when they are included)
    /// - `limit_beta_app_localizations`: maximum number of related
    ///   betaAppLocalizations returned (when they are included)
    /// - `limit_beta_groups`: maximum number of related betaGroups returned
    ///   (when they are included)
    /// - `limit_builds`: maximum number of related builds returned (when they
    ///   are included)
    /// - `limit_game_center_enabled_versions`: maximum number of related
    ///   gameCenterEnabledVersions returned (when they are included)
    /// - `limit_in_app_purchases_v2`: maximum number of related
    ///   inAppPurchasesV2 returned (when they are included)
    /// - `limit_in_app_purchases`: maximum number of related inAppPurchases
    ///   returned (when they are included)
    /// - `limit_pre_release_versions`: maximum number of related
    ///   preReleaseVersions returned (when they are included)
    /// - `limit_promoted_purchases`: maximum number of related
    ///   promotedPurchases returned (when they are included)
    /// - `limit_review_submissions`: maximum number of related
    ///   reviewSubmissions returned (when they are included)
    /// - `limit_subscription_groups`: maximum number of related
    ///   subscriptionGroups returned (when they are included)
    pub async fn apps_get_instance<'a>(
        &'a self,
        id: &'a str,
        fields_android_to_ios_app_mapping_details: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsAndroidToIosAppMappingDetailsItem>,
        >,
        fields_app_clips: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppClipsItem>>,
        fields_app_custom_product_pages: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppCustomProductPagesItem>,
        >,
        fields_app_encryption_declarations: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppEncryptionDeclarationsItem>,
        >,
        fields_app_events: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppEventsItem>>,
        fields_app_infos: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppInfosItem>>,
        fields_app_store_version_experiments: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppStoreVersionExperimentsItem>,
        >,
        fields_app_store_versions: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppStoreVersionsItem>,
        >,
        fields_apps: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsAppsItem>>,
        fields_beta_app_localizations: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsBetaAppLocalizationsItem>,
        >,
        fields_beta_app_review_details: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsBetaAppReviewDetailsItem>,
        >,
        fields_beta_groups: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsBetaGroupsItem>>,
        fields_beta_license_agreements: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsBetaLicenseAgreementsItem>,
        >,
        fields_build_icons: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsBuildIconsItem>>,
        fields_builds: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsBuildsItem>>,
        fields_ci_products: Option<&'a ::std::vec::Vec<types::AppsGetInstanceFieldsCiProductsItem>>,
        fields_end_user_license_agreements: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsEndUserLicenseAgreementsItem>,
        >,
        fields_game_center_details: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsGameCenterDetailsItem>,
        >,
        fields_game_center_enabled_versions: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsGameCenterEnabledVersionsItem>,
        >,
        fields_in_app_purchases: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsInAppPurchasesItem>,
        >,
        fields_pre_release_versions: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsPreReleaseVersionsItem>,
        >,
        fields_promoted_purchases: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsPromotedPurchasesItem>,
        >,
        fields_review_submissions: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsReviewSubmissionsItem>,
        >,
        fields_subscription_grace_periods: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsSubscriptionGracePeriodsItem>,
        >,
        fields_subscription_groups: Option<
            &'a ::std::vec::Vec<types::AppsGetInstanceFieldsSubscriptionGroupsItem>,
        >,
        include: Option<&'a ::std::vec::Vec<types::AppsGetInstanceIncludeItem>>,
        limit_android_to_ios_app_mapping_details: Option<i64>,
        limit_app_clips: Option<i64>,
        limit_app_custom_product_pages: Option<i64>,
        limit_app_encryption_declarations: Option<i64>,
        limit_app_events: Option<i64>,
        limit_app_infos: Option<i64>,
        limit_app_store_version_experiments_v2: Option<i64>,
        limit_app_store_versions: Option<i64>,
        limit_beta_app_localizations: Option<i64>,
        limit_beta_groups: Option<i64>,
        limit_builds: Option<i64>,
        limit_game_center_enabled_versions: Option<i64>,
        limit_in_app_purchases_v2: Option<i64>,
        limit_in_app_purchases: Option<i64>,
        limit_pre_release_versions: Option<i64>,
        limit_promoted_purchases: Option<i64>,
        limit_review_submissions: Option<i64>,
        limit_subscription_groups: Option<i64>,
    ) -> Result<ResponseValue<types::AppResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/v1/apps/{}", self.baseurl, encode_path(&id.to_string()),);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "fields[androidToIosAppMappingDetails]",
                &fields_android_to_ios_app_mapping_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appClips]",
                &fields_app_clips,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appCustomProductPages]",
                &fields_app_custom_product_pages,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appEncryptionDeclarations]",
                &fields_app_encryption_declarations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appEvents]",
                &fields_app_events,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appInfos]",
                &fields_app_infos,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersionExperiments]",
                &fields_app_store_version_experiments,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersions]",
                &fields_app_store_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[apps]",
                &fields_apps,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaAppLocalizations]",
                &fields_beta_app_localizations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaAppReviewDetails]",
                &fields_beta_app_review_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaGroups]",
                &fields_beta_groups,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[betaLicenseAgreements]",
                &fields_beta_license_agreements,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[buildIcons]",
                &fields_build_icons,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[builds]",
                &fields_builds,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[ciProducts]",
                &fields_ci_products,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[endUserLicenseAgreements]",
                &fields_end_user_license_agreements,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[gameCenterDetails]",
                &fields_game_center_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[gameCenterEnabledVersions]",
                &fields_game_center_enabled_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[inAppPurchases]",
                &fields_in_app_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[preReleaseVersions]",
                &fields_pre_release_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[promotedPurchases]",
                &fields_promoted_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[reviewSubmissions]",
                &fields_review_submissions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[subscriptionGracePeriods]",
                &fields_subscription_grace_periods,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[subscriptionGroups]",
                &fields_subscription_groups,
            ))
            .query(&progenitor_client::QueryParam::new("include", &include))
            .query(&progenitor_client::QueryParam::new(
                "limit[androidToIosAppMappingDetails]",
                &limit_android_to_ios_app_mapping_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appClips]",
                &limit_app_clips,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appCustomProductPages]",
                &limit_app_custom_product_pages,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appEncryptionDeclarations]",
                &limit_app_encryption_declarations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appEvents]",
                &limit_app_events,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appInfos]",
                &limit_app_infos,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appStoreVersionExperimentsV2]",
                &limit_app_store_version_experiments_v2,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appStoreVersions]",
                &limit_app_store_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[betaAppLocalizations]",
                &limit_beta_app_localizations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[betaGroups]",
                &limit_beta_groups,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[builds]",
                &limit_builds,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[gameCenterEnabledVersions]",
                &limit_game_center_enabled_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[inAppPurchasesV2]",
                &limit_in_app_purchases_v2,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[inAppPurchases]",
                &limit_in_app_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[preReleaseVersions]",
                &limit_pre_release_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[promotedPurchases]",
                &limit_promoted_purchases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[reviewSubmissions]",
                &limit_review_submissions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[subscriptionGroups]",
                &limit_subscription_groups,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "apps_get_instance",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            429u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/apps/{id}/appStoreVersions`
    ///
    ///Arguments:
    /// - `id`: the id of the requested resource
    /// - `fields_alternative_distribution_packages`: the fields to include for
    ///   returned resources of type alternativeDistributionPackages
    /// - `fields_app_clip_default_experiences`: the fields to include for
    ///   returned resources of type appClipDefaultExperiences
    /// - `fields_app_store_review_details`: the fields to include for returned
    ///   resources of type appStoreReviewDetails
    /// - `fields_app_store_version_experiments`: the fields to include for
    ///   returned resources of type appStoreVersionExperiments
    /// - `fields_app_store_version_localizations`: the fields to include for
    ///   returned resources of type appStoreVersionLocalizations
    /// - `fields_app_store_version_phased_releases`: the fields to include for
    ///   returned resources of type appStoreVersionPhasedReleases
    /// - `fields_app_store_version_submissions`: the fields to include for
    ///   returned resources of type appStoreVersionSubmissions
    /// - `fields_app_store_versions`: the fields to include for returned
    ///   resources of type appStoreVersions
    /// - `fields_apps`: the fields to include for returned resources of type
    ///   apps
    /// - `fields_builds`: the fields to include for returned resources of type
    ///   builds
    /// - `fields_game_center_app_versions`: the fields to include for returned
    ///   resources of type gameCenterAppVersions
    /// - `fields_routing_app_coverages`: the fields to include for returned
    ///   resources of type routingAppCoverages
    /// - `filter_app_store_state`: filter by attribute 'appStoreState'
    /// - `filter_app_version_state`: filter by attribute 'appVersionState'
    /// - `filter_id`: filter by id(s)
    /// - `filter_platform`: filter by attribute 'platform'
    /// - `filter_version_string`: filter by attribute 'versionString'
    /// - `include`: comma-separated list of relationships to include
    /// - `limit`: maximum resources per page
    /// - `limit_app_store_version_experiments_v2`: maximum number of related
    ///   appStoreVersionExperimentsV2 returned (when they are included)
    /// - `limit_app_store_version_experiments`: maximum number of related
    ///   appStoreVersionExperiments returned (when they are included)
    /// - `limit_app_store_version_localizations`: maximum number of related
    ///   appStoreVersionLocalizations returned (when they are included)
    pub async fn apps_app_store_versions_get_to_many_related<'a>(
        &'a self,
        id: &'a str,
        fields_alternative_distribution_packages : Option < & 'a :: std :: vec :: Vec < types :: AppsAppStoreVersionsGetToManyRelatedFieldsAlternativeDistributionPackagesItem > >,
        fields_app_clip_default_experiences: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsAppClipDefaultExperiencesItem,
            >,
        >,
        fields_app_store_review_details: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreReviewDetailsItem,
            >,
        >,
        fields_app_store_version_experiments: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionExperimentsItem,
            >,
        >,
        fields_app_store_version_localizations: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionLocalizationsItem,
            >,
        >,
        fields_app_store_version_phased_releases: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionPhasedReleasesItem,
            >,
        >,
        fields_app_store_version_submissions: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionSubmissionsItem,
            >,
        >,
        fields_app_store_versions: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsAppStoreVersionsItem,
            >,
        >,
        fields_apps: Option<
            &'a ::std::vec::Vec<types::AppsAppStoreVersionsGetToManyRelatedFieldsAppsItem>,
        >,
        fields_builds: Option<
            &'a ::std::vec::Vec<types::AppsAppStoreVersionsGetToManyRelatedFieldsBuildsItem>,
        >,
        fields_game_center_app_versions: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsGameCenterAppVersionsItem,
            >,
        >,
        fields_routing_app_coverages: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFieldsRoutingAppCoveragesItem,
            >,
        >,
        filter_app_store_state: Option<
            &'a ::std::vec::Vec<types::AppsAppStoreVersionsGetToManyRelatedFilterAppStoreStateItem>,
        >,
        filter_app_version_state: Option<
            &'a ::std::vec::Vec<
                types::AppsAppStoreVersionsGetToManyRelatedFilterAppVersionStateItem,
            >,
        >,
        filter_id: Option<&'a ::std::vec::Vec<::std::string::String>>,
        filter_platform: Option<
            &'a ::std::vec::Vec<types::AppsAppStoreVersionsGetToManyRelatedFilterPlatformItem>,
        >,
        filter_version_string: Option<&'a ::std::vec::Vec<::std::string::String>>,
        include: Option<
            &'a ::std::vec::Vec<types::AppsAppStoreVersionsGetToManyRelatedIncludeItem>,
        >,
        limit: Option<i64>,
        limit_app_store_version_experiments_v2: Option<i64>,
        limit_app_store_version_experiments: Option<i64>,
        limit_app_store_version_localizations: Option<i64>,
    ) -> Result<ResponseValue<types::AppStoreVersionsResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/v1/apps/{}/appStoreVersions",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "fields[alternativeDistributionPackages]",
                &fields_alternative_distribution_packages,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appClipDefaultExperiences]",
                &fields_app_clip_default_experiences,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreReviewDetails]",
                &fields_app_store_review_details,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersionExperiments]",
                &fields_app_store_version_experiments,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersionLocalizations]",
                &fields_app_store_version_localizations,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersionPhasedReleases]",
                &fields_app_store_version_phased_releases,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersionSubmissions]",
                &fields_app_store_version_submissions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[appStoreVersions]",
                &fields_app_store_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[apps]",
                &fields_apps,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[builds]",
                &fields_builds,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[gameCenterAppVersions]",
                &fields_game_center_app_versions,
            ))
            .query(&progenitor_client::QueryParam::new(
                "fields[routingAppCoverages]",
                &fields_routing_app_coverages,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[appStoreState]",
                &filter_app_store_state,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[appVersionState]",
                &filter_app_version_state,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[id]",
                &filter_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[platform]",
                &filter_platform,
            ))
            .query(&progenitor_client::QueryParam::new(
                "filter[versionString]",
                &filter_version_string,
            ))
            .query(&progenitor_client::QueryParam::new("include", &include))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new(
                "limit[appStoreVersionExperimentsV2]",
                &limit_app_store_version_experiments_v2,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appStoreVersionExperiments]",
                &limit_app_store_version_experiments,
            ))
            .query(&progenitor_client::QueryParam::new(
                "limit[appStoreVersionLocalizations]",
                &limit_app_store_version_localizations,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "apps_app_store_versions_get_to_many_related",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            429u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
