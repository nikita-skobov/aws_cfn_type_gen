/// The AWS::ApiGateway::UsagePlan resource creates a usage plan for deployed APIs. A usage plan sets a target for the throttling and quota limits on individual client API keys. For more information, see Creating and Using API Usage Plans in Amazon API Gateway in the API Gateway Developer Guide.
///
/// In some cases clients can exceed the targets that you set. Don’t rely on usage plans to control costs. Consider using AWS Budgets to monitor costs     and AWS WAF to manage API requests.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnUsagePlan {
    ///
    /// The associated API stages of a usage plan.
    ///
    /// Required: No
    ///
    /// Type: List of ApiStage
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_stages: Option<Vec<ApiStage>>,

    ///
    /// The description of a usage plan.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The target maximum number of permitted requests per a given unit time interval.
    ///
    /// Required: No
    ///
    /// Type: QuotaSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<QuotaSettings>,

    ///
    /// The collection of tags. Each tag element is associated with a given resource.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// A map containing method level throttling information for API stage in a usage plan.
    ///
    /// Required: No
    ///
    /// Type: ThrottleSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Throttle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle: Option<ThrottleSettings>,

    ///
    /// The name of a usage plan.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UsagePlanName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_plan_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnUsagePlan {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGateway::UsagePlan"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.quota.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.throttle
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// API stage name of the associated API stage in a usage plan.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ApiStage {
    ///
    /// API Id of the associated API stage in a usage plan.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<cfn_resources::StrVal>,

    ///
    /// API stage name of the associated API stage in a usage plan.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<cfn_resources::StrVal>,

    ///
    /// Map containing method level throttling information for API stage in a usage plan.
    ///
    /// Required: No
    ///
    /// Type: Map of ThrottleSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Throttle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle: Option<std::collections::HashMap<String, ThrottleSettings>>,
}

impl cfn_resources::CfnResource for ApiStage {
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

/// QuotaSettings is a property of the AWS::ApiGateway::UsagePlan resource that specifies a target for the maximum number of requests users can make to your REST APIs.
///
/// In some cases clients can exceed the targets that you set. Don’t rely on usage plans to control costs. Consider using AWS Budgets to monitor costs     and AWS WAF to manage API requests.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct QuotaSettings {
    ///
    /// The target maximum number of requests that can be made in a given time period.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    ///
    /// The number of requests subtracted from the given limit in the initial time period.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    ///
    /// The time period in which the limit applies. Valid values are "DAY", "WEEK" or "MONTH".
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | MONTH | WEEK
    ///
    /// Update requires: No interruption
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<QuotaSettingsPeriodEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum QuotaSettingsPeriodEnum {
    /// DAY
    #[serde(rename = "DAY")]
    Day,

    /// MONTH
    #[serde(rename = "MONTH")]
    Month,

    /// WEEK
    #[serde(rename = "WEEK")]
    Week,
}

impl Default for QuotaSettingsPeriodEnum {
    fn default() -> Self {
        QuotaSettingsPeriodEnum::Day
    }
}

impl cfn_resources::CfnResource for QuotaSettings {
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

/// ThrottleSettings is a property of the AWS::ApiGateway::UsagePlan resource that specifies the overall request rate (average requests per second) and burst capacity when users call your REST APIs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ThrottleSettings {
    ///
    /// The API target request burst rate limit. This allows more requests through for a period of time than the target rate limit.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BurstLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burst_limit: Option<i64>,

    ///
    /// The API target request rate limit.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "RateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<f64>,
}

impl cfn_resources::CfnResource for ThrottleSettings {
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
