

/// Specifies a granted license.
///
/// Granted licenses are licenses for products that your organization purchased from AWS Marketplace      or directly from a seller who integrated their software with managed entitlements. For more information,      see Granted       licenses in the AWS License Manager User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnLicense {


    /// 
    /// Product SKU.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProductSKU")]
    pub product_sku: Option<String>,


    /// 
    /// License beneficiary.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Beneficiary")]
    pub beneficiary: Option<String>,


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
    /// Home Region of the license.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HomeRegion")]
    pub home_region: String,


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
    /// License metadata.
    /// 
    /// Required: No
    ///
    /// Type: List of Metadata
    ///
    /// Update requires: No interruption
    #[serde(rename = "LicenseMetadata")]
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
    pub license_name: String,


    /// 
    /// License status.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,


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
    /// Product name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProductName")]
    pub product_name: String,


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

}


/// Details about a consumption configuration.
#[derive(Default, serde::Serialize)]
pub struct ConsumptionConfiguration {


    /// 
    /// Renewal frequency.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RenewType")]
    pub renew_type: Option<String>,


    /// 
    /// Details about a borrow configuration.
    /// 
    /// Required: No
    ///
    /// Type: BorrowConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "BorrowConfiguration")]
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
    pub provisional_configuration: Option<ProvisionalConfiguration>,

}


/// Describes a resource entitled for use with a license.
#[derive(Default, serde::Serialize)]
pub struct Entitlement {


    /// 
    /// Entitlement unit.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    pub unit: String,


    /// 
    /// Indicates whether check-ins are allowed.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowCheckIn")]
    pub allow_check_in: Option<bool>,


    /// 
    /// Entitlement resource. Use only if the unit is None.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,


    /// 
    /// Indicates whether overages are allowed.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Overage")]
    pub overage: Option<bool>,


    /// 
    /// Entitlement name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Maximum entitlement count. Use if the unit is not None.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCount")]
    pub max_count: Option<i64>,

}


/// Details about a borrow configuration.
#[derive(Default, serde::Serialize)]
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


/// Describes key/value pairs.
#[derive(Default, serde::Serialize)]
pub struct Metadata {


    /// 
    /// The value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// Date and time range during which the license is valid, in ISO8601-UTC format.
#[derive(Default, serde::Serialize)]
pub struct ValidityDateFormat {


    /// 
    /// End of the time range.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "End")]
    pub end: String,


    /// 
    /// Start of the time range.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Begin")]
    pub begin: String,

}


/// Details about a provisional configuration.
#[derive(Default, serde::Serialize)]
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


/// Details associated with the issuer of a license.
#[derive(Default, serde::Serialize)]
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
    pub name: String,


    /// 
    /// Asymmetric KMS key from AWS Key Management Service. The KMS key must have a key usage of sign and verify,      and support the RSASSA-PSS SHA-256 signing algorithm.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SignKey")]
    pub sign_key: Option<String>,

}