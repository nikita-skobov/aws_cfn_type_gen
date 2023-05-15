
pub mod cfn_account {

#[derive(serde::Serialize, Default)]
pub struct CfnAccount {
    /// The friendly name of the member account.
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// The email address of the owner to assign to the new member account.
    #[serde(rename = "Email")]
    pub email: String,
    /// A list of tags that you want to attach to the newly created account. For each tag in the list, you must specify both a tag key and a value.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of parent nodes for the member account. Currently only one parent at a time is supported. Default is root.
    #[serde(rename = "ParentIds")]
    pub parent_ids: Option<Vec<String>>,
    /// The name of an IAM role that AWS Organizations automatically preconfigures in the new member account. Default name is OrganizationAccountAccessRole if not specified.
    #[serde(rename = "RoleName")]
    pub role_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_organizational_unit {

#[derive(serde::Serialize, Default)]
pub struct CfnOrganizationalUnit {
    /// The unique identifier (ID) of the parent root or OU that you want to create the new OU in.
    #[serde(rename = "ParentId")]
    pub parent_id: String,
    /// A list of tags that you want to attach to the newly created OU.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The friendly name of this OU.
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnPolicy {
    /// The Policy text content. For AWS CloudFormation templates formatted in YAML, you can provide the policy in JSON or YAML format. AWS CloudFormation always converts a YAML policy to JSON format before submitting it.
    #[serde(rename = "Content")]
    pub content: (),
    /// Human readable description of the policy
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// List of unique identifiers (IDs) of the root, OU, or account that you want to attach the policy to
    #[serde(rename = "TargetIds")]
    pub target_ids: Option<Vec<String>>,
    /// A list of tags that you want to attach to the newly created policy. For each tag in the list, you must specify both a tag key and a value. You can set the value to an empty string, but you can't set it to null.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The type of policy to create. You can specify one of the following values: AISERVICES_OPT_OUT_POLICY, BACKUP_POLICY, SERVICE_CONTROL_POLICY, TAG_POLICY
    #[serde(rename = "Type")]
    pub ty: String,
    /// Name of the Policy
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_resource_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnResourcePolicy {
    /// The policy document. For AWS CloudFormation templates formatted in YAML, you can provide the policy in JSON or YAML format. AWS CloudFormation always converts a YAML policy to JSON format before submitting it.
    #[serde(rename = "Content")]
    pub content: (),
    /// A list of tags that you want to attach to the resource policy
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
