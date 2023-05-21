

/// Create a new configuration in the AWS AppConfig hosted configuration store.    Configurations must be 1 MB or smaller. The AWS AppConfig hosted configuration store    provides the following benefits over other configuration store options.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnHostedConfigurationVersion {


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
    /// An optional locking token used to prevent race conditions from overwriting configuration     updates when creating a new version. To ensure your data is not overwritten when creating     multiple hosted configuration versions in rapid succession, specify the version number of     the latest hosted configuration version.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "LatestVersionNumber")]
    pub latest_version_number: Option<f64>,


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

}



impl cfn_resources::CfnResource for CfnHostedConfigurationVersion {
    fn type_string() -> &'static str {
        "AWS::AppConfig::HostedConfigurationVersion"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.content_type;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'content_type'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.content_type;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'content_type'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.description {

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'description'. {} is greater than 1024", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.description {

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'description'. {} is less than 0", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.version_label {

        if the_val.len() > 64 as _ {
            return Err(format!("Max validation failed on field 'version_label'. {} is greater than 64", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.version_label {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'version_label'. {} is less than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}