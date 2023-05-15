
pub mod cfn_repository_association {

#[derive(serde::Serialize, Default)]
pub struct CfnRepositoryAssociation {
    /// The Amazon Resource Name (ARN) of an AWS CodeStar Connections connection.
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: Option<String>,
    /// The tags associated with a repository association.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Name of the repository to be associated.
    #[serde(rename = "Name")]
    pub name: String,
    /// The type of repository to be associated.
    #[serde(rename = "Type")]
    pub ty: String,
    /// The name of the S3 bucket associated with an associated S3 repository. It must start with `codeguru-reviewer-`.
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<String>,
    /// The owner of the repository. For a Bitbucket repository, this is the username for the account that owns the repository.
    #[serde(rename = "Owner")]
    pub owner: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
