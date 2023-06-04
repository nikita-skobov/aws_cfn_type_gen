/// The     AWS::Greengrass::GroupVersion resource represents a group version in AWS IoT Greengrass.     A group version references a core definition version,      device definition version, subscription definition version, and other version types     that contain the components you want to deploy to a Greengrass core device.      The group version must reference a core definition version that contains one core.     Other version types are optionally included, depending on your business need.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGroupVersion {
    ///
    /// The Amazon Resource Name (ARN) of the connector definition version that contains the connectors you want to deploy with the group version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectorDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_definition_version_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the core definition version that contains the core you want to deploy with the group version. 				 Currently, the core definition version can contain only one core.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CoreDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_definition_version_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the device definition version that contains the devices you want to deploy with the group version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_definition_version_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the function definition version that contains the functions you want to deploy with the group version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FunctionDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_definition_version_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the group associated with this version. This value is a GUID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupId")]
    pub group_id: cfn_resources::StrVal,

    ///
    /// The ARN of the logger definition version that contains the loggers you want to deploy with the group version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoggerDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger_definition_version_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the resource definition version that contains the resources you want to deploy with the group version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_definition_version_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the subscription definition version that contains the subscriptions you want to deploy with the group version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubscriptionDefinitionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_definition_version_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnGroupVersion {
    fn type_string(&self) -> &'static str {
        "AWS::Greengrass::GroupVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
