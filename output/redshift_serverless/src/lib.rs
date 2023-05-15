
pub mod cfn_namespace {

#[derive(serde::Serialize, Default)]
pub struct CfnNamespace {
    /// The name of the namespace the source snapshot was created from. Please specify the name if needed before deleting namespace
    #[serde(rename = "FinalSnapshotName")]
    pub final_snapshot_name: Option<String>,
    /// The list of tags for the namespace.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The password associated with the admin user for the namespace that is being created. Password must be at least 8 characters in length, should be any printable ASCII character. Must contain at least one lowercase letter, one uppercase letter and one decimal digit.
    #[serde(rename = "AdminUserPassword")]
    pub admin_user_password: Option<String>,
    /// The number of days to retain automated snapshot in the destination region after they are copied from the source region. If the value is -1, the manual snapshot is retained indefinitely. The value must be either -1 or an integer between 1 and 3,653.
    #[serde(rename = "FinalSnapshotRetentionPeriod")]
    pub final_snapshot_retention_period: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type LogExport = String;
#[derive(serde::Serialize, Default)]
pub struct Namespace {
    #[serde(rename = "AdminUsername")]
    pub admin_username: Option<String>,
    #[serde(rename = "LogExports")]
    pub log_exports: Option<Vec<LogExport>>,
    #[serde(rename = "NamespaceArn")]
    pub namespace_arn: Option<String>,
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,
    #[serde(rename = "NamespaceName")]
    pub namespace_name: Option<String>,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "DefaultIamRoleArn")]
    pub default_iam_role_arn: Option<String>,
    #[serde(rename = "IamRoles")]
    pub iam_roles: Option<Vec<String>>,
    #[serde(rename = "NamespaceId")]
    pub namespace_id: Option<String>,
    #[serde(rename = "DbName")]
    pub db_name: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<NamespaceStatus>,

}
pub type NamespaceStatus = String;

}

pub mod cfn_workgroup {

#[derive(serde::Serialize, Default)]
pub struct CfnWorkgroup {
    /// List of ConfigParameter
    #[serde(rename = "ConfigParameters")]
    pub config_parameters: Option<Vec<ConfigParameter>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct VpcEndpoint {
    #[serde(rename = "NetworkInterfaces")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    #[serde(rename = "VpcEndpointId")]
    pub vpc_endpoint_id: Option<String>,
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,

}
pub type WorkgroupStatus = String;
#[derive(serde::Serialize, Default)]
pub struct Endpoint {
    #[serde(rename = "Address")]
    pub address: Option<String>,
    #[serde(rename = "VpcEndpoints")]
    pub vpc_endpoints: Option<Vec<VpcEndpoint>>,
    #[serde(rename = "Port")]
    pub port: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Workgroup {
    #[serde(rename = "WorkgroupArn")]
    pub workgroup_arn: Option<String>,
    #[serde(rename = "WorkgroupId")]
    pub workgroup_id: Option<String>,
    #[serde(rename = "WorkgroupName")]
    pub workgroup_name: Option<String>,
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "PubliclyAccessible")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "Endpoint")]
    pub endpoint: Option<Endpoint>,
    #[serde(rename = "EnhancedVpcRouting")]
    pub enhanced_vpc_routing: Option<bool>,
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,
    #[serde(rename = "BaseCapacity")]
    pub base_capacity: Option<usize>,
    #[serde(rename = "Status")]
    pub status: Option<WorkgroupStatus>,
    #[serde(rename = "NamespaceName")]
    pub namespace_name: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "ConfigParameters")]
    pub config_parameters: Option<Vec<ConfigParameter>>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkInterface {
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ConfigParameter {
    #[serde(rename = "ParameterKey")]
    pub parameter_key: Option<String>,
    #[serde(rename = "ParameterValue")]
    pub parameter_value: Option<String>,

}


}
