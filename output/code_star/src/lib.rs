
pub mod cfn_git_hub_repository {

#[derive(serde::Serialize, Default)]
pub struct CfnGitHubRepository {
    /// No documentation provided by AWS
    #[serde(rename = "EnableIssues")]
    pub enable_issues: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "RepositoryDescription")]
    pub repository_description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RepositoryAccessToken")]
    pub repository_access_token: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "IsPrivate")]
    pub is_private: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "RepositoryName")]
    pub repository_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RepositoryOwner")]
    pub repository_owner: String,
    /// No documentation provided by AWS
    #[serde(rename = "Code")]
    pub code: Option<Code>,

}


#[derive(serde::Serialize, Default)]
pub struct S3 {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "ObjectVersion")]
    pub object_version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Code {
    #[serde(rename = "S3")]
    pub s3: S3,

}


}
