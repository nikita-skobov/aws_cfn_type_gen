
pub mod cfn_server {

#[derive(serde::Serialize, Default)]
pub struct CfnServer {
    /// No documentation provided by AWS
    #[serde(rename = "PreferredBackupWindow")]
    pub preferred_backup_window: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EngineModel")]
    pub engine_model: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Engine")]
    pub engine: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DisableAutomatedBackup")]
    pub disable_automated_backup: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "CustomCertificate")]
    pub custom_certificate: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "BackupId")]
    pub backup_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "AssociatePublicIpAddress")]
    pub associate_public_ip_address: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceProfileArn")]
    pub instance_profile_arn: String,
    /// List of EngineAttribute
    #[serde(rename = "EngineAttributes")]
    pub engine_attributes: Option<Vec<EngineAttribute>>,
    /// No documentation provided by AWS
    #[serde(rename = "KeyPair")]
    pub key_pair: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "BackupRetentionCount")]
    pub backup_retention_count: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "CustomDomain")]
    pub custom_domain: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "CustomPrivateKey")]
    pub custom_private_key: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct EngineAttribute {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


}
