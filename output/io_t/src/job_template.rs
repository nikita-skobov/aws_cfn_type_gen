

/// Represents a job template.
#[derive(Default, serde::Serialize)]
pub struct CfnJobTemplate {


    /// 
    /// The job document.
    /// 
    /// Required if you don't specify a value for documentSource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Document")]
    pub document: Option<String>,


    /// 
    /// An S3 link to the job document to use in the template. Required if you don't specify a value for document.
    /// 
    /// NoteIf the job document resides in an S3 bucket, you must use a placeholder link when specifying the document.The placeholder link is of the following form:        ${aws:iot:s3-presigned-url:https://s3.amazonaws.com/bucket/key}       where bucket is your bucket name and key is the object in the bucket to which you are linking.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DocumentSource")]
    pub document_source: Option<String>,


    /// 
    /// Specifies the amount of time each device has to finish its execution of the job.      A timer is started when the job execution status is set to IN_PROGRESS.      If the job execution status is not set to another terminal state before the timer expires, it will be automatically set to TIMED_OUT.
    /// 
    /// Required: No
    ///
    /// Type: TimeoutConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "TimeoutConfig")]
    pub timeout_config: Option<TimeoutConfig>,


    /// 
    /// Configuration for pre-signed S3 URLs.
    /// 
    /// Required: No
    ///
    /// Type: PresignedUrlConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "PresignedUrlConfig")]
    pub presigned_url_config: Option<PresignedUrlConfig>,


    /// 
    /// Metadata that can be used to manage the job template.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// A unique identifier for the job template. We recommend using a UUID. Alpha-numeric     characters, "-", and "_" are valid for use here.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "JobTemplateId")]
    pub job_template_id: String,


    /// 
    /// An optional configuration within the SchedulingConfig to setup a recurring maintenance window with a predetermined start time and duration for the rollout of a job document to all devices in a target group for a job.
    /// 
    /// Required: No
    ///
    /// Type: List of MaintenanceWindow
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaintenanceWindows")]
    pub maintenance_windows: Option<Vec<MaintenanceWindow>>,


    /// 
    /// The ARN of the job to use as the basis for the job template.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "JobArn")]
    pub job_arn: Option<String>,


    /// 
    /// Allows you to create a staged rollout of a job.
    /// 
    /// Required: No
    ///
    /// Type: JobExecutionsRolloutConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "JobExecutionsRolloutConfig")]
    pub job_executions_rollout_config: Option<JobExecutionsRolloutConfig>,


    /// 
    /// The criteria that determine when and how a job abort takes place.
    /// 
    /// Required: No
    ///
    /// Type: AbortConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "AbortConfig")]
    pub abort_config: Option<AbortConfig>,


    /// 
    /// Allows you to create the criteria to retry a job.
    /// 
    /// Required: No
    ///
    /// Type: JobExecutionsRetryConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "JobExecutionsRetryConfig")]
    pub job_executions_retry_config: Option<JobExecutionsRetryConfig>,


    /// 
    /// A description of the job template.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: String,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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


/// The configuration that determines how many retries are allowed for each failure       type for a job.
#[derive(Default, serde::Serialize)]
pub struct JobExecutionsRetryConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of RetryCriteria
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryCriteriaList")]
    pub retry_criteria_list: Option<Vec<RetryCriteria>>,

}


/// Allows you to create an exponential rate of rollout for a job.
#[derive(Default, serde::Serialize)]
pub struct ExponentialRolloutRate {


    /// 
    /// The exponential factor to increase the rate of rollout for a job.
    /// 
    /// AWS IoT Core supports up to one digit after the decimal (for example, 1.5, but not 1.55).
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "IncrementFactor")]
    pub increment_factor: f64,


    /// 
    /// The criteria to initiate the increase in rate of rollout for a job.
    /// 
    /// Required: Yes
    ///
    /// Type: RateIncreaseCriteria
    ///
    /// Update requires: Replacement
    #[serde(rename = "RateIncreaseCriteria")]
    pub rate_increase_criteria: RateIncreaseCriteria,


    /// 
    /// The minimum number of things that will be notified of a pending job, per minute at the start of job rollout.       This parameter allows you to define the initial rate of rollout.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "BaseRatePerMinute")]
    pub base_rate_per_minute: i64,

}


/// Allows you to create a staged rollout of a job.
#[derive(Default, serde::Serialize)]
pub struct JobExecutionsRolloutConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ExponentialRolloutRate
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExponentialRolloutRate")]
    pub exponential_rollout_rate: Option<ExponentialRolloutRate>,


    /// 
    /// The maximum number of things that will be notified of a pending job, per minute.     This parameter allows you to create a staged rollout.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaximumPerMinute")]
    pub maximum_per_minute: Option<i64>,

}


/// The criteria that determines how many retries are allowed for each failure       type for a job.
#[derive(Default, serde::Serialize)]
pub struct RetryCriteria {


    /// 
    /// The number of retries allowed for a failure type for the job.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberOfRetries")]
    pub number_of_retries: Option<i64>,


    /// 
    /// The type of job execution failures that can initiate a job retry.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailureType")]
    pub failure_type: Option<String>,

}


/// The criteria that determine when and how a job abort takes place.
#[derive(Default, serde::Serialize)]
pub struct AbortConfig {


    /// 
    /// The list of criteria that determine when and how to abort the job.
    /// 
    /// Required: Yes
    ///
    /// Type: List of AbortCriteria
    ///
    /// Update requires: Replacement
    #[serde(rename = "CriteriaList")]
    pub criteria_list: Vec<AbortCriteria>,

}


/// The criteria that determine when and how a job abort takes place.
#[derive(Default, serde::Serialize)]
pub struct AbortCriteria {


    /// 
    /// The type of job execution failures that can initiate a job abort.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FailureType")]
    pub failure_type: String,


    /// 
    /// The minimum percentage of job execution failures that must occur to initiate the job abort.
    /// 
    /// AWS IoT Core supports up to two digits after the decimal (for example, 10.9 and 10.99, but not 10.999).
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThresholdPercentage")]
    pub threshold_percentage: f64,


    /// 
    /// The type of job action to take to initiate the job abort.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Action")]
    pub action: String,


    /// 
    /// The minimum number of things which must receive job execution notifications before the job       can be aborted.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MinNumberOfExecutedThings")]
    pub min_number_of_executed_things: i64,

}


/// Configuration for pre-signed S3 URLs.
#[derive(Default, serde::Serialize)]
pub struct PresignedUrlConfig {


    /// 
    /// How long (in seconds) pre-signed URLs are valid. Valid values are 60 - 3600, the default value is 3600       seconds. Pre-signed URLs are generated when Jobs receives an MQTT request for the job document.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExpiresInSec")]
    pub expires_in_sec: Option<i64>,


    /// 
    /// The ARN of an IAM role that grants grants permission to download files from the S3 bucket where the job       data/updates are stored. The role must also grant permission for IoT to download the files.
    /// 
    /// ImportantFor information about addressing the confused deputy problem, see cross-service         confused deputy prevention in the          AWS IoT Core developer guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}


/// Allows you to define a criteria to initiate the increase in rate of rollout for a job.
#[derive(Default, serde::Serialize)]
pub struct RateIncreaseCriteria {


    /// 
    /// The threshold for number of notified things that will initiate the increase in rate of rollout.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "NumberOfNotifiedThings")]
    pub number_of_notified_things: Option<i64>,


    /// 
    /// The threshold for number of succeeded things that will initiate the increase in rate of rollout.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "NumberOfSucceededThings")]
    pub number_of_succeeded_things: Option<i64>,

}


/// Specifies the amount of time each device has to finish its execution of the job. A timer       is started when the job execution status is set to IN_PROGRESS. If the job       execution status is not set to another terminal state before the timer expires, it will      be automatically set to TIMED_OUT.
#[derive(Default, serde::Serialize)]
pub struct TimeoutConfig {


    /// 
    /// Specifies the amount of time, in minutes, this device has to finish execution of this job.       The timeout interval can be anywhere between 1 minute and 7 days (1 to 10080 minutes). The       in progress timer can't be updated and will apply to all job executions for the job. Whenever a job       execution remains in the IN_PROGRESS status for longer than this interval, the job execution will fail       and switch to the terminal TIMED_OUT status.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "InProgressTimeoutInMinutes")]
    pub in_progress_timeout_in_minutes: i64,

}


/// An optional configuration within the SchedulingConfig to setup a recurring maintenance window with a predetermined start time and duration for the rollout of a job document to all devices in a target group for a job.
#[derive(Default, serde::Serialize)]
pub struct MaintenanceWindow {


    /// 
    /// Displays the start time of the next maintenance window.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,


    /// 
    /// Displays the duration of the next maintenance window.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DurationInMinutes")]
    pub duration_in_minutes: Option<i64>,

}