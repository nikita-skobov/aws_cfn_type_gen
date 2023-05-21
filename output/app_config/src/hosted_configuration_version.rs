

/// Create a new configuration in the AWS AppConfig hosted configuration store.    Configurations must be 1 MB or smaller. The AWS AppConfig hosted configuration store    provides the following benefits over other configuration store options.
#[derive(Default, serde::Serialize)]
pub struct CfnHostedConfigurationVersion {


    /// 
    /// A description of the configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The configuration profile ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [a-z0-9]{4,7}
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigurationProfileId")]
    pub configuration_profile_id: String,


    /// 
    /// The application ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [a-z0-9]{4,7}
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationId")]
    pub application_id: String,


    /// 
    /// A standard MIME type describing the format of the configuration content. For more     information, see Content-Type.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContentType")]
    pub content_type: String,


    /// 
    /// The content of the configuration or the configuration data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Content")]
    pub content: String,


    /// 
    /// A user-defined label for an AWS AppConfig hosted configuration version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: .*[^0-9].*
    ///
    /// Update requires: Replacement
    #[serde(rename = "VersionLabel")]
    pub version_label: Option<String>,


    /// 
    /// An optional locking token used to prevent race conditions from overwriting configuration     updates when creating a new version. To ensure your data is not overwritten when creating     multiple hosted configuration versions in rapid succession, specify the version number of     the latest hosted configuration version.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "LatestVersionNumber")]
    pub latest_version_number: Option<f64>,

}
