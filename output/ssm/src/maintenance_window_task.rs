

/// The AWS::SSM::MaintenanceWindowTask resource defines information about a     task for an AWS Systems Manager maintenance window. For more information, see RegisterTaskWithMaintenanceWindow in the AWS Systems Manager API       Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnMaintenanceWindowTask {


    /// 
    /// The resource that the task uses during execution.
    /// 
    /// For RUN_COMMAND and AUTOMATION task types, TaskArn    is the SSM document name or Amazon Resource Name (ARN).
    /// 
    /// For LAMBDA tasks, TaskArn is the function name or ARN.
    /// 
    /// For STEP_FUNCTIONS tasks, TaskArn is the state machine    ARN.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1600
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskArn")]
    pub task_arn: String,


    /// 
    /// The parameters to pass to the task when it runs.
    /// 
    /// Note       TaskParameters has been deprecated. To specify parameters to pass to a task       when it runs, instead use the Parameters option in the        TaskInvocationParameters structure. For information about how Systems       Manager handles these options for the supported maintenance window task types, see        MaintenanceWindowTaskInvocationParameters.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskParameters")]
    pub task_parameters: Option<serde_json::Value>,


    /// 
    /// The specification for whether tasks should continue to run after the cutoff time specified  in the maintenance windows is reached.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CANCEL_TASK | CONTINUE_TASK
    ///
    /// Update requires: No interruption
    #[serde(rename = "CutoffBehavior")]
    pub cutoff_behavior: Option<String>,


    /// 
    /// Information about an Amazon S3 bucket to write Run Command task-level logs to.
    /// 
    /// Note       LoggingInfo has been deprecated. To specify an Amazon S3 bucket to contain logs for Run Command tasks,       instead use the OutputS3BucketName and OutputS3KeyPrefix       options in the TaskInvocationParameters structure. For information about       how Systems Manager handles these options for the supported maintenance window task       types, see AWS::SSM::MaintenanceWindowTask MaintenanceWindowRunCommandParameters.
    /// 
    /// Required: No
    ///
    /// Type: LoggingInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingInfo")]
    pub logging_info: Option<LoggingInfo>,


    /// 
    /// The type of task. Valid values: RUN_COMMAND, AUTOMATION,       LAMBDA, STEP_FUNCTIONS.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AUTOMATION | LAMBDA | RUN_COMMAND | STEP_FUNCTIONS
    ///
    /// Update requires: Replacement
    #[serde(rename = "TaskType")]
    pub task_type: String,


    /// 
    /// The maximum number of errors allowed before this task stops being scheduled.
    /// 
    /// NoteAlthough this element is listed as "Required: No", a value can be omitted only when you are   registering or updating a targetless   task You must provide a value in all other cases.For maintenance window tasks without a target specified, you can't supply a value for this   option. Instead, the system inserts a placeholder value of 1. This value doesn't   affect the running of your task.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 7
    ///
    /// Pattern: ^([1-9][0-9]*|[0]|[1-9][0-9]%|[0-9]%|100%)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxErrors")]
    pub max_errors: Option<String>,


    /// 
    /// The priority of the task in the maintenance window. The lower the number, the higher the  priority. Tasks that have the same priority are scheduled in parallel.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    pub priority: i64,


    /// 
    /// The maximum number of targets this task can be run for, in parallel.
    /// 
    /// NoteAlthough this element is listed as "Required: No", a value can be omitted only when you are   registering or updating a targetless   task You must provide a value in all other cases.For maintenance window tasks without a target specified, you can't supply a value for this   option. Instead, the system inserts a placeholder value of 1. This value doesn't   affect the running of your task.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 7
    ///
    /// Pattern: ^([1-9][0-9]*|[1-9][0-9]%|[1-9]%|100%)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxConcurrency")]
    pub max_concurrency: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) service role to use to publish Amazon Simple Notification Service (Amazon SNS) notifications for maintenance window Run Command tasks.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: Option<String>,


    /// 
    /// The ID of the maintenance window where the task is registered.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 20
    ///
    /// Pattern: ^mw-[0-9a-f]{17}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "WindowId")]
    pub window_id: String,


    /// 
    /// The targets, either instances or window target IDs.
    /// 
    /// Specify instances using Key=InstanceIds,Values=instanceid1,instanceid2      .        Specify window target IDs using Key=WindowTargetIds,Values=window-target-id-1,window-target-id-2.
    /// 
    /// Required: No
    ///
    /// Type: List of Target
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "Targets")]
    pub targets: Option<Vec<Target>>,


    /// 
    /// A description of the task.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The parameters to pass to the task when it runs. Populate only the fields that match the    task type. All other fields should be empty.
    /// 
    /// ImportantWhen you update a maintenance window task that has options specified in      TaskInvocationParameters, you must provide again all the      TaskInvocationParameters values that you want to retain. The values you do     not specify again are removed. For example, suppose that when you registered a Run Command     task, you specified TaskInvocationParameters values for Comment,      NotificationConfig, and OutputS3BucketName. If you update the     maintenance window task and specify only a different OutputS3BucketName value,     the values for Comment and NotificationConfig are removed.
    /// 
    /// Required: No
    ///
    /// Type: TaskInvocationParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskInvocationParameters")]
    pub task_invocation_parameters: Option<TaskInvocationParameters>,


    /// 
    /// The task name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9_\-.]{3,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

impl cfn_resources::CfnResource for CfnMaintenanceWindowTask {
    fn type_string() -> &'static str {
        "AWS::SSM::MaintenanceWindowTask"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The MaintenanceWindowRunCommandParameters property type specifies the    parameters for a RUN_COMMAND task type for a maintenance window task in AWS Systems Manager. This means that these parameters are the same as those for the     SendCommand API call. For more information about SendCommand    parameters, see SendCommand in the     AWS Systems Manager API Reference.
///
/// For information about available parameters in SSM Command documents, you can view the    content of the document itself in the Systems Manager console. For information, see Viewing SSM command     document content in the AWS Systems Manager User Guide.
///
/// MaintenanceWindowRunCommandParameters is a property of the TaskInvocationParameters property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MaintenanceWindowRunCommandParameters {


    /// 
    /// The S3 bucket subfolder.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 500
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputS3KeyPrefix")]
    pub output_s3_key_prefix: Option<String>,


    /// 
    /// Information about the command or commands to run.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comment")]
    pub comment: Option<String>,


    /// 
    /// The SHA-256 or SHA-1 hash created by the system when the document was created. SHA-1 hashes  have been deprecated.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentHash")]
    pub document_hash: Option<String>,


    /// 
    /// Configuration options for sending command output to Amazon CloudWatch Logs.
    /// 
    /// Required: No
    ///
    /// Type: CloudWatchOutputConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchOutputConfig")]
    pub cloud_watch_output_config: Option<CloudWatchOutputConfig>,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) service role to use to publish Amazon Simple Notification Service (Amazon SNS) notifications for maintenance window Run Command tasks.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: Option<String>,


    /// 
    /// The parameters for the RUN_COMMAND task execution.
    /// 
    /// The supported parameters are the same as those for the SendCommand API call.    For more information, see SendCommand in the     AWS Systems Manager API Reference.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<serde_json::Value>,


    /// 
    /// The SHA-256 or SHA-1 hash type. SHA-1 hashes are deprecated.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Sha1 | Sha256
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentHashType")]
    pub document_hash_type: Option<String>,


    /// 
    /// If this time is reached and the command hasn't already started running, it doesn't  run.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 30
    ///
    /// Maximum: 2592000
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeoutSeconds")]
    pub timeout_seconds: Option<i64>,


    /// 
    /// Configurations for sending notifications about command status changes on a per-managed node  basis.
    /// 
    /// Required: No
    ///
    /// Type: NotificationConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationConfig")]
    pub notification_config: Option<NotificationConfig>,


    /// 
    /// The AWS Systems Manager document (SSM document) version to use in the request. You can specify   $DEFAULT, $LATEST, or a specific version number. If you run commands  by using the AWS CLI, then you must escape the first two options by using a backslash. If you  specify a version number, then you don't need to use the backslash. For example:
    /// 
    /// --document-version "\$DEFAULT"
    /// 
    /// --document-version "\$LATEST"
    /// 
    /// --document-version "3"
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ([$]LATEST|[$]DEFAULT|^[1-9][0-9]*$)
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentVersion")]
    pub document_version: Option<String>,


    /// 
    /// The name of the Amazon Simple Storage Service (Amazon S3) bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputS3BucketName")]
    pub output_s3_bucket_name: Option<String>,

}


/// The MaintenanceWindowStepFunctionsParameters property type specifies the    parameters for the execution of a STEP_FUNCTIONS task in a Systems Manager    maintenance window.
///
/// MaintenanceWindowStepFunctionsParameters is a property of the TaskInvocationParameters property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MaintenanceWindowStepFunctionsParameters {


    /// 
    /// The name of the STEP_FUNCTIONS task.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 80
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The inputs for the STEP_FUNCTIONS task.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "Input")]
    pub input: Option<String>,

}


/// The MaintenanceWindowAutomationParameters property type specifies the    parameters for an AUTOMATION task type for a maintenance window task in AWS Systems Manager.
///
/// MaintenanceWindowAutomationParameters is a property of the TaskInvocationParameters property type.
///
/// For information about available parameters in Automation runbooks, you can view the    content of the runbook itself in the Systems Manager console. For information, see View runbook content in the AWS Systems Manager User    Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MaintenanceWindowAutomationParameters {


    /// 
    /// The parameters for the AUTOMATION task.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<serde_json::Value>,


    /// 
    /// The version of an Automation runbook to use during task execution.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ([$]LATEST|[$]DEFAULT|^[1-9][0-9]*$)
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentVersion")]
    pub document_version: Option<String>,

}


/// The Target property type specifies targets (either instances or window    target IDs). You specify instances by using Key=InstanceIds,Values=<instanceid1>,<instanceid2>. You specify window target IDs using    Key=WindowTargetIds,Values=<window-target-id-1>,<window-target-id-2> for a maintenance window task in AWS Systems Manager.
///
/// Target is a property of the AWS::SSM::MaintenanceWindowTask property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Target {


    /// 
    /// User-defined criteria that maps to Key. For example, if you specify     InstanceIds, you can specify     i-1234567890abcdef0,i-9876543210abcdef0 to run a command on two EC2 instances.    For more information about how to target instances within a maintenance window task, see     About      'register-task-with-maintenance-window' Options and Values in the AWS Systems Manager User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Vec<String>,


    /// 
    /// User-defined criteria for sending commands that target instances that meet the criteria.     Key can be InstanceIds or WindowTargetIds. For more    information about how to target instances within a maintenance window task, see About 'register-task-with-maintenance-window' Options and Values in the     AWS Systems Manager User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 163
    ///
    /// Pattern: ^[\p{L}\p{Z}\p{N}_.:/=\-@]*$|resource-groups:ResourceTypeFilters|resource-groups:Name
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

}


/// The LoggingInfo property type specifies information about the Amazon S3    bucket to write instance-level logs to.
///
/// LoggingInfo is a property of the AWS::SSM::MaintenanceWindowTask resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoggingInfo {


    /// 
    /// The name of an S3 bucket where execution logs are stored.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,


    /// 
    /// The AWS Region where the S3 bucket is located.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    pub region: String,


    /// 
    /// The Amazon S3 bucket subfolder.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 500
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Prefix")]
    pub s3_prefix: Option<String>,

}


/// The NotificationConfig property type specifies configurations for sending    notifications for a maintenance window task in AWS Systems Manager.
///
/// NotificationConfig is a property of the MaintenanceWindowRunCommandParameters property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NotificationConfig {


    /// 
    /// The different events that you can receive notifications for. These events include the    following: All (events), InProgress, Success,     TimedOut, Cancelled, Failed. To learn more about    these events, see Configuring Amazon     SNS Notifications for AWS Systems Manager in the AWS Systems Manager User     Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationEvents")]
    pub notification_events: Option<Vec<String>>,


    /// 
    /// The notification type.
    /// 
    /// Command: Receive notification when the status of a command changes.              Invocation: For commands sent to multiple instances, receive notification on      a per-instance basis when the status of a command changes.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Command | Invocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationType")]
    pub notification_type: Option<String>,


    /// 
    /// An Amazon Resource Name (ARN) for an Amazon Simple Notification Service (Amazon SNS) topic. Run  Command pushes notifications about command status changes to this topic.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationArn")]
    pub notification_arn: String,

}


/// Configuration options for sending command output to Amazon CloudWatch Logs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudWatchOutputConfig {


    /// 
    /// Enables Systems Manager to send command output to CloudWatch Logs.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchOutputEnabled")]
    pub cloud_watch_output_enabled: Option<bool>,


    /// 
    /// The name of the CloudWatch Logs log group where you want to send command output. If you  don't specify a group name, AWS Systems Manager automatically creates a log group for you. The log group  uses the following naming format:
    /// 
    /// aws/ssm/SystemsManagerDocumentName
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogGroupName")]
    pub cloud_watch_log_group_name: Option<String>,

}


/// The MaintenanceWindowLambdaParameters property type specifies the parameters    for a LAMBDA task type for a maintenance window task in AWS Systems Manager.
///
/// MaintenanceWindowLambdaParameters is a property of the TaskInvocationParameters property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MaintenanceWindowLambdaParameters {


    /// 
    /// An AWS Lambda function version or alias name. If you specify a function version, the action    uses the qualified function Amazon Resource Name (ARN) to invoke a specific Lambda function.    If you specify an alias name, the action uses the alias ARN to invoke the Lambda function    version that the alias points to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Qualifier")]
    pub qualifier: Option<String>,


    /// 
    /// JSON to provide to your AWS Lambda function as input.
    /// 
    /// ImportantAlthough Type is listed as "String" for this property, the payload content     must be formatted as a Base64-encoded binary data object.
    /// 
    /// Length Constraint: 4096
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Payload")]
    pub payload: Option<String>,


    /// 
    /// Client-specific information to pass to the AWS Lambda function that you're invoking. You can    then use the context variable to process the client information in your AWS Lambda    function.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 8000
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientContext")]
    pub client_context: Option<String>,

}


/// The TaskInvocationParameters property type specifies the task execution    parameters for a maintenance window task in AWS Systems Manager.
///
/// TaskInvocationParameters is a property of the AWS::SSM::MaintenanceWindowTask property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TaskInvocationParameters {


    /// 
    /// The parameters for a RUN_COMMAND task type.
    /// 
    /// Required: No
    ///
    /// Type: MaintenanceWindowRunCommandParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaintenanceWindowRunCommandParameters")]
    pub maintenance_window_run_command_parameters: Option<MaintenanceWindowRunCommandParameters>,


    /// 
    /// The parameters for a LAMBDA task type.
    /// 
    /// Required: No
    ///
    /// Type: MaintenanceWindowLambdaParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaintenanceWindowLambdaParameters")]
    pub maintenance_window_lambda_parameters: Option<MaintenanceWindowLambdaParameters>,


    /// 
    /// The parameters for a STEP_FUNCTIONS task type.
    /// 
    /// Required: No
    ///
    /// Type: MaintenanceWindowStepFunctionsParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaintenanceWindowStepFunctionsParameters")]
    pub maintenance_window_step_functions_parameters: Option<MaintenanceWindowStepFunctionsParameters>,


    /// 
    /// The parameters for an AUTOMATION task type.
    /// 
    /// Required: No
    ///
    /// Type: MaintenanceWindowAutomationParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaintenanceWindowAutomationParameters")]
    pub maintenance_window_automation_parameters: Option<MaintenanceWindowAutomationParameters>,

}
