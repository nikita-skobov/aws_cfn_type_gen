/// Creates an AWS account that is automatically a member of the       organization whose credentials made the request.
///
/// AWS CloudFormation uses the CreateAccount operation to create accounts. This is an       asynchronous request that AWS performs in the background. Because         CreateAccount operates asynchronously, it can return a successful       completion message even though account initialization might still be in progress. You       might need to wait a few minutes before you can successfully access the account. To       check the status of the request, do one of the following:
///
/// The user who calls the API to create an account must have the         organizations:CreateAccount permission. If you enabled all features in       the organization, AWS Organizations creates the required service-linked role named         AWSServiceRoleForOrganizations. For more information, see AWS Organizations and Service-Linked Roles in the         AWS Organizations User Guide.
///
/// If the request includes tags, then the requester must have the         organizations:TagResource permission.
///
/// AWS Organizations preconfigures the new member account with a role (named         OrganizationAccountAccessRole by default) that grants users in the       management account administrator permissions in the new member account. Principals in       the management account can assume the role. AWS Organizations clones the company       name and address information for the new account from the organization's management       account.
///
/// For more information about creating accounts, see Creating an           AWS account in Your Organization in the         AWS Organizations User Guide.
///
/// This operation can be called only from the organization's management account.
///
/// Deleting Account resources
///
/// The default DeletionPolicy for resource         AWS::Organizations::Account is Retain. For more       information about how AWS CloudFormation deletes resources, see         DeletionPolicy Attribute.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccount {
    ///
    /// The account name given to the account when it was created.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: [\u0020-\u007E]+
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "AccountName")]
    pub account_name: String,

    ///
    /// The email address associated with the AWS account.
    ///
    /// The regex pattern for this parameter is a string of characters that represents a       standard internet email address.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 6
    ///
    /// Maximum: 64
    ///
    /// Pattern: [^\s@]+@[^\s@]+\.[^\s@]+
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "Email")]
    pub email: String,

    ///
    /// The unique identifier (ID) of the root or organizational unit (OU) that you want to       create the new account in. If you don't specify this parameter, the         ParentId defaults to the root ID.
    ///
    /// This parameter only accepts a string array with one string value.
    ///
    /// The regex pattern for a parent ID       string requires one of the following:
    ///
    /// Root - A string that begins with "r-" followed           by from 4 to 32 lowercase letters or digits.                          Organizational unit (OU) - A string that begins           with "ou-" followed by from 4 to 32 lowercase letters or digits (the ID of the           root that the OU is in). This string is followed by a second "-" dash and from 8           to 32 additional lowercase letters or digits.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^(r-[0-9a-z]{4,32})|(ou-[0-9a-z]{4,32}-[a-z0-9]{8,32})$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParentIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_ids: Option<Vec<String>>,

    ///
    /// The name of an IAM role that AWS Organizations automatically preconfigures in the new member       account. This role trusts the management account, allowing users in the management       account to assume the role, as permitted by the management account administrator. The       role has administrator permissions in the new member account.
    ///
    /// If you don't specify this parameter, the role name defaults to         OrganizationAccountAccessRole.
    ///
    /// For more information about how to use this role to access the member account, see the       following links:
    ///
    /// Accessing and Administering the Member Accounts in Your             Organization in the            AWS Organizations User Guide                       Steps 2 and 3 in Tutorial:             Delegate Access Across AWS accounts Using IAM Roles in the             IAM User Guide
    ///
    /// The regex pattern that   is used to validate this parameter. The pattern can include uppercase   letters, lowercase letters, digits with no spaces, and any of the following characters: =,.@-
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Pattern: [\w+=,.@-]{1,64}
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,

    ///
    /// A list of tags that you want to attach to the newly created account. For each tag in       the list, you must specify both a tag key and a value. You can set the value to an empty       string, but you can't set it to null. For more information about tagging,       see Tagging AWS Organizations         resources in the AWS Organizations User Guide.
    ///
    /// NoteIf any one of the tags is not valid or if you exceed the maximum allowed number of         tags for an account, then the entire request fails and the account is not         created.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnAccount {
    fn type_string(&self) -> &'static str {
        "AWS::Organizations::Account"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.account_name;

        if the_val.len() > 50 as _ {
            return Err(format!(
                "Max validation failed on field 'account_name'. {} is greater than 50",
                the_val.len()
            ));
        }

        let the_val = &self.account_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'account_name'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.email;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'email'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.email;

        if the_val.len() < 6 as _ {
            return Err(format!(
                "Min validation failed on field 'email'. {} is less than 6",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.parent_ids {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'parent_ids'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.role_name {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'role_name'. {} is greater than 64",
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
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: String,
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
