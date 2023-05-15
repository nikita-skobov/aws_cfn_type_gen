
pub mod cfn_profile_permission {

#[derive(serde::Serialize, Default)]
pub struct CfnProfilePermission {
    /// No documentation provided by AWS
    #[serde(rename = "StatementId")]
    pub statement_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ProfileVersion")]
    pub profile_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Action")]
    pub action: String,
    /// No documentation provided by AWS
    #[serde(rename = "ProfileName")]
    pub profile_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Principal")]
    pub principal: String,

}



}

pub mod cfn_signing_profile {

#[derive(serde::Serialize, Default)]
pub struct CfnSigningProfile {
    /// A list of tags associated with the signing profile.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The ID of the target signing platform.
    #[serde(rename = "PlatformId")]
    pub platform_id: PlatformId,
    /// Signature validity period of the profile.
    #[serde(rename = "SignatureValidityPeriod")]
    pub signature_validity_period: Option<SignatureValidityPeriod>,

}

pub type ProfileVersion = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}
pub type PlatformId = String;
#[derive(serde::Serialize, Default)]
pub struct SignatureValidityPeriod {
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<usize>,

}
pub type Arn = String;

}
