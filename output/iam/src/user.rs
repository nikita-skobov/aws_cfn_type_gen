/// Creates a new IAM user for your AWS account.
///
/// For information about quotas for the number of IAM users you can create, see IAM and AWS STS         quotas in the IAM User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnUser {
    ///
    /// A list of group names to which you want to add the user.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub groups: Option<Vec<String>>,

    ///
    /// Creates a password for the specified IAM user. A password allows an       IAM user to access AWS services through the AWS Management Console.
    ///
    /// You can use the AWS CLI, the AWS API, or the Users page in the IAM console to create a     password for any IAM user. Use ChangePassword to update     your own existing password in the My Security Credentials     page in the AWS Management Console.
    ///
    /// For more information about managing passwords, see Managing passwords in the        IAM User Guide.
    ///
    /// Required: No
    ///
    /// Type: LoginProfile
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoginProfile")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub login_profile: Option<LoginProfile>,

    ///
    /// A list of Amazon Resource Names (ARNs) of the IAM managed policies that     you want to attach to the user.
    ///
    /// For more information about ARNs, see Amazon Resource Names (ARNs) and        AWS Service Namespaces in the AWS       General Reference.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManagedPolicyArns")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub managed_policy_arns: Option<Vec<String>>,

    ///
    /// The path for the user name. For more information about paths, see IAM         identifiers in the IAM User Guide.
    ///
    /// This parameter is optional. If it is not included, it defaults to a slash (/).
    ///
    /// This parameter allows (through its regex pattern) a string of characters consisting   of either a forward slash (/) by itself or a string that must begin and end with forward slashes.   In addition, it can contain any ASCII character from the ! (\u0021) through the DEL character (\u007F), including   most punctuation characters, digits, and upper and lowercased letters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: (\u002F)|(\u002F[\u0021-\u007E]+\u002F)
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub path: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the managed policy that is used to set the permissions boundary for the       user.
    ///
    /// A permissions boundary policy defines the maximum permissions that identity-based       policies can grant to an entity, but does not grant permissions. Permissions boundaries       do not define the maximum permissions that a resource-based policy can grant to an       entity. To learn more, see Permissions boundaries         for IAM entities in the IAM User Guide.
    ///
    /// For more information about policy types, see Policy types       in the IAM User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PermissionsBoundary")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub permissions_boundary: Option<cfn_resources::StrVal>,

    ///
    /// Adds or updates an inline policy document that is embedded in the specified IAM user. To view AWS::IAM::User snippets, see Declaring       an IAM User Resource.
    ///
    /// ImportantThe name of each policy for a role, user, or group must be unique. If you don't       choose unique names, updates to the IAM identity will fail.
    ///
    /// For information about limits on the number of inline policies that you can embed in a     user, see Limitations on IAM Entities in the IAM User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Policy
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub policies: Option<Vec<Policy>>,

    ///
    /// A list of tags that you want to attach to the new user. Each tag consists of a key name and an associated value.    For more information about tagging, see Tagging IAM resources in the    IAM User Guide.
    ///
    /// NoteIf any one of the tags is invalid or if you exceed the allowed maximum number of tags, then the entire request   fails and the resource is not created.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The name of the user to create. Do not include the path in this value.
    ///
    /// This parameter allows (per its regex       pattern) a string of characters consisting of upper and lowercase alphanumeric     characters with no spaces. You can also include any of the following characters: _+=,.@-.     The user name must be unique within the account. User names are not distinguished by case.     For example, you cannot create users named both "John" and "john".
    ///
    /// If you don't specify a name, AWS CloudFormation generates a unique physical ID and     uses that ID for the user name.
    ///
    /// If you specify a name, you must specify the CAPABILITY_NAMED_IAM value to     acknowledge your template's capabilities. For more information, see Acknowledging IAM Resources in AWS CloudFormation     Templates.
    ///
    /// ImportantNaming an IAM resource can cause an unrecoverable error if you reuse       the same template in multiple Regions. To prevent this, we recommend using        Fn::Join and AWS::Region to create a Region-specific name,       as in the following example: {"Fn::Join": ["", [{"Ref": "AWS::Region"}, {"Ref":        "MyResourceName"}]]}.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub user_name: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_arn: CfnUserarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnUserarn;
impl CfnUserarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnUser {
    fn type_string(&self) -> &'static str {
        "AWS::IAM::User"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.login_profile
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.path {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 512 as _ {
                    return Err(format!(
                        "Max validation failed on field 'path'. {} is greater than 512",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.path {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'path'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Creates a password for the specified user, giving the user the ability to access AWS services through the AWS Management Console. For more information about     managing passwords, see Managing Passwords in the        IAM User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LoginProfile {
    ///
    /// The user's password.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: cfn_resources::StrVal,

    ///
    /// Specifies whether the user is required to set a new password on next sign-in.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PasswordResetRequired")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub password_reset_required: Option<bool>,
}

impl cfn_resources::CfnResource for LoginProfile {
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

/// Contains information about an attached policy.
///
/// An attached policy is a managed policy that has been attached to a user, group, or     role.
///
/// For more information about managed policies, refer to Managed Policies and Inline       Policies in the IAM User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Policy {
    ///
    /// The entire contents of the policy that defines permissions. For more information, see       Overview of JSON       policies.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    pub policy_document: serde_json::Value,

    ///
    /// The friendly name (not ARN) identifying the policy.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w+=,.@-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyName")]
    pub policy_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Policy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.policy_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'policy_name'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.policy_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'policy_name'. {} is less than 1",
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
