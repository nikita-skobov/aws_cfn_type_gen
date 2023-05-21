/// Specifies a grant.
///
/// A grant shares the use of license entitlements with specific AWS accounts. For more information,      see Granted       licenses in the AWS License Manager User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGrant {
    ///
    /// Allowed operations for the grant.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedOperations")]
    pub allowed_operations: Option<Vec<String>>,

    ///
    /// Grant name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GrantName")]
    pub grant_name: Option<String>,

    ///
    /// Home Region of the grant.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HomeRegion")]
    pub home_region: Option<String>,

    ///
    /// License ARN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LicenseArn")]
    pub license_arn: Option<String>,

    ///
    /// The grant principals. You can specify one of the following as an Amazon Resource Name     (ARN):
    ///
    /// An AWS account, which includes only the account specified.
    ///
    /// An organizational unit (OU), which includes all accounts in the OU.
    ///
    /// An organization, which will include all accounts across your organization.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Principals")]
    pub principals: Option<Vec<String>>,

    ///
    /// Granted license status.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

impl cfn_resources::CfnResource for CfnGrant {
    fn type_string(&self) -> &'static str {
        "AWS::LicenseManager::Grant"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
