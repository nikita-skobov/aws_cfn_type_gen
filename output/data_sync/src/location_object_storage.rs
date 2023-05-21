

/// The AWS::DataSync::LocationObjectStorage resource specifies an endpoint for     a self-managed object storage bucket. For more information about self-managed object     storage locations, see Creating a Location for       Object Storage.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLocationObjectStorage {


    /// 
    /// Specifies a file with the certificates that are used to sign the object storage server's    certificate (for example, file:///home/user/.ssh/storage_sys_certificate.pem).    The file you specify must include the following:
    /// 
    /// The certificate of the signing certificate authority (CA)               Any intermediate certificates               base64 encoding               A .pem extension
    /// 
    /// The file can be up to 32768 bytes (before base64 encoding).
    /// 
    /// To use this parameter, configure ServerProtocol to HTTPS.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerCertificate")]
    pub server_certificate: Option<String>,


    /// 
    /// Specifies the object prefix for your object storage server. If this is a source location,     DataSync only copies objects with this prefix. If this is a destination location,     DataSync writes all objects with this prefix.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: ^[a-zA-Z0-9_\-\+\./\(\)\p{Zs}]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subdirectory")]
    pub subdirectory: Option<String>,


    /// 
    /// Specifies the key-value pair that represents a tag that you want to add to the resource.    Tags can help you manage, filter, and search for your resources. We recommend creating a name    tag for your location.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Specifies the protocol that your object storage server uses to communicate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HTTP | HTTPS
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerProtocol")]
    pub server_protocol: Option<LocationObjectStorageServerProtocolEnum>,


    /// 
    /// Specifies the domain name or IP address of the object storage server. A DataSync    agent uses this hostname to mount the object storage server in a network.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^(([a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9\-]*[A-Za-z0-9])$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServerHostname")]
    pub server_hostname: Option<String>,


    /// 
    /// Specifies the access key (for example, a user name) if credentials are required to    authenticate with the object storage server.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Pattern: ^.+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessKey")]
    pub access_key: Option<String>,


    /// 
    /// Specifies the port that your object storage server accepts inbound network traffic on (for    example, port 443).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65536
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerPort")]
    pub server_port: Option<i64>,


    /// 
    /// Specifies the secret key (for example, a password) if credentials are required to    authenticate with the object storage server.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Pattern: ^.+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretKey")]
    pub secret_key: Option<String>,


    /// 
    /// Specifies the name of the object storage bucket involved in the transfer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9_\-\+\./\(\)\$\p{Zs}]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<String>,


    /// 
    /// Specifies the Amazon Resource Names (ARNs) of the DataSync agents that can    securely connect with your location.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 4
    ///
    /// Update requires: No interruption
    #[serde(rename = "AgentArns")]
    pub agent_arns: Vec<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum LocationObjectStorageServerProtocolEnum {

    /// HTTP
    #[serde(rename = "HTTP")]
    Http,

    /// HTTPS
    #[serde(rename = "HTTPS")]
    Https,

}

impl Default for LocationObjectStorageServerProtocolEnum {
    fn default() -> Self {
        LocationObjectStorageServerProtocolEnum::Http
    }
}


impl cfn_resources::CfnResource for CfnLocationObjectStorage {
    fn type_string() -> &'static str {
        "AWS::DataSync::LocationObjectStorage"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


