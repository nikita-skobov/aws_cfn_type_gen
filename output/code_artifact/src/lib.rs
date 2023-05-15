
pub mod cfn_domain {

#[derive(serde::Serialize, Default)]
pub struct CfnDomain {
    /// The name of the domain.
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// The access control resource policy on the provided domain.
    #[serde(rename = "PermissionsPolicyDocument")]
    pub permissions_policy_document: Option<()>,
    /// An array of key-value pairs to apply to this resource.
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

pub mod cfn_repository {

#[derive(serde::Serialize, Default)]
pub struct CfnRepository {
    /// A text description of the repository.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The name of the repository.
    #[serde(rename = "RepositoryName")]
    pub repository_name: String,
    /// The access control resource policy on the provided repository.
    #[serde(rename = "PermissionsPolicyDocument")]
    pub permissions_policy_document: Option<()>,
    /// The name of the domain that contains the repository.
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// A list of external connections associated with the repository.
    #[serde(rename = "ExternalConnections")]
    pub external_connections: Option<Vec<String>>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// A list of upstream repositories associated with the repository.
    #[serde(rename = "Upstreams")]
    pub upstreams: Option<Vec<String>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
