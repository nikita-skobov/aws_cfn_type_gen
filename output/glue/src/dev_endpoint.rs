/// The AWS::Glue::DevEndpoint resource specifies a development endpoint       where a developer can remotely debug ETL scripts for AWS Glue. For more information, see         DevEndpoint Structure in the AWS Glue Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDevEndpoint {
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_jars_s3_path: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_python_libs_s3_path: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<cfn_resources::StrVal>,

    ///
    /// The number of AWS Glue Data Processing Units (DPUs) allocated to this         DevEndpoint.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberOfNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_workers: Option<i64>,

    ///
    /// The public key to be used by this DevEndpoint for authentication. This       attribute is provided for backward compatibility because the recommended attribute to       use is public keys.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<Vec<String>>,

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
    pub role_arn: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<cfn_resources::StrVal>,

    ///
    /// A list of security group identifiers used in this DevEndpoint.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// The subnet ID for this DevEndpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<cfn_resources::StrVal>,

    ///
    /// The tags to use with this DevEndpoint.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnDevEndpoint {
    fn type_string(&self) -> &'static str {
        "AWS::Glue::DevEndpoint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.public_keys {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'public_keys'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.security_configuration {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!("Max validation failed on field 'security_configuration'. {} is greater than 255", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.security_configuration {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'security_configuration'. {} is less than 1", s.len()));
                }
            }
        }

        Ok(())
    }
}
