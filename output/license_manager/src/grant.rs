/// Specifies a grant.
///
/// A grant shares the use of license entitlements with specific AWS accounts. For more information,      see Granted       licenses in the AWS License Manager User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_name: Option<cfn_resources::StrVal>,

    ///
    /// Home Region of the grant.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HomeRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<cfn_resources::StrVal>,

    ///
    /// License ARN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LicenseArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_arn: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_grant_arn: CfnGrantgrantarn,

    #[serde(skip_serializing)]
    pub att_version: CfnGrantversion,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGrantgrantarn;
impl CfnGrantgrantarn {
    pub fn att_name(&self) -> &'static str {
        r#"GrantArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGrantversion;
impl CfnGrantversion {
    pub fn att_name(&self) -> &'static str {
        r#"Version"#
    }
}

impl cfn_resources::CfnResource for CfnGrant {
    fn type_string(&self) -> &'static str {
        "AWS::LicenseManager::Grant"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
