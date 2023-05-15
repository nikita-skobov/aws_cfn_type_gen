
pub mod cfn_crl {

#[derive(serde::Serialize, Default)]
pub struct CfnCRL {
    /// No documentation provided by AWS
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "CrlData")]
    pub crl_data: String,
    /// No documentation provided by AWS
    #[serde(rename = "TrustAnchorArn")]
    pub trust_anchor_arn: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_profile {

#[derive(serde::Serialize, Default)]
pub struct CfnProfile {
    /// No documentation provided by AWS
    #[serde(rename = "ManagedPolicyArns")]
    pub managed_policy_arns: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "RoleArns")]
    pub role_arns: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RequireInstanceProperties")]
    pub require_instance_properties: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "SessionPolicy")]
    pub session_policy: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DurationSeconds")]
    pub duration_seconds: Option<f64>,
    /// List of Tag
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

pub mod cfn_trust_anchor {

#[derive(serde::Serialize, Default)]
pub struct CfnTrustAnchor {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Source")]
    pub source: Source,

}


#[derive(serde::Serialize, Default)]
pub struct Source {
    #[serde(rename = "SourceType")]
    pub source_type: Option<TrustAnchorType>,
    #[serde(rename = "SourceData")]
    pub source_data: Option<SourceData>,

}
pub type TrustAnchorType = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct SourceData {

}


}
