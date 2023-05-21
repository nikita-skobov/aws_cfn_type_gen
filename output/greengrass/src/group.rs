

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
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGroup {


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
    pub tags: Option<serde_json::Value>,


    /// 
    /// The name of the group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role attached to the group. This role contains the permissions that           Lambda functions and connectors use to interact with other AWS services.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


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
    pub initial_version: Option<GroupVersion>,

}



impl cfn_resources::CfnResource for CfnGroup {
    fn type_string() -> &'static str {
        "AWS::Greengrass::Group"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// A group version in AWS IoT Greengrass, 		   which references of a core definition version,      device definition version, subscription definition version, and other version types 				 that contain the components you want to deploy to a Greengrass core device. 		The group version must reference a core definition version that contains one core. 		Other version types are optionally included, depending on your business need.
///
/// In an AWS CloudFormation template, GroupVersion is the property type of the InitialVersion property      in the AWS::Greengrass::Group resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GroupVersion {


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
    /// 
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
    /// The ARN of the function definition version that contains the functions you want to deploy with the group version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FunctionDefinitionVersionArn")]
    pub function_definition_version_arn: Option<String>,


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
    /// The Amazon Resource Name (ARN) of the connector definition version that contains the connectors you want to deploy with the group version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectorDefinitionVersionArn")]
    pub connector_definition_version_arn: Option<String>,

}


