/// Creates a policy of a specified type that you can attach to a root, an organizational       unit (OU), or an individual AWS account.
///
/// For more information about policies and their use, see Managing Organization         Policies.
///
/// If the request includes tags, then the requester must have the         organizations:TagResource permission.
///
/// This operation can be called only from the organization's management account.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnPolicy {
    ///
    /// The policy text content. You can specify the policy content as a JSON object or a JSON       string.
    ///
    /// ImportantWhen you specify the policy content as a JSON string, you can't perform drift         detection on the CloudFormation stack. For this reason, we recommend         specifying the policy content as a JSON object instead.
    ///
    /// The text that you supply must adhere to the rules of the policy type you specify in       the Type parameter. The following AWS Organizations quotas are enforced       for the maximum size of a policy document:
    ///
    /// Service control policies: 5,120 bytes (not           characters)               AI services opt-out policies: 2,500 characters               Backup policies: 10,000 characters               Tag policies: 10,000 characters
    ///
    /// For more information about Organizations service quotas, see Quotas for AWS Organizations in the AWS Organizations         User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000000
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Content")]
    pub content: serde_json::Value,

    ///
    /// Human readable description of the policy.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// Name of the policy.
    ///
    /// The regex pattern that is used to       validate this parameter is a string of any of the characters in the ASCII character       range.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// A list of tags that you want to attach to the newly created policy. For each tag in       the list, you must specify both a tag key and a value. You can set the value to an empty       string, but you can't set it to null. For more information about tagging,       see Tagging AWS Organizations         resources in the AWS Organizations User Guide.
    ///
    /// NoteIf any one of the tags is not valid or if you exceed the allowed number of tags         for a policy, then the entire request fails and the policy is not created.
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
    /// List of unique identifiers (IDs) of the root, OU, or account that you want to attach       the policy to. You can get the ID by calling the ListRoots, ListOrganizationalUnitsForParent, or ListAccounts       operations. If you don't specify this parameter, the policy is created but not attached       to any organization resource.
    ///
    /// The regex pattern for a target ID       string requires one of the following:
    ///
    /// Root - A string that begins with "r-" followed           by from 4 to 32 lowercase letters or digits.                          Account - A string that consists of exactly 12           digits.                          Organizational unit (OU) - A string that begins           with "ou-" followed by from 4 to 32 lowercase letters or digits (the ID of the           root that the OU is in). This string is followed by a second "-" dash and from 8           to 32 additional lowercase letters or digits.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^(r-[0-9a-z]{4,32})|(\d{12})|(ou-[0-9a-z]{4,32}-[a-z0-9]{8,32})$
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ids: Option<Vec<String>>,

    ///
    /// The type of policy to create.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AISERVICES_OPT_OUT_POLICY | BACKUP_POLICY | SERVICE_CONTROL_POLICY | TAG_POLICY
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: PolicyTypeEnum,

    #[serde(skip_serializing)]
    pub att_arn: CfnPolicyarn,

    #[serde(skip_serializing)]
    pub att_id: CfnPolicyid,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PolicyTypeEnum {
    /// AISERVICES_OPT_OUT_POLICY
    #[serde(rename = "AISERVICES_OPT_OUT_POLICY")]
    Aiservicesoptoutpolicy,

    /// BACKUP_POLICY
    #[serde(rename = "BACKUP_POLICY")]
    Backuppolicy,

    /// SERVICE_CONTROL_POLICY
    #[serde(rename = "SERVICE_CONTROL_POLICY")]
    Servicecontrolpolicy,

    /// TAG_POLICY
    #[serde(rename = "TAG_POLICY")]
    Tagpolicy,
}

impl Default for PolicyTypeEnum {
    fn default() -> Self {
        PolicyTypeEnum::Aiservicesoptoutpolicy
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPolicyarn;
impl CfnPolicyarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPolicyid;
impl CfnPolicyid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::Organizations::Policy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 512 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 512",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 128",
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

        if let Some(the_val) = &self.target_ids {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'target_ids'. {} is greater than 100",
                    the_val.len()
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
