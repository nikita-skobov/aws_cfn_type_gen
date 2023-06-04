/// The AWS::Budgets::Budget resource allows customers to take pre-defined actions that will trigger once a budget threshold has been exceeded. creates, replaces, or deletes budgets for Billing and Cost Management. 			For more information, see 			Managing Your Costs with Budgets 			in the AWS Billing and Cost Management User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnBudget {
    ///
    /// The budget object that you want to create.
    ///
    /// Required: Yes
    ///
    /// Type: BudgetData
    ///
    /// Update requires: No interruption
    #[serde(rename = "Budget")]
    pub budget: BudgetData,

    ///
    /// A notification that you want to associate with a budget. A budget can have up to five notifications, and each notification can have one SNS subscriber and up to 10 email subscribers. If you include notifications and subscribers in your CreateBudget call, AWS creates the notifications and subscribers for you.
    ///
    /// Required: No
    ///
    /// Type: List of NotificationWithSubscribers
    ///
    /// Maximum: 10
    ///
    /// Update requires: Replacement
    #[serde(rename = "NotificationsWithSubscribers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_with_subscribers: Option<Vec<NotificationWithSubscribers>>,
}

impl cfn_resources::CfnResource for CfnBudget {
    fn type_string(&self) -> &'static str {
        "AWS::Budgets::Budget"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.budget.validate()?;

        if let Some(the_val) = &self.notifications_with_subscribers {
            if the_val.len() > 10 as _ {
                return Err(format!("Max validation failed on field 'notifications_with_subscribers'. {} is greater than 10", the_val.len()));
            }
        }

        Ok(())
    }
}

/// The AutoAdjustData property type specifies Property description not available. for an AWS::Budgets::Budget.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AutoAdjustData {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoAdjustType")]
    pub auto_adjust_type: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: HistoricalOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "HistoricalOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_options: Option<HistoricalOptions>,
}

impl cfn_resources::CfnResource for AutoAdjustData {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.historical_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Represents the output of the CreateBudget operation. The content consists of the detailed metadata and data file information, and the current status of the budget object.
///
/// This is the Amazon Resource Name (ARN) pattern for a budget:
///
/// arn:aws:budgets::AccountId:budget/budgetName
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BudgetData {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AutoAdjustData
    ///
    /// Update requires: Replacement
    #[serde(rename = "AutoAdjustData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_adjust_data: Option<AutoAdjustData>,

    ///
    /// The total amount of cost, usage, RI utilization, RI coverage, Savings Plans utilization, or 			Savings Plans coverage that you want to track with your budget.
    ///
    /// BudgetLimit is required for cost or usage budgets, but optional for RI or 			Savings Plans utilization or coverage budgets. RI and Savings Plans utilization or 			coverage budgets default to 100. This is the only valid value for RI or 			Savings Plans utilization or coverage budgets. You can't use BudgetLimit 			with PlannedBudgetLimits for CreateBudget and 				UpdateBudget actions.
    ///
    /// Required: No
    ///
    /// Type: Spend
    ///
    /// Update requires: No interruption
    #[serde(rename = "BudgetLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_limit: Option<Spend>,

    ///
    /// The name of a budget. The value must be unique within an account. BudgetName can't include 			: and \ characters. If you don't include value for BudgetName in the template, 			Billing and Cost Management assigns your budget a randomly generated name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BudgetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether this budget tracks costs, usage, RI utilization, RI coverage, Savings 			Plans utilization, or Savings Plans coverage.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: COST | RI_COVERAGE | RI_UTILIZATION | SAVINGS_PLANS_COVERAGE | SAVINGS_PLANS_UTILIZATION | USAGE
    ///
    /// Update requires: No interruption
    #[serde(rename = "BudgetType")]
    pub budget_type: BudgetDataBudgetTypeEnum,

    ///
    /// The cost filters, such as Region, Service, member account, Tag, or Cost Category, that are applied to a budget.
    ///
    /// AWS Budgets supports the following services as a Service filter for RI budgets:
    ///
    /// Amazon EC2               Amazon Redshift               Amazon Relational Database Service               Amazon ElastiCache               Amazon OpenSearch Service
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "CostFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_filters: Option<serde_json::Value>,

    ///
    /// The types of costs that are included in this COST budget.
    ///
    /// USAGE, RI_UTILIZATION, RI_COVERAGE, SAVINGS_PLANS_UTILIZATION, and SAVINGS_PLANS_COVERAGE budgets do not have CostTypes.
    ///
    /// Required: No
    ///
    /// Type: CostTypes
    ///
    /// Update requires: No interruption
    #[serde(rename = "CostTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_types: Option<CostTypes>,

    ///
    /// A map containing multiple BudgetLimit, including current or future limits.
    ///
    /// PlannedBudgetLimits is available for cost or usage budget and supports both 			monthly and quarterly TimeUnit.
    ///
    /// For monthly budgets, provide 12 months of PlannedBudgetLimits values. This must start from the current month and include the next 11 months. The key is the start of the month, UTC in epoch seconds.
    ///
    /// For quarterly budgets, provide four quarters of PlannedBudgetLimits value 			entries in standard calendar quarter increments. This must start from the current 			quarter and include the next three quarters. The key is the start of the 			quarter, UTC in epoch seconds.
    ///
    /// If the planned budget expires before 12 months for monthly or four quarters for quarterly, 			provide the PlannedBudgetLimits values only for the remaining 			periods.
    ///
    /// If the budget begins at a date in the future, provide PlannedBudgetLimits values from the start date of the budget.
    ///
    /// After all of the BudgetLimit values in PlannedBudgetLimits are used, the budget continues to use the last limit as the BudgetLimit. At that point, the planned budget provides the same experience as a fixed budget.
    ///
    /// DescribeBudget and DescribeBudgets response along with 				PlannedBudgetLimits also contain BudgetLimit representing 			the current month or quarter limit present in PlannedBudgetLimits. This 			only applies to budgets that are created with PlannedBudgetLimits. Budgets 			that are created without PlannedBudgetLimits only contain 				BudgetLimit. They don't contain 			PlannedBudgetLimits.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "PlannedBudgetLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_budget_limits: Option<serde_json::Value>,

    ///
    /// The period of time that is covered by a budget. The period has a start date and an end 			date. The start date must come before the end date. There are no restrictions on the end date.
    ///
    /// The start date for a budget. If you created your budget and didn't specify a start 			date, the start date defaults to the start of the chosen time period (MONTHLY, QUARTERLY, or 			ANNUALLY). For example, if you create your budget on January 24, 2019, choose 			MONTHLY, and don't set a start date, the start date defaults to 			01/01/19 00:00 UTC. The defaults are the same for the AWS Billing and Cost Management console and the API.
    ///
    /// You can change your start date with the UpdateBudget operation.
    ///
    /// After the end date, AWS deletes the budget and all associated notifications and subscribers.
    ///
    /// Required: No
    ///
    /// Type: TimePeriod
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<TimePeriod>,

    ///
    /// The length of time until a budget resets the actual and forecasted spend. DAILY is available only for 			RI_UTILIZATION and RI_COVERAGE budgets.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ANNUALLY | DAILY | MONTHLY | QUARTERLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeUnit")]
    pub time_unit: BudgetDataTimeUnitEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum BudgetDataBudgetTypeEnum {
    /// COST
    #[serde(rename = "COST")]
    Cost,

    /// RI_COVERAGE
    #[serde(rename = "RI_COVERAGE")]
    Ricoverage,

    /// RI_UTILIZATION
    #[serde(rename = "RI_UTILIZATION")]
    Riutilization,

    /// SAVINGS_PLANS_COVERAGE
    #[serde(rename = "SAVINGS_PLANS_COVERAGE")]
    Savingsplanscoverage,

    /// SAVINGS_PLANS_UTILIZATION
    #[serde(rename = "SAVINGS_PLANS_UTILIZATION")]
    Savingsplansutilization,

    /// USAGE
    #[serde(rename = "USAGE")]
    Usage,
}

impl Default for BudgetDataBudgetTypeEnum {
    fn default() -> Self {
        BudgetDataBudgetTypeEnum::Cost
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum BudgetDataTimeUnitEnum {
    /// ANNUALLY
    #[serde(rename = "ANNUALLY")]
    Annually,

    /// DAILY
    #[serde(rename = "DAILY")]
    Daily,

    /// MONTHLY
    #[serde(rename = "MONTHLY")]
    Monthly,

    /// QUARTERLY
    #[serde(rename = "QUARTERLY")]
    Quarterly,
}

impl Default for BudgetDataTimeUnitEnum {
    fn default() -> Self {
        BudgetDataTimeUnitEnum::Annually
    }
}

impl cfn_resources::CfnResource for BudgetData {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.auto_adjust_data
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.budget_limit
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.cost_types
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.time_period
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The types of cost that are included in a COST budget, such as tax and subscriptions.
///
/// USAGE, RI_UTILIZATION, RI_COVERAGE, 				SAVINGS_PLANS_UTILIZATION, and SAVINGS_PLANS_COVERAGE 			budgets don't have CostTypes.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CostTypes {
    ///
    /// Specifies whether a budget includes credits.
    ///
    /// The default value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeCredit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_credit: Option<bool>,

    ///
    /// Specifies whether a budget includes discounts.
    ///
    /// The default value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeDiscount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_discount: Option<bool>,

    ///
    /// Specifies whether a budget includes non-RI subscription costs.
    ///
    /// The default value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeOtherSubscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_other_subscription: Option<bool>,

    ///
    /// Specifies whether a budget includes recurring fees such as monthly RI fees.
    ///
    /// The default value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeRecurring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_recurring: Option<bool>,

    ///
    /// Specifies whether a budget includes refunds.
    ///
    /// The default value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeRefund")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_refund: Option<bool>,

    ///
    /// Specifies whether a budget includes subscriptions.
    ///
    /// The default value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeSubscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subscription: Option<bool>,

    ///
    /// Specifies whether a budget includes support subscription fees.
    ///
    /// The default value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeSupport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_support: Option<bool>,

    ///
    /// Specifies whether a budget includes taxes.
    ///
    /// The default value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeTax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_tax: Option<bool>,

    ///
    /// Specifies whether a budget includes upfront RI costs.
    ///
    /// The default value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeUpfront")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_upfront: Option<bool>,

    ///
    /// Specifies whether a budget uses the amortized rate.
    ///
    /// The default value is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseAmortized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_amortized: Option<bool>,

    ///
    /// Specifies whether a budget uses a blended rate.
    ///
    /// The default value is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseBlended")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_blended: Option<bool>,
}

impl cfn_resources::CfnResource for CostTypes {
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

/// The HistoricalOptions property type specifies Property description not available. for an AWS::Budgets::Budget.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct HistoricalOptions {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BudgetAdjustmentPeriod")]
    pub budget_adjustment_period: i64,
}

impl cfn_resources::CfnResource for HistoricalOptions {
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

/// A notification that's associated with a budget. A budget can have up to ten notifications.
///
/// Each notification must have at least one subscriber. A notification can have one SNS subscriber and up to 10 email subscribers, for a total of 11 subscribers.
///
/// For example, if you have a budget for 200 dollars and you want to be notified when you go over 160 dollars, create a notification with the following parameters:
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Notification {
    ///
    /// The comparison that's used for this notification.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: EQUAL_TO | GREATER_THAN | LESS_THAN
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: NotificationComparisonOperatorEnum,

    ///
    /// Specifies whether the notification is for how much you have spent (ACTUAL) or 			for how much that you're forecasted to spend (FORECASTED).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ACTUAL | FORECASTED
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationType")]
    pub notification_type: NotificationNotificationTypeEnum,

    ///
    /// The threshold that's associated with a notification. Thresholds are always a percentage, and 			many customers find value being alerted between 50% - 200% of the budgeted amount. The 			maximum limit for your threshold is 1,000,000% above the budgeted amount.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Threshold")]
    pub threshold: f64,

    ///
    /// The type of threshold for a notification. For ABSOLUTE_VALUE thresholds, AWS notifies you when you go over or are forecasted to go over your total cost threshold. For PERCENTAGE thresholds, AWS notifies you when you go over or are forecasted to go over a certain percentage of your forecasted spend. For example, if you have a budget for 200 dollars and you have a PERCENTAGE threshold of 80%, AWS notifies you when you go over 160 dollars.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ABSOLUTE_VALUE | PERCENTAGE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThresholdType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_type: Option<NotificationThresholdTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NotificationComparisonOperatorEnum {
    /// EQUAL_TO
    #[serde(rename = "EQUAL_TO")]
    Equalto,

    /// GREATER_THAN
    #[serde(rename = "GREATER_THAN")]
    Greaterthan,

    /// LESS_THAN
    #[serde(rename = "LESS_THAN")]
    Lessthan,
}

impl Default for NotificationComparisonOperatorEnum {
    fn default() -> Self {
        NotificationComparisonOperatorEnum::Equalto
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NotificationNotificationTypeEnum {
    /// ACTUAL
    #[serde(rename = "ACTUAL")]
    Actual,

    /// FORECASTED
    #[serde(rename = "FORECASTED")]
    Forecasted,
}

impl Default for NotificationNotificationTypeEnum {
    fn default() -> Self {
        NotificationNotificationTypeEnum::Actual
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NotificationThresholdTypeEnum {
    /// ABSOLUTE_VALUE
    #[serde(rename = "ABSOLUTE_VALUE")]
    Absolutevalue,

    /// PERCENTAGE
    #[serde(rename = "PERCENTAGE")]
    Percentage,
}

impl Default for NotificationThresholdTypeEnum {
    fn default() -> Self {
        NotificationThresholdTypeEnum::Absolutevalue
    }
}

impl cfn_resources::CfnResource for Notification {
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

/// A notification with subscribers. A notification can have one SNS subscriber and up to 10 email subscribers, for a total of 11 subscribers.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct NotificationWithSubscribers {
    ///
    /// The notification that's associated with a budget.
    ///
    /// Required: Yes
    ///
    /// Type: Notification
    ///
    /// Update requires: No interruption
    #[serde(rename = "Notification")]
    pub notification: Notification,

    ///
    /// A list of subscribers who are subscribed to this notification.
    ///
    /// Required: Yes
    ///
    /// Type: List of Subscriber
    ///
    /// Maximum: 11
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subscribers")]
    pub subscribers: Vec<Subscriber>,
}

impl cfn_resources::CfnResource for NotificationWithSubscribers {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.notification.validate()?;

        let the_val = &self.subscribers;

        if the_val.len() > 11 as _ {
            return Err(format!(
                "Max validation failed on field 'subscribers'. {} is greater than 11",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The amount of cost or usage that's measured for a budget.
///
/// For example, a Spend for 3 GB of S3 usage has the following 			parameters:
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Spend {
    ///
    /// The cost or usage amount that's associated with a budget forecast, actual spend, or budget 			threshold.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Amount")]
    pub amount: f64,

    ///
    /// The unit of measurement that's used for the budget forecast, actual spend, or budget 			threshold, such as USD or GBP.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    pub unit: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Spend {
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

/// The Subscriber property type specifies who to notify for a Billing and Cost Management budget notification. 			The subscriber consists of a subscription type, and either an Amazon SNS topic or an email address.
///
/// For example, an email subscriber would have the following parameters:
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Subscriber {
    ///
    /// The address that AWS sends budget notifications to, either an SNS topic or an email.
    ///
    /// When you create a subscriber, the value of Address can't contain line breaks.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Address")]
    pub address: cfn_resources::StrVal,

    ///
    /// The type of notification that AWS sends to a subscriber.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: EMAIL | SNS
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubscriptionType")]
    pub subscription_type: SubscriberSubscriptionTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SubscriberSubscriptionTypeEnum {
    /// EMAIL
    #[serde(rename = "EMAIL")]
    Email,

    /// SNS
    #[serde(rename = "SNS")]
    Sns,
}

impl Default for SubscriberSubscriptionTypeEnum {
    fn default() -> Self {
        SubscriberSubscriptionTypeEnum::Email
    }
}

impl cfn_resources::CfnResource for Subscriber {
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

/// The period of time that is covered by a budget. The period has a start date and an end 			date. The start date must come before the end date. There are no restrictions on the end date.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TimePeriod {
    ///
    /// The end date for a budget. If you didn't specify an end date, AWS set your end date to 06/15/87 00:00 UTC. The defaults are the same for the AWS Billing and Cost Management console and the API.
    ///
    /// After the end date, AWS deletes the budget and all the associated 			notifications and subscribers. You can change your end date with the 				UpdateBudget operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<cfn_resources::StrVal>,

    ///
    /// The start date for a budget. If you created your budget and didn't specify a start 			date, the start date defaults to the start of the chosen time period (MONTHLY, QUARTERLY, or 			ANNUALLY). For example, if you create your budget on January 24, 2019, choose 			MONTHLY, and don't set a start date, the start date defaults to 			01/01/19 00:00 UTC. The defaults are the same for the AWS Billing and Cost Management console and the API.
    ///
    /// You can change your start date with the UpdateBudget operation.
    ///
    /// Valid values depend on the value of BudgetType:
    ///
    /// If BudgetType is COST or USAGE: Valid values are 					MONTHLY, QUARTERLY, and ANNUALLY.If BudgetType is RI_UTILIZATION or RI_COVERAGE: Valid values are 					DAILY, MONTHLY, QUARTERLY, and ANNUALLY.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for TimePeriod {
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
