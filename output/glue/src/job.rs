/// The AWS::Glue::Job resource specifies an AWS Glue job in the data       catalog. For more information, see Adding Jobs in AWS Glue and Job         Structure in the AWS Glue Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnJob {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_capacity: Option<f64>,

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
    /// The connections used for this job.
    ///
    /// Required: No
    ///
    /// Type: ConnectionsList
    ///
    /// Update requires: No interruption
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<ConnectionsList>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_arguments: Option<serde_json::Value>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_class: Option<cfn_resources::StrVal>,

    ///
    /// The maximum number of concurrent runs that are allowed for this job.
    ///
    /// Required: No
    ///
    /// Type: ExecutionProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_property: Option<ExecutionProperty>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_version: Option<cfn_resources::StrVal>,

    ///
    /// This field is reserved for future use.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,

    ///
    /// The maximum number of times to retry this job after a JobRun fails.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<f64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Non-overridable arguments for this job, specified as name-value pairs.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "NonOverridableArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_overridable_arguments: Option<serde_json::Value>,

    /// Specifies configuration properties of a notification.
    ///
    /// Required: No
    ///
    /// Type: NotificationProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub role: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<cfn_resources::StrVal>,

    ///
    /// The tags to use with this job.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    /// The job timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters TIMEOUT status. The default is 2,880 minutes (48 hours).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<JobWorkerTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum JobWorkerTypeEnum {
    /// G.025X
    #[serde(rename = "G.025X")]
    G025x,

    /// G.1X
    #[serde(rename = "G.1X")]
    G1x,

    /// G.2X
    #[serde(rename = "G.2X")]
    G2x,

    /// Standard
    #[serde(rename = "Standard")]
    Standard,
}

impl Default for JobWorkerTypeEnum {
    fn default() -> Self {
        JobWorkerTypeEnum::G025x
    }
}

impl cfn_resources::CfnResource for CfnJob {
    fn type_string(&self) -> &'static str {
        "AWS::Glue::Job"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.command.validate()?;

        self.connections
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 2048",
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

        self.execution_property
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.glue_version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'glue_version'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.glue_version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'glue_version'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.notification_property
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

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

/// Specifies the connections used by a job.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for ConnectionsList {
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

/// An execution property of a job.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_runs: Option<f64>,
}

impl cfn_resources::CfnResource for ExecutionProperty {
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

/// Specifies code executed when a job is run.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JobCommand {
    ///
    /// The name of the job command. For an Apache Spark ETL job, this must be    glueetl. For a Python shell job, it must be pythonshell.    For an Apache Spark streaming ETL job, this must be gluestreaming.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_version: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_location: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for JobCommand {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.script_location {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 400000 as _ {
                    return Err(format!("Max validation failed on field 'script_location'. {} is greater than 400000", s.len()));
                }
            }
        }

        Ok(())
    }
}

/// Specifies configuration properties of a notification.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct NotificationProperty {
    /// After a job run starts, the number of minutes to wait before sending a job run delay notification.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotifyDelayAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_delay_after: Option<i64>,
}

impl cfn_resources::CfnResource for NotificationProperty {
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
