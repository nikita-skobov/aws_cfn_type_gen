
pub mod cfn_agreement {

#[derive(serde::Serialize, Default)]
pub struct CfnAgreement {
    /// A unique identifier for the server.
    #[serde(rename = "ServerId")]
    pub server_id: String,
    /// Specifies the base directory for the agreement.
    #[serde(rename = "BaseDirectory")]
    pub base_directory: String,
    /// A textual description for the agreement.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// A unique identifier for the partner profile.
    #[serde(rename = "PartnerProfileId")]
    pub partner_profile_id: String,
    /// A unique identifier for the local profile.
    #[serde(rename = "LocalProfileId")]
    pub local_profile_id: String,
    /// Specifies the access role for the agreement.
    #[serde(rename = "AccessRole")]
    pub access_role: String,
    /// Specifies the status of the agreement.
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// Key-value pairs that can be used to group and search for agreements. Tags are metadata attached to agreements for any purpose.
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

pub mod cfn_certificate {

#[derive(serde::Serialize, Default)]
pub struct CfnCertificate {
    /// Specifies the usage type for the certificate.
    #[serde(rename = "Usage")]
    pub usage: String,
    /// Specifies the private key for the certificate.
    #[serde(rename = "PrivateKey")]
    pub private_key: Option<String>,
    /// Specifies the certificate chain to be imported.
    #[serde(rename = "CertificateChain")]
    pub certificate_chain: Option<String>,
    /// Specifies the active date for the certificate.
    #[serde(rename = "ActiveDate")]
    pub active_date: Option<String>,
    /// Specifies the certificate body to be imported.
    #[serde(rename = "Certificate")]
    pub certificate: String,
    /// A textual description for the certificate.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Key-value pairs that can be used to group and search for certificates. Tags are metadata attached to certificates for any purpose.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Specifies the inactive date for the certificate.
    #[serde(rename = "InactiveDate")]
    pub inactive_date: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_connector {

#[derive(serde::Serialize, Default)]
pub struct CfnConnector {
    /// Configuration for an AS2 connector.
    #[serde(rename = "As2Config")]
    pub as2_config: (),
    /// Specifies the logging role for the connector.
    #[serde(rename = "LoggingRole")]
    pub logging_role: Option<String>,
    /// Specifies the access role for the connector.
    #[serde(rename = "AccessRole")]
    pub access_role: String,
    /// Key-value pairs that can be used to group and search for workflows. Tags are metadata attached to workflows for any purpose.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// URL for Connector
    #[serde(rename = "Url")]
    pub url: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_profile {

#[derive(serde::Serialize, Default)]
pub struct CfnProfile {
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Enum specifying whether the profile is local or associated with a trading partner.
    #[serde(rename = "ProfileType")]
    pub profile_type: String,
    /// AS2 identifier agreed with a trading partner.
    #[serde(rename = "As2Id")]
    pub as2_id: String,
    /// List of the certificate IDs associated with this profile to be used for encryption and signing of AS2 messages.
    #[serde(rename = "CertificateIds")]
    pub certificate_ids: Option<Vec<CertificateId>>,

}

pub type CertificateId = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_server {

#[derive(serde::Serialize, Default)]
pub struct CfnServer {
    /// No documentation provided by AWS
    #[serde(rename = "EndpointDetails")]
    pub endpoint_details: Option<EndpointDetails>,
    /// No documentation provided by AWS
    #[serde(rename = "IdentityProviderType")]
    pub identity_provider_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PostAuthenticationLoginBanner")]
    pub post_authentication_login_banner: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ProtocolDetails")]
    pub protocol_details: Option<ProtocolDetails>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of Protocol
    #[serde(rename = "Protocols")]
    pub protocols: Option<Vec<Protocol>>,
    /// No documentation provided by AWS
    #[serde(rename = "LoggingRole")]
    pub logging_role: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "IdentityProviderDetails")]
    pub identity_provider_details: Option<IdentityProviderDetails>,
    /// No documentation provided by AWS
    #[serde(rename = "PreAuthenticationLoginBanner")]
    pub pre_authentication_login_banner: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Domain")]
    pub domain: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "WorkflowDetails")]
    pub workflow_details: Option<WorkflowDetails>,
    /// No documentation provided by AWS
    #[serde(rename = "EndpointType")]
    pub endpoint_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityPolicyName")]
    pub security_policy_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Certificate")]
    pub certificate: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Protocol {

}

#[derive(serde::Serialize, Default)]
pub struct EndpointDetails {
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,
    #[serde(rename = "VpcEndpointId")]
    pub vpc_endpoint_id: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "AddressAllocationIds")]
    pub address_allocation_ids: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct WorkflowDetail {
    #[serde(rename = "WorkflowId")]
    pub workflow_id: String,
    #[serde(rename = "ExecutionRole")]
    pub execution_role: String,

}

#[derive(serde::Serialize, Default)]
pub struct WorkflowDetails {
    #[serde(rename = "OnUpload")]
    pub on_upload: Option<Vec<WorkflowDetail>>,
    #[serde(rename = "OnPartialUpload")]
    pub on_partial_upload: Option<Vec<WorkflowDetail>>,

}

#[derive(serde::Serialize, Default)]
pub struct As2Transport {

}

#[derive(serde::Serialize, Default)]
pub struct ProtocolDetails {
    #[serde(rename = "SetStatOption")]
    pub set_stat_option: Option<String>,
    #[serde(rename = "TlsSessionResumptionMode")]
    pub tls_session_resumption_mode: Option<String>,
    #[serde(rename = "PassiveIp")]
    pub passive_ip: Option<String>,
    #[serde(rename = "As2Transports")]
    pub as2_transports: Option<Vec<As2Transport>>,

}

#[derive(serde::Serialize, Default)]
pub struct IdentityProviderDetails {
    #[serde(rename = "Function")]
    pub function: Option<String>,
    #[serde(rename = "SftpAuthenticationMethods")]
    pub sftp_authentication_methods: Option<String>,
    #[serde(rename = "DirectoryId")]
    pub directory_id: Option<String>,
    #[serde(rename = "Url")]
    pub url: Option<String>,
    #[serde(rename = "InvocationRole")]
    pub invocation_role: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_user {

#[derive(serde::Serialize, Default)]
pub struct CfnUser {
    /// No documentation provided by AWS
    #[serde(rename = "HomeDirectoryType")]
    pub home_directory_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Role")]
    pub role: String,
    /// No documentation provided by AWS
    #[serde(rename = "PosixProfile")]
    pub posix_profile: Option<PosixProfile>,
    /// List of SshPublicKey
    #[serde(rename = "SshPublicKeys")]
    pub ssh_public_keys: Option<Vec<SshPublicKey>>,
    /// No documentation provided by AWS
    #[serde(rename = "ServerId")]
    pub server_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "UserName")]
    pub user_name: String,
    /// List of HomeDirectoryMapEntry
    #[serde(rename = "HomeDirectoryMappings")]
    pub home_directory_mappings: Option<Vec<HomeDirectoryMapEntry>>,
    /// No documentation provided by AWS
    #[serde(rename = "HomeDirectory")]
    pub home_directory: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Policy")]
    pub policy: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct SshPublicKey {

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct HomeDirectoryMapEntry {
    #[serde(rename = "Entry")]
    pub entry: String,
    #[serde(rename = "Target")]
    pub target: String,

}

#[derive(serde::Serialize, Default)]
pub struct PosixProfile {
    #[serde(rename = "Gid")]
    pub gid: f64,
    #[serde(rename = "SecondaryGids")]
    pub secondary_gids: Option<Vec<f64>>,
    #[serde(rename = "Uid")]
    pub uid: f64,

}


}

pub mod cfn_workflow {

#[derive(serde::Serialize, Default)]
pub struct CfnWorkflow {
    /// Key-value pairs that can be used to group and search for workflows. Tags are metadata attached to workflows for any purpose.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Specifies the details for the steps that are in the specified workflow.
    #[serde(rename = "Steps")]
    pub steps: Vec<WorkflowStep>,
    /// A textual description for the workflow.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Specifies the steps (actions) to take if any errors are encountered during execution of the workflow.
    #[serde(rename = "OnExceptionSteps")]
    pub on_exception_steps: Option<Vec<WorkflowStep>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct InputFileLocation {
    #[serde(rename = "EfsFileLocation")]
    pub efs_file_location: Option<EfsInputFileLocation>,
    #[serde(rename = "S3FileLocation")]
    pub s3_file_location: Option<S3InputFileLocation>,

}

#[derive(serde::Serialize, Default)]
pub struct S3InputFileLocation {
    #[serde(rename = "Bucket")]
    pub bucket: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EfsInputFileLocation {
    #[serde(rename = "FileSystemId")]
    pub file_system_id: Option<String>,
    #[serde(rename = "Path")]
    pub path: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct WorkflowStep {
    #[serde(rename = "DeleteStepDetails")]
    pub delete_step_details: Option<()>,
    #[serde(rename = "CopyStepDetails")]
    pub copy_step_details: Option<()>,
    #[serde(rename = "TagStepDetails")]
    pub tag_step_details: Option<()>,
    #[serde(rename = "DecryptStepDetails")]
    pub decrypt_step_details: Option<()>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "CustomStepDetails")]
    pub custom_step_details: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct S3FileLocation {
    #[serde(rename = "S3FileLocation")]
    pub s3_file_location: Option<S3InputFileLocation>,

}

#[derive(serde::Serialize, Default)]
pub struct S3Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}
