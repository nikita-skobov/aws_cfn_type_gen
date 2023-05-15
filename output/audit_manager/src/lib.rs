
pub mod cfn_assessment {

#[derive(serde::Serialize, Default)]
pub struct CfnAssessment {
    /// The list of delegations.
    #[serde(rename = "Delegations")]
    pub delegations: Option<Vec<Delegation>>,
    /// The identifier for the specified framework.
    #[serde(rename = "FrameworkId")]
    pub framework_id: Option<FrameworkId>,
    /// No documentation provided by AWS
    #[serde(rename = "AwsAccount")]
    pub aws_account: Option<AWSAccount>,
    /// The tags associated with the assessment.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The wrapper that contains the AWS accounts and AWS services in scope for the assessment.
    #[serde(rename = "Scope")]
    pub scope: Option<Scope>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<AssessmentName>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<AssessmentDescription>,
    /// The destination in which evidence reports are stored for the specified assessment.
    #[serde(rename = "AssessmentReportsDestination")]
    pub assessment_reports_destination: Option<AssessmentReportsDestination>,
    /// No documentation provided by AWS
    #[serde(rename = "Status")]
    pub status: Option<AssessmentStatus>,
    /// The list of roles for the specified assessment.
    #[serde(rename = "Roles")]
    pub roles: Option<Vec<Role>>,

}

pub type DelegationStatus = String;
#[derive(serde::Serialize, Default)]
pub struct Scope {
    #[serde(rename = "AwsServices")]
    pub aws_services: Option<Vec<AWSService>>,
    #[serde(rename = "AwsAccounts")]
    pub aws_accounts: Option<Vec<AWSAccount>>,

}
pub type AWSServiceName = String;pub type Timestamp = f64;pub type RoleType = String;
#[derive(serde::Serialize, Default)]
pub struct AWSAccount {
    #[serde(rename = "Id")]
    pub id: Option<AccountId>,
    #[serde(rename = "EmailAddress")]
    pub email_address: Option<EmailAddress>,
    #[serde(rename = "Name")]
    pub name: Option<AccountName>,

}
pub type DelegationComment = String;pub type EmailAddress = String;pub type UUID = String;
#[derive(serde::Serialize, Default)]
pub struct Role {
    #[serde(rename = "RoleType")]
    pub role_type: Option<RoleType>,
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<IamArn>,

}

#[derive(serde::Serialize, Default)]
pub struct AssessmentReportsDestination {
    #[serde(rename = "DestinationType")]
    pub destination_type: Option<AssessmentReportDestinationType>,
    #[serde(rename = "Destination")]
    pub destination: Option<S3Url>,

}
pub type ControlSetId = String;pub type AssessmentName = String;pub type AssessmentStatus = String;pub type AssessmentArn = String;pub type S3Url = String;
#[derive(serde::Serialize, Default)]
pub struct AWSService {
    #[serde(rename = "ServiceName")]
    pub service_name: Option<AWSServiceName>,

}
pub type IamArn = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type FrameworkId = String;pub type AccountName = String;pub type AccountId = String;pub type CreatedBy = String;pub type AssessmentReportDestinationType = String;pub type AssessmentDescription = String;
#[derive(serde::Serialize, Default)]
pub struct Delegation {
    #[serde(rename = "LastUpdated")]
    pub last_updated: Option<Timestamp>,
    #[serde(rename = "Comment")]
    pub comment: Option<DelegationComment>,
    #[serde(rename = "AssessmentName")]
    pub assessment_name: Option<AssessmentName>,
    #[serde(rename = "Id")]
    pub id: Option<UUID>,
    #[serde(rename = "AssessmentId")]
    pub assessment_id: Option<UUID>,
    #[serde(rename = "CreationTime")]
    pub creation_time: Option<Timestamp>,
    #[serde(rename = "ControlSetId")]
    pub control_set_id: Option<ControlSetId>,
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<IamArn>,
    #[serde(rename = "CreatedBy")]
    pub created_by: Option<CreatedBy>,
    #[serde(rename = "RoleType")]
    pub role_type: Option<RoleType>,
    #[serde(rename = "Status")]
    pub status: Option<DelegationStatus>,

}


}
