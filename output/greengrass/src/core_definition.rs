/// The     AWS::Greengrass::CoreDefinition resource represents a core definition for AWS IoT Greengrass.      Core definitions are used to organize your core definition versions.
///
/// Core definitions can reference multiple core definition versions. All core definition versions      must be associated with a core definition. Each core definition version can contain one Greengrass core.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCoreDefinition {
    ///
    /// The core definition version to include when the core definition is created.          Currently, a core definition version can contain only one          core.
    ///
    /// NoteTo associate a core definition version after the core definition is created, 				   create an AWS::Greengrass::CoreDefinitionVersion 				   resource and specify the ID of this core definition.
    ///
    /// Required: No
    ///
    /// Type: CoreDefinitionVersion
    ///
    /// Update requires: Replacement
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<CoreDefinitionVersion>,

    ///
    /// The name of the core definition.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Application-specific metadata to attach to the core definition. 		  You can use tags in IAM policies to control access to AWS IoT Greengrass resources. 		  You can also use tags to categorize your resources. For more information, see 		  Tagging Your AWS IoT Greengrass 		  Resources in the AWS IoT Greengrass Version 1 Developer Guide.
    ///
    /// This Json property type is processed as a map of key-value pairs. It uses the following format, which 		    is different from most Tags implementations in AWS CloudFormation templates.
    ///
    /// "Tags": {   "KeyName0": "value",   "KeyName1": "value",   "KeyName2": "value" }
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    #[serde(skip_serializing)]
    pub att_arn: CfnCoreDefinitionarn,

    #[serde(skip_serializing)]
    pub att_id: CfnCoreDefinitionid,

    #[serde(skip_serializing)]
    pub att_latest_version_arn: CfnCoreDefinitionlatestversionarn,

    #[serde(skip_serializing)]
    pub att_name: CfnCoreDefinitionname,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCoreDefinitionarn;
impl CfnCoreDefinitionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCoreDefinitionid;
impl CfnCoreDefinitionid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCoreDefinitionlatestversionarn;
impl CfnCoreDefinitionlatestversionarn {
    pub fn att_name(&self) -> &'static str {
        r#"LatestVersionArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCoreDefinitionname;
impl CfnCoreDefinitionname {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

impl cfn_resources::CfnResource for CfnCoreDefinition {
    fn type_string(&self) -> &'static str {
        "AWS::Greengrass::CoreDefinition"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.initial_version
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A core is an AWS IoT device that runs the AWS IoT Greengrass core software 		and manages local processes for a Greengrass group. For more information, see    What Is AWS IoT Greengrass? in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, the Cores 		 property of the CoreDefinitionVersion property type contains a list       of Core property types. Currently, the list can contain only one core.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Core {
    ///
    /// The Amazon Resource Name (ARN) of the device certificate for the core. This X.509 certificate is used to authenticate           the core with AWS IoT and AWS IoT Greengrass services.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_shadow: Option<bool>,

    ///
    /// The ARN of the core, which is an AWS IoT device (thing).
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

/// A core definition version contains a Greengrass core.
///
/// In an AWS CloudFormation template, CoreDefinitionVersion is the property type of the InitialVersion property      in the AWS::Greengrass::CoreDefinition resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CoreDefinitionVersion {
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

impl cfn_resources::CfnResource for CoreDefinitionVersion {
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
