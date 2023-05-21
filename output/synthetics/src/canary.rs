

/// Creates or updates a canary. Canaries are scripts that monitor your endpoints and APIs from the     outside-in. Canaries help you check the availability and latency of your web services and     troubleshoot anomalies by investigating load time data, screenshots of the UI, logs, and     metrics. You can set up a canary to run continuously or just once.
///
/// To create canaries, you must have the CloudWatchSyntheticsFullAccess policy.     If you are creating a new IAM role for the canary, you also need the     the iam:CreateRole, iam:CreatePolicy and       iam:AttachRolePolicy permissions. For more information, see Necessary       Roles and Permissions.
///
/// Do not include secrets or proprietary information in your canary names. The canary name     makes up part of the Amazon Resource Name (ARN) for the canary, and the ARN is included in     outbound calls over the internet. For more information, see Security       Considerations for Synthetics Canaries.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCanary {


    /// 
    /// A structure that contains the configuration for canary artifacts, including      the encryption-at-rest settings for artifacts that the canary uploads to Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: ArtifactConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArtifactConfig")]
    pub artifact_config: Option<ArtifactConfig>,


    /// 
    /// The location in Amazon S3 where Synthetics stores artifacts from the runs of this     canary. Artifacts include the log file, screenshots, and HAR files.      Specify the full location path, including s3:// at the beginning       of the path.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArtifactS3Location")]
    pub artifact_s3_location: String,


    /// 
    /// Use this structure to input your script code for the canary. This structure contains the     Lambda handler with the location where the canary should start running the script. If the     script is stored in an S3 bucket, the bucket name, key, and version are also included. If     the script is passed into the canary directly, the script code is contained in the value     of Script.
    /// 
    /// Required: Yes
    ///
    /// Type: Code
    ///
    /// Update requires: No interruption
    #[serde(rename = "Code")]
    pub code: Code,


    /// 
    /// The ARN of the IAM role to be used to run the canary. This role must already exist,      and must include lambda.amazonaws.com as a principal in the trust     policy. The role must also have the following permissions:
    /// 
    /// s3:PutObject                                s3:GetBucketLocation                                s3:ListAllMyBuckets                                cloudwatch:PutMetricData                                logs:CreateLogGroup                                logs:CreateLogStream                                logs:PutLogEvents
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:(aws[a-zA-Z-]*)?:iam::\d{12}:role/?[a-zA-Z_0-9+=,.@\-_/]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: String,


    /// 
    /// The number of days to retain data about failed runs of this canary. If you omit      this field, the default of 31 days is used. The valid range is 1 to 455 days.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailureRetentionPeriod")]
    pub failure_retention_period: Option<i64>,


    /// 
    /// The name for this canary. Be sure to give it a descriptive name      that distinguishes it from other canaries in your account.
    /// 
    /// Do not include secrets or proprietary information in your canary names. The canary name     makes up part of the canary ARN, and the ARN is included in outbound calls over the     internet. For more information, see Security       Considerations for Synthetics Canaries.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 21
    ///
    /// Pattern: ^[0-9a-z_\-]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// A structure that contains input information for a canary run. If you omit       this structure, the       frequency of the canary is used as canary's timeout value, up to a maximum of 900 seconds.
    /// 
    /// Required: No
    ///
    /// Type: RunConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RunConfig")]
    pub run_config: Option<RunConfig>,


    /// 
    /// Specifies the runtime version to use for the canary. For more information about     runtime versions, see       Canary Runtime Versions.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuntimeVersion")]
    pub runtime_version: String,


    /// 
    /// A structure that contains information about how often the canary is to run, and when     these runs are to stop.
    /// 
    /// Required: Yes
    ///
    /// Type: Schedule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schedule")]
    pub schedule: Schedule,


    /// 
    /// Specify TRUE to have the canary start making runs immediately after it is created.
    /// 
    /// A canary that you create using CloudFormation can't be used to monitor the    CloudFormation stack that creates the canary or to roll back that stack if there is a failure.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartCanaryAfterCreation")]
    pub start_canary_after_creation: Option<bool>,


    /// 
    /// The number of days to retain data about successful runs of this canary. If you omit      this field, the default of 31 days is used. The valid range is 1 to 455 days.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "SuccessRetentionPeriod")]
    pub success_retention_period: Option<i64>,


    /// 
    /// The list of key-value pairs that are associated with the canary.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// If this canary is to test an endpoint in a VPC, this structure contains    information about the subnet and security groups of the VPC endpoint.    For more information, see      Running a Canary in a VPC.
    /// 
    /// Required: No
    ///
    /// Type: VPCConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "VPCConfig")]
    pub vpcconfig: Option<VPCConfig>,


    /// 
    /// If this canary performs visual monitoring by comparing screenshots, this structure contains the ID of the canary run to use as the baseline for screenshots, and the coordinates    of any parts of the screen to ignore during the visual monitoring comparison.
    /// 
    /// Required: No
    ///
    /// Type: VisualReference
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisualReference")]
    pub visual_reference: Option<VisualReference>,

}



impl cfn_resources::CfnResource for CfnCanary {
    fn type_string(&self) -> &'static str {
        "AWS::Synthetics::Canary"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.artifact_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.artifact_s3_location;

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'artifact_s3_location'. {} is greater than 1024", the_val.len()));
        }

        
        let the_val = &self.artifact_s3_location;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'artifact_s3_location'. {} is less than 1", the_val.len()));
        }

        
        self.code.validate()?;

        let the_val = &self.execution_role_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!("Max validation failed on field 'execution_role_arn'. {} is greater than 2048", the_val.len()));
        }

        
        let the_val = &self.execution_role_arn;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'execution_role_arn'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.failure_retention_period {

        if *the_val > 1024 as _ {
            return Err(format!("Max validation failed on field 'failure_retention_period'. {} is greater than 1024", the_val));
        }

        }
        
        if let Some(the_val) = &self.failure_retention_period {

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'failure_retention_period'. {} is less than 1", the_val));
        }

        }
        
        let the_val = &self.name;

        if the_val.len() > 21 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 21", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        self.run_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.runtime_version;

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'runtime_version'. {} is greater than 1024", the_val.len()));
        }

        
        let the_val = &self.runtime_version;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'runtime_version'. {} is less than 1", the_val.len()));
        }

        
        self.schedule.validate()?;

        if let Some(the_val) = &self.success_retention_period {

        if *the_val > 1024 as _ {
            return Err(format!("Max validation failed on field 'success_retention_period'. {} is greater than 1024", the_val));
        }

        }
        
        if let Some(the_val) = &self.success_retention_period {

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'success_retention_period'. {} is less than 1", the_val));
        }

        }
        
        self.vpcconfig.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.visual_reference.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A structure that contains the configuration for canary artifacts,    including the encryption-at-rest settings for artifacts that the canary uploads to Amazon S3.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ArtifactConfig {


    /// A structure that contains the configuration    of the encryption-at-rest settings for artifacts that the canary uploads to Amazon S3.    Artifact encryption functionality is available only for canaries that use Synthetics runtime version syn-nodejs-puppeteer-3.3    or later. For more information, see    Encrypting canary artifacts.
    ///
    /// Required: No
    ///
    /// Type: S3Encryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Encryption")]
    pub s3_encryption: Option<S3Encryption>,

}



impl cfn_resources::CfnResource for ArtifactConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.s3_encryption.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A structure representing a screenshot that is used as a baseline during visual monitoring comparisons made by the canary.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BaseScreenshot {


    /// Coordinates that define the part of a screen to ignore during screenshot comparisons. To obtain the coordinates to use here, use the      CloudWatch Logs console to draw the boundaries on the screen. For more information, see {LINK}
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IgnoreCoordinates")]
    pub ignore_coordinates: Option<Vec<String>>,


    /// The name of the screenshot. This is generated the first time the canary is run after the UpdateCanary operation that     specified for this canary to perform visual monitoring.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScreenshotName")]
    pub screenshot_name: String,

}



impl cfn_resources::CfnResource for BaseScreenshot {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Use this structure to input your script code for the canary. This structure contains the     Lambda handler with the location where the canary should start running the script. If the     script is stored in an S3 bucket, the bucket name, key, and version are also included. If     the script is passed into the canary directly, the script code is contained in the value     of Script.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Code {


    /// 
    /// The entry point to use for the source code when running the canary. For canaries that use the      syn-python-selenium-1.0 runtime     or a syn-nodejs.puppeteer runtime earlier than syn-nodejs.puppeteer-3.4,      the handler must be specified as         fileName.handler. For      syn-python-selenium-1.1, syn-nodejs.puppeteer-3.4, and later runtimes, the handler can be specified as               fileName.functionName       , or       you can specify a folder where canary scripts reside as         folder/fileName.functionName       .
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^([0-9a-zA-Z_-]+\/)*[0-9A-Za-z_\\-]+\.[A-Za-z_][A-Za-z0-9_]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Handler")]
    pub handler: String,


    /// 
    /// If your canary script is located in S3, specify the bucket name here. The bucket     must already exist.
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
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: Option<String>,


    /// 
    /// The S3 key of your script. For more information, see Working with Amazon S3 Objects.
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
    pub s3_key: Option<String>,


    /// 
    /// The S3 version ID of your script.
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
    pub s3_object_version: Option<String>,


    /// 
    /// If you input your canary script directly into the canary instead of referring to an S3     location, the value of this parameter is the script in plain text. It can be     up to 5 MB.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Script")]
    pub script: Option<String>,


    /// 
    /// The ARN of the Lambda layer where Synthetics stores the canary script code.
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
    #[serde(rename = "SourceLocationArn")]
    pub source_location_arn: Option<String>,

}



impl cfn_resources::CfnResource for Code {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.handler;

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'handler'. {} is greater than 128", the_val.len()));
        }

        
        let the_val = &self.handler;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'handler'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.s3_bucket {

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 's3_bucket'. {} is greater than 1024", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.s3_bucket {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 's3_bucket'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.s3_key {

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 's3_key'. {} is greater than 1024", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.s3_key {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 's3_key'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.s3_object_version {

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 's3_object_version'. {} is greater than 1024", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.s3_object_version {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 's3_object_version'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.source_location_arn {

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'source_location_arn'. {} is greater than 1024", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.source_location_arn {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'source_location_arn'. {} is less than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// A structure that contains input information for a canary run. This structure     is required.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RunConfig {


    /// 
    /// Specifies whether this canary is to use active AWS X-Ray tracing when it runs. Active tracing      enables this canary run to be displayed in the ServiceLens and X-Ray service maps even if the      canary does not hit an endpoint that has X-Ray tracing enabled. Using X-Ray tracing      incurs charges. For more information, see      Canaries and X-Ray tracing.
    /// 
    /// You can enable active tracing only for canaries that use version syn-nodejs-2.0 or later      for their canary runtime.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActiveTracing")]
    pub active_tracing: Option<bool>,


    /// 
    /// Specifies the keys and values to use for any environment variables     used in the canary script. Use the following format:
    /// 
    /// { "key1" : "value1", "key2" : "value2", ...}
    /// 
    /// Keys must start with a letter and be at least two characters. The total size     of your environment variables cannot exceed 4 KB. You can't specify any Lambda     reserved environment variables as the keys for your environment variables. For      more information about reserved keys, see       Runtime environment variables.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentVariables")]
    pub environment_variables: Option<std::collections::HashMap<String, String>>,


    /// The maximum amount of memory that the    canary can use while running. This value must be a multiple of 64. The range is 960 to 3008.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 960
    ///
    /// Maximum: 3008
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemoryInMB")]
    pub memory_in_mb: Option<i64>,


    /// 
    /// How long the canary is allowed to run before it must stop. You can't set this time to be longer     than the frequency of the runs of this canary.
    /// 
    /// If you omit this field, the     frequency of the canary is used as this value, up to a maximum of 900 seconds.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 3
    ///
    /// Maximum: 840
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeoutInSeconds")]
    pub timeout_in_seconds: Option<i64>,

}



impl cfn_resources::CfnResource for RunConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.memory_in_mb {

        if *the_val > 3008 as _ {
            return Err(format!("Max validation failed on field 'memory_in_mb'. {} is greater than 3008", the_val));
        }

        }
        
        if let Some(the_val) = &self.memory_in_mb {

        if *the_val < 960 as _ {
            return Err(format!("Min validation failed on field 'memory_in_mb'. {} is less than 960", the_val));
        }

        }
        
        if let Some(the_val) = &self.timeout_in_seconds {

        if *the_val > 840 as _ {
            return Err(format!("Max validation failed on field 'timeout_in_seconds'. {} is greater than 840", the_val));
        }

        }
        
        if let Some(the_val) = &self.timeout_in_seconds {

        if *the_val < 3 as _ {
            return Err(format!("Min validation failed on field 'timeout_in_seconds'. {} is less than 3", the_val));
        }

        }
        
        Ok(())
    }
}

/// A structure that contains the configuration      of the encryption-at-rest settings for artifacts that the canary uploads to Amazon S3.      Artifact encryption functionality is available only for canaries that use Synthetics runtime version syn-nodejs-puppeteer-3.3      or later. For more information, see     Encrypting canary artifacts.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Encryption {


    /// The encryption method to use      for artifacts created by this canary. Specify SSE_S3 to use     server-side encryption (SSE) with an Amazon S3-managed     key. Specify SSE-KMS to use server-side encryption with a customer-managed AWS KMS key.
    /// 
    /// If you omit this parameter, an       AWS-managed AWS KMS key is used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionMode")]
    pub encryption_mode: Option<String>,


    /// 
    /// The ARN of the customer-managed AWS KMS key to use, if you specify SSE-KMS     for EncryptionMode
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,

}



impl cfn_resources::CfnResource for S3Encryption {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// This structure specifies how often a canary is to make runs and the date and time     when it should stop making runs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Schedule {


    /// 
    /// How long, in seconds, for the canary to continue making regular runs according to     the schedule in the Expression value. If you specify 0, the canary continues     making runs until you stop it. If you omit this field, the default of 0 is used.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<String>,


    /// 
    /// A rate expression or a cron expression that defines how often the canary is to run.
    /// 
    /// For a rate expression, The syntax is        rate(number unit). unit     can be minute, minutes, or hour.
    /// 
    /// For example, rate(1 minute) runs the canary once a minute, rate(10 minutes) runs it once every    10 minutes, and rate(1 hour) runs it once every hour. You can     specify a frequency between rate(1 minute) and rate(1 hour).
    /// 
    /// Specifying rate(0 minute) or rate(0 hour) is a special value      that causes the      canary to run only once when it is started.
    /// 
    /// Use cron(expression) to specify a cron      expression. You can't schedule a canary to wait for more than a year before running. For information about the syntax for cron expressions, see            Scheduling canary runs using cron.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: String,

}



impl cfn_resources::CfnResource for Schedule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.expression;

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'expression'. {} is greater than 1024", the_val.len()));
        }

        
        let the_val = &self.expression;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'expression'. {} is less than 1", the_val.len()));
        }

        
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
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}



impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// If this canary is to test an endpoint in a VPC, this structure contains    information about the subnet and security groups of the VPC endpoint.    For more information, see      Running a Canary in a VPC.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VPCConfig {


    /// 
    /// The IDs of the security groups for this canary.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,


    /// 
    /// The IDs of the subnets where this canary is to run.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 16
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,


    /// 
    /// The ID of the VPC where this canary is to run.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,

}



impl cfn_resources::CfnResource for VPCConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.security_group_ids;

        if the_val.len() > 5 as _ {
            return Err(format!("Max validation failed on field 'security_group_ids'. {} is greater than 5", the_val.len()));
        }

        
        let the_val = &self.subnet_ids;

        if the_val.len() > 16 as _ {
            return Err(format!("Max validation failed on field 'subnet_ids'. {} is greater than 16", the_val.len()));
        }

        
        Ok(())
    }
}

/// Defines the screenshots to use as the baseline for comparisons during visual monitoring comparisons during future runs of this canary. If you omit this      parameter, no changes are made to any baseline screenshots that the canary might be using already.
///
/// Visual monitoring is supported only on canaries running the syn-puppeteer-node-3.2       runtime or later. For more information, see         Visual monitoring and          Visual monitoring blueprint
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VisualReference {


    /// Specifies which canary run to use the screenshots from as the baseline for future visual monitoring with this canary. Valid values are      nextrun to use the screenshots from the next run after this update is made, lastrun to use the screenshots from the most recent run      before this update was made, or the value of Id in the       CanaryRun from any past run of this canary.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseCanaryRunId")]
    pub base_canary_run_id: String,


    /// An array of screenshots that are used as the baseline for comparisons during visual monitoring.
    /// 
    /// Required: No
    ///
    /// Type: List of BaseScreenshot
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseScreenshots")]
    pub base_screenshots: Option<Vec<BaseScreenshot>>,

}



impl cfn_resources::CfnResource for VisualReference {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}