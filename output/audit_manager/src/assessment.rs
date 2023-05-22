/// The AWS::AuditManager::Assessment resource is an Audit Manager     resource type that defines the scope of audit evidence collected by Audit Manager. An       Audit Manager assessment is an implementation of an Audit Manager framework.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAssessment {
    ///
    /// The destination that evidence reports are stored in for the assessment.
    ///
    /// Required: No
    ///
    /// Type: AssessmentReportsDestination
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssessmentReportsDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account: Option<AWSAccount>,

    ///
    /// The delegations that are associated with the assessment.
    ///
    /// Required: No
    ///
    /// Type: List of Delegation
    ///
    /// Update requires: No interruption
    #[serde(rename = "Delegations")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The roles that are associated with the assessment.
    ///
    /// Required: No
    ///
    /// Type: List of Role
    ///
    /// Update requires: No interruption
    #[serde(rename = "Roles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,

    ///
    /// The wrapper of AWS accounts and services that are in scope for the     assessment.
    ///
    /// Required: No
    ///
    /// Type: Scope
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AssessmentStatusEnum>,

    ///
    /// The tags that are associated with the assessment.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnAssessmentarn,

    #[serde(skip_serializing)]
    pub att_assessment_id: CfnAssessmentassessmentid,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum AssessmentStatusEnum {
    /// ACTIVE
    #[serde(rename = "ACTIVE")]
    Active,

    /// INACTIVE
    #[serde(rename = "INACTIVE")]
    Inactive,
}

impl Default for AssessmentStatusEnum {
    fn default() -> Self {
        AssessmentStatusEnum::Active
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAssessmentarn;
impl CfnAssessmentarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAssessmentassessmentid;
impl CfnAssessmentassessmentid {
    pub fn att_name(&self) -> &'static str {
        r#"AssessmentId"#
    }
}

impl cfn_resources::CfnResource for CfnAssessment {
    fn type_string(&self) -> &'static str {
        "AWS::AuditManager::Assessment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.assessment_reports_destination
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.aws_account
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.framework_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 36 as _ {
                    return Err(format!(
                        "Max validation failed on field 'framework_id'. {} is greater than 36",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.framework_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 36 as _ {
                    return Err(format!(
                        "Min validation failed on field 'framework_id'. {} is less than 36",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 300 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 300",
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

        self.scope.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The AWSAccount property type specifies the wrapper of the AWS account details, such as account ID, email address, and so on.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AWSAccount {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AWSAccount {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.email_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 320 as _ {
                    return Err(format!(
                        "Max validation failed on field 'email_address'. {} is greater than 320",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.email_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'email_address'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 12 as _ {
                    return Err(format!(
                        "Max validation failed on field 'id'. {} is greater than 12",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 12 as _ {
                    return Err(format!(
                        "Min validation failed on field 'id'. {} is less than 12",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 50 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 50",
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

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AWSService {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.service_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 40 as _ {
                    return Err(format!(
                        "Max validation failed on field 'service_name'. {} is greater than 40",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.service_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'service_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<AssessmentReportsDestinationDestinationTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum AssessmentReportsDestinationDestinationTypeEnum {
    /// S3
    #[serde(rename = "S3")]
    S3,
}

impl Default for AssessmentReportsDestinationDestinationTypeEnum {
    fn default() -> Self {
        AssessmentReportsDestinationDestinationTypeEnum::S3
    }
}

impl cfn_resources::CfnResource for AssessmentReportsDestination {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.destination {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'destination'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.destination {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'destination'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The Delegation property type specifies the assignment of a control set to a delegate for review.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Delegation {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_set_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<cfn_resources::StrVal>,

    ///
    /// Specifies when the delegation was created.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<cfn_resources::StrVal>,

    ///
    /// Specifies when the delegation was last updated.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<DelegationRoleTypeEnum>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DelegationStatusEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DelegationRoleTypeEnum {
    /// PROCESS_OWNER
    #[serde(rename = "PROCESS_OWNER")]
    Processowner,

    /// RESOURCE_OWNER
    #[serde(rename = "RESOURCE_OWNER")]
    Resourceowner,
}

impl Default for DelegationRoleTypeEnum {
    fn default() -> Self {
        DelegationRoleTypeEnum::Processowner
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DelegationStatusEnum {
    /// COMPLETE
    #[serde(rename = "COMPLETE")]
    Complete,

    /// IN_PROGRESS
    #[serde(rename = "IN_PROGRESS")]
    Inprogress,

    /// UNDER_REVIEW
    #[serde(rename = "UNDER_REVIEW")]
    Underreview,
}

impl Default for DelegationStatusEnum {
    fn default() -> Self {
        DelegationStatusEnum::Complete
    }
}

impl cfn_resources::CfnResource for Delegation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.assessment_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 36 as _ {
                    return Err(format!(
                        "Max validation failed on field 'assessment_id'. {} is greater than 36",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.assessment_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 36 as _ {
                    return Err(format!(
                        "Min validation failed on field 'assessment_id'. {} is less than 36",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.assessment_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 300 as _ {
                    return Err(format!(
                        "Max validation failed on field 'assessment_name'. {} is greater than 300",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.assessment_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'assessment_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.comment {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 350 as _ {
                    return Err(format!(
                        "Max validation failed on field 'comment'. {} is greater than 350",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.control_set_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 300 as _ {
                    return Err(format!(
                        "Max validation failed on field 'control_set_id'. {} is greater than 300",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.control_set_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'control_set_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.created_by {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'created_by'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.created_by {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'created_by'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 36 as _ {
                    return Err(format!(
                        "Max validation failed on field 'id'. {} is greater than 36",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 36 as _ {
                    return Err(format!(
                        "Min validation failed on field 'id'. {} is less than 36",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'role_arn'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'role_arn'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<RoleRoleTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RoleRoleTypeEnum {
    /// PROCESS_OWNER
    #[serde(rename = "PROCESS_OWNER")]
    Processowner,

    /// RESOURCE_OWNER
    #[serde(rename = "RESOURCE_OWNER")]
    Resourceowner,
}

impl Default for RoleRoleTypeEnum {
    fn default() -> Self {
        RoleRoleTypeEnum::Processowner
    }
}

impl cfn_resources::CfnResource for Role {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'role_arn'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'role_arn'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The Scope property type specifies the wrapper that contains the AWS accounts and services that are in scope for the assessment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Scope {
    ///
    /// The AWS accounts that are included in the scope of the assessment.
    ///
    /// Required: No
    ///
    /// Type: List of AWSAccount
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_accounts: Option<Vec<AWSAccount>>,

    ///
    /// The AWS services that are included in the scope of the assessment.
    ///
    /// Required: No
    ///
    /// Type: List of AWSService
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_services: Option<Vec<AWSService>>,
}

impl cfn_resources::CfnResource for Scope {
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
