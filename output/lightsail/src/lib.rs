
pub mod cfn_alarm {

#[derive(serde::Serialize, Default)]
pub struct CfnAlarm {
    /// Sets how this alarm will handle missing data points.
    #[serde(rename = "TreatMissingData")]
    pub treat_missing_data: Option<String>,
    /// The number of most recent periods over which data is compared to the specified threshold. If you are setting an "M out of N" alarm, this value (evaluationPeriods) is the N.
    #[serde(rename = "EvaluationPeriods")]
    pub evaluation_periods: usize,
    /// The number of data points that must be not within the specified threshold to trigger the alarm. If you are setting an "M out of N" alarm, this value (datapointsToAlarm) is the M.
    #[serde(rename = "DatapointsToAlarm")]
    pub datapoints_to_alarm: Option<usize>,
    /// Indicates whether the alarm is enabled. Notifications are enabled by default if you don't specify this parameter.
    #[serde(rename = "NotificationEnabled")]
    pub notification_enabled: Option<bool>,
    /// The alarm states that trigger a notification.
    #[serde(rename = "NotificationTriggers")]
    pub notification_triggers: Option<Vec<String>>,
    /// The validation status of the SSL/TLS certificate.
    #[serde(rename = "MonitoredResourceName")]
    pub monitored_resource_name: String,
    /// The contact protocols to use for the alarm, such as Email, SMS (text messaging), or both.
    #[serde(rename = "ContactProtocols")]
    pub contact_protocols: Option<Vec<String>>,
    /// The name for the alarm. Specify the name of an existing alarm to update, and overwrite the previous configuration of the alarm.
    #[serde(rename = "AlarmName")]
    pub alarm_name: String,
    /// The arithmetic operation to use when comparing the specified statistic to the threshold. The specified statistic value is used as the first operand.
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,
    /// The value against which the specified statistic is compared.
    #[serde(rename = "Threshold")]
    pub threshold: f64,
    /// The name of the metric to associate with the alarm.
    #[serde(rename = "MetricName")]
    pub metric_name: String,

}



}

pub mod cfn_bucket {

#[derive(serde::Serialize, Default)]
pub struct CfnBucket {
    /// Specifies whether to enable or disable versioning of objects in the bucket.
    #[serde(rename = "ObjectVersioning")]
    pub object_versioning: Option<bool>,
    /// The names of the Lightsail resources for which to set bucket access.
    #[serde(rename = "ResourcesReceivingAccess")]
    pub resources_receiving_access: Option<Vec<String>>,
    /// An array of strings to specify the AWS account IDs that can access the bucket.
    #[serde(rename = "ReadOnlyAccessAccounts")]
    pub read_only_access_accounts: Option<Vec<String>>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The name for the bucket.
    #[serde(rename = "BucketName")]
    pub bucket_name: String,
    /// The ID of the bundle to use for the bucket.
    #[serde(rename = "BundleId")]
    pub bundle_id: String,
    /// An object that sets the public accessibility of objects in the specified bucket.
    #[serde(rename = "AccessRules")]
    pub access_rules: Option<AccessRules>,

}


#[derive(serde::Serialize, Default)]
pub struct AccessRules {
    #[serde(rename = "AllowPublicOverrides")]
    pub allow_public_overrides: Option<bool>,
    #[serde(rename = "GetObject")]
    pub get_object: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_certificate {

#[derive(serde::Serialize, Default)]
pub struct CfnCertificate {
    /// An array of strings that specify the alternate domains (e.g., example2.com) and subdomains (e.g., blog.example.com) for the certificate.
    #[serde(rename = "SubjectAlternativeNames")]
    pub subject_alternative_names: Option<Vec<String>>,
    /// The domain name (e.g., example.com ) for the certificate.
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// The name for the certificate.
    #[serde(rename = "CertificateName")]
    pub certificate_name: String,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_container {

#[derive(serde::Serialize, Default)]
pub struct CfnContainer {
    /// The power specification for the container service.
    #[serde(rename = "Power")]
    pub power: String,
    /// The scale specification for the container service.
    #[serde(rename = "Scale")]
    pub scale: usize,
    /// The public domain names to use with the container service, such as example.com and www.example.com.
    #[serde(rename = "PublicDomainNames")]
    pub public_domain_names: Option<Vec<PublicDomainName>>,
    /// A Boolean value to indicate whether the container service is disabled.
    #[serde(rename = "IsDisabled")]
    pub is_disabled: Option<bool>,
    /// The name for the container service.
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    /// Describes a container deployment configuration of an Amazon Lightsail container service.
    #[serde(rename = "ContainerServiceDeployment")]
    pub container_service_deployment: Option<ContainerServiceDeployment>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct HealthCheckConfig {
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "IntervalSeconds")]
    pub interval_seconds: Option<usize>,
    #[serde(rename = "TimeoutSeconds")]
    pub timeout_seconds: Option<usize>,
    #[serde(rename = "UnhealthyThreshold")]
    pub unhealthy_threshold: Option<usize>,
    #[serde(rename = "HealthyThreshold")]
    pub healthy_threshold: Option<usize>,
    #[serde(rename = "SuccessCodes")]
    pub success_codes: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EnvironmentVariable {
    #[serde(rename = "Variable")]
    pub variable: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ContainerServiceDeployment {
    #[serde(rename = "Containers")]
    pub containers: Option<Vec<Container>>,
    #[serde(rename = "PublicEndpoint")]
    pub public_endpoint: Option<PublicEndpoint>,

}

#[derive(serde::Serialize, Default)]
pub struct PublicEndpoint {
    #[serde(rename = "ContainerName")]
    pub container_name: Option<String>,
    #[serde(rename = "ContainerPort")]
    pub container_port: Option<usize>,
    #[serde(rename = "HealthCheckConfig")]
    pub health_check_config: Option<HealthCheckConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct Container {
    #[serde(rename = "Ports")]
    pub ports: Option<Vec<PortInfo>>,
    #[serde(rename = "Command")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "ContainerName")]
    pub container_name: Option<String>,
    #[serde(rename = "Environment")]
    pub environment: Option<Vec<EnvironmentVariable>>,
    #[serde(rename = "Image")]
    pub image: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PublicDomainName {
    #[serde(rename = "CertificateName")]
    pub certificate_name: Option<String>,
    #[serde(rename = "DomainNames")]
    pub domain_names: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct PortInfo {
    #[serde(rename = "Port")]
    pub port: Option<String>,
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,

}


}

pub mod cfn_database {

#[derive(serde::Serialize, Default)]
pub struct CfnDatabase {
    /// When true, the master user password is changed to a new strong password generated by Lightsail. Use the get relational database master user password operation to get the new password.
    #[serde(rename = "RotateMasterUserPassword")]
    pub rotate_master_user_password: Option<bool>,
    /// The password for the master user. The password can include any printable ASCII character except "/", """, or "@". It cannot contain spaces.
    #[serde(rename = "MasterUserPassword")]
    pub master_user_password: Option<String>,
    /// The Availability Zone in which to create your new database. Use the us-east-2a case-sensitive format.
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    /// Indicates the certificate that needs to be associated with the database.
    #[serde(rename = "CaCertificateIdentifier")]
    pub ca_certificate_identifier: Option<String>,
    /// The bundle ID for your new database. A bundle describes the performance specifications for your database.
    #[serde(rename = "RelationalDatabaseBundleId")]
    pub relational_database_bundle_id: String,
    /// The name of the database to create when the Lightsail database resource is created. For MySQL, if this parameter isn't specified, no database is created in the database resource. For PostgreSQL, if this parameter isn't specified, a database named postgres is created in the database resource.
    #[serde(rename = "MasterDatabaseName")]
    pub master_database_name: String,
    /// When true, enables automated backup retention for your database. Updates are applied during the next maintenance window because this can result in an outage.
    #[serde(rename = "BackupRetention")]
    pub backup_retention: Option<bool>,
    /// The blueprint ID for your new database. A blueprint describes the major engine version of a database.
    #[serde(rename = "RelationalDatabaseBlueprintId")]
    pub relational_database_blueprint_id: String,
    /// The name for the master user.
    #[serde(rename = "MasterUsername")]
    pub master_username: String,
    /// Specifies the accessibility options for your new database. A value of true specifies a database that is available to resources outside of your Lightsail account. A value of false specifies a database that is available only to your Lightsail resources in the same region as your database.
    #[serde(rename = "PubliclyAccessible")]
    pub publicly_accessible: Option<bool>,
    /// The daily time range during which automated backups are created for your new database if automated backups are enabled.
    #[serde(rename = "PreferredBackupWindow")]
    pub preferred_backup_window: Option<String>,
    /// The name to use for your new Lightsail database resource.
    #[serde(rename = "RelationalDatabaseName")]
    pub relational_database_name: String,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The weekly time range during which system maintenance can occur on your new database.
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// Update one or more parameters of the relational database.
    #[serde(rename = "RelationalDatabaseParameters")]
    pub relational_database_parameters: Option<Vec<RelationalDatabaseParameter>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct RelationalDatabaseParameter {
    #[serde(rename = "ApplyMethod")]
    pub apply_method: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "DataType")]
    pub data_type: Option<String>,
    #[serde(rename = "IsModifiable")]
    pub is_modifiable: Option<bool>,
    #[serde(rename = "ParameterValue")]
    pub parameter_value: Option<String>,
    #[serde(rename = "AllowedValues")]
    pub allowed_values: Option<String>,
    #[serde(rename = "ApplyType")]
    pub apply_type: Option<String>,
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,

}


}

pub mod cfn_disk {

#[derive(serde::Serialize, Default)]
pub struct CfnDisk {
    /// An array of objects representing the add-ons to enable for the new instance.
    #[serde(rename = "AddOns")]
    pub add_ons: Option<Vec<AddOn>>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Size of the Lightsail disk
    #[serde(rename = "SizeInGb")]
    pub size_in_gb: usize,
    /// The names to use for your new Lightsail disk.
    #[serde(rename = "DiskName")]
    pub disk_name: String,
    /// Location of a resource.
    #[serde(rename = "Location")]
    pub location: Option<Location>,

}


#[derive(serde::Serialize, Default)]
pub struct AddOn {
    #[serde(rename = "AutoSnapshotAddOnRequest")]
    pub auto_snapshot_add_on_request: Option<AutoSnapshotAddOn>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "AddOnType")]
    pub add_on_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct AutoSnapshotAddOn {
    #[serde(rename = "SnapshotTimeOfDay")]
    pub snapshot_time_of_day: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Location {
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    #[serde(rename = "RegionName")]
    pub region_name: Option<String>,

}


}

pub mod cfn_instance {

#[derive(serde::Serialize, Default)]
pub struct CfnInstance {
    /// Current State of the Instance.
    #[serde(rename = "State")]
    pub state: Option<State>,
    /// The names to use for your new Lightsail instance.
    #[serde(rename = "InstanceName")]
    pub instance_name: String,
    /// An array of objects representing the add-ons to enable for the new instance.
    #[serde(rename = "AddOns")]
    pub add_ons: Option<Vec<AddOn>>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The bundle of specification information for your virtual private server (or instance ), including the pricing plan (e.g., micro_1_0 ).
    #[serde(rename = "BundleId")]
    pub bundle_id: String,
    /// The ID for a virtual private server image (e.g., app_wordpress_4_4 or app_lamp_7_0 ). Use the get blueprints operation to return a list of available images (or blueprints ).
    #[serde(rename = "BlueprintId")]
    pub blueprint_id: String,
    /// Hardware of the Instance.
    #[serde(rename = "Hardware")]
    pub hardware: Option<Hardware>,
    /// Networking of the Instance.
    #[serde(rename = "Networking")]
    pub networking: Option<Networking>,
    /// Location of a resource.
    #[serde(rename = "Location")]
    pub location: Option<Location>,
    /// The name of your key pair.
    #[serde(rename = "KeyPairName")]
    pub key_pair_name: Option<String>,
    /// A launch script you can create that configures a server with additional user data. For example, you might want to run apt-get -y update.
    #[serde(rename = "UserData")]
    pub user_data: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct AddOn {
    #[serde(rename = "AddOnType")]
    pub add_on_type: String,
    #[serde(rename = "AutoSnapshotAddOnRequest")]
    pub auto_snapshot_add_on_request: Option<AutoSnapshotAddOn>,
    #[serde(rename = "Status")]
    pub status: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MonthlyTransfer {
    #[serde(rename = "GbPerMonthAllocated")]
    pub gb_per_month_allocated: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Port {
    #[serde(rename = "FromPort")]
    pub from_port: Option<usize>,
    #[serde(rename = "ToPort")]
    pub to_port: Option<usize>,
    #[serde(rename = "AccessDirection")]
    pub access_direction: Option<String>,
    #[serde(rename = "Ipv6Cidrs")]
    pub ipv6_cidrs: Option<ipv6Cidrs>,
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    #[serde(rename = "AccessFrom")]
    pub access_from: Option<String>,
    #[serde(rename = "CidrListAliases")]
    pub cidr_list_aliases: Option<cidrListAliases>,
    #[serde(rename = "AccessType")]
    pub access_type: Option<String>,
    #[serde(rename = "Cidrs")]
    pub cidrs: Option<cidrs>,
    #[serde(rename = "CommonName")]
    pub common_name: Option<String>,

}
pub type cidrs = Vec<String>;
#[derive(serde::Serialize, Default)]
pub struct Disk {
    #[serde(rename = "DiskName")]
    pub disk_name: String,
    #[serde(rename = "IsSystemDisk")]
    pub is_system_disk: Option<bool>,
    #[serde(rename = "SizeInGb")]
    pub size_in_gb: Option<String>,
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "IOPS")]
    pub iops: Option<usize>,
    #[serde(rename = "AttachedTo")]
    pub attached_to: Option<String>,
    #[serde(rename = "AttachmentState")]
    pub attachment_state: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Location {
    #[serde(rename = "RegionName")]
    pub region_name: Option<String>,
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,

}
pub type ipv6Cidrs = Vec<String>;
#[derive(serde::Serialize, Default)]
pub struct State {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Networking {
    #[serde(rename = "MonthlyTransfer")]
    pub monthly_transfer: Option<MonthlyTransfer>,
    #[serde(rename = "Ports")]
    pub ports: Vec<Port>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type cidrListAliases = Vec<String>;
#[derive(serde::Serialize, Default)]
pub struct AutoSnapshotAddOn {
    #[serde(rename = "SnapshotTimeOfDay")]
    pub snapshot_time_of_day: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Hardware {
    #[serde(rename = "RamSizeInGb")]
    pub ram_size_in_gb: Option<usize>,
    #[serde(rename = "CpuCount")]
    pub cpu_count: Option<usize>,
    #[serde(rename = "Disks")]
    pub disks: Option<Vec<Disk>>,

}


}

pub mod cfn_load_balancer {

#[derive(serde::Serialize, Default)]
pub struct CfnLoadBalancer {
    /// The name of the TLS policy to apply to the load balancer.
    #[serde(rename = "TlsPolicyName")]
    pub tls_policy_name: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The name of your load balancer.
    #[serde(rename = "LoadBalancerName")]
    pub load_balancer_name: String,
    /// The IP address type for the load balancer. The possible values are ipv4 for IPv4 only, and dualstack for IPv4 and IPv6. The default value is dualstack.
    #[serde(rename = "IpAddressType")]
    pub ip_address_type: Option<String>,
    /// The names of the instances attached to the load balancer.
    #[serde(rename = "AttachedInstances")]
    pub attached_instances: Option<Vec<String>>,
    /// Configuration option to enable session stickiness.
    #[serde(rename = "SessionStickinessEnabled")]
    pub session_stickiness_enabled: Option<bool>,
    /// Configuration option to adjust session stickiness cookie duration parameter.
    #[serde(rename = "SessionStickinessLBCookieDurationSeconds")]
    pub session_stickiness_lbcookie_duration_seconds: Option<String>,
    /// The path you provided to perform the load balancer health check. If you didn't specify a health check path, Lightsail uses the root path of your website (e.g., "/").
    #[serde(rename = "HealthCheckPath")]
    pub health_check_path: Option<String>,
    /// The instance port where you're creating your load balancer.
    #[serde(rename = "InstancePort")]
    pub instance_port: usize,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_load_balancer_tls_certificate {

#[derive(serde::Serialize, Default)]
pub struct CfnLoadBalancerTlsCertificate {
    /// When true, the SSL/TLS certificate is attached to the Lightsail load balancer.
    #[serde(rename = "IsAttached")]
    pub is_attached: Option<bool>,
    /// The domain name (e.g., example.com ) for your SSL/TLS certificate.
    #[serde(rename = "CertificateDomainName")]
    pub certificate_domain_name: String,
    /// The name of your load balancer.
    #[serde(rename = "LoadBalancerName")]
    pub load_balancer_name: String,
    /// A Boolean value that indicates whether HTTPS redirection is enabled for the load balancer.
    #[serde(rename = "HttpsRedirectionEnabled")]
    pub https_redirection_enabled: Option<bool>,
    /// An array of strings listing alternative domains and subdomains for your SSL/TLS certificate.
    #[serde(rename = "CertificateAlternativeNames")]
    pub certificate_alternative_names: Option<Vec<String>>,
    /// The SSL/TLS certificate name.
    #[serde(rename = "CertificateName")]
    pub certificate_name: String,

}



}

pub mod cfn_static_ip {

#[derive(serde::Serialize, Default)]
pub struct CfnStaticIp {
    /// The name of the static IP address.
    #[serde(rename = "StaticIpName")]
    pub static_ip_name: String,
    /// The instance where the static IP is attached.
    #[serde(rename = "AttachedTo")]
    pub attached_to: Option<String>,

}



}
