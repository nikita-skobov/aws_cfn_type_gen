/// Creates a fleet provisioning template.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnProvisioningTemplate {
    ///
    /// The description of the fleet provisioning template.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// True to enable the fleet provisioning template, otherwise false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    ///
    /// Creates a pre-provisioning hook template.
    ///
    /// Required: No
    ///
    /// Type: ProvisioningHook
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreProvisioningHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_provisioning_hook: Option<ProvisioningHook>,

    ///
    /// The role ARN for the role associated with the fleet provisioning template. This IoT role grants permission to provision a device.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisioningRoleArn")]
    pub provisioning_role_arn: cfn_resources::StrVal,

    ///
    /// Metadata that can be used to manage the fleet provisioning template.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The JSON formatted contents of the fleet provisioning template version.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateBody")]
    pub template_body: cfn_resources::StrVal,

    ///
    /// The name of the fleet provisioning template.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<cfn_resources::StrVal>,

    ///
    /// The type of the provisioning template.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TemplateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_template_arn: CfnProvisioningTemplatetemplatearn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnProvisioningTemplatetemplatearn;
impl CfnProvisioningTemplatetemplatearn {
    pub fn att_name(&self) -> &'static str {
        r#"TemplateArn"#
    }
}

impl cfn_resources::CfnResource for CfnProvisioningTemplate {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::ProvisioningTemplate"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.pre_provisioning_hook
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Structure that contains payloadVersion and targetArn. Provisioning hooks can be used when fleet provisioning to validate device parameters before allowing the device to be provisioned.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ProvisioningHook {
    ///
    /// The payload that was sent to the target function. The valid payload is "2020-04-01".
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PayloadVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_version: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the target function.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ProvisioningHook {
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
