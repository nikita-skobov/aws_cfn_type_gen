/// Use the AWS::IoT::ScheduledAudit resource to create a scheduled audit that     is run at a specified time interval. For API reference, see CreateScheduleAudit     and for general information, see Audit.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnScheduledAudit {
    ///
    /// The day of the month on which the scheduled audit is run (if the      frequency is "MONTHLY").     If days 29-31 are specified, and the month does not have that many     days, the audit takes place on the "LAST" day of the month.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DayOfMonth")]
    pub day_of_month: Option<String>,

    ///
    /// The day of the week on which the scheduled audit is run (if the       frequency is "WEEKLY" or "BIWEEKLY").
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: Option<String>,

    ///
    /// How often the scheduled audit occurs.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Frequency")]
    pub frequency: String,

    ///
    /// The name of the scheduled audit.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ScheduledAuditName")]
    pub scheduled_audit_name: Option<String>,

    ///
    /// Metadata that can be used to manage the scheduled audit.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// Which checks are performed during the scheduled audit. Checks must be enabled for your     account. (Use DescribeAccountAuditConfiguration to see the list of all checks,     including those that are enabled or use UpdateAccountAuditConfiguration to     select which checks are enabled.)
    ///
    /// The following checks are currently aviable:
    ///
    /// AUTHENTICATED_COGNITO_ROLE_OVERLY_PERMISSIVE_CHECKCA_CERTIFICATE_EXPIRING_CHECKCA_CERTIFICATE_KEY_QUALITY_CHECKCONFLICTING_CLIENT_IDS_CHECKDEVICE_CERTIFICATE_EXPIRING_CHECKDEVICE_CERTIFICATE_KEY_QUALITY_CHECKDEVICE_CERTIFICATE_SHARED_CHECKIOT_POLICY_OVERLY_PERMISSIVE_CHECKIOT_ROLE_ALIAS_ALLOWS_ACCESS_TO_UNUSED_SERVICES_CHECKIOT_ROLE_ALIAS_OVERLY_PERMISSIVE_CHECKLOGGING_DISABLED_CHECKREVOKED_CA_CERTIFICATE_STILL_ACTIVE_CHECKREVOKED_DEVICE_CERTIFICATE_STILL_ACTIVE_CHECKUNAUTHENTICATED_COGNITO_ROLE_OVERLY_PERMISSIVE_CHECK
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetCheckNames")]
    pub target_check_names: Vec<String>,
}

impl cfn_resources::CfnResource for CfnScheduledAudit {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::ScheduledAudit"
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
