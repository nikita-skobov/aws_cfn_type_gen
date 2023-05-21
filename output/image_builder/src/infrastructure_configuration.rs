

/// The infrastructure configuration allows you to specify the infrastructure within which     to build and test your image. In the infrastructure configuration, you can specify instance     types, subnets, and security groups to associate with your instance. You can also associate     an Amazon EC2 key pair with the instance used to build your image. This allows you to log     on to your instance to troubleshoot if your build fails and you set     terminateInstanceOnFailure to false.
#[derive(Default, serde::Serialize)]
pub struct CfnInfrastructureConfiguration {


    /// 
    /// The logging configuration defines where Image Builder uploads your logs.
    /// 
    /// Required: No
    ///
    /// Type: Logging
    ///
    /// Update requires: No interruption
    #[serde(rename = "Logging")]
    pub logging: Option<Logging>,


    /// 
    /// The Amazon EC2 key pair of the infrastructure configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyPair")]
    pub key_pair: Option<String>,


    /// 
    /// The subnet ID of the infrastructure configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,


    /// 
    /// The name of the infrastructure configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[-_A-Za-z-0-9][-_A-Za-z0-9 ]{1,126}[-_A-Za-z-0-9]$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The tags attached to the resource created by Image Builder.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceTags")]
    pub resource_tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The description of the infrastructure configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The instance types of the infrastructure configuration.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceTypes")]
    pub instance_types: Option<Vec<String>>,


    /// 
    /// The instance metadata option settings for the infrastructure configuration.
    /// 
    /// Required: No
    ///
    /// Type: InstanceMetadataOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceMetadataOptions")]
    pub instance_metadata_options: Option<InstanceMetadataOptions>,


    /// 
    /// The terminate instance on failure configuration of the infrastructure 			configuration.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "TerminateInstanceOnFailure")]
    pub terminate_instance_on_failure: Option<bool>,


    /// 
    /// The security group IDs of the infrastructure configuration.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// The tags of the infrastructure configuration.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The Amazon Resource Name (ARN) of the SNS topic for the infrastructure 			configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: Option<String>,


    /// 
    /// The instance profile of the infrastructure configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[\w+=,.@-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceProfileName")]
    pub instance_profile_name: String,

}


/// Amazon S3 logging configuration.
#[derive(Default, serde::Serialize)]
pub struct S3Logs {


    /// 
    /// The S3 bucket in which to store the logs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: Option<String>,


    /// 
    /// The Amazon S3 path to the bucket where the logs are stored.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3KeyPrefix")]
    pub s3_key_prefix: Option<String>,

}


/// The instance metadata options that apply to the HTTP requests that pipeline builds use 			to launch EC2 build and test instances. For more information about instance metadata 			options, see Configure the instance metadata options in the 				        Amazon EC2 User Guide       for Linux instances, or Configure the instance metadata options in the 				        Amazon EC2 Windows Guide       for Windows instances.
#[derive(Default, serde::Serialize)]
pub struct InstanceMetadataOptions {


    /// 
    /// Limit the number of hops that an instance metadata request can traverse to reach its 			destination. The default is one hop. However, if HTTP tokens are required, container 			image builds need a minimum of two hops.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpPutResponseHopLimit")]
    pub http_put_response_hop_limit: Option<i64>,


    /// 
    /// Indicates whether a signed token header is required for instance metadata retrieval 			requests. The values affect the response as follows:
    /// 
    /// required – When you retrieve the IAM 					role credentials, version 2.0 credentials are returned in all cases.                        optional – You can include a signed 					token header in your request to retrieve instance metadata, or you can leave it 					out. If you include it, version 2.0 credentials are returned for the IAM role. 					Otherwise, version 1.0 credentials are returned.
    /// 
    /// The default setting is optional.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: optional|required
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpTokens")]
    pub http_tokens: Option<String>,

}


/// Logging configuration defines where Image Builder uploads your logs.
#[derive(Default, serde::Serialize)]
pub struct Logging {


    /// 
    /// The Amazon S3 logging configuration.
    /// 
    /// Required: No
    ///
    /// Type: S3Logs
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Logs")]
    pub s3_logs: Option<S3Logs>,

}