
pub mod cfn_connection {

#[derive(serde::Serialize, Default)]
pub struct CfnConnection {
    /// The host arn configured to represent the infrastructure where your third-party provider is installed. You must specify either a ProviderType or a HostArn.
    #[serde(rename = "HostArn")]
    pub host_arn: Option<String>,
    /// The name of the external provider where your third-party code repository is configured. You must specify either a ProviderType or a HostArn.
    #[serde(rename = "ProviderType")]
    pub provider_type: Option<String>,
    /// Specifies the tags applied to a connection.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The name of the connection. Connection names must be unique in an AWS user account.
    #[serde(rename = "ConnectionName")]
    pub connection_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
