

/// The AWS::DataSync::StorageSystem resource creates an AWS resource for an on-premises storage system that you want DataSync Discovery to collect     information about. For more information, see discovering your storage with DataSync Discovery.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnStorageSystem {


    /// 
    /// Specifies the ARN of the Amazon CloudWatch log group for monitoring and logging    discovery job events.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 562
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):logs:[a-z\-0-9]+:[0-9]{12}:log-group:([^:\*]*)(:\*)?$
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogGroupArn")]
    pub cloud_watch_log_group_arn: Option<String>,


    /// 
    /// Specifies the user name and password for accessing your on-premises storage system's    management interface.
    /// 
    /// Required: No
    ///
    /// Type: ServerCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerCredentials")]
    pub server_credentials: Option<ServerCredentials>,


    /// 
    /// Specifies a familiar name for your on-premises storage system.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[\p{L}\p{M}\p{N}\s+=._:@\/-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// Specifies the Amazon Resource Name (ARN) of the DataSync agent that connects to    and reads from your on-premises storage system's management interface.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "AgentArns")]
    pub agent_arns: Vec<String>,


    /// 
    /// Specifies the server name and network port required to connect with the management    interface of your on-premises storage system.
    /// 
    /// Required: Yes
    ///
    /// Type: ServerConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerConfiguration")]
    pub server_configuration: ServerConfiguration,


    /// 
    /// Specifies the type of on-premises storage system that you want DataSync Discovery to collect    information about.
    /// 
    /// NoteDataSync Discovery currently supports NetApp     Fabric-Attached     Storage (FAS) and All Flash FAS (AFF) systems running ONTAP 9.7 or     later.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: NetAppONTAP
    ///
    /// Update requires: No interruption
    #[serde(rename = "SystemType")]
    pub system_type: StorageSystemSystemTypeEnum,


    /// 
    /// Specifies labels that help you categorize, filter, and search for your AWS    resources. We recommend creating at least a name tag for your on-premises storage    system.
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

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum StorageSystemSystemTypeEnum {

    /// NetAppONTAP
    #[serde(rename = "NetAppONTAP")]
    Netappontap,

}

impl Default for StorageSystemSystemTypeEnum {
    fn default() -> Self {
        StorageSystemSystemTypeEnum::Netappontap
    }
}


impl cfn_resources::CfnResource for CfnStorageSystem {
    fn type_string() -> &'static str {
        "AWS::DataSync::StorageSystem"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The credentials that provide DataSync Discovery read access to your on-premises storage system's    management interface.
///
/// DataSync Discovery stores these credentials in AWS Secrets Manager. For more    information, see Accessing your on-premises     storage system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServerCredentials {


    /// 
    /// Specifies the user name for your storage system's management interface.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^(?!.*[:\"][^:"]*$).+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    pub username: String,


    /// 
    /// Specifies the password for your storage system's management interface.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^(?!.*[:\"][^:"]*$).+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: String,

}




/// The network settings that DataSync Discovery uses to connect with your on-premises storage system's    management interface.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServerConfiguration {


    /// 
    /// The network port for accessing the storage system's management interface.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerPort")]
    pub server_port: Option<i64>,


    /// 
    /// The domain name or IP address of your storage system's management interface.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^(([a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9\-]*[A-Za-z0-9])$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerHostname")]
    pub server_hostname: String,

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


