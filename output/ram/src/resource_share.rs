/// Creates a resource share. You can provide a list of the Amazon Resource Names (ARNs)       for the resources that you want to share, a list of principals you want to share the       resources with, and the permissions to grant those principals.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourceShare {
    ///
    /// Specifies whether principals outside your organization in AWS Organizations can be associated       with a resource share. A value of true lets you share with individual AWS accounts       that are not in your organization. A value of false       only has meaning if your account is a member of an AWS Organization. The default value       is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowExternalPrincipals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_principals: Option<bool>,

    ///
    /// Specifies the name of the resource share.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Specifies the Amazon Resource Names (ARNs) of the AWS RAM permission to associate with the resource share. If you do       not specify an ARN for the permission, AWS RAM automatically attaches the default version       of the permission for each resource type. You can associate only one permission with       each resource type included in the resource share.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PermissionArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_arns: Option<Vec<String>>,

    ///
    /// Specifies the principals to associate with the resource share. The possible values       are:
    ///
    /// An AWS account ID               An Amazon Resource Name (ARN) of an organization in AWS Organizations                        An ARN of an organizational unit (OU) in AWS Organizations                        An ARN of an IAM role               An ARN of an IAM user
    ///
    /// NoteNot all resource types can be shared with IAM roles and users. For         more information, see the column Can share with IAM roles and users in the tables on Shareable AWS resources in the           AWS Resource Access Manager User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Principals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,

    ///
    /// Specifies a list of one or more ARNs of the resources to associate with the       resource share.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,

    ///
    /// Specifies one or more tags to attach to the resource share itself. It doesn't attach the tags to       the resources associated with the resource share.
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
    pub att_arn: CfnResourceSharearn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourceSharearn;
impl CfnResourceSharearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnResourceShare {
    fn type_string(&self) -> &'static str {
        "AWS::RAM::ResourceShare"
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
