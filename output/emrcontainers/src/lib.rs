
pub mod cfn_virtual_cluster {

#[derive(serde::Serialize, Default)]
pub struct CfnVirtualCluster {
    /// Name of the virtual cluster.
    #[serde(rename = "Name")]
    pub name: String,
    /// Container provider of the virtual cluster.
    #[serde(rename = "ContainerProvider")]
    pub container_provider: ContainerProvider,
    /// An array of key-value pairs to apply to this virtual cluster.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ContainerProvider {
    #[serde(rename = "Info")]
    pub info: ContainerInfo,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Id")]
    pub id: String,

}

#[derive(serde::Serialize, Default)]
pub struct ContainerInfo {
    #[serde(rename = "EksInfo")]
    pub eks_info: EksInfo,

}

#[derive(serde::Serialize, Default)]
pub struct EksInfo {
    #[serde(rename = "Namespace")]
    pub namespace: String,

}


}
