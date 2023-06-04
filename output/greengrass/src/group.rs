/// AWS IoT Greengrass seamlessly extends AWS to edge devices so they can act locally on the data they generate, while still using the cloud for management, analytics, and durable storage. With AWS IoT Greengrass, connected devices can run AWS Lambda functions, execute predictions based on machine learning models, keep device data in sync, and communicate with other devices securely – even when not connected to the internet.  For more information, see the AWS IoT Greengrass Version 1 Developer Guide.
///
/// The     AWS::Greengrass::Group resource represents a group in AWS IoT Greengrass.   In the AWS IoT Greengrass API, groups are used to organize your group versions.
///
/// Groups can reference multiple group versions. All group versions      must be associated with a group. A group version references a     device definition version, subscription definition version, and other version types     that contain the components you want to deploy to a Greengrass core device.
///
/// To deploy a group version, the group version must reference a core definition version that      contains one core. Other version types are optionally included, depending on your business need.
///
/// Deploying a Group Version
///
/// After you create the group version in your AWS CloudFormation template, you can deploy it using the  		 aws greengrass create-deployment command in the AWS CLI or 		 from the Greengrass node in the AWS IoT console. To deploy a group version, you must have a Greengrass service role associated with 		    your AWS account. For more information, see AWS CloudFormation Support for AWS IoT Greengrass 		 in the AWS IoT Greengrass Version 1 Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnGroup {
    ///
    /// The group version to include when the group is created.          A group version references the Amazon Resource Name (ARN) of a core definition version,           device definition version, subscription definition version, and other version types.  				 The group version must reference a core definition version that contains one core. 				 Other version types are optionally included, depending on your business need.
    ///
    /// NoteTo associate a group version after the group is created, 				   create an AWS::Greengrass::GroupVersion resource and specify the ID of this group.
    ///
    /// Required: No
    ///
    /// Type: GroupVersion
    ///
    /// Update requires: Replacement
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub initial_version: Option<GroupVersion>,

    ///
    /// The name of the group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The Amazon Resource Name (ARN) of the IAM role attached to the group. This role contains the permissions that           Lambda functions and connectors use to interact with other AWS services.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub role_arn: Option<cfn_resources::StrVal>,

    ///
    /// Application-specific metadata to attach to the group. 		  You can use tags in IAM policies to control access to AWS IoT Greengrass resources. 		  You can also use tags to categorize your resources. For more information, see 		  Tagging Your AWS IoT Greengrass 		  Resources in the AWS IoT Greengrass Version 1 Developer Guide.
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<serde_json::Value>,

    #[serde(skip_serializing)]
    pub att_arn: CfnGrouparn,

    #[serde(skip_serializing)]
    pub att_id: CfnGroupid,

    #[serde(skip_serializing)]
    pub att_latest_version_arn: CfnGrouplatestversionarn,

    #[serde(skip_serializing)]
    pub att_name: CfnGroupname,

    #[serde(skip_serializing)]
    pub att_role_arn: CfnGrouprolearn,

    #[serde(skip_serializing)]
    pub att_role_attached_at: CfnGrouproleattachedat,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGrouparn;
impl CfnGrouparn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGroupid;
impl CfnGroupid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGrouplatestversionarn;
impl CfnGrouplatestversionarn {
    pub fn att_name(&self) -> &'static str {
        r#"LatestVersionArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGroupname;
impl CfnGroupname {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGrouprolearn;
impl CfnGrouprolearn {
    pub fn att_name(&self) -> &'static str {
        r#"RoleArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGrouproleattachedat;
impl CfnGrouproleattachedat {
    pub fn att_name(&self) -> &'static str {
        r#"RoleAttachedAt"#
    }
}

impl cfn_resources::CfnResource for CfnGroup {
    fn type_string(&self) -> &'static str {
        "AWS::Greengrass::Group"
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

/// A group version in AWS IoT Greengrass, 		   which references of a core definition version,      device definition version, subscription definition version, and other version types 				 that contain the components you want to deploy to a Greengrass core device. 		The group version must reference a core definition version that contains one core. 		Other version types are optionally included, depending on your business need.
///
/// In an AWS CloudFormation template, GroupVersion is the property type of the InitialVersion property      in the AWS::Greengrass::Group resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GroupVersion {
    ///
    /// The Amazon Resource Name (ARN) of the connector definition version that contains the connectors you want to deploy with the group version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectorDefinitionVersionArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub function_definition_version_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the logger definition version that contains the loggers you want to deploy with the group version.
    ///
    ///
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoggerDefinitionVersionArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub subscription_definition_version_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for GroupVersion {
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
