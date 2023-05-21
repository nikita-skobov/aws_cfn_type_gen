

/// The AWS::Config::ConfigurationRecorder resource describes the AWS resource types for which AWS Config records configuration changes.       The configuration recorder stores the configurations of the supported resources in your account as configuration items.
///
/// AWS CloudFormation starts the recorder as soon as the delivery channel is available.
///
/// To stop the recorder and delete it, delete the configuration recorder from your stack. To stop the recorder without deleting it,       call the StopConfigurationRecorder action of the AWS Config API directly.
///
/// For more information, see Configuration Recorder in the AWS Config Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnConfigurationRecorder {


    /// 
    /// The Amazon Resource Name (ARN) of the IAM (IAM) role that is used to make read or write requests to the delivery channel that you specify and to get configuration details for supported AWS resources.       For more information, see Permissions for the IAM Role Assigned to AWS Config in the AWS Config Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleARN")]
    pub role_arn: String,


    /// 
    /// A name for the configuration recorder. If you don't specify a name, AWS CloudFormation CloudFormation generates a unique physical ID and uses that ID for the configuration recorder name.       For more information, see Name Type.
    /// 
    /// NoteAfter you create a configuration recorder, you cannot rename it. If you don't want a name that AWS CloudFormation generates, specify a value for this property.
    /// 
    /// Updates are not supported.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// Indicates whether to record configurations for all supported resources or for a list of resource types. The resource types that you list must be supported by AWS Config.
    /// 
    /// Required: No
    ///
    /// Type: RecordingGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordingGroup")]
    pub recording_group: Option<RecordingGroup>,

}


/// Specifies which AWS resource types AWS Config 			records for configuration changes. In the recording group, you specify whether you want to record all supported resource types 			or only specific types of resources.
///
/// By default, AWS Config records the configuration changes for all supported types of 				regional resources that AWS Config discovers in the region in which it is 				running. Regional resources are tied to a region and can be used only in that region. Examples 				of regional resources are EC2 instances and EBS volumes.
///
/// You can also have AWS Config record supported types of global resources. 				Global resources are not tied to a specific region and can be used in all regions. The global 				resource types that AWS Config supports include IAM users, groups, roles, and customer managed 				policies.
///
/// If you don't want AWS Config to record all resources, you can 			specify which types of resources it will record with the 				resourceTypes parameter.
///
/// For a list of supported resource types, see Supported Resource Types.
///
/// For more information and a table of the Home Regions for Global Resource Types Onboarded after February 2022, see Selecting Which Resources AWS Config Records.
#[derive(Default, serde::Serialize)]
pub struct RecordingGroup {


    /// 
    /// A comma-separated list that specifies the types of AWS resources for which AWS Config records configuration changes (for 			example, AWS::EC2::Instance or 				AWS::CloudTrail::Trail).
    /// 
    /// To record all configuration changes, you must 			set the AllSupported option to 			false.
    /// 
    /// If you set the AllSupported option to false and populate the ResourceTypes option with values,       when AWS Config adds support for a new type of resource,       it will not record resources of that type unless you manually add that type to your recording group.
    /// 
    /// For a list of valid resourceTypes values, see the 				resourceType Value column in 				Supported AWS Resource Types.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceTypes")]
    pub resource_types: Option<Vec<String>>,


    /// 
    /// Specifies whether AWS Config records configuration changes for 			every supported type of regional resource.
    /// 
    /// If you set this option to true, when AWS Config 			adds support for a new type of regional resource, it starts 			recording resources of that type automatically.
    /// 
    /// If you set this option to true, you cannot 			enumerate a list of resourceTypes.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllSupported")]
    pub all_supported: Option<bool>,


    /// 
    /// Specifies whether AWS Config includes all supported types of 			global resources (for example, IAM resources) with the resources 			that it records.
    /// 
    /// Before you can set this option to true, you must 			set the AllSupported option to 			true.
    /// 
    /// If you set this option to true, when AWS Config 			adds support for a new type of global resource, it starts recording 			resources of that type automatically.
    /// 
    /// The configuration details for any global resource are the same 			in all regions. To prevent duplicate configuration items, you should 			consider customizing AWS Config in only one region to record global 			resources.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeGlobalResourceTypes")]
    pub include_global_resource_types: Option<bool>,

}
