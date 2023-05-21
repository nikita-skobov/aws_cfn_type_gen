

/// Defines an action that can be applied to audit findings by using StartAuditMitigationActionsTask. For API reference, see     CreateMitigationAction and for general information,      see Mitigation actions.
#[derive(Default, serde::Serialize)]
pub struct CfnMitigationAction {


    /// 
    /// The set of parameters for this mitigation action. The parameters vary, depending on the kind of action you apply.
    /// 
    /// Required: Yes
    ///
    /// Type: ActionParams
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionParams")]
    pub action_params: ActionParams,


    /// 
    /// The friendly name of the mitigation action.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ActionName")]
    pub action_name: Option<String>,


    /// 
    /// Metadata that can be used to manage the mitigation action.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The IAM role ARN used to apply this mitigation action.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}


/// Parameters used when defining a mitigation action that move a set of things to a thing group.
#[derive(Default, serde::Serialize)]
pub struct AddThingsToThingGroupParams {


    /// 
    /// The list of groups to which you want to add the things that triggered the mitigation action. You can add a thing to a maximum of 10 groups, but you can't add a thing to more than one group in the same hierarchy.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThingGroupNames")]
    pub thing_group_names: Vec<String>,


    /// 
    /// Specifies if this mitigation action can move the things that triggered the mitigation action even if they are part of one or more dynamic thing groups.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "OverrideDynamicGroups")]
    pub override_dynamic_groups: Option<bool>,

}


/// Defines the type of action and the parameters for that action.
#[derive(Default, serde::Serialize)]
pub struct ActionParams {


    /// 
    /// Specifies the group to which you want to add the devices.
    /// 
    /// Required: No
    ///
    /// Type: AddThingsToThingGroupParams
    ///
    /// Update requires: No interruption
    #[serde(rename = "AddThingsToThingGroupParams")]
    pub add_things_to_thing_group_params: Option<AddThingsToThingGroupParams>,


    /// 
    /// Specifies the new state for a device certificate. Only a value of DEACTIVATE is currently supported.
    /// 
    /// Required: No
    ///
    /// Type: UpdateDeviceCertificateParams
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpdateDeviceCertificateParams")]
    pub update_device_certificate_params: Option<UpdateDeviceCertificateParams>,


    /// 
    /// Specifies the topic to which the finding should be published.
    /// 
    /// Required: No
    ///
    /// Type: PublishFindingToSnsParams
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublishFindingToSnsParams")]
    pub publish_finding_to_sns_params: Option<PublishFindingToSnsParams>,


    /// 
    /// Specifies the new state for the CA certificate. Only a value of DEACTIVATE is currently supported.
    /// 
    /// Required: No
    ///
    /// Type: UpdateCACertificateParams
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpdateCACertificateParams")]
    pub update_cacertificate_params: Option<UpdateCACertificateParams>,


    /// 
    /// Specifies the logging level and the role with permissions for logging. You cannot specify a logging level of DISABLED.
    /// 
    /// Required: No
    ///
    /// Type: EnableIoTLoggingParams
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableIoTLoggingParams")]
    pub enable_io_tlogging_params: Option<EnableIoTLoggingParams>,


    /// 
    /// Replaces the policy version with a default or blank policy. You specify the template name. Only a value of BLANK_POLICY is currently supported.
    /// 
    /// Required: No
    ///
    /// Type: ReplaceDefaultPolicyVersionParams
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplaceDefaultPolicyVersionParams")]
    pub replace_default_policy_version_params: Option<ReplaceDefaultPolicyVersionParams>,

}


/// Parameters used when defining a mitigation action that enable AWS IoT Core logging.
#[derive(Default, serde::Serialize)]
pub struct EnableIoTLoggingParams {


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role used for logging.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArnForLogging")]
    pub role_arn_for_logging: String,


    /// 
    /// Specifies the type of information to be logged.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogLevel")]
    pub log_level: String,

}


/// Parameters to define a mitigation action that publishes findings to Amazon SNS. You can implement your own custom actions in response to the Amazon SNS messages.
#[derive(Default, serde::Serialize)]
pub struct PublishFindingToSnsParams {


    /// 
    /// The ARN of the topic to which you want to publish the findings.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TopicArn")]
    pub topic_arn: String,

}


/// Parameters to define a mitigation action that adds a blank policy to restrict permissions.
#[derive(Default, serde::Serialize)]
pub struct ReplaceDefaultPolicyVersionParams {


    /// 
    /// The name of the template to be applied. The only supported value is BLANK_POLICY.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateName")]
    pub template_name: String,

}


/// Parameters to define a mitigation action that changes the state of the device certificate to inactive.
#[derive(Default, serde::Serialize)]
pub struct UpdateDeviceCertificateParams {


    /// 
    /// The action that you want to apply to the device certificate. The only supported value is DEACTIVATE.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: String,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// Parameters to define a mitigation action that changes the state of the CA certificate to inactive.
#[derive(Default, serde::Serialize)]
pub struct UpdateCACertificateParams {


    /// 
    /// The action that you want to apply to the CA certificate. The only supported value is DEACTIVATE.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: String,

}