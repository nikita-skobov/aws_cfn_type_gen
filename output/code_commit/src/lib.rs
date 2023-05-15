
pub mod cfn_repository {

#[derive(serde::Serialize, Default)]
pub struct CfnRepository {
    /// No documentation provided by AWS
    #[serde(rename = "RepositoryName")]
    pub repository_name: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of RepositoryTrigger
    #[serde(rename = "Triggers")]
    pub triggers: Option<Vec<RepositoryTrigger>>,
    /// No documentation provided by AWS
    #[serde(rename = "RepositoryDescription")]
    pub repository_description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Code")]
    pub code: Option<Code>,

}


#[derive(serde::Serialize, Default)]
pub struct Code {
    #[serde(rename = "BranchName")]
    pub branch_name: Option<String>,
    #[serde(rename = "S3")]
    pub s3: S3,

}

#[derive(serde::Serialize, Default)]
pub struct S3 {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "ObjectVersion")]
    pub object_version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct RepositoryTrigger {
    #[serde(rename = "Events")]
    pub events: Vec<String>,
    #[serde(rename = "Branches")]
    pub branches: Option<Vec<String>>,
    #[serde(rename = "DestinationArn")]
    pub destination_arn: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CustomData")]
    pub custom_data: Option<String>,

}


}
