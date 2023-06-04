/// The     AWS::Greengrass::CoreDefinitionVersion resource represents a core definition version for AWS IoT Greengrass.     A core definition version contains a Greengrass core.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnCoreDefinitionVersion {
    ///
    /// The ID of the core definition associated with this version. This value is a GUID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: cfn_resources::StrVal,

    ///
    /// The Greengrass core in this version. Currently, the Cores property for a core definition version can contain only one core.
    ///
    /// Required: Yes
    ///
    /// Type: List of Core
    ///
    /// Update requires: Replacement
    #[serde(rename = "Cores")]
    pub cores: Vec<Core>,
}

impl cfn_resources::CfnResource for CfnCoreDefinitionVersion {
    fn type_string(&self) -> &'static str {
        "AWS::Greengrass::CoreDefinitionVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A core is an AWS IoT device that runs the AWS IoT Greengrass core 		software and manages local processes for a Greengrass group. For more information,   see What Is AWS IoT Greengrass? in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, the Cores 		 property of the AWS::Greengrass::CoreDefinitionVersion resource contains a      list of Core property types. Currently, the list can contain only one core.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Core {
    ///
    /// The ARN of the device certificate for the core. This X.509 certificate is used to authenticate           the core with AWS IoT and AWS IoT Greengrass services.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: cfn_resources::StrVal,

    ///
    /// A descriptive or arbitrary ID for the core. This value must be unique within the core definition version. Maximum length is 128 characters with pattern [a-zA-Z0-9:_-]+.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// Indicates whether the core's local shadow is synced with the cloud automatically. 				 The default is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "SyncShadow")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub sync_shadow: Option<bool>,

    ///
    /// The Amazon Resource Name (ARN) of the core, which is an AWS IoT device (thing).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThingArn")]
    pub thing_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Core {
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
