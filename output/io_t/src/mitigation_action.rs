/// Defines an action that can be applied to audit findings by using StartAuditMitigationActionsTask. For API reference, see     CreateMitigationAction and for general information,      see Mitigation actions.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnMitigationAction {
    ///
    /// The friendly name of the mitigation action.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ActionName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub action_name: Option<cfn_resources::StrVal>,

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
    /// The IAM role ARN used to apply this mitigation action.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// Metadata that can be used to manage the mitigation action.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_mitigation_action_arn: CfnMitigationActionmitigationactionarn,

    #[serde(skip_serializing)]
    pub att_mitigation_action_id: CfnMitigationActionmitigationactionid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMitigationActionmitigationactionarn;
impl CfnMitigationActionmitigationactionarn {
    pub fn att_name(&self) -> &'static str {
        r#"MitigationActionArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMitigationActionmitigationactionid;
impl CfnMitigationActionmitigationactionid {
    pub fn att_name(&self) -> &'static str {
        r#"MitigationActionId"#
    }
}

impl cfn_resources::CfnResource for CfnMitigationAction {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::MitigationAction"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.action_params.validate()?;

        Ok(())
    }
}

/// Defines the type of action and the parameters for that action.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub add_things_to_thing_group_params: Option<AddThingsToThingGroupParams>,

    ///
    /// Specifies the logging level and the role with permissions for logging. You cannot specify a logging level of DISABLED.
    ///
    /// Required: No
    ///
    /// Type: EnableIoTLoggingParams
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableIoTLoggingParams")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub enable_io_tlogging_params: Option<EnableIoTLoggingParams>,

    ///
    /// Specifies the topic to which the finding should be published.
    ///
    /// Required: No
    ///
    /// Type: PublishFindingToSnsParams
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublishFindingToSnsParams")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub publish_finding_to_sns_params: Option<PublishFindingToSnsParams>,

    ///
    /// Replaces the policy version with a default or blank policy. You specify the template name. Only a value of BLANK_POLICY is currently supported.
    ///
    /// Required: No
    ///
    /// Type: ReplaceDefaultPolicyVersionParams
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplaceDefaultPolicyVersionParams")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub replace_default_policy_version_params: Option<ReplaceDefaultPolicyVersionParams>,

    ///
    /// Specifies the new state for the CA certificate. Only a value of DEACTIVATE is currently supported.
    ///
    /// Required: No
    ///
    /// Type: UpdateCACertificateParams
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpdateCACertificateParams")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub update_cacertificate_params: Option<UpdateCACertificateParams>,

    ///
    /// Specifies the new state for a device certificate. Only a value of DEACTIVATE is currently supported.
    ///
    /// Required: No
    ///
    /// Type: UpdateDeviceCertificateParams
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpdateDeviceCertificateParams")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub update_device_certificate_params: Option<UpdateDeviceCertificateParams>,
}

impl cfn_resources::CfnResource for ActionParams {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.add_things_to_thing_group_params
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.enable_io_tlogging_params
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.publish_finding_to_sns_params
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.replace_default_policy_version_params
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.update_cacertificate_params
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.update_device_certificate_params
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Parameters used when defining a mitigation action that move a set of things to a thing group.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AddThingsToThingGroupParams {
    ///
    /// Specifies if this mitigation action can move the things that triggered the mitigation action even if they are part of one or more dynamic thing groups.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "OverrideDynamicGroups")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub override_dynamic_groups: Option<bool>,

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
}

impl cfn_resources::CfnResource for AddThingsToThingGroupParams {
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

/// Parameters used when defining a mitigation action that enable AWS IoT Core logging.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EnableIoTLoggingParams {
    ///
    /// Specifies the type of information to be logged.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogLevel")]
    pub log_level: cfn_resources::StrVal,

    ///
    /// The Amazon Resource Name (ARN) of the IAM role used for logging.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArnForLogging")]
    pub role_arn_for_logging: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for EnableIoTLoggingParams {
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

/// Parameters to define a mitigation action that publishes findings to Amazon SNS. You can implement your own custom actions in response to the Amazon SNS messages.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub topic_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for PublishFindingToSnsParams {
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

/// Parameters to define a mitigation action that adds a blank policy to restrict permissions.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub template_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ReplaceDefaultPolicyVersionParams {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
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

/// Parameters to define a mitigation action that changes the state of the CA certificate to inactive.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub action: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for UpdateCACertificateParams {
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

/// Parameters to define a mitigation action that changes the state of the device certificate to inactive.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub action: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for UpdateDeviceCertificateParams {
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
