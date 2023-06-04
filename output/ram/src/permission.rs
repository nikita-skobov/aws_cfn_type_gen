/// Creates a customer managed permission for a specified resource type that you can attach to resource shares.       It is created in the AWS Region in which you call the operation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnPermission {
    ///
    /// Specifies the name of the customer managed permission. The name must be unique within the       AWS Region.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 36
    ///
    /// Pattern: [\w.-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// A string in JSON format string that contains the following elements of a       resource-based policy:
    ///
    /// Effect: must be set to           ALLOW.                        Action: specifies the actions that are           allowed by this customer managed permission. The list must contain only actions that are supported by           the specified resource type. For a list of all actions supported by each           resource type, see Actions, resources, and condition keys for AWS services in the                        AWS Identity and Access Management User Guide.                        Condition: (optional) specifies conditional           parameters that must evaluate to true when a user attempts an action for that           action to be allowed. For more information about the Condition element, see           IAM             policies: Condition element in the            AWS Identity and Access Management User           Guide.
    ///
    /// This template can't include either the Resource or       Principal elements. Those are both filled in by AWS RAM when it instantiates       the resource-based policy on each resource shared using this managed permission. The       Resource comes from the ARN of the specific resource that you are sharing.       The Principal comes from the list of identities added to the resource       share.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "PolicyTemplate")]
    pub policy_template: serde_json::Value,

    ///
    /// Specifies the name of the resource type that this customer managed permission applies       to.
    ///
    /// The format is         <service-code>:<resource-type>       and is not case sensitive. For example, to specify an Amazon EC2 Subnet, you can       use the string ec2:subnet. To see the list of valid values for this       parameter, query the ListResourceTypes       operation.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceType")]
    pub resource_type: cfn_resources::StrVal,

    ///
    /// Specifies a list of one or more tag key and value pairs to attach to the       permission.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnPermissionarn,

    #[serde(skip_serializing)]
    pub att_permission_type: CfnPermissionpermissiontype,

    #[serde(skip_serializing)]
    pub att_version: CfnPermissionversion,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPermissionarn;
impl CfnPermissionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPermissionpermissiontype;
impl CfnPermissionpermissiontype {
    pub fn att_name(&self) -> &'static str {
        r#"PermissionType"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPermissionversion;
impl CfnPermissionversion {
    pub fn att_name(&self) -> &'static str {
        r#"Version"#
    }
}

impl cfn_resources::CfnResource for CfnPermission {
    fn type_string(&self) -> &'static str {
        "AWS::RAM::Permission"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 36 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 36",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

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
