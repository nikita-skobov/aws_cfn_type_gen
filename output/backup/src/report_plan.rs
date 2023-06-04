/// Creates a report plan. A report plan is a document that contains information about the     contents of the report and where AWS Backup will deliver it.
///
/// If you call CreateReportPlan with a plan that already exists, you receive     an AlreadyExistsException exception.
///
/// For a sample AWS CloudFormation template, see the AWS Backup Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnReportPlan {
    ///
    /// Contains information about where and how to deliver your reports, specifically your       Amazon S3 bucket name, S3 key prefix, and the formats of your reports.
    ///
    /// Required: Yes
    ///
    /// Type: ReportDeliveryChannel
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReportDeliveryChannel")]
    pub report_delivery_channel: ReportDeliveryChannel,

    ///
    /// An optional description of the report plan with a maximum 1,024 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReportPlanDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plan_description: Option<cfn_resources::StrVal>,

    ///
    /// The unique name of the report plan. This name is between 1 and 256 characters starting     with a letter, and consisting of letters (a-z, A-Z), numbers (0-9), and underscores     (_).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z][_a-zA-Z0-9]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReportPlanName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plan_name: Option<cfn_resources::StrVal>,

    ///
    /// A list of tags to tag your report plan.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReportPlanTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_plan_tags: Option<Vec<Tag>>,

    ///
    /// Identifies the report template for the report. Reports are built using a report     template. The report templates are:
    ///
    /// RESOURCE_COMPLIANCE_REPORT | CONTROL_COMPLIANCE_REPORT | BACKUP_JOB_REPORT |       COPY_JOB_REPORT | RESTORE_JOB_REPORT
    ///
    /// If the report template is RESOURCE_COMPLIANCE_REPORT or       CONTROL_COMPLIANCE_REPORT, this API resource also describes the report     coverage by AWS Regions and frameworks.
    ///
    /// Required: Yes
    ///
    /// Type: ReportSetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReportSetting")]
    pub report_setting: ReportSetting,

    #[serde(skip_serializing)]
    pub att_report_plan_arn: CfnReportPlanreportplanarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnReportPlanreportplanarn;
impl CfnReportPlanreportplanarn {
    pub fn att_name(&self) -> &'static str {
        r#"ReportPlanArn"#
    }
}

impl cfn_resources::CfnResource for CfnReportPlan {
    fn type_string(&self) -> &'static str {
        "AWS::Backup::ReportPlan"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.report_delivery_channel.validate()?;

        if let Some(the_val) = &self.report_plan_description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!("Max validation failed on field 'report_plan_description'. {} is greater than 1024", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.report_plan_description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!("Min validation failed on field 'report_plan_description'. {} is less than 0", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.report_plan_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'report_plan_name'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.report_plan_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'report_plan_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.report_setting.validate()?;

        Ok(())
    }
}

/// Contains information from your report plan about where to deliver your reports,     specifically your Amazon S3 bucket name, S3 key prefix, and the formats of your     reports.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ReportDeliveryChannel {
    ///
    /// A list of the format of your reports: CSV, JSON, or both. If     not specified, the default format is CSV.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Formats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formats: Option<Vec<String>>,

    ///
    /// The unique name of the S3 bucket that receives your reports.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: cfn_resources::StrVal,

    ///
    /// The prefix for where AWS Backup Audit Manager delivers your reports to Amazon S3. The prefix is this part of the following path:       s3://your-bucket-name/prefix/Backup/us-west-2/year/month/day/report-name.     If not specified, there is no prefix.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ReportDeliveryChannel {
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

/// Contains detailed information about a report setting.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ReportSetting {
    ///
    /// These are the accounts to be included in the report.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Accounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>,

    ///
    /// The Amazon Resource Names (ARNs) of the frameworks a report covers.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameworkArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_arns: Option<Vec<String>>,

    ///
    /// These are the Organizational Units to be included in the report.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_units: Option<Vec<String>>,

    ///
    /// These are the Regions to be included in the report.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,

    ///
    /// Identifies the report template for the report. Reports are built using a report     template. The report templates are:
    ///
    /// RESOURCE_COMPLIANCE_REPORT | CONTROL_COMPLIANCE_REPORT | BACKUP_JOB_REPORT |       COPY_JOB_REPORT | RESTORE_JOB_REPORT
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReportTemplate")]
    pub report_template: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ReportSetting {
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
