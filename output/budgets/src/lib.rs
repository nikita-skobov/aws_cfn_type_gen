
pub mod cfn_budget {

#[derive(serde::Serialize, Default)]
pub struct CfnBudget {
    /// No documentation provided by AWS
    #[serde(rename = "Budget")]
    pub budget: BudgetData,
    /// List of NotificationWithSubscribers
    #[serde(rename = "NotificationsWithSubscribers")]
    pub notifications_with_subscribers: Option<Vec<NotificationWithSubscribers>>,

}


#[derive(serde::Serialize, Default)]
pub struct NotificationWithSubscribers {
    #[serde(rename = "Subscribers")]
    pub subscribers: Vec<Subscriber>,
    #[serde(rename = "Notification")]
    pub notification: Notification,

}

#[derive(serde::Serialize, Default)]
pub struct Spend {
    #[serde(rename = "Unit")]
    pub unit: String,
    #[serde(rename = "Amount")]
    pub amount: f64,

}

#[derive(serde::Serialize, Default)]
pub struct Notification {
    #[serde(rename = "ThresholdType")]
    pub threshold_type: Option<String>,
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,
    #[serde(rename = "Threshold")]
    pub threshold: f64,
    #[serde(rename = "NotificationType")]
    pub notification_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct HistoricalOptions {
    #[serde(rename = "BudgetAdjustmentPeriod")]
    pub budget_adjustment_period: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Subscriber {
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "SubscriptionType")]
    pub subscription_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct BudgetData {
    #[serde(rename = "BudgetLimit")]
    pub budget_limit: Option<Spend>,
    #[serde(rename = "TimeUnit")]
    pub time_unit: String,
    #[serde(rename = "CostFilters")]
    pub cost_filters: Option<()>,
    #[serde(rename = "BudgetName")]
    pub budget_name: Option<String>,
    #[serde(rename = "CostTypes")]
    pub cost_types: Option<CostTypes>,
    #[serde(rename = "TimePeriod")]
    pub time_period: Option<TimePeriod>,
    #[serde(rename = "AutoAdjustData")]
    pub auto_adjust_data: Option<AutoAdjustData>,
    #[serde(rename = "BudgetType")]
    pub budget_type: String,
    #[serde(rename = "PlannedBudgetLimits")]
    pub planned_budget_limits: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct AutoAdjustData {
    #[serde(rename = "AutoAdjustType")]
    pub auto_adjust_type: String,
    #[serde(rename = "HistoricalOptions")]
    pub historical_options: Option<HistoricalOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct TimePeriod {
    #[serde(rename = "End")]
    pub end: Option<String>,
    #[serde(rename = "Start")]
    pub start: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CostTypes {
    #[serde(rename = "IncludeCredit")]
    pub include_credit: Option<bool>,
    #[serde(rename = "IncludeRecurring")]
    pub include_recurring: Option<bool>,
    #[serde(rename = "IncludeDiscount")]
    pub include_discount: Option<bool>,
    #[serde(rename = "IncludeSubscription")]
    pub include_subscription: Option<bool>,
    #[serde(rename = "UseBlended")]
    pub use_blended: Option<bool>,
    #[serde(rename = "UseAmortized")]
    pub use_amortized: Option<bool>,
    #[serde(rename = "IncludeOtherSubscription")]
    pub include_other_subscription: Option<bool>,
    #[serde(rename = "IncludeRefund")]
    pub include_refund: Option<bool>,
    #[serde(rename = "IncludeUpfront")]
    pub include_upfront: Option<bool>,
    #[serde(rename = "IncludeSupport")]
    pub include_support: Option<bool>,
    #[serde(rename = "IncludeTax")]
    pub include_tax: Option<bool>,

}


}

pub mod cfn_budgets_action {

#[derive(serde::Serialize, Default)]
pub struct CfnBudgetsAction {
    /// No documentation provided by AWS
    #[serde(rename = "ActionThreshold")]
    pub action_threshold: ActionThreshold,
    /// No documentation provided by AWS
    #[serde(rename = "ApprovalModel")]
    pub approval_model: Option<String>,
    /// List of Subscriber
    #[serde(rename = "Subscribers")]
    pub subscribers: Vec<Subscriber>,
    /// No documentation provided by AWS
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "ActionType")]
    pub action_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "NotificationType")]
    pub notification_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "Definition")]
    pub definition: Definition,
    /// No documentation provided by AWS
    #[serde(rename = "BudgetName")]
    pub budget_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct ScpActionDefinition {
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    #[serde(rename = "TargetIds")]
    pub target_ids: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Definition {
    #[serde(rename = "SsmActionDefinition")]
    pub ssm_action_definition: Option<SsmActionDefinition>,
    #[serde(rename = "IamActionDefinition")]
    pub iam_action_definition: Option<IamActionDefinition>,
    #[serde(rename = "ScpActionDefinition")]
    pub scp_action_definition: Option<ScpActionDefinition>,

}

#[derive(serde::Serialize, Default)]
pub struct IamActionDefinition {
    #[serde(rename = "Groups")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "PolicyArn")]
    pub policy_arn: String,
    #[serde(rename = "Roles")]
    pub roles: Option<Vec<String>>,
    #[serde(rename = "Users")]
    pub users: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct Subscriber {
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct SsmActionDefinition {
    #[serde(rename = "Subtype")]
    pub subtype: String,
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ActionThreshold {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Value")]
    pub value: f64,

}


}
