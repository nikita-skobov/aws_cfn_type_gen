/// Manages a variable.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnVariable {
    ///
    /// The data source of the variable.
    ///
    /// Valid values: EVENT | EXTERNAL_MODEL_SCORE
    ///
    /// When defining a variable within a detector, you can only use the EVENT value for DataSource when the Inline property is set to true.      If the Inline property is set false, you can use either EVENT or MODEL_SCORE for DataSource.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSource")]
    pub data_source: cfn_resources::StrVal,

    ///
    /// The data type of the variable.
    ///
    /// Valid data types: STRING | INTEGER | BOOLEAN | FLOAT
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataType")]
    pub data_type: cfn_resources::StrVal,

    ///
    /// The default value of the variable.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValue")]
    pub default_value: cfn_resources::StrVal,

    ///
    /// The description of the variable.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the variable.
    ///
    /// Pattern: ^[0-9a-z_-]+$
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The type of the variable. For more information see Variable types.
    ///
    /// Valid Values: AUTH_CODE | AVS | BILLING_ADDRESS_L1 | BILLING_ADDRESS_L2 | BILLING_CITY | BILLING_COUNTRY | BILLING_NAME | BILLING_PHONE | BILLING_STATE | BILLING_ZIP | CARD_BIN | CATEGORICAL | CURRENCY_CODE | EMAIL_ADDRESS | FINGERPRINT | FRAUD_LABEL | FREE_FORM_TEXT | IP_ADDRESS | NUMERIC | ORDER_ID | PAYMENT_TYPE | PHONE_NUMBER | PRICE | PRODUCT_CATEGORY | SHIPPING_ADDRESS_L1 | SHIPPING_ADDRESS_L2 | SHIPPING_CITY | SHIPPING_COUNTRY | SHIPPING_NAME | SHIPPING_PHONE | SHIPPING_STATE | SHIPPING_ZIP | USERAGENT
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VariableType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub variable_type: Option<VariableVariableTypeEnum>,

    #[serde(skip_serializing)]
    pub att_arn: CfnVariablearn,

    #[serde(skip_serializing)]
    pub att_created_time: CfnVariablecreatedtime,

    #[serde(skip_serializing)]
    pub att_last_updated_time: CfnVariablelastupdatedtime,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum VariableVariableTypeEnum {
    /// AUTH_CODE
    #[serde(rename = "AUTH_CODE")]
    Authcode,

    /// AVS
    #[serde(rename = "AVS")]
    Avs,

    /// BILLING_ADDRESS_L1
    #[serde(rename = "BILLING_ADDRESS_L1")]
    Billingaddressl1,

    /// BILLING_ADDRESS_L2
    #[serde(rename = "BILLING_ADDRESS_L2")]
    Billingaddressl2,

    /// BILLING_CITY
    #[serde(rename = "BILLING_CITY")]
    Billingcity,

    /// BILLING_COUNTRY
    #[serde(rename = "BILLING_COUNTRY")]
    Billingcountry,

    /// BILLING_NAME
    #[serde(rename = "BILLING_NAME")]
    Billingname,

    /// BILLING_PHONE
    #[serde(rename = "BILLING_PHONE")]
    Billingphone,

    /// BILLING_STATE
    #[serde(rename = "BILLING_STATE")]
    Billingstate,

    /// BILLING_ZIP
    #[serde(rename = "BILLING_ZIP")]
    Billingzip,

    /// CARD_BIN
    #[serde(rename = "CARD_BIN")]
    Cardbin,

    /// CATEGORICAL
    #[serde(rename = "CATEGORICAL")]
    Categorical,

    /// CURRENCY_CODE
    #[serde(rename = "CURRENCY_CODE")]
    Currencycode,

    /// EMAIL_ADDRESS
    #[serde(rename = "EMAIL_ADDRESS")]
    Emailaddress,

    /// FINGERPRINT
    #[serde(rename = "FINGERPRINT")]
    Fingerprint,

    /// FRAUD_LABEL
    #[serde(rename = "FRAUD_LABEL")]
    Fraudlabel,

    /// FREE_FORM_TEXT
    #[serde(rename = "FREE_FORM_TEXT")]
    Freeformtext,

    /// IP_ADDRESS
    #[serde(rename = "IP_ADDRESS")]
    Ipaddress,

    /// NUMERIC
    #[serde(rename = "NUMERIC")]
    Numeric,

    /// ORDER_ID
    #[serde(rename = "ORDER_ID")]
    Orderid,

    /// PAYMENT_TYPE
    #[serde(rename = "PAYMENT_TYPE")]
    Paymenttype,

    /// PHONE_NUMBER
    #[serde(rename = "PHONE_NUMBER")]
    Phonenumber,

    /// PRICE
    #[serde(rename = "PRICE")]
    Price,

    /// PRODUCT_CATEGORY
    #[serde(rename = "PRODUCT_CATEGORY")]
    Productcategory,

    /// SHIPPING_ADDRESS_L1
    #[serde(rename = "SHIPPING_ADDRESS_L1")]
    Shippingaddressl1,

    /// SHIPPING_ADDRESS_L2
    #[serde(rename = "SHIPPING_ADDRESS_L2")]
    Shippingaddressl2,

    /// SHIPPING_CITY
    #[serde(rename = "SHIPPING_CITY")]
    Shippingcity,

    /// SHIPPING_COUNTRY
    #[serde(rename = "SHIPPING_COUNTRY")]
    Shippingcountry,

    /// SHIPPING_NAME
    #[serde(rename = "SHIPPING_NAME")]
    Shippingname,

    /// SHIPPING_PHONE
    #[serde(rename = "SHIPPING_PHONE")]
    Shippingphone,

    /// SHIPPING_STATE
    #[serde(rename = "SHIPPING_STATE")]
    Shippingstate,

    /// SHIPPING_ZIP
    #[serde(rename = "SHIPPING_ZIP")]
    Shippingzip,

    /// USERAGENT
    #[serde(rename = "USERAGENT")]
    Useragent,
}

impl Default for VariableVariableTypeEnum {
    fn default() -> Self {
        VariableVariableTypeEnum::Authcode
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVariablearn;
impl CfnVariablearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVariablecreatedtime;
impl CfnVariablecreatedtime {
    pub fn att_name(&self) -> &'static str {
        r#"CreatedTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVariablelastupdatedtime;
impl CfnVariablelastupdatedtime {
    pub fn att_name(&self) -> &'static str {
        r#"LastUpdatedTime"#
    }
}

impl cfn_resources::CfnResource for CfnVariable {
    fn type_string(&self) -> &'static str {
        "AWS::FraudDetector::Variable"
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
#[serde(default)]
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
