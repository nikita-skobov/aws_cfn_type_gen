
pub mod cfn_permission {

#[derive(serde::Serialize, Default)]
pub struct CfnPermission {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Policy template for the permission.
    #[serde(rename = "PolicyTemplate")]
    pub policy_template: (),
    /// The resource type this permission can be used with.
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// The name of the permission.
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

pub mod cfn_resource_share {

#[derive(serde::Serialize, Default)]
pub struct CfnResourceShare {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "AllowExternalPrincipals")]
    pub allow_external_principals: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "PermissionArns")]
    pub permission_arns: Option<Vec<String>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Principals")]
    pub principals: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceArns")]
    pub resource_arns: Option<Vec<String>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}
