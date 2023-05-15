
pub mod cfn_environment_ec2 {

#[derive(serde::Serialize, Default)]
pub struct CfnEnvironmentEC2 {
    /// No documentation provided by AWS
    #[serde(rename = "AutomaticStopTimeMinutes")]
    pub automatic_stop_time_minutes: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "OwnerArn")]
    pub owner_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// List of Repository
    #[serde(rename = "Repositories")]
    pub repositories: Option<Vec<Repository>>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "ConnectionType")]
    pub connection_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ImageId")]
    pub image_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Repository {
    #[serde(rename = "PathComponent")]
    pub path_component: String,
    #[serde(rename = "RepositoryUrl")]
    pub repository_url: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
