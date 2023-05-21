

/// The     AWS::Greengrass::CoreDefinitionVersion resource represents a core definition version for AWS IoT Greengrass.     A core definition version contains a Greengrass core.
#[derive(Default, serde::Serialize)]
pub struct CfnCoreDefinitionVersion {


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


    /// 
    /// The ID of the core definition associated with this version. This value is a GUID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,

}


/// A core is an AWS IoT device that runs the AWS IoT Greengrass core 		software and manages local processes for a Greengrass group. For more information,   see What Is AWS IoT Greengrass? in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, the Cores 		 property of the AWS::Greengrass::CoreDefinitionVersion resource contains a      list of Core property types. Currently, the list can contain only one core.
#[derive(Default, serde::Serialize)]
pub struct Core {


    /// 
    /// The Amazon Resource Name (ARN) of the core, which is an AWS IoT device (thing).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThingArn")]
    pub thing_arn: String,


    /// 
    /// A descriptive or arbitrary ID for the core. This value must be unique within the core definition version. Maximum length is 128 characters with pattern [a-zA-Z0-9:_-]+.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: String,


    /// 
    /// Indicates whether the core's local shadow is synced with the cloud automatically. 				 The default is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "SyncShadow")]
    pub sync_shadow: Option<bool>,


    /// 
    /// The ARN of the device certificate for the core. This X.509 certificate is used to authenticate           the core with AWS IoT and AWS IoT Greengrass services.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,

}