/// The AWS::Lambda::Function resource creates a Lambda function. To create a function, you need a    deployment package and an    execution role.    The deployment package is a .zip file archive or container image that contains your function code.    The execution role grants the function permission to use AWS services, such as Amazon CloudWatch Logs    for log streaming and AWS X-Ray for request tracing.
///
/// You set the package type to Image if the deployment package is a    container image. For a container image,    the code property must include the URI of a container image in the Amazon ECR registry.    You do not need to specify the handler and runtime properties.
///
/// You set the package type to Zip if the deployment package is a    .zip file archive.    For a .zip file archive, the code property specifies the location of the .zip file. You must also specify the handler and    runtime properties. For a Python example, see    Deploy Python Lambda functions with .zip file archives.
///
/// You can use code signing    if your deployment package is a .zip file archive. To enable code signing for this function,    specify the ARN of a code-signing configuration. When a user    attempts to deploy a code package with UpdateFunctionCode, Lambda checks that the code    package has a valid signature from a trusted publisher. The code-signing configuration    includes a set of signing profiles, which define the trusted publishers for this function.
///
/// Note that you configure provisioned concurrency     on a AWS::Lambda::Version or a AWS::Lambda::Alias.
///
/// For a complete introduction to Lambda functions, see    What is Lambda?   in the Lambda developer guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnFunction {
    ///
    /// The instruction set architecture that the function supports. Enter a string array with one of the valid values (arm64 or x86_64).   The default value is x86_64.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Architectures")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub architectures: Option<Vec<String>>,

    ///
    /// The code for the function.
    ///
    /// Required: Yes
    ///
    /// Type: Code
    ///
    /// Update requires: No interruption
    #[serde(rename = "Code")]
    pub code: Code,

    ///
    /// To enable code signing for this function, specify the ARN of a code-signing configuration. A code-signing configuration includes a set of signing profiles, which define the trusted publishers for this function.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 200
    ///
    /// Pattern: arn:(aws[a-zA-Z-]*)?:lambda:[a-z]{2}((-gov)|(-iso(b?)))?-[a-z]+-\d{1}:\d{12}:code-signing-config:csc-[a-z0-9]{17}
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodeSigningConfigArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub code_signing_config_arn: Option<cfn_resources::StrVal>,

    ///
    /// A dead-letter queue configuration that specifies the queue or topic where Lambda sends asynchronous events    when they fail processing. For more information, see Dead-letter queues.
    ///
    /// Required: No
    ///
    /// Type: DeadLetterConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeadLetterConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub dead_letter_config: Option<DeadLetterConfig>,

    ///
    /// A description of the function.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// Environment variables that are accessible from function code during execution.
    ///
    /// Required: No
    ///
    /// Type: Environment
    ///
    /// Update requires: No interruption
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub environment: Option<Environment>,

    ///
    /// The size of the function's /tmp directory in MB. The default value is 512,      but it can be any whole number between 512 and 10,240 MB.
    ///
    /// Required: No
    ///
    /// Type: EphemeralStorage
    ///
    /// Update requires: No interruption
    #[serde(rename = "EphemeralStorage")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub ephemeral_storage: Option<EphemeralStorage>,

    ///
    /// Connection settings for an Amazon EFS file system. To connect a function to a file system, a mount target must be available    in every Availability Zone that your function connects to. If your template contains an    AWS::EFS::MountTarget resource,    you must also specify a DependsOn attribute to ensure that the mount target is created or updated before the function.
    ///
    /// For more information about using the DependsOn attribute, see DependsOn Attribute.
    ///
    /// Required: No
    ///
    /// Type: List of FileSystemConfig
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileSystemConfigs")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub file_system_configs: Option<Vec<FileSystemConfig>>,

    ///
    /// The name of the Lambda function, up to 64 characters in length. If you don't specify a name, AWS CloudFormation     generates one.
    ///
    /// If you specify a name, you cannot perform updates that require replacement of this resource. You can perform    updates that require no or some interruption. If you must replace the resource, specify a new name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub function_name: Option<cfn_resources::StrVal>,

    ///
    /// The name of the method within your code that Lambda calls to run your function. Handler is required if the deployment package is a .zip file archive. The format includes the    file name. It can also include namespaces and other qualifiers, depending on the runtime. For more information,    see Lambda programming model.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: [^\s]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Handler")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub handler: Option<cfn_resources::StrVal>,

    ///
    /// Configuration values that override the container image Dockerfile settings. For more information, see Container image    settings.
    ///
    /// Required: No
    ///
    /// Type: ImageConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub image_config: Option<ImageConfig>,

    ///
    /// The ARN of the AWS Key Management Service (AWS KMS) customer managed key that's used to encrypt your function's environment variables. When Lambda SnapStart is activated, Lambda also uses this key is to encrypt your function's snapshot. If you deploy your function using a container image, Lambda also uses this key to encrypt your function when it's deployed. Note that this is not the same key that's used to protect your container image in the Amazon Elastic Container Registry (Amazon ECR). If you don't provide a customer managed key, Lambda uses a default service key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: (arn:(aws[a-zA-Z-]*)?:[a-z0-9-.]+:.*)|()
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub kms_key_arn: Option<cfn_resources::StrVal>,

    ///
    /// A list of function layers    to add to the function's execution environment. Specify each layer by its ARN, including the version.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub layers: Option<Vec<String>>,

    ///
    /// The amount of memory available to the function at runtime.    Increasing the function memory also increases its CPU allocation. The default value is 128 MB. The value can be any multiple of 1 MB.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 128
    ///
    /// Maximum: 10240
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub memory_size: Option<i64>,

    ///
    /// The type of deployment package. Set to Image for container image and set Zip for .zip file archive.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Image | Zip
    ///
    /// Update requires: No interruption
    #[serde(rename = "PackageType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub package_type: Option<FunctionPackageTypeEnum>,

    ///
    /// The number of simultaneous executions to reserve for the function.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReservedConcurrentExecutions")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub reserved_concurrent_executions: Option<i64>,

    ///
    /// The Amazon Resource Name (ARN) of the function's execution role.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: arn:(aws[a-zA-Z-]*)?:iam::\d{12}:role/?[a-zA-Z_0-9+=,.@\-_/]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Role")]
    pub role: cfn_resources::StrVal,

    ///
    /// The identifier of the function's runtime. Runtime is required if the deployment package is a .zip file archive.
    ///
    /// The following list includes deprecated runtimes. For more information, see Runtime deprecation policy.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: dotnet6 | dotnetcore1.0 | dotnetcore2.0 | dotnetcore2.1 | dotnetcore3.1 | go1.x | java11 | java17 | java8 | java8.al2 | nodejs | nodejs10.x | nodejs12.x | nodejs14.x | nodejs16.x | nodejs18.x | nodejs4.3 | nodejs4.3-edge | nodejs6.10 | nodejs8.10 | provided | provided.al2 | python2.7 | python3.10 | python3.6 | python3.7 | python3.8 | python3.9 | ruby2.5 | ruby2.7
    ///
    /// Update requires: No interruption
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub runtime: Option<FunctionRuntimeEnum>,

    ///
    /// Sets the runtime management configuration for a function's version. For more information,    see Runtime updates.
    ///
    /// Required: No
    ///
    /// Type: RuntimeManagementConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuntimeManagementConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub runtime_management_config: Option<RuntimeManagementConfig>,

    ///
    /// The function's AWS Lambda SnapStart setting.
    ///
    /// Required: No
    ///
    /// Type: SnapStart
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapStart")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub snap_start: Option<SnapStart>,

    ///
    /// A list of tags to apply to the    function.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The amount of time (in seconds) that Lambda allows a function to run before stopping it. The default is 3 seconds. The    maximum allowed value is 900 seconds. For more information, see Lambda execution environment.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub timeout: Option<i64>,

    ///
    /// Set Mode to Active to sample and trace a subset of incoming requests with X-Ray.
    ///
    /// Required: No
    ///
    /// Type: TracingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "TracingConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tracing_config: Option<TracingConfig>,

    ///
    /// For network connectivity to AWS resources in a VPC, specify a list of security groups and subnets in the VPC.    When you connect a function to a VPC, it can access resources and the internet only through that VPC. For more    information, see Configuring a Lambda function to access resources in a VPC.
    ///
    /// Required: No
    ///
    /// Type: VpcConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub vpc_config: Option<VpcConfig>,

    #[serde(skip_serializing)]
    pub att_arn: CfnFunctionarn,

    #[serde(skip_serializing)]
    pub att_snap_start_response_apply_on: CfnFunctionsnapstartresponseapplyon,

    #[serde(skip_serializing)]
    pub att_snap_start_response_optimization_status: CfnFunctionsnapstartresponseoptimizationstatus,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum FunctionPackageTypeEnum {
    /// Image
    #[serde(rename = "Image")]
    Image,

    /// Zip
    #[serde(rename = "Zip")]
    Zip,
}

impl Default for FunctionPackageTypeEnum {
    fn default() -> Self {
        FunctionPackageTypeEnum::Image
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum FunctionRuntimeEnum {
    /// dotnet6
    #[serde(rename = "dotnet6")]
    Dotnet6,

    /// dotnetcore1.0
    #[serde(rename = "dotnetcore1.0")]
    Dotnetcore10,

    /// dotnetcore2.0
    #[serde(rename = "dotnetcore2.0")]
    Dotnetcore20,

    /// dotnetcore2.1
    #[serde(rename = "dotnetcore2.1")]
    Dotnetcore21,

    /// dotnetcore3.1
    #[serde(rename = "dotnetcore3.1")]
    Dotnetcore31,

    /// go1.x
    #[serde(rename = "go1.x")]
    Go1x,

    /// java11
    #[serde(rename = "java11")]
    Java11,

    /// java17
    #[serde(rename = "java17")]
    Java17,

    /// java8
    #[serde(rename = "java8")]
    Java8,

    /// java8.al2
    #[serde(rename = "java8.al2")]
    Java8al2,

    /// nodejs
    #[serde(rename = "nodejs")]
    Nodejs,

    /// nodejs10.x
    #[serde(rename = "nodejs10.x")]
    Nodejs10x,

    /// nodejs12.x
    #[serde(rename = "nodejs12.x")]
    Nodejs12x,

    /// nodejs14.x
    #[serde(rename = "nodejs14.x")]
    Nodejs14x,

    /// nodejs16.x
    #[serde(rename = "nodejs16.x")]
    Nodejs16x,

    /// nodejs18.x
    #[serde(rename = "nodejs18.x")]
    Nodejs18x,

    /// nodejs4.3
    #[serde(rename = "nodejs4.3")]
    Nodejs43,

    /// nodejs4.3-edge
    #[serde(rename = "nodejs4.3-edge")]
    Nodejs43edge,

    /// nodejs6.10
    #[serde(rename = "nodejs6.10")]
    Nodejs610,

    /// nodejs8.10
    #[serde(rename = "nodejs8.10")]
    Nodejs810,

    /// provided
    #[serde(rename = "provided")]
    Provided,

    /// provided.al2
    #[serde(rename = "provided.al2")]
    Providedal2,

    /// python2.7
    #[serde(rename = "python2.7")]
    Python27,

    /// python3.10
    #[serde(rename = "python3.10")]
    Python310,

    /// python3.6
    #[serde(rename = "python3.6")]
    Python36,

    /// python3.7
    #[serde(rename = "python3.7")]
    Python37,

    /// python3.8
    #[serde(rename = "python3.8")]
    Python38,

    /// python3.9
    #[serde(rename = "python3.9")]
    Python39,

    /// ruby2.5
    #[serde(rename = "ruby2.5")]
    Ruby25,

    /// ruby2.7
    #[serde(rename = "ruby2.7")]
    Ruby27,
}

impl Default for FunctionRuntimeEnum {
    fn default() -> Self {
        FunctionRuntimeEnum::Dotnet6
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFunctionarn;
impl CfnFunctionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFunctionsnapstartresponseapplyon;
impl CfnFunctionsnapstartresponseapplyon {
    pub fn att_name(&self) -> &'static str {
        r#"SnapStartResponse.ApplyOn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFunctionsnapstartresponseoptimizationstatus;
impl CfnFunctionsnapstartresponseoptimizationstatus {
    pub fn att_name(&self) -> &'static str {
        r#"SnapStartResponse.OptimizationStatus"#
    }
}

impl cfn_resources::CfnResource for CfnFunction {
    fn type_string(&self) -> &'static str {
        "AWS::Lambda::Function"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.architectures {
            if the_val.len() > 1 as _ {
                return Err(format!(
                    "Max validation failed on field 'architectures'. {} is greater than 1",
                    the_val.len()
                ));
            }
        }

        self.code.validate()?;

        if let Some(the_val) = &self.code_signing_config_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 200 as _ {
                    return Err(format!("Max validation failed on field 'code_signing_config_arn'. {} is greater than 200", s.len()));
                }
            }
        }

        self.dead_letter_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        self.environment
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ephemeral_storage
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.file_system_configs {
            if the_val.len() > 1 as _ {
                return Err(format!(
                    "Max validation failed on field 'file_system_configs'. {} is greater than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.handler {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'handler'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        self.image_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.memory_size {
            if *the_val > 10240 as _ {
                return Err(format!(
                    "Max validation failed on field 'memory_size'. {} is greater than 10240",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.memory_size {
            if *the_val < 128 as _ {
                return Err(format!(
                    "Min validation failed on field 'memory_size'. {} is less than 128",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.reserved_concurrent_executions {
            if *the_val < 0 as _ {
                return Err(format!("Min validation failed on field 'reserved_concurrent_executions'. {} is less than 0", the_val));
            }
        }

        self.runtime_management_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.snap_start
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.timeout {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'timeout'. {} is less than 1",
                    the_val
                ));
            }
        }

        self.tracing_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vpc_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The deployment package    for a Lambda function. To deploy a function defined as a container image,    you specify the location of a container image in the Amazon ECR registry.    For a .zip file deployment package, you can specify the location of an object in    Amazon S3. For Node.js and Python functions, you can specify the function code inline in the template.
///
/// Changes to a deployment package in Amazon S3 are not detected automatically during stack updates. To update    the function code, change the object key or version in the template.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Code {
    ///
    /// URI of a container image in the     Amazon ECR registry.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageUri")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub image_uri: Option<cfn_resources::StrVal>,

    ///
    /// An Amazon S3 bucket in the same AWS Region as your function. The bucket can be in a different AWS account.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[0-9A-Za-z\.\-_]*(?<!\.)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Bucket")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub s3_bucket: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon S3 key of the deployment package.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub s3_key: Option<cfn_resources::StrVal>,

    ///
    /// For versioned objects, the version of the deployment package object to use.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3ObjectVersion")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub s3_object_version: Option<cfn_resources::StrVal>,

    ///
    /// (Node.js and Python) The source code of your Lambda function. If you include your function source inline with    this parameter, AWS CloudFormation places it in a file named index and zips it to create a    deployment package.    This zip file cannot exceed 4MB. For the Handler property, the first part of the handler identifier must be    index. For example, index.handler.
    ///
    /// For JSON, you must escape quotes and special characters such as newline (\n) with a backslash.
    ///
    /// If you specify a function that interacts with an AWS CloudFormation custom resource, you don't have to write    your own functions to send responses to the custom resource that invoked the function. AWS CloudFormation provides    a response module (cfn-response)    that simplifies sending responses. See Using AWS Lambda with AWS CloudFormation for    details.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ZipFile")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub zip_file: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Code {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.s3_bucket {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 63 as _ {
                    return Err(format!(
                        "Max validation failed on field 's3_bucket'. {} is greater than 63",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.s3_bucket {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 3 as _ {
                    return Err(format!(
                        "Min validation failed on field 's3_bucket'. {} is less than 3",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.s3_key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 's3_key'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.s3_key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 's3_key'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.s3_object_version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!("Max validation failed on field 's3_object_version'. {} is greater than 1024", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.s3_object_version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 's3_object_version'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The dead-letter queue for    failed asynchronous invocations.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeadLetterConfig {
    ///
    /// The Amazon Resource Name (ARN) of an Amazon SQS queue or Amazon SNS topic.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: (arn:(aws[a-zA-Z-]*)?:[a-z0-9-.]+:.*)|()
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub target_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DeadLetterConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A function's environment variable settings. You can use environment variables to adjust your function's    behavior without updating code. An environment variable is a pair of strings that are stored in a function's    version-specific configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Environment {
    ///
    /// Environment variable key-value pairs. For more information, see Using Lambda environment variables.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Variables")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub variables: Option<std::collections::HashMap<String, String>>,
}

impl cfn_resources::CfnResource for Environment {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The size of the function's /tmp directory in MB. The default value is 512,      but it can be any whole number between 512 and 10,240 MB.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EphemeralStorage {
    ///
    /// The size of the function's /tmp directory.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 512
    ///
    /// Maximum: 10240
    ///
    /// Update requires: No interruption
    #[serde(rename = "Size")]
    pub size: i64,
}

impl cfn_resources::CfnResource for EphemeralStorage {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.size;

        if *the_val > 10240 as _ {
            return Err(format!(
                "Max validation failed on field 'size'. {} is greater than 10240",
                the_val
            ));
        }

        let the_val = &self.size;

        if *the_val < 512 as _ {
            return Err(format!(
                "Min validation failed on field 'size'. {} is less than 512",
                the_val
            ));
        }

        Ok(())
    }
}

/// Details about the connection between a Lambda function and an Amazon EFS file system.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FileSystemConfig {
    ///
    /// The Amazon Resource Name (ARN) of the Amazon EFS access point that provides access to the file    system.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 200
    ///
    /// Pattern: arn:aws[a-zA-Z-]*:elasticfilesystem:[a-z]{2}((-gov)|(-iso(b?)))?-[a-z]+-\d{1}:\d{12}:access-point/fsap-[a-f0-9]{17}
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: cfn_resources::StrVal,

    ///
    /// The path where the function can access the file system, starting with /mnt/.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 160
    ///
    /// Pattern: ^/mnt/[a-zA-Z0-9-_.]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalMountPath")]
    pub local_mount_path: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for FileSystemConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'arn'. {} is greater than 200",
                    s.len()
                ));
            }
        }

        let the_val = &self.local_mount_path;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 160 as _ {
                return Err(format!(
                    "Max validation failed on field 'local_mount_path'. {} is greater than 160",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Configuration values that override the container image Dockerfile settings. For more information, see Container image    settings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ImageConfig {
    ///
    /// Specifies parameters that you want to pass in with ENTRYPOINT. You can specify a maximum of 1,500 parameters      in the list.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1500
    ///
    /// Update requires: No interruption
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub command: Option<Vec<String>>,

    ///
    /// Specifies the entry point to their application, which is typically the location of the runtime    executable. You can specify a maximum of 1,500 string entries in the list.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1500
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntryPoint")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub entry_point: Option<Vec<String>>,

    ///
    /// Specifies the working directory. The length of the directory string cannot exceed 1,000 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkingDirectory")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub working_directory: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ImageConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.command {
            if the_val.len() > 1500 as _ {
                return Err(format!(
                    "Max validation failed on field 'command'. {} is greater than 1500",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.entry_point {
            if the_val.len() > 1500 as _ {
                return Err(format!(
                    "Max validation failed on field 'entry_point'. {} is greater than 1500",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.working_directory {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1000 as _ {
                    return Err(format!("Max validation failed on field 'working_directory'. {} is greater than 1000", s.len()));
                }
            }
        }

        Ok(())
    }
}

/// Sets the runtime management configuration for a function's version. For more information,    see Runtime updates.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RuntimeManagementConfig {
    ///
    /// The ARN of the runtime version you want the function to use.
    ///
    /// NoteThis is only required if you're using the Manual runtime update mode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 26
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^arn:(aws[a-zA-Z-]*):lambda:[a-z]{2}((-gov)|(-iso(b?)))?-[a-z]+-\d{1}::runtime:.+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuntimeVersionArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub runtime_version_arn: Option<cfn_resources::StrVal>,

    ///
    /// Specify the runtime update mode.
    ///
    /// Auto (default) - Automatically update to the most recent and secure runtime version using a Two-phase runtime version rollout. This is the best     choice for most customers to ensure they always benefit from runtime updates.FunctionUpdate - Lambda updates the runtime of you function to the most recent and secure runtime version when you update your     function. This approach synchronizes runtime updates with function deployments, giving you control over when runtime updates are applied and allowing you to detect and     mitigate rare runtime update incompatibilities early. When using this setting, you need to regularly update your functions to keep their runtime up-to-date.Manual - You specify a runtime version in your function configuration. The function will use this runtime version indefinitely.     In the rare case where a new runtime version is incompatible with an existing function, this allows you to roll back your function to an earlier runtime version. For more information,     see Roll back a runtime version.
    ///
    /// Valid Values: Auto | FunctionUpdate | Manual
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpdateRuntimeOn")]
    pub update_runtime_on: RuntimeManagementConfigUpdateRuntimeOnEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum RuntimeManagementConfigUpdateRuntimeOnEnum {
    /// Auto
    #[serde(rename = "Auto")]
    Auto,

    /// FunctionUpdate
    #[serde(rename = "FunctionUpdate")]
    Functionupdate,

    /// Manual
    #[serde(rename = "Manual")]
    Manual,
}

impl Default for RuntimeManagementConfigUpdateRuntimeOnEnum {
    fn default() -> Self {
        RuntimeManagementConfigUpdateRuntimeOnEnum::Auto
    }
}

impl cfn_resources::CfnResource for RuntimeManagementConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.runtime_version_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'runtime_version_arn'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.runtime_version_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 26 as _ {
                    return Err(format!(
                        "Min validation failed on field 'runtime_version_arn'. {} is less than 26",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The function's AWS Lambda SnapStart setting.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SnapStart {
    ///
    /// Set ApplyOn to PublishedVersions to create a snapshot of the initialized execution environment when you publish a function version.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: None | PublishedVersions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplyOn")]
    pub apply_on: SnapStartApplyOnEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SnapStartApplyOnEnum {
    /// None
    #[serde(rename = "None")]
    None,

    /// PublishedVersions
    #[serde(rename = "PublishedVersions")]
    Publishedversions,
}

impl Default for SnapStartApplyOnEnum {
    fn default() -> Self {
        SnapStartApplyOnEnum::None
    }
}

impl cfn_resources::CfnResource for SnapStart {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The function's SnapStart setting.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SnapStartResponse {
    ///
    /// When set to PublishedVersions, Lambda creates a snapshot of the execution environment when you publish a function version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: None | PublishedVersions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplyOn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub apply_on: Option<SnapStartResponseApplyOnEnum>,

    ///
    /// When you provide a qualified Amazon Resource Name (ARN), this response element indicates whether SnapStart is activated for the specified function version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Off | On
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptimizationStatus")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub optimization_status: Option<SnapStartResponseOptimizationStatusEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SnapStartResponseApplyOnEnum {
    /// None
    #[serde(rename = "None")]
    None,

    /// PublishedVersions
    #[serde(rename = "PublishedVersions")]
    Publishedversions,
}

impl Default for SnapStartResponseApplyOnEnum {
    fn default() -> Self {
        SnapStartResponseApplyOnEnum::None
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SnapStartResponseOptimizationStatusEnum {
    /// Off
    #[serde(rename = "Off")]
    Off,

    /// On
    #[serde(rename = "On")]
    On,
}

impl Default for SnapStartResponseOptimizationStatusEnum {
    fn default() -> Self {
        SnapStartResponseOptimizationStatusEnum::Off
    }
}

impl cfn_resources::CfnResource for SnapStartResponse {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The function's AWS X-Ray tracing configuration.    To sample and record incoming requests, set Mode to Active.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TracingConfig {
    ///
    /// The tracing mode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Active | PassThrough
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub mode: Option<TracingConfigModeEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum TracingConfigModeEnum {
    /// Active
    #[serde(rename = "Active")]
    Active,

    /// PassThrough
    #[serde(rename = "PassThrough")]
    Passthrough,
}

impl Default for TracingConfigModeEnum {
    fn default() -> Self {
        TracingConfigModeEnum::Active
    }
}

impl cfn_resources::CfnResource for TracingConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The VPC security groups and subnets that are attached to a Lambda function. When you connect a function to a    VPC, Lambda creates an elastic network interface for each combination of security group and subnet in the    function's VPC configuration. The function can only access resources and the internet through that VPC. For more    information, see VPC    Settings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VpcConfig {
    ///
    /// A list of VPC security group IDs.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// A list of VPC subnet IDs.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 16
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub subnet_ids: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for VpcConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.security_group_ids {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'security_group_ids'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.subnet_ids {
            if the_val.len() > 16 as _ {
                return Err(format!(
                    "Max validation failed on field 'subnet_ids'. {} is greater than 16",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
