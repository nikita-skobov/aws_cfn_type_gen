/// The AWS::MWAA::Environment resource creates an Amazon Managed Workflows for Apache Airflow (MWAA) environment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironment {
    ///
    /// A list of key-value pairs containing the Airflow configuration options for your environment. For example, core.default_timezone: utc. To learn more, see Apache Airflow configuration options.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "AirflowConfigurationOptions")]
    pub airflow_configuration_options: Option<serde_json::Value>,

    ///
    /// The version of Apache Airflow to use for the environment. If no value is specified, defaults to the latest version.
    ///
    /// Allowed Values: 2.0.2 | 1.10.12 | 2.2.2 | 2.4.3 | 2.5.1 (latest)
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AirflowVersion")]
    pub airflow_version: Option<EnvironmentAirflowVersionEnum>,

    ///
    /// The relative path to the DAGs folder on your Amazon S3 bucket. For example, dags. To learn more, see Adding or updating DAGs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DagS3Path")]
    pub dag_s3_path: Option<String>,

    ///
    /// The environment class type. Valid values: mw1.small, mw1.medium, mw1.large. To learn more, see Amazon MWAA environment class.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentClass")]
    pub environment_class: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) of the execution role in IAM that allows MWAA to access AWS resources in your environment. For example, arn:aws:iam::123456789:role/my-execution-role. To learn more, see Amazon MWAA Execution role.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: Option<String>,

    ///
    /// The AWS Key Management Service (KMS) key to encrypt and decrypt the data in your environment. You can use an AWS KMS key managed by MWAA, or a customer-managed KMS key (advanced).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKey")]
    pub kms_key: Option<String>,

    ///
    /// The Apache Airflow logs being sent to CloudWatch Logs: DagProcessingLogs, SchedulerLogs, TaskLogs, WebserverLogs, WorkerLogs.
    ///
    /// Required: No
    ///
    /// Type: LoggingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingConfiguration")]
    pub logging_configuration: Option<LoggingConfiguration>,

    ///
    /// The maximum number of workers that you want to run in your environment. MWAA scales the number of Apache Airflow workers up to the number you specify in the MaxWorkers field. For example, 20. When there are no more tasks running, and no more in the queue, MWAA disposes of the extra workers leaving the one worker that is included with your environment, or the number you specify in MinWorkers.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxWorkers")]
    pub max_workers: Option<i64>,

    ///
    /// The minimum number of workers that you want to run in your environment. MWAA scales the number of Apache Airflow workers up to the number you specify in the MaxWorkers field. When there are no more tasks running, and no more in the queue, MWAA disposes of the extra workers leaving the worker count you specify in the MinWorkers field. For example, 2.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinWorkers")]
    pub min_workers: Option<i64>,

    ///
    /// The name of your Amazon MWAA environment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The VPC networking components used to secure and enable network traffic between the AWS resources for your environment. To learn more, see About networking on Amazon MWAA.
    ///
    /// Required: No
    ///
    /// Type: NetworkConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<NetworkConfiguration>,

    ///
    /// The version of the plugins.zip file on your Amazon S3 bucket. To learn more, see Installing custom plugins.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PluginsS3ObjectVersion")]
    pub plugins_s3_object_version: Option<String>,

    ///
    /// The relative path to the plugins.zip file on your Amazon S3 bucket. For example, plugins.zip. To learn more, see Installing custom plugins.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PluginsS3Path")]
    pub plugins_s3_path: Option<String>,

    ///
    /// The version of the requirements.txt file on your Amazon S3 bucket. To learn more, see Installing Python dependencies.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequirementsS3ObjectVersion")]
    pub requirements_s3_object_version: Option<String>,

    ///
    /// The relative path to the requirements.txt file on your Amazon S3 bucket. For example, requirements.txt. To learn more, see Installing Python dependencies.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequirementsS3Path")]
    pub requirements_s3_path: Option<String>,

    ///
    /// The number of schedulers that you want to run in your environment. Valid values:
    ///
    /// v2 - Accepts between 2 to 5. Defaults to 2.               v1 - Accepts 1.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schedulers")]
    pub schedulers: Option<i64>,

    ///
    /// The Amazon Resource Name (ARN) of the Amazon S3 bucket where your DAG code and supporting files are stored. For example, arn:aws:s3:::my-airflow-bucket-unique-name. To learn more, see Create an Amazon S3 bucket for Amazon MWAA.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceBucketArn")]
    pub source_bucket_arn: Option<String>,

    ///
    /// The version of the startup shell script in your Amazon S3 bucket. You must specify the version ID that Amazon S3 assigns to the file       every time you update the script.
    ///
    /// Version IDs are Unicode, UTF-8 encoded, URL-ready, opaque strings that are no more than 1,024 bytes long. The following is an example:
    ///
    /// 3sL4kqtJlcpXroDTDmJ+rmSpXd3dIbrHY+MTRCxf3vjVBH40Nr8X8gdRQBpUMLUo
    ///
    /// For more information, see Using a startup script.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartupScriptS3ObjectVersion")]
    pub startup_script_s3_object_version: Option<String>,

    ///
    /// The relative path to the startup shell script in your Amazon S3 bucket. For example, s3://mwaa-environment/startup.sh.
    ///
    /// Amazon MWAA runs the script as your environment starts, and before running the Apache Airflow process.       You can use this script to install dependencies, modify Apache Airflow configuration options, and set environment variables. For more information, see       Using a startup script.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartupScriptS3Path")]
    pub startup_script_s3_path: Option<String>,

    ///
    /// The key-value tag pairs associated to your environment. For example, "Environment": "Staging". To learn more, see Tagging.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<serde_json::Value>,

    ///
    /// The Apache Airflow Web server access mode. To learn more, see Apache Airflow access modes. Valid values: PRIVATE_ONLY or PUBLIC_ONLY.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WebserverAccessMode")]
    pub webserver_access_mode: Option<String>,

    ///
    /// The day and time of the week to start weekly maintenance updates of your environment in the following format: DAY:HH:MM. For example: TUE:03:30. You can specify a start time in 30 minute increments only. Supported input includes the following:
    ///
    /// MON|TUE|WED|THU|FRI|SAT|SUN:([01]\\d|2[0-3]):(00|30)
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WeeklyMaintenanceWindowStart")]
    pub weekly_maintenance_window_start: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum EnvironmentAirflowVersionEnum {
    /// 2.0.2
    #[serde(rename = "2.0.2")]
    E202,

    /// 1.10.12
    #[serde(rename = "1.10.12")]
    E11012,

    /// 2.2.2
    #[serde(rename = "2.2.2")]
    E222,

    /// 2.4.3
    #[serde(rename = "2.4.3")]
    E243,

    /// 2.5.1 (latest)
    #[serde(rename = "2.5.1 (latest)")]
    E251latest,
}

impl Default for EnvironmentAirflowVersionEnum {
    fn default() -> Self {
        EnvironmentAirflowVersionEnum::E202
    }
}

impl cfn_resources::CfnResource for CfnEnvironment {
    fn type_string(&self) -> &'static str {
        "AWS::MWAA::Environment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.logging_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.network_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The type of Apache Airflow logs to send to CloudWatch Logs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoggingConfiguration {
    ///
    /// Defines the processing logs sent to CloudWatch Logs and the logging level to send.
    ///
    /// Required: No
    ///
    /// Type: ModuleLoggingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DagProcessingLogs")]
    pub dag_processing_logs: Option<ModuleLoggingConfiguration>,

    ///
    /// Defines the scheduler logs sent to CloudWatch Logs and the logging level to send.
    ///
    /// Required: No
    ///
    /// Type: ModuleLoggingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchedulerLogs")]
    pub scheduler_logs: Option<ModuleLoggingConfiguration>,

    ///
    /// Defines the task logs sent to CloudWatch Logs and the logging level to send.
    ///
    /// Required: No
    ///
    /// Type: ModuleLoggingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskLogs")]
    pub task_logs: Option<ModuleLoggingConfiguration>,

    ///
    /// Defines the web server logs sent to CloudWatch Logs and the logging level to send.
    ///
    /// Required: No
    ///
    /// Type: ModuleLoggingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "WebserverLogs")]
    pub webserver_logs: Option<ModuleLoggingConfiguration>,

    ///
    /// Defines the worker logs sent to CloudWatch Logs and the logging level to send.
    ///
    /// Required: No
    ///
    /// Type: ModuleLoggingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkerLogs")]
    pub worker_logs: Option<ModuleLoggingConfiguration>,
}

impl cfn_resources::CfnResource for LoggingConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.dag_processing_logs
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.scheduler_logs
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.task_logs
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.webserver_logs
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.worker_logs
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Defines the type of logs to send for the Apache Airflow log type (e.g. DagProcessingLogs).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ModuleLoggingConfiguration {
    ///
    /// The ARN of the CloudWatch Logs log group for each type of Apache Airflow log type that you have enabled.
    ///
    /// Note         CloudWatchLogGroupArn is available only as a return value, accessible when specified as an attribute in the         Fn:GetAtt         intrinsic function. Any value you provide for CloudWatchLogGroupArn is discarded by Amazon MWAA.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogGroupArn")]
    pub cloud_watch_log_group_arn: Option<String>,

    ///
    /// Indicates whether to enable the Apache Airflow log type (e.g. DagProcessingLogs) in CloudWatch Logs.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

    ///
    /// Defines the Apache Airflow logs to send for the log type (e.g. DagProcessingLogs) to CloudWatch Logs. Valid values: CRITICAL, ERROR, WARNING, INFO.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogLevel")]
    pub log_level: Option<String>,
}

impl cfn_resources::CfnResource for ModuleLoggingConfiguration {
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

/// The VPC networking components used to secure and enable network traffic between the AWS resources for your environment. To learn more, see About networking on Amazon MWAA.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkConfiguration {
    ///
    /// A list of one or more security group IDs. Accepts up to 5 security group IDs. A security group must be attached to the same VPC as the subnets. To learn more, see Security in your VPC on Amazon MWAA.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// A list of subnet IDs. Required to create an environment. Must be private subnets in two different availability zones. A subnet must be attached to the same VPC as the security group. To learn more, see About networking on Amazon MWAA.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for NetworkConfiguration {
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
