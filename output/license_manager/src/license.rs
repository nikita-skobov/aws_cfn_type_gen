/// Specifies a granted license.
///
/// Granted licenses are licenses for products that your organization purchased from AWS Marketplace      or directly from a seller who integrated their software with managed entitlements. For more information,      see Granted       licenses in the AWS License Manager User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLicense {
    ///
    /// License beneficiary.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Beneficiary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<cfn_resources::StrVal>,

    ///
    /// Configuration for consumption of the license.
    ///
    /// Required: Yes
    ///
    /// Type: ConsumptionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConsumptionConfiguration")]
    pub consumption_configuration: ConsumptionConfiguration,

    ///
    /// License entitlements.
    ///
    /// Required: Yes
    ///
    /// Type: List of Entitlement
    ///
    /// Update requires: No interruption
    #[serde(rename = "Entitlements")]
    pub entitlements: Vec<Entitlement>,

    ///
    /// Home Region of the license.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HomeRegion")]
    pub home_region: cfn_resources::StrVal,

    ///
    /// License issuer.
    ///
    /// Required: Yes
    ///
    /// Type: IssuerData
    ///
    /// Update requires: No interruption
    #[serde(rename = "Issuer")]
    pub issuer: IssuerData,

    ///
    /// License metadata.
    ///
    /// Required: No
    ///
    /// Type: List of Metadata
    ///
    /// Update requires: No interruption
    #[serde(rename = "LicenseMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_metadata: Option<Vec<Metadata>>,

    ///
    /// License name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LicenseName")]
    pub license_name: cfn_resources::StrVal,

    ///
    /// Product name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProductName")]
    pub product_name: cfn_resources::StrVal,

    ///
    /// Product SKU.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProductSKU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_sku: Option<cfn_resources::StrVal>,

    ///
    /// License status.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<cfn_resources::StrVal>,

    ///
    /// Date and time range during which the license is valid, in ISO8601-UTC format.
    ///
    /// Required: Yes
    ///
    /// Type: ValidityDateFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "Validity")]
    pub validity: ValidityDateFormat,

    #[serde(skip_serializing)]
    pub att_license_arn: CfnLicenselicensearn,

    #[serde(skip_serializing)]
    pub att_version: CfnLicenseversion,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLicenselicensearn;
impl CfnLicenselicensearn {
    pub fn att_name(&self) -> &'static str {
        r#"LicenseArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLicenseversion;
impl CfnLicenseversion {
    pub fn att_name(&self) -> &'static str {
        r#"Version"#
    }
}

impl cfn_resources::CfnResource for CfnLicense {
    fn type_string(&self) -> &'static str {
        "AWS::LicenseManager::License"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.consumption_configuration.validate()?;

        self.issuer.validate()?;

        self.validity.validate()?;

        Ok(())
    }
}

/// Details about a borrow configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BorrowConfiguration {
    ///
    /// Indicates whether early check-ins are allowed.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowEarlyCheckIn")]
    pub allow_early_check_in: bool,

    ///
    /// Maximum time for the borrow configuration, in minutes.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxTimeToLiveInMinutes")]
    pub max_time_to_live_in_minutes: i64,
}

impl cfn_resources::CfnResource for BorrowConfiguration {
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

/// Details about a consumption configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ConsumptionConfiguration {
    ///
    /// Details about a borrow configuration.
    ///
    /// Required: No
    ///
    /// Type: BorrowConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "BorrowConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_configuration: Option<BorrowConfiguration>,

    ///
    /// Details about a provisional configuration.
    ///
    /// Required: No
    ///
    /// Type: ProvisionalConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisionalConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisional_configuration: Option<ProvisionalConfiguration>,

    ///
    /// Renewal frequency.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RenewType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renew_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ConsumptionConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.borrow_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.provisional_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes a resource entitled for use with a license.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Entitlement {
    ///
    /// Indicates whether check-ins are allowed.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowCheckIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_check_in: Option<bool>,

    ///
    /// Maximum entitlement count. Use if the unit is not None.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i64>,

    ///
    /// Entitlement name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Indicates whether overages are allowed.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Overage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overage: Option<bool>,

    ///
    /// Entitlement unit.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    pub unit: cfn_resources::StrVal,

    ///
    /// Entitlement resource. Use only if the unit is None.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Entitlement {
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

/// Details associated with the issuer of a license.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IssuerData {
    ///
    /// Issuer name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Asymmetric KMS key from AWS Key Management Service. The KMS key must have a key usage of sign and verify,      and support the RSASSA-PSS SHA-256 signing algorithm.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SignKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_key: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for IssuerData {
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

/// Describes key/value pairs.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    ///
    /// The key name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Metadata {
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

/// Details about a provisional configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ProvisionalConfiguration {
    ///
    /// Maximum time for the provisional configuration, in minutes.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxTimeToLiveInMinutes")]
    pub max_time_to_live_in_minutes: i64,
}

impl cfn_resources::CfnResource for ProvisionalConfiguration {
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

/// Date and time range during which the license is valid, in ISO8601-UTC format.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ValidityDateFormat {
    ///
    /// Start of the time range.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Begin")]
    pub begin: cfn_resources::StrVal,

    ///
    /// End of the time range.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "End")]
    pub end: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ValidityDateFormat {
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
