

/// The AWS::Glue::DevEndpoint resource specifies a development endpoint       where a developer can remotely debug ETL scripts for AWS Glue. For more information, see         DevEndpoint Structure in the AWS Glue Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnDevEndpoint {


    /// 
    /// A list of public keys to be used by the DevEndpoints for authentication.    Using this attribute is preferred over a single public key because the public keys allow you    to have a different private key per client.
    /// 
    /// NoteIf you previously created an endpoint with a public key, you must remove that key to be     able to set a list of public keys. Call the UpdateDevEndpoint API operation     with the public key content in the deletePublicKeys attribute, and the list of     new keys in the addPublicKeys attribute.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublicKeys")]
    pub public_keys: Option<Vec<String>>,


    /// 
    /// The tags to use with this DevEndpoint.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<serde_json::Value>,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role used in this       DevEndpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: arn:aws:iam::\d{12}:role/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The subnet ID for this DevEndpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,


    /// 
    /// A map of arguments used to configure the DevEndpoint.
    /// 
    /// Valid arguments are:
    /// 
    /// "--enable-glue-datacatalog": """GLUE_PYTHON_VERSION": "3""GLUE_PYTHON_VERSION": "2"
    /// 
    /// You can specify a version of Python support for development endpoints by using the Arguments parameter in the CreateDevEndpoint or UpdateDevEndpoint APIs. If no arguments are provided, the version defaults to Python 2.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arguments")]
    pub arguments: Option<serde_json::Value>,


    /// 
    /// The name of the DevEndpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndpointName")]
    pub endpoint_name: Option<String>,


    /// 
    /// The number of AWS Glue Data Processing Units (DPUs) allocated to this         DevEndpoint.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberOfNodes")]
    pub number_of_nodes: Option<i64>,


    /// 
    /// The name of the SecurityConfiguration structure to be used with this     DevEndpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityConfiguration")]
    pub security_configuration: Option<String>,


    /// 
    /// A list of security group identifiers used in this DevEndpoint.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// The public key to be used by this DevEndpoint for authentication. This       attribute is provided for backward compatibility because the recommended attribute to       use is public keys.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublicKey")]
    pub public_key: Option<String>,


    /// 
    /// The AWS Glue version determines the versions of Apache Spark and Python that AWS Glue supports. The Python version indicates the version supported for running your ETL scripts on development endpoints.
    /// 
    /// For more information about the available AWS Glue versions and corresponding Spark and Python versions, see Glue version in the developer guide.
    /// 
    /// Development endpoints that are created without specifying a Glue version default to Glue 0.9.
    /// 
    /// You can specify a version of Python support for development endpoints by using the Arguments parameter in the CreateDevEndpoint or UpdateDevEndpoint APIs. If no arguments are provided, the version defaults to Python 2.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlueVersion")]
    pub glue_version: Option<String>,


    /// 
    /// The paths to one or more Python libraries in an Amazon S3 bucket that should be loaded       in your DevEndpoint. Multiple values must be complete paths separated by a       comma.
    /// 
    /// NoteYou can only use pure Python libraries with a DevEndpoint. Libraries         that rely on C extensions, such as the pandas Python data analysis library, are not currently         supported.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExtraPythonLibsS3Path")]
    pub extra_python_libs_s3_path: Option<String>,


    /// 
    /// The type of predefined worker that is allocated to the development endpoint. Accepts a value of Standard, G.1X, or G.2X.
    /// 
    /// For the Standard worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.For the G.1X worker type, each worker maps to 1 DPU (4 vCPU, 16 GB of memory, 64 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.For the G.2X worker type, each worker maps to 2 DPU (8 vCPU, 32 GB of memory, 128 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.
    /// 
    /// Known issue: when a development endpoint is created with the G.2X WorkerType configuration, the Spark drivers for the development endpoint will run on 4 vCPU, 16 GB of memory, and a 64 GB disk.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkerType")]
    pub worker_type: Option<String>,


    /// 
    /// The number of workers of a defined workerType that are allocated to the development endpoint.
    /// 
    /// The maximum number of workers you can define are 299 for G.1X, and 149 for G.2X.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberOfWorkers")]
    pub number_of_workers: Option<i64>,


    /// 
    /// The path to one or more Java .jar files in an S3 bucket that should be       loaded in your DevEndpoint.
    /// 
    /// NoteYou can only use pure Java/Scala libraries with a DevEndpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExtraJarsS3Path")]
    pub extra_jars_s3_path: Option<String>,

}