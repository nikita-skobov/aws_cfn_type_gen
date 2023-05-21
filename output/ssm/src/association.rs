

/// The AWS::SSM::Association resource creates a State Manager association for    your managed instances. A State Manager association defines the state that you want to    maintain on your instances. For example, an association can specify that anti-virus software    must be installed and running on your instances, or that certain ports must be closed. For    static targets, the association specifies a schedule for when the configuration is reapplied.    For dynamic targets, such as an AWS Resource Groups or an AWS Auto Scaling Group, State Manager    applies the configuration when new instances are added to the group. The association also    specifies actions to take when applying the configuration. For example, an association for    anti-virus software might run once a day. If the software is not installed, then State Manager    installs it. If the software is installed, but the service is not running, then the    association might instruct State Manager to start the service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAssociation {


    /// 
    /// A cron expression that specifies a schedule when the association runs. The schedule runs in  Coordinated Universal Time (UTC).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: Option<String>,


    /// 
    /// The targets for the association. You must specify the InstanceId or    Targets property. You can target all instances in an AWS account by specifying the InstanceIds key with a value of *. To view a JSON and a YAML example that targets all instances, see "Create an association for all managed instances in an AWS account" on the Examples page.
    /// 
    /// Required: Conditional
    ///
    /// Type: List of Target
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "Targets")]
    pub targets: Option<Vec<Target>>,


    /// 
    /// The severity level that is assigned to the association.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CRITICAL | HIGH | LOW | MEDIUM | UNSPECIFIED
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComplianceSeverity")]
    pub compliance_severity: Option<String>,


    /// 
    /// The number of errors that are allowed before the system stops sending requests to run the  association on additional targets. You can specify either an absolute number of errors, for  example 10, or a percentage of the target set, for example 10%. If you specify 3, for example,  the system stops sending requests when the fourth error is received. If you specify 0, then the  system stops sending requests after the first error is returned. If you run an association on 50  managed nodes and set MaxError to 10%, then the system stops sending the request  when the sixth error is received.
    /// 
    /// Executions that are already running an association when MaxErrors is reached  are allowed to complete, but some of these executions may fail as well. If you need to ensure  that there won't be more than max-errors failed executions, set MaxConcurrency to 1  so that executions proceed one at a time.
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
    /// The number of seconds the service should wait for the association status to show "Success"    before proceeding with the stack execution. If the association status doesn't show "Success"    after the specified number of seconds, then stack creation fails.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "WaitForSuccessTimeoutSeconds")]
    pub wait_for_success_timeout_seconds: Option<i64>,


    /// 
    /// Choose the parameter that will define how your automation will branch out. This target is required for associations that use an  Automation runbook and target resources by using rate controls. Automation is a capability of  AWS Systems Manager.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutomationTargetParameterName")]
    pub automation_target_parameter_name: Option<String>,


    /// 
    /// The parameters for the runtime configuration of the document.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<serde_json::Value>,


    /// 
    /// The names or Amazon Resource Names (ARNs) of the Change Calendar type documents your  associations are gated under. The associations only run when that Change Calendar is open. For  more information, see AWS Systems Manager Change   Calendar.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CalendarNames")]
    pub calendar_names: Option<Vec<String>>,


    /// 
    /// The name of the SSM document that contains the configuration information for the instance.    You can specify Command or Automation documents. The documents can    be AWS-predefined documents, documents you created, or a document that is shared with you from    another account. For SSM documents that are shared with you from other AWS accounts, you must    specify the complete SSM document ARN, in the following format:
    /// 
    /// arn:partition:ssm:region:account-id:document/document-name
    /// 
    /// For example:    arn:aws:ssm:us-east-2:12345678912:document/My-Shared-Document
    /// 
    /// For AWS-predefined documents and SSM documents you created in your account, you only need    to specify the document name. For example, AWS-ApplyPatchBaseline or My-Document.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9_\-.:/]{3,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// By default, when you create a new association, the system runs it immediately after it is      created and then according to the schedule you specified. Specify this option if you don't want      an association to run immediately after you create it. This parameter is not supported for rate expressions.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplyOnlyAtCronInterval")]
    pub apply_only_at_cron_interval: Option<bool>,


    /// 
    /// Specify a descriptive name for the association.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9_\-.]{3,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssociationName")]
    pub association_name: Option<String>,


    /// 
    /// The mode for generating association compliance. You can specify AUTO or     MANUAL. In AUTO mode, the system uses the status of the    association execution to determine the compliance status. If the association execution runs    successfully, then the association is COMPLIANT. If the association execution    doesn't run successfully, the association is NON-COMPLIANT.
    /// 
    /// In MANUAL mode, you must specify the AssociationId as a    parameter for the PutComplianceItems API action. In this case, compliance data is not managed    by State Manager. It is managed by your direct call to the PutComplianceItems API    action.
    /// 
    /// By default, all associations use AUTO mode.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTO | MANUAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "SyncCompliance")]
    pub sync_compliance: Option<String>,


    /// 
    /// The version of the SSM document to associate with the target.
    /// 
    /// ImportantNote the following important information.                      State Manager doesn't support running associations that use a new          version of a document if that document is shared from another          account. State Manager always runs the default version          of a document if shared from another account, even though the Systems Manager          console shows that a new version was processed. If you want to run          an association using a new version of a document shared form another          account, you must set the document version to          default.DocumentVersion is not valid for documents owned by AWS, such as AWS-RunPatchBaseline or AWS-UpdateSSMAgent. If you specify DocumentVersion for an AWS document, the system returns the following error: "Error occurred during operation 'CreateAssociation'." (RequestToken: <token>, HandlerErrorCode: GeneralServiceException).
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
    /// An Amazon Simple Storage Service (Amazon S3) bucket where you want to store the output  details of the request.
    /// 
    /// Required: No
    ///
    /// Type: InstanceAssociationOutputLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputLocation")]
    pub output_location: Option<InstanceAssociationOutputLocation>,


    /// 
    /// Number of days to wait after the scheduled day to run an association.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 6
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleOffset")]
    pub schedule_offset: Option<i64>,


    /// 
    /// The ID of the instance that the SSM document is associated with. You must specify the     InstanceId or Targets property.
    /// 
    /// NoteInstanceId has been deprecated. To specify an instance ID for an     association, use the Targets parameter. If you use the parameter      InstanceId, you cannot use the parameters AssociationName,      DocumentVersion, MaxErrors, MaxConcurrency,      OutputLocation, or ScheduleExpression. To use these parameters,     you must use the Targets parameter.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Pattern: (^i-(\w{8}|\w{17})$)|(^mi-\w{17}$)
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,


    /// 
    /// The maximum number of targets allowed to run the association at the same time. You can  specify a number, for example 10, or a percentage of the target set, for example 10%. The default  value is 100%, which means all targets run the association at the same time.
    /// 
    /// If a new managed node starts and attempts to run an association while Systems Manager is running   MaxConcurrency associations, the association is allowed to run. During the next  association interval, the new managed node will process its association within the limit  specified for MaxConcurrency.
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

}

impl cfn_resources::CfnResource for CfnAssociation {
    fn type_string() -> &'static str {
        "AWS::SSM::Association"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// S3OutputLocation is a property of the AWS::SSM::Association resource that specifies an Amazon S3 bucket where you want to    store the results of this association request.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3OutputLocation {


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
    /// The AWS Region of the S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputS3Region")]
    pub output_s3_region: Option<String>,


    /// 
    /// The name of the S3 bucket.
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


/// Target is a property of the AWS::SSM::Association resource that specifies the targets for an SSM document in    Systems Manager. You can target all instances in an AWS account by specifying the InstanceIds key with a value of *. To view a JSON and a YAML example that targets all instances, see "Create an association for all managed instances in an AWS account" on the Examples page.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Target {


    /// 
    /// User-defined criteria for sending commands that target managed nodes that meet the  criteria.
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


    /// 
    /// User-defined criteria that maps to Key. For example, if you specified   tag:ServerRole, you could specify value:WebServer to run a command on  instances that include EC2 tags of ServerRole,WebServer.
    /// 
    /// Depending on the type of target, the maximum number of values for a key might be lower than  the global maximum of 50.
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

}


/// InstanceAssociationOutputLocation is a property of the AWS::SSM::Association resource that specifies an Amazon S3 bucket where you want to    store the results of this association request.
///
/// For the minimal permissions required to enable Amazon S3 output for an association, see Creating   associations in the Systems Manager User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InstanceAssociationOutputLocation {


    /// 
    /// S3OutputLocation is a property of the InstanceAssociationOutputLocation property that specifies an Amazon S3 bucket where    you want to store the results of this request.
    /// 
    /// Required: No
    ///
    /// Type: S3OutputLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Location")]
    pub s3_location: Option<S3OutputLocation>,

}
