/// The AWS::S3Outposts::AccessPoint resource specifies an access point and associates it with    the specified Amazon S3 on Outposts bucket. For more information, see Managing data access     with Amazon S3 access points.
///
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    pub bucket: cfn_resources::StrVal,

    ///
    /// The name of this access point.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The access point policy associated with this access point.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
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

    #[serde(skip_serializing)]
    pub att_arn: CfnAccessPointarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAccessPointarn;
impl CfnAccessPointarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnAccessPoint {
    fn type_string(&self) -> &'static str {
        "AWS::S3Outposts::AccessPoint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.vpc_configuration.validate()?;

        Ok(())
    }
}

/// Contains the virtual private cloud (VPC) configuration for the specified access    point.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for VpcConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
