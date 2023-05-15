
pub mod cfn_agent {

#[derive(serde::Serialize, Default)]
pub struct CfnAgent {
    /// The ARNs of the security group used to protect your data transfer task subnets.
    #[serde(rename = "SecurityGroupArns")]
    pub security_group_arns: Option<Vec<String>>,
    /// The ARNs of the subnets in which DataSync will create elastic network interfaces for each data transfer task.
    #[serde(rename = "SubnetArns")]
    pub subnet_arns: Option<Vec<String>>,
    /// The name configured for the agent. Text reference used to identify the agent in the console.
    #[serde(rename = "AgentName")]
    pub agent_name: Option<String>,
    /// The ID of the VPC endpoint that the agent has access to.
    #[serde(rename = "VpcEndpointId")]
    pub vpc_endpoint_id: Option<String>,
    /// Activation key of the Agent.
    #[serde(rename = "ActivationKey")]
    pub activation_key: Option<String>,
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

pub mod cfn_location_efs {

#[derive(serde::Serialize, Default)]
pub struct CfnLocationEFS {
    /// The Amazon Resource Name (ARN) for the Amazon EFS file system.
    #[serde(rename = "EfsFilesystemArn")]
    pub efs_filesystem_arn: Option<String>,
    /// The Amazon Resource Name (ARN) of the AWS IAM role that the DataSync will assume when mounting the EFS file system.
    #[serde(rename = "FileSystemAccessRoleArn")]
    pub file_system_access_role_arn: Option<String>,
    /// Protocol that is used for encrypting the traffic exchanged between the DataSync Agent and the EFS file system.
    #[serde(rename = "InTransitEncryption")]
    pub in_transit_encryption: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The subnet and security group that DataSync uses to access target EFS file system.
    #[serde(rename = "Ec2Config")]
    pub ec2_config: Ec2Config,
    /// A subdirectory in the location's path. This subdirectory in the EFS file system is used to read data from the EFS source location or write data to the EFS destination.
    #[serde(rename = "Subdirectory")]
    pub subdirectory: Option<String>,
    /// The Amazon Resource Name (ARN) for the Amazon EFS Access point that DataSync uses when accessing the EFS file system.
    #[serde(rename = "AccessPointArn")]
    pub access_point_arn: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Ec2Config {
    #[serde(rename = "SecurityGroupArns")]
    pub security_group_arns: Vec<String>,
    #[serde(rename = "SubnetArn")]
    pub subnet_arn: String,

}


}

pub mod cfn_location_fsx_lustre {

#[derive(serde::Serialize, Default)]
pub struct CfnLocationFSxLustre {
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// A subdirectory in the location's path.
    #[serde(rename = "Subdirectory")]
    pub subdirectory: Option<String>,
    /// The Amazon Resource Name (ARN) for the FSx for Lustre file system.
    #[serde(rename = "FsxFilesystemArn")]
    pub fsx_filesystem_arn: Option<String>,
    /// The ARNs of the security groups that are to use to configure the FSx for Lustre file system.
    #[serde(rename = "SecurityGroupArns")]
    pub security_group_arns: Vec<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_location_fsx_ontap {

#[derive(serde::Serialize, Default)]
pub struct CfnLocationFSxONTAP {
    /// The Amazon Resource Name (ARN) for the FSx ONTAP SVM.
    #[serde(rename = "StorageVirtualMachineArn")]
    pub storage_virtual_machine_arn: String,
    /// The ARNs of the security groups that are to use to configure the FSx ONTAP file system.
    #[serde(rename = "SecurityGroupArns")]
    pub security_group_arns: Vec<String>,
    /// A subdirectory in the location's path.
    #[serde(rename = "Subdirectory")]
    pub subdirectory: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Configuration settings for NFS or SMB protocol.
    #[serde(rename = "Protocol")]
    pub protocol: Option<Protocol>,

}


#[derive(serde::Serialize, Default)]
pub struct SMB {
    #[serde(rename = "MountOptions")]
    pub mount_options: SmbMountOptions,
    #[serde(rename = "Domain")]
    pub domain: Option<String>,
    #[serde(rename = "User")]
    pub user: String,
    #[serde(rename = "Password")]
    pub password: String,

}

#[derive(serde::Serialize, Default)]
pub struct NFS {
    #[serde(rename = "MountOptions")]
    pub mount_options: NfsMountOptions,

}

#[derive(serde::Serialize, Default)]
pub struct NfsMountOptions {
    #[serde(rename = "Version")]
    pub version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Protocol {
    #[serde(rename = "SMB")]
    pub smb: Option<SMB>,
    #[serde(rename = "NFS")]
    pub nfs: Option<NFS>,

}

#[derive(serde::Serialize, Default)]
pub struct SmbMountOptions {
    #[serde(rename = "Version")]
    pub version: Option<String>,

}


}

pub mod cfn_location_fsx_open_zfs {

#[derive(serde::Serialize, Default)]
pub struct CfnLocationFSxOpenZFS {
    /// Configuration settings for an NFS or SMB protocol, currently only support NFS
    #[serde(rename = "Protocol")]
    pub protocol: Protocol,
    /// A subdirectory in the location's path.
    #[serde(rename = "Subdirectory")]
    pub subdirectory: Option<String>,
    /// The Amazon Resource Name (ARN) for the FSx OpenZFS file system.
    #[serde(rename = "FsxFilesystemArn")]
    pub fsx_filesystem_arn: Option<String>,
    /// The ARNs of the security groups that are to use to configure the FSx OpenZFS file system.
    #[serde(rename = "SecurityGroupArns")]
    pub security_group_arns: Vec<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct NFS {
    #[serde(rename = "MountOptions")]
    pub mount_options: MountOptions,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct MountOptions {
    #[serde(rename = "Version")]
    pub version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Protocol {
    #[serde(rename = "NFS")]
    pub nfs: Option<NFS>,

}


}

pub mod cfn_location_fsx_windows {

#[derive(serde::Serialize, Default)]
pub struct CfnLocationFSxWindows {
    /// The ARNs of the security groups that are to use to configure the FSx for Windows file system.
    #[serde(rename = "SecurityGroupArns")]
    pub security_group_arns: Vec<String>,
    /// A subdirectory in the location's path.
    #[serde(rename = "Subdirectory")]
    pub subdirectory: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The password of the user who has the permissions to access files and folders in the FSx for Windows file system.
    #[serde(rename = "Password")]
    pub password: Option<String>,
    /// The user who has the permissions to access files and folders in the FSx for Windows file system.
    #[serde(rename = "User")]
    pub user: String,
    /// The name of the Windows domain that the FSx for Windows server belongs to.
    #[serde(rename = "Domain")]
    pub domain: Option<String>,
    /// The Amazon Resource Name (ARN) for the FSx for Windows file system.
    #[serde(rename = "FsxFilesystemArn")]
    pub fsx_filesystem_arn: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_location_hdfs {

#[derive(serde::Serialize, Default)]
pub struct CfnLocationHDFS {
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The subdirectory in HDFS that is used to read data from the HDFS source location or write data to the HDFS destination.
    #[serde(rename = "Subdirectory")]
    pub subdirectory: Option<String>,
    /// The user name that has read and write permissions on the specified HDFS cluster.
    #[serde(rename = "SimpleUser")]
    pub simple_user: Option<String>,
    /// Size of chunks (blocks) in bytes that the data is divided into when stored in the HDFS cluster.
    #[serde(rename = "BlockSize")]
    pub block_size: Option<usize>,
    /// An array of Name Node(s) of the HDFS location.
    #[serde(rename = "NameNodes")]
    pub name_nodes: Vec<NameNode>,
    /// The Base64 string representation of the Keytab file.
    #[serde(rename = "KerberosKeytab")]
    pub kerberos_keytab: Option<String>,
    /// ARN(s) of the agent(s) to use for an HDFS location.
    #[serde(rename = "AgentArns")]
    pub agent_arns: Vec<String>,
    /// Number of copies of each block that exists inside the HDFS cluster.
    #[serde(rename = "ReplicationFactor")]
    pub replication_factor: Option<usize>,
    /// The unique identity, or principal, to which Kerberos can assign tickets.
    #[serde(rename = "KerberosPrincipal")]
    pub kerberos_principal: Option<String>,
    /// The identifier for the Key Management Server where the encryption keys that encrypt data inside HDFS clusters are stored.
    #[serde(rename = "KmsKeyProviderUri")]
    pub kms_key_provider_uri: Option<String>,
    /// Configuration information for RPC Protection and Data Transfer Protection. These parameters can be set to AUTHENTICATION, INTEGRITY, or PRIVACY. The default value is PRIVACY.
    #[serde(rename = "QopConfiguration")]
    pub qop_configuration: Option<QopConfiguration>,
    /// The authentication mode used to determine identity of user.
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: String,
    /// The string representation of the Krb5Conf file, or the presigned URL to access the Krb5.conf file within an S3 bucket.
    #[serde(rename = "KerberosKrb5Conf")]
    pub kerberos_krb5_conf: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct NameNode {
    #[serde(rename = "Port")]
    pub port: usize,
    #[serde(rename = "Hostname")]
    pub hostname: String,

}

#[derive(serde::Serialize, Default)]
pub struct QopConfiguration {
    #[serde(rename = "RpcProtection")]
    pub rpc_protection: Option<String>,
    #[serde(rename = "DataTransferProtection")]
    pub data_transfer_protection: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_location_nfs {

#[derive(serde::Serialize, Default)]
pub struct CfnLocationNFS {
    /// The subdirectory in the NFS file system that is used to read data from the NFS source location or write data to the NFS destination.
    #[serde(rename = "Subdirectory")]
    pub subdirectory: Option<String>,
    /// Contains a list of Amazon Resource Names (ARNs) of agents that are used to connect an NFS server.
    #[serde(rename = "OnPremConfig")]
    pub on_prem_config: OnPremConfig,
    /// The NFS mount options that DataSync can use to mount your NFS share.
    #[serde(rename = "MountOptions")]
    pub mount_options: Option<MountOptions>,
    /// The name of the NFS server. This value is the IP address or DNS name of the NFS server.
    #[serde(rename = "ServerHostname")]
    pub server_hostname: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct MountOptions {
    #[serde(rename = "Version")]
    pub version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct OnPremConfig {
    #[serde(rename = "AgentArns")]
    pub agent_arns: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_location_object_storage {

#[derive(serde::Serialize, Default)]
pub struct CfnLocationObjectStorage {
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The protocol that the object storage server uses to communicate.
    #[serde(rename = "ServerProtocol")]
    pub server_protocol: Option<String>,
    /// The name of the self-managed object storage server. This value is the IP address or Domain Name Service (DNS) name of the object storage server.
    #[serde(rename = "ServerHostname")]
    pub server_hostname: Option<String>,
    /// Optional. The secret key is used if credentials are required to access the self-managed object storage server.
    #[serde(rename = "SecretKey")]
    pub secret_key: Option<String>,
    /// The name of the bucket on the self-managed object storage server.
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<String>,
    /// X.509 PEM content containing a certificate authority or chain to trust.
    #[serde(rename = "ServerCertificate")]
    pub server_certificate: Option<String>,
    /// The Amazon Resource Name (ARN) of the agents associated with the self-managed object storage server location.
    #[serde(rename = "AgentArns")]
    pub agent_arns: Vec<String>,
    /// The subdirectory in the self-managed object storage server that is used to read data from.
    #[serde(rename = "Subdirectory")]
    pub subdirectory: Option<String>,
    /// The port that your self-managed server accepts inbound network traffic on.
    #[serde(rename = "ServerPort")]
    pub server_port: Option<usize>,
    /// Optional. The access key is used if credentials are required to access the self-managed object storage server.
    #[serde(rename = "AccessKey")]
    pub access_key: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_location_s3 {

#[derive(serde::Serialize, Default)]
pub struct CfnLocationS3 {
    /// A subdirectory in the Amazon S3 bucket. This subdirectory in Amazon S3 is used to read data from the S3 source location or write data to the S3 destination.
    #[serde(rename = "Subdirectory")]
    pub subdirectory: Option<String>,
    /// The Amazon Resource Name (ARN) of the AWS IAM role that is used to access an Amazon S3 bucket.
    #[serde(rename = "S3Config")]
    pub s3_config: S3Config,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The Amazon S3 storage class you want to store your files in when this location is used as a task destination.
    #[serde(rename = "S3StorageClass")]
    pub s3_storage_class: Option<String>,
    /// The Amazon Resource Name (ARN) of the Amazon S3 bucket.
    #[serde(rename = "S3BucketArn")]
    pub s3_bucket_arn: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct S3Config {
    #[serde(rename = "BucketAccessRoleArn")]
    pub bucket_access_role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_location_smb {

#[derive(serde::Serialize, Default)]
pub struct CfnLocationSMB {
    /// The name of the Windows domain that the SMB server belongs to.
    #[serde(rename = "Domain")]
    pub domain: Option<String>,
    /// The Amazon Resource Names (ARNs) of agents to use for a Simple Message Block (SMB) location.
    #[serde(rename = "AgentArns")]
    pub agent_arns: Vec<String>,
    /// The name of the SMB server. This value is the IP address or Domain Name Service (DNS) name of the SMB server.
    #[serde(rename = "ServerHostname")]
    pub server_hostname: Option<String>,
    /// The user who can mount the share, has the permissions to access files and folders in the SMB share.
    #[serde(rename = "User")]
    pub user: String,
    /// The subdirectory in the SMB file system that is used to read data from the SMB source location or write data to the SMB destination
    #[serde(rename = "Subdirectory")]
    pub subdirectory: Option<String>,
    /// The mount options used by DataSync to access the SMB server.
    #[serde(rename = "MountOptions")]
    pub mount_options: Option<MountOptions>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The password of the user who can mount the share and has the permissions to access files and folders in the SMB share.
    #[serde(rename = "Password")]
    pub password: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct MountOptions {
    #[serde(rename = "Version")]
    pub version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_storage_system {

#[derive(serde::Serialize, Default)]
pub struct CfnStorageSystem {
    /// The username and password for accessing your on-premises storage system's management interface.
    #[serde(rename = "ServerCredentials")]
    pub server_credentials: Option<ServerCredentials>,
    /// The server name and network port required to connect with the management interface of the on-premises storage system.
    #[serde(rename = "ServerConfiguration")]
    pub server_configuration: ServerConfiguration,
    /// The ARN of the Amazon CloudWatch log group used to monitor and log discovery job events.
    #[serde(rename = "CloudWatchLogGroupArn")]
    pub cloud_watch_log_group_arn: Option<String>,
    /// The ARN of the DataSync agent that connects to and reads from the on-premises storage system's management interface.
    #[serde(rename = "AgentArns")]
    pub agent_arns: Vec<String>,
    /// The type of on-premises storage system that DataSync Discovery will analyze.
    #[serde(rename = "SystemType")]
    pub system_type: String,
    /// A familiar name for the on-premises storage system.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct ServerConfiguration {
    #[serde(rename = "ServerPort")]
    pub server_port: Option<usize>,
    #[serde(rename = "ServerHostname")]
    pub server_hostname: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ServerCredentials {
    #[serde(rename = "Username")]
    pub username: String,
    #[serde(rename = "Password")]
    pub password: String,

}


}

pub mod cfn_task {

#[derive(serde::Serialize, Default)]
pub struct CfnTask {
    /// The ARN of the source location for the task.
    #[serde(rename = "SourceLocationArn")]
    pub source_location_arn: String,
    /// List of FilterRule
    #[serde(rename = "Includes")]
    pub includes: Option<Vec<FilterRule>>,
    /// Represents the options that are available to control the behavior of a StartTaskExecution operation.
    #[serde(rename = "Options")]
    pub options: Option<Options>,
    /// No documentation provided by AWS
    #[serde(rename = "Schedule")]
    pub schedule: Option<TaskSchedule>,
    /// The ARN of an AWS storage resource's location.
    #[serde(rename = "DestinationLocationArn")]
    pub destination_location_arn: String,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The ARN of the Amazon CloudWatch log group that is used to monitor and log events in the task.
    #[serde(rename = "CloudWatchLogGroupArn")]
    pub cloud_watch_log_group_arn: Option<String>,
    /// The name of a task. This value is a text reference that is used to identify the task in the console.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// List of FilterRule
    #[serde(rename = "Excludes")]
    pub excludes: Option<Vec<FilterRule>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type DestinationNetworkInterfaceArns = Vec<String>;pub type SourceNetworkInterfaceArns = Vec<String>;
#[derive(serde::Serialize, Default)]
pub struct FilterRule {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "FilterType")]
    pub filter_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TaskSchedule {
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: String,

}

#[derive(serde::Serialize, Default)]
pub struct Options {
    #[serde(rename = "PreserveDevices")]
    pub preserve_devices: Option<String>,
    #[serde(rename = "Atime")]
    pub atime: Option<String>,
    #[serde(rename = "ObjectTags")]
    pub object_tags: Option<String>,
    #[serde(rename = "Uid")]
    pub uid: Option<String>,
    #[serde(rename = "Gid")]
    pub gid: Option<String>,
    #[serde(rename = "LogLevel")]
    pub log_level: Option<String>,
    #[serde(rename = "BytesPerSecond")]
    pub bytes_per_second: Option<usize>,
    #[serde(rename = "PreserveDeletedFiles")]
    pub preserve_deleted_files: Option<String>,
    #[serde(rename = "PosixPermissions")]
    pub posix_permissions: Option<String>,
    #[serde(rename = "SecurityDescriptorCopyFlags")]
    pub security_descriptor_copy_flags: Option<String>,
    #[serde(rename = "TaskQueueing")]
    pub task_queueing: Option<String>,
    #[serde(rename = "Mtime")]
    pub mtime: Option<String>,
    #[serde(rename = "VerifyMode")]
    pub verify_mode: Option<String>,
    #[serde(rename = "OverwriteMode")]
    pub overwrite_mode: Option<String>,
    #[serde(rename = "TransferMode")]
    pub transfer_mode: Option<String>,

}


}
