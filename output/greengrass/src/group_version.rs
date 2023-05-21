

/// The     AWS::Greengrass::GroupVersion resource represents a group version in AWS IoT Greengrass.     A group version references a core definition version,      device definition version, subscription definition version, and other version types     that contain the components you want to deploy to a Greengrass core device.      The group version must reference a core definition version that contains one core.     Other version types are optionally included, depending on your business need.
#[derive(Default, serde::Serialize)]
pub struct CfnGroupVersion {


    /// 
    /// The ARN of the resource definition version that contains the resources you want to deploy with the group version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceDefinitionVersionArn")]
    pub resource_definition_version_arn: Option<String>,


    /// 
    /// The ARN of the device definition version that contains the devices you want to deploy with the group version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceDefinitionVersionArn")]
    pub device_definition_version_arn: Option<String>,


    /// 
    /// The ARN of the logger definition version that contains the loggers you want to deploy with the group version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoggerDefinitionVersionArn")]
    pub logger_definition_version_arn: Option<String>,


    /// 
    /// The ARN of the subscription definition version that contains the subscriptions you want to deploy with the group version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubscriptionDefinitionVersionArn")]
    pub subscription_definition_version_arn: Option<String>,


    /// 
    /// The ARN of the core definition version that contains the core you want to deploy with the group version. 				 Currently, the core definition version can contain only one core.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CoreDefinitionVersionArn")]
    pub core_definition_version_arn: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the connector definition version that contains the connectors you want to deploy with the group version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectorDefinitionVersionArn")]
    pub connector_definition_version_arn: Option<String>,


    /// 
    /// The ID of the group associated with this version. This value is a GUID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupId")]
    pub group_id: String,


    /// 
    /// The ARN of the function definition version that contains the functions you want to deploy with the group version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FunctionDefinitionVersionArn")]
    pub function_definition_version_arn: Option<String>,

}
