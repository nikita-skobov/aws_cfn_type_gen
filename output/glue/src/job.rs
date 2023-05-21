

/// The AWS::Glue::Job resource specifies an AWS Glue job in the data       catalog. For more information, see Adding Jobs in AWS Glue and Job         Structure in the AWS Glue Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnJob {


    /// 
    /// The number of workers of a defined workerType that are allocated when a job runs.
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
    /// The name or Amazon Resource Name (ARN) of the IAM role associated with this       job.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Role")]
    pub role: String,


    /// 
    /// Glue version determines the versions of Apache Spark and Python that AWS Glue supports. The Python version indicates the version supported for jobs of type Spark.
    /// 
    /// For more information about the available AWS Glue versions and corresponding Spark and Python versions, see Glue version in the developer guide.
    /// 
    /// Jobs that are created without specifying a Glue version default to Glue 0.9.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^\w+\.\w+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlueVersion")]
    pub glue_version: Option<String>,


    /// 
    /// The default arguments for this job, specified as name-value pairs.
    /// 
    /// You can specify arguments here that your own job-execution script consumes, in       addition to arguments that AWS Glue itself consumes.
    /// 
    /// For information about how to specify and consume your own job arguments, see Calling AWS Glue APIs in Python in the AWS Glue Developer         Guide.
    /// 
    /// For information about the key-value pairs that AWS Glue consumes to set up your job,       see Special Parameters         Used by AWS Glue in the AWS Glue Developer       Guide.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultArguments")]
    pub default_arguments: Option<serde_json::Value>,


    /// 
    /// Non-overridable arguments for this job, specified as name-value pairs.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "NonOverridableArguments")]
    pub non_overridable_arguments: Option<serde_json::Value>,


    /// 
    /// The name you assign to this job definition.
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
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The maximum number of times to retry this job after a JobRun fails.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxRetries")]
    pub max_retries: Option<f64>,


    /// Specifies configuration properties of a notification.
    ///
    /// Required: No
    ///
    /// Type: NotificationProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationProperty")]
    pub notification_property: Option<NotificationProperty>,


    /// 
    /// The maximum number of concurrent runs that are allowed for this job.
    /// 
    /// Required: No
    ///
    /// Type: ExecutionProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionProperty")]
    pub execution_property: Option<ExecutionProperty>,


    /// 
    /// The code that executes a job.
    /// 
    /// Required: Yes
    ///
    /// Type: JobCommand
    ///
    /// Update requires: No interruption
    #[serde(rename = "Command")]
    pub command: JobCommand,


    /// 
    /// This field is reserved for future use.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogUri")]
    pub log_uri: Option<String>,


    /// 
    /// The tags to use with this job.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<serde_json::Value>,


    /// 
    /// Indicates whether the job is run with a standard or flexible execution class. The standard execution class is ideal for time-sensitive workloads that require fast job startup and dedicated resources.
    /// 
    /// The flexible execution class is appropriate for time-insensitive jobs whose start and completion times may vary.
    /// 
    /// Only jobs with AWS Glue version 3.0 and above and command type glueetl will be allowed to set ExecutionClass to FLEX. The flexible execution class is available for Spark jobs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionClass")]
    pub execution_class: Option<String>,


    /// 
    /// The type of predefined worker that is allocated when a job runs. Accepts a value of Standard, G.1X, or G.2X.
    /// 
    /// For the Standard worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.For the G.1X worker type, each worker maps to 1 DPU (4 vCPU, 16 GB of memory, 64 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.For the G.2X worker type, each worker maps to 2 DPU (8 vCPU, 32 GB of memory, 128 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: G.025X | G.1X | G.2X | Standard
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkerType")]
    pub worker_type: Option<String>,


    /// 
    /// A description of the job.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2048
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of the SecurityConfiguration structure to be used with this       job.
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
    /// The connections used for this job.
    /// 
    /// Required: No
    ///
    /// Type: ConnectionsList
    ///
    /// Update requires: No interruption
    #[serde(rename = "Connections")]
    pub connections: Option<ConnectionsList>,


    /// The job timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters TIMEOUT status. The default is 2,880 minutes (48 hours).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    pub timeout: Option<i64>,


    /// 
    /// The number of AWS Glue data processing units (DPUs) that can be allocated when this job runs. A DPU is a relative measure    of processing power that consists of 4 vCPUs of compute capacity and 16 GB of memory.
    /// 
    /// Do not set Max Capacity if using WorkerType and NumberOfWorkers.
    /// 
    /// The value that can be allocated for MaxCapacity depends on whether you are    running a Python shell job or an Apache Spark ETL job:
    /// 
    /// When you specify a Python shell job (JobCommand.Name="pythonshell"), you can      allocate either 0.0625 or 1 DPU. The default is 0.0625 DPU.When you specify an Apache Spark ETL job (JobCommand.Name="glueetl"), you can allocate from 2 to 100 DPUs. The default is 10 DPUs. This job type cannot have a fractional DPU allocation.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: Option<f64>,


    /// 
    /// This parameter is no longer supported. Use MaxCapacity instead.
    /// 
    /// The number of capacity units that are allocated to this job.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllocatedCapacity")]
    pub allocated_capacity: Option<f64>,

}


/// An execution property of a job.
#[derive(Default, serde::Serialize)]
pub struct ExecutionProperty {


    /// 
    /// The maximum number of concurrent runs allowed for the job. The default is 1. An error       is returned when this threshold is reached. The maximum value you can specify is       controlled by a service limit.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxConcurrentRuns")]
    pub max_concurrent_runs: Option<f64>,

}


/// Specifies code executed when a job is run.
#[derive(Default, serde::Serialize)]
pub struct JobCommand {


    /// 
    /// Specifies the Amazon Simple Storage Service (Amazon S3) path to a script that executes       a job (required).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 400000
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScriptLocation")]
    pub script_location: Option<String>,


    /// 
    /// The Python version being used to execute a Python shell job. Allowed values are 3 or 3.9. Version 2 is deprecated.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^([2-3]|3[.]9)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "PythonVersion")]
    pub python_version: Option<String>,


    /// 
    /// The name of the job command. For an Apache Spark ETL job, this must be    glueetl. For a Python shell job, it must be pythonshell.    For an Apache Spark streaming ETL job, this must be gluestreaming.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// Specifies configuration properties of a notification.
#[derive(Default, serde::Serialize)]
pub struct NotificationProperty {


    /// After a job run starts, the number of minutes to wait before sending a job run delay notification.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotifyDelayAfter")]
    pub notify_delay_after: Option<i64>,

}


/// Specifies the connections used by a job.
#[derive(Default, serde::Serialize)]
pub struct ConnectionsList {


    /// 
    /// A list of connections used by the job.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Connections")]
    pub connections: Option<Vec<String>>,

}