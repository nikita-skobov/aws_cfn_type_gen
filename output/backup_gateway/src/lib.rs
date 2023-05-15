
pub mod cfn_hypervisor {

#[derive(serde::Serialize, Default)]
pub struct CfnHypervisor {
    /// No documentation provided by AWS
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Username")]
    pub username: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LogGroupArn")]
    pub log_group_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Password")]
    pub password: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Host")]
    pub host: Option<String>,

}

pub type HypervisorState = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type SyncMetadataStatus = String;

}
