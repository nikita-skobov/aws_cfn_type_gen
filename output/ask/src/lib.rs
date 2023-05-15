
pub mod cfn_skill {

#[derive(serde::Serialize, Default)]
pub struct CfnSkill {
    /// No documentation provided by AWS
    #[serde(rename = "AuthenticationConfiguration")]
    pub authentication_configuration: AuthenticationConfiguration,
    /// No documentation provided by AWS
    #[serde(rename = "VendorId")]
    pub vendor_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "SkillPackage")]
    pub skill_package: SkillPackage,

}


#[derive(serde::Serialize, Default)]
pub struct AuthenticationConfiguration {
    #[serde(rename = "ClientSecret")]
    pub client_secret: String,
    #[serde(rename = "RefreshToken")]
    pub refresh_token: String,
    #[serde(rename = "ClientId")]
    pub client_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct Overrides {
    #[serde(rename = "Manifest")]
    pub manifest: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct SkillPackage {
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,
    #[serde(rename = "S3BucketRole")]
    pub s3_bucket_role: Option<String>,
    #[serde(rename = "S3Key")]
    pub s3_key: String,
    #[serde(rename = "S3ObjectVersion")]
    pub s3_object_version: Option<String>,
    #[serde(rename = "Overrides")]
    pub overrides: Option<Overrides>,

}


}
