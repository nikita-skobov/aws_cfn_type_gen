/// The AWS::S3Outposts::AccessPoint resource specifies an access point and associates it with    the specified Amazon S3 on Outposts bucket. For more information, see Managing data access     with Amazon S3 access points.
///
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccessPoint {
    ///
    /// The Amazon Resource Name (ARN) of the S3 on Outposts bucket that is associated with this    access point.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Bucket")]
    pub bucket: String,

    ///
    /// The name of this access point.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The access point policy associated with this access point.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    pub policy: Option<serde_json::Value>,

    ///
    /// The virtual private cloud (VPC) configuration for this access point, if one exists.
    ///
    /// Required: Yes
    ///
    /// Type: VpcConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: VpcConfiguration,
}

impl cfn_resources::CfnResource for CfnAccessPoint {
    fn type_string(&self) -> &'static str {
        "AWS::S3Outposts::AccessPoint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.vpc_configuration.validate()?;

        Ok(())
    }
}

/// Contains the virtual private cloud (VPC) configuration for the specified access    point.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConfiguration {
    ///
    /// The ID of the VPC configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,
}

impl cfn_resources::CfnResource for VpcConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
