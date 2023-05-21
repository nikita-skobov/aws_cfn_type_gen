

/// The AWS::AuditManager::Assessment resource is an Audit Manager     resource type that defines the scope of audit evidence collected by Audit Manager. An       Audit Manager assessment is an implementation of an Audit Manager framework.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAssessment {


    /// 
    /// The roles that are associated with the assessment.
    /// 
    /// Required: No
    ///
    /// Type: List of Role
    ///
    /// Update requires: No interruption
    #[serde(rename = "Roles")]
    pub roles: Option<Vec<Role>>,


    /// 
    /// The name of the assessment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 300
    ///
    /// Pattern: ^[^\\]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The delegations that are associated with the assessment.
    /// 
    /// Required: No
    ///
    /// Type: List of Delegation
    ///
    /// Update requires: No interruption
    #[serde(rename = "Delegations")]
    pub delegations: Option<Vec<Delegation>>,


    /// 
    /// The description of the assessment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1000
    ///
    /// Pattern: ^[\w\W\s\S]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The wrapper of AWS accounts and services that are in scope for the     assessment.
    /// 
    /// Required: No
    ///
    /// Type: Scope
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scope")]
    pub scope: Option<Scope>,


    /// 
    /// The tags that are associated with the assessment.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The unique identifier for the framework.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 36
    ///
    /// Maximum: 36
    ///
    /// Pattern: ^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FrameworkId")]
    pub framework_id: Option<String>,


    /// 
    /// The destination that evidence reports are stored in for the assessment.
    /// 
    /// Required: No
    ///
    /// Type: AssessmentReportsDestination
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssessmentReportsDestination")]
    pub assessment_reports_destination: Option<AssessmentReportsDestination>,


    /// 
    /// The AWS account that's associated with the assessment.
    /// 
    /// Required: No
    ///
    /// Type: AWSAccount
    ///
    /// Update requires: Replacement
    #[serde(rename = "AwsAccount")]
    pub aws_account: Option<AWSAccount>,


    /// 
    /// The overall status of the assessment.
    /// 
    /// When you create a new assessment, the initial Status value is always       ACTIVE. When you create an assessment, even if you specify the value as       INACTIVE, the value overrides to ACTIVE.
    /// 
    /// After you create an assessment, you can change the value of the Status     property at any time. For example, when you want to stop collecting evidence for your     assessment, you can change the assessment status to INACTIVE.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ACTIVE | INACTIVE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,

}

impl cfn_resources::CfnResource for CfnAssessment {
    fn type_string() -> &'static str {
        "AWS::AuditManager::Assessment"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The Role property type specifies the wrapper that contains AWS Audit Manager role information, such as the role type and IAM Amazon     Resource Name (ARN).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Role {


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^arn:.*:iam:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// The type of customer persona.
    /// 
    /// NoteIn CreateAssessment, roleType can only be        PROCESS_OWNER. In UpdateSettings, roleType can only be        PROCESS_OWNER.In BatchCreateDelegationByAssessment, roleType can only be        RESOURCE_OWNER.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PROCESS_OWNER | RESOURCE_OWNER
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleType")]
    pub role_type: Option<String>,

}


/// The Delegation property type specifies the assignment of a control set to a delegate for review.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Delegation {


    /// 
    /// The comment that's related to the delegation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 350
    ///
    /// Pattern: ^[\w\W\s\S]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comment")]
    pub comment: Option<String>,


    /// 
    /// The name of the assessment that's associated with the delegation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 300
    ///
    /// Pattern: ^[^\\]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssessmentName")]
    pub assessment_name: Option<String>,


    /// 
    /// Specifies when the delegation was created.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreationTime")]
    pub creation_time: Option<f64>,


    /// 
    /// The user or role that created the delegation.
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 100
    /// 
    /// Pattern: ^[a-zA-Z0-9-_()\\[\\]\\s]+$
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedBy")]
    pub created_by: Option<String>,


    /// 
    /// The type of customer persona.
    /// 
    /// NoteIn CreateAssessment, roleType can only be        PROCESS_OWNER. In UpdateSettings, roleType can only be        PROCESS_OWNER.In BatchCreateDelegationByAssessment, roleType can only be        RESOURCE_OWNER.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PROCESS_OWNER | RESOURCE_OWNER
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleType")]
    pub role_type: Option<String>,


    /// 
    /// The unique identifier for the delegation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 36
    ///
    /// Maximum: 36
    ///
    /// Pattern: ^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: Option<String>,


    /// 
    /// The identifier for the assessment that's associated with the delegation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 36
    ///
    /// Maximum: 36
    ///
    /// Pattern: ^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssessmentId")]
    pub assessment_id: Option<String>,


    /// 
    /// The status of the delegation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COMPLETE | IN_PROGRESS | UNDER_REVIEW
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,


    /// 
    /// The identifier for the control set that's associated with the delegation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 300
    ///
    /// Pattern: ^[\w\W\s\S]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ControlSetId")]
    pub control_set_id: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^arn:.*:iam:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// Specifies when the delegation was last updated.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastUpdated")]
    pub last_updated: Option<f64>,

}


/// The AssessmentReportsDestination property type specifies the location in     which AWS Audit Manager saves assessment reports for the given assessment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AssessmentReportsDestination {


    /// 
    /// The destination of the assessment report.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^(S|s)3:\/\/[a-zA-Z0-9\-\.\(\)\'\*\_\!\/]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: Option<String>,


    /// 
    /// The destination type, such as Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: S3
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationType")]
    pub destination_type: Option<String>,

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


/// The AWSService property type specifies an AWS service     such as Amazon S3, AWS CloudTrail, and so on.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AWSService {


    /// 
    /// The name of the AWS service.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 40
    ///
    /// Pattern: ^[a-zA-Z0-9-\s().]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,

}


/// The AWSAccount property type specifies the wrapper of the AWS account details, such as account ID, email address, and so on.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AWSAccount {


    /// 
    /// The name of the AWS account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: ^[\u0020-\u007E]+$
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The email address that's associated with the AWS account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 320
    ///
    /// Pattern: ^.*@.*$
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "EmailAddress")]
    pub email_address: Option<String>,


    /// 
    /// The identifier for the AWS account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 12
    ///
    /// Pattern: ^[0-9]{12}$
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Id")]
    pub id: Option<String>,

}


/// The Scope property type specifies the wrapper that contains the AWS accounts and services that are in scope for the assessment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Scope {


    /// 
    /// The AWS services that are included in the scope of the assessment.
    /// 
    /// Required: No
    ///
    /// Type: List of AWSService
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsServices")]
    pub aws_services: Option<Vec<AWSService>>,


    /// 
    /// The AWS accounts that are included in the scope of the assessment.
    /// 
    /// Required: No
    ///
    /// Type: List of AWSAccount
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsAccounts")]
    pub aws_accounts: Option<Vec<AWSAccount>>,

}
