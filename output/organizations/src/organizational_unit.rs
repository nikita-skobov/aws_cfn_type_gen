/// Creates an organizational unit (OU) within a root or parent OU. An OU is a container       for accounts that enables you to organize your accounts to apply policies according to       your business requirements. The number of levels deep that you can nest OUs is dependent       upon the policy types enabled for that root. For service control policies, the limit is       five.
///
/// For more information about OUs, see Managing Organizational Units in the                 AWS Organizations User Guide.
///
/// If the request includes tags, then the requester must have the         organizations:TagResource permission.
///
/// This operation can be called only from the organization's management account.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnOrganizationalUnit {
    ///
    /// The friendly name of this OU.
    ///
    /// The regex pattern   that is used to validate this parameter is a string of any of the characters in the ASCII   character range.
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
    /// The unique identifier (ID) of the parent root or OU that you want to create the new OU       in.
    ///
    /// ImportantTo update the ParentId parameter value, you must first remove all         accounts attached to the organizational unit (OU). OUs can't be moved within the         organization with accounts still attached.
    ///
    /// The regex pattern for a parent ID       string requires one of the following:
    ///
    /// Root - A string that begins with "r-" followed           by from 4 to 32 lowercase letters or digits.                          Organizational unit (OU) - A string that begins           with "ou-" followed by from 4 to 32 lowercase letters or digits (the ID of the           root that the OU is in). This string is followed by a second "-" dash and from 8           to 32 additional lowercase letters or digits.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^(r-[0-9a-z]{4,32})|(ou-[0-9a-z]{4,32}-[a-z0-9]{8,32})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ParentId")]
    pub parent_id: cfn_resources::StrVal,

    ///
    /// A list of tags that you want to attach to the newly created OU. For each tag in the       list, you must specify both a tag key and a value. You can set the value to an empty       string, but you can't set it to null. For more information about tagging,       see Tagging AWS Organizations         resources in the AWS Organizations User Guide.
    ///
    /// NoteIf any one of the tags is not valid or if you exceed the allowed number of tags         for an OU, then the entire request fails and the OU is not created.
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
    pub att_arn: CfnOrganizationalUnitarn,

    #[serde(skip_serializing)]
    pub att_id: CfnOrganizationalUnitid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnOrganizationalUnitarn;
impl CfnOrganizationalUnitarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnOrganizationalUnitid;
impl CfnOrganizationalUnitid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnOrganizationalUnit {
    fn type_string(&self) -> &'static str {
        "AWS::Organizations::OrganizationalUnit"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
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

        let the_val = &self.parent_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'parent_id'. {} is greater than 100",
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
