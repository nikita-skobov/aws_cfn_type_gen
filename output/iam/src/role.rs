/// Creates a new role for your AWS account. For more information about roles, see         IAM         roles. For information about quotas for role names and the number of roles       you can create, see IAM and AWS STS quotas in the         IAM User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnRole {
    ///
    /// The trust policy that is associated with this role. Trust policies define which entities     can assume the role. You can associate only one trust policy with a role. For an example of     a policy that can be used to assume a role, see Template Examples. For more information about the elements that you can use in     an IAM policy, see IAM Policy       Elements Reference in the IAM User     Guide.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssumeRolePolicyDocument")]
    pub assume_role_policy_document: serde_json::Value,

    ///
    /// A description of the role that you provide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1000
    ///
    /// Pattern: [\u0009\u000A\u000D\u0020-\u007E\u00A1-\u00FF]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// A list of Amazon Resource Names (ARNs) of the IAM managed policies that     you want to attach to the role.
    ///
    /// For more information about ARNs, see Amazon Resource Names (ARNs) and        AWS Service Namespaces in the AWS       General Reference.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManagedPolicyArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_policy_arns: Option<Vec<String>>,

    ///
    /// The maximum session duration (in seconds) that you want to set for the specified role.       If you do not specify a value for this setting, the default value of one hour is       applied. This setting can have a value from 1 hour to 12 hours.
    ///
    /// Anyone who assumes the role from the AWS CLI or API can use the         DurationSeconds API parameter or the duration-seconds       AWS CLI parameter to request a longer session. The MaxSessionDuration setting       determines the maximum duration that can be requested using the         DurationSeconds parameter. If users don't specify a value for the         DurationSeconds parameter, their security credentials are valid for one       hour by default. This applies when you use the AssumeRole* API operations       or the assume-role*       AWS CLI operations but does not apply when you use those       operations to create a console URL. For more information, see Using IAM         roles in the IAM User Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 3600
    ///
    /// Maximum: 43200
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxSessionDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_session_duration: Option<i64>,

    ///
    /// The path to the role. For more information about paths, see IAM         Identifiers in the IAM User Guide.
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
    /// Update requires: Replacement
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the policy used to set the permissions boundary for the role.
    ///
    /// For more information about permissions boundaries, see Permissions boundaries for IAM       identities in the IAM User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PermissionsBoundary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary: Option<cfn_resources::StrVal>,

    ///
    /// Adds or updates an inline policy document that is embedded in the specified IAM role.
    ///
    /// When you embed an inline policy in a role, the inline policy is used as part of the     role's access (permissions) policy. The role's trust policy is created at the same time as     the role. You can update a role's trust policy later. For more information about IAM roles, go to Using Roles to Delegate Permissions and       Federate Identities.
    ///
    /// A role can also have an attached managed policy. For information about policies, see       Managed Policies and Inline Policies in the IAM User       Guide.
    ///
    /// For information about limits on the number of inline policies that you can embed with a     role, see Limitations on IAM Entities in the IAM User Guide.
    ///
    /// NoteIf an external policy (such as AWS::IAM::Policy or        AWS::IAM::ManagedPolicy) has a Ref to a role and if a       resource (such as AWS::ECS::Service) also has a Ref to the       same role, add a DependsOn attribute to the resource to make the resource       depend on the external policy. This dependency ensures that the role's policy is       available throughout the resource's lifecycle. For example, when you delete a stack with       an AWS::ECS::Service resource, the DependsOn attribute ensures       that AWS CloudFormation deletes the AWS::ECS::Service resource before       deleting its role's policy.
    ///
    /// Required: No
    ///
    /// Type: List of Policy
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,

    ///
    /// A name for the IAM role, up to 64 characters in length. For valid     values, see the RoleName parameter for the CreateRole action in the IAM User Guide.
    ///
    /// This parameter allows (per its regex       pattern) a string of characters consisting of upper and lowercase alphanumeric     characters with no spaces. You can also include any of the following characters: _+=,.@-.     The role name must be unique within the account. Role names are not distinguished by case.     For example, you cannot create roles named both "Role1" and "role1".
    ///
    /// If you don't specify a name, AWS CloudFormation generates a unique physical ID and     uses that ID for the role name.
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
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<cfn_resources::StrVal>,

    ///
    /// A list of tags that are attached to the role. For more information about tagging, see Tagging IAM resources in the    IAM User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnRolearn,

    #[serde(skip_serializing)]
    pub att_role_id: CfnRoleroleid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnRolearn;
impl CfnRolearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnRoleroleid;
impl CfnRoleroleid {
    pub fn att_name(&self) -> &'static str {
        r#"RoleId"#
    }
}

impl cfn_resources::CfnResource for CfnRole {
    fn type_string(&self) -> &'static str {
        "AWS::IAM::Role"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.max_session_duration {
            if *the_val > 43200 as _ {
                return Err(format!("Max validation failed on field 'max_session_duration'. {} is greater than 43200", the_val));
            }
        }

        if let Some(the_val) = &self.max_session_duration {
            if *the_val < 3600 as _ {
                return Err(format!(
                    "Min validation failed on field 'max_session_duration'. {} is less than 3600",
                    the_val
                ));
            }
        }

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
