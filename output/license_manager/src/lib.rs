
pub mod cfn_grant {

#[derive(serde::Serialize, Default)]
pub struct CfnGrant {
    /// Home region for the created grant.
    #[serde(rename = "HomeRegion")]
    pub home_region: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// License Arn for the grant.
    #[serde(rename = "LicenseArn")]
    pub license_arn: Option<Arn>,
    /// Name for the created Grant.
    #[serde(rename = "GrantName")]
    pub grant_name: Option<String>,
    /// List of Arn
    #[serde(rename = "Principals")]
    pub principals: Option<Vec<Arn>>,
    /// No documentation provided by AWS
    #[serde(rename = "AllowedOperations")]
    pub allowed_operations: Option<Vec<String>>,

}

pub type Arn = String;

}

pub mod cfn_license {

#[derive(serde::Serialize, Default)]
pub struct CfnLicense {
    /// List of Entitlement
    #[serde(rename = "Entitlements")]
    pub entitlements: Vec<Entitlement>,
    /// No documentation provided by AWS
    #[serde(rename = "Issuer")]
    pub issuer: IssuerData,
    /// ProductSKU of the license.
    #[serde(rename = "ProductSKU")]
    pub product_sku: Option<String>,
    /// Home region for the created license.
    #[serde(rename = "HomeRegion")]
    pub home_region: String,
    /// No documentation provided by AWS
    #[serde(rename = "ConsumptionConfiguration")]
    pub consumption_configuration: ConsumptionConfiguration,
    /// No documentation provided by AWS
    #[serde(rename = "Validity")]
    pub validity: ValidityDateFormat,
    /// Name for the created license.
    #[serde(rename = "LicenseName")]
    pub license_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Status")]
    pub status: Option<LicenseStatus>,
    /// Beneficiary of the license.
    #[serde(rename = "Beneficiary")]
    pub beneficiary: Option<String>,
    /// Product name for the created license.
    #[serde(rename = "ProductName")]
    pub product_name: String,
    /// List of Metadata
    #[serde(rename = "LicenseMetadata")]
    pub license_metadata: Option<Vec<Metadata>>,

}

pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct ProvisionalConfiguration {
    #[serde(rename = "MaxTimeToLiveInMinutes")]
    pub max_time_to_live_in_minutes: usize,

}

#[derive(serde::Serialize, Default)]
pub struct ValidityDateFormat {
    #[serde(rename = "Begin")]
    pub begin: String,
    #[serde(rename = "End")]
    pub end: String,

}

#[derive(serde::Serialize, Default)]
pub struct IssuerData {
    #[serde(rename = "SignKey")]
    pub sign_key: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct BorrowConfiguration {
    #[serde(rename = "MaxTimeToLiveInMinutes")]
    pub max_time_to_live_in_minutes: usize,
    #[serde(rename = "AllowEarlyCheckIn")]
    pub allow_early_check_in: bool,

}

#[derive(serde::Serialize, Default)]
pub struct Metadata {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Entitlement {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Unit")]
    pub unit: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Overage")]
    pub overage: Option<bool>,
    #[serde(rename = "AllowCheckIn")]
    pub allow_check_in: Option<bool>,
    #[serde(rename = "MaxCount")]
    pub max_count: Option<usize>,

}
pub type LicenseStatus = String;
#[derive(serde::Serialize, Default)]
pub struct ConsumptionConfiguration {
    #[serde(rename = "ProvisionalConfiguration")]
    pub provisional_configuration: Option<ProvisionalConfiguration>,
    #[serde(rename = "RenewType")]
    pub renew_type: Option<String>,
    #[serde(rename = "BorrowConfiguration")]
    pub borrow_configuration: Option<BorrowConfiguration>,

}


}
