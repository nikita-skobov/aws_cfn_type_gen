

/// The AWS::Budgets::BudgetsAction resource enables you to take predefined actions that are initiated when a budget threshold has been exceeded. 		For more information, see Managing Your Costs with Budgets 			in the AWS Billing and Cost Management User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnBudgetsAction {


    /// The trigger threshold of the action.
    ///
    /// Required: Yes
    ///
    /// Type: ActionThreshold
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionThreshold")]
    pub action_threshold: ActionThreshold,


    /// The type of action. This defines the type of tasks that can be carried out by this action. This field also determines the format for definition.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: APPLY_IAM_POLICY | APPLY_SCP_POLICY | RUN_SSM_DOCUMENTS
    ///
    /// Update requires: Replacement
    #[serde(rename = "ActionType")]
    pub action_type: BudgetsActionActionTypeEnum,


    /// This specifies if the action needs manual or automatic approval.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTOMATIC | MANUAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApprovalModel")]
    pub approval_model: Option<BudgetsActionApprovalModelEnum>,


    /// A string that represents the budget name. ":" and "\" characters aren't allowed.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BudgetName")]
    pub budget_name: String,


    /// Specifies all of the type-specific parameters.
    ///
    /// Required: Yes
    ///
    /// Type: Definition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Definition")]
    pub definition: Definition,


    /// The role passed for action execution and reversion. Roles and actions must be in the same account.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 32
    ///
    /// Maximum: 618
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|us-iso-east-1|us-isob-east-1):iam::\d{12}:role(\u002F[\u0021-\u007F]+\u002F|\u002F)[\w+=,.@-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: String,


    /// The type of a notification.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ACTUAL | FORECASTED
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationType")]
    pub notification_type: BudgetsActionNotificationTypeEnum,


    /// A list of subscribers.
    ///
    /// Required: Yes
    ///
    /// Type: List of Subscriber
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subscribers")]
    pub subscribers: Vec<Subscriber>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum BudgetsActionNotificationTypeEnum {

    /// ACTUAL
    #[serde(rename = "ACTUAL")]
    Actual,

    /// FORECASTED
    #[serde(rename = "FORECASTED")]
    Forecasted,

}

impl Default for BudgetsActionNotificationTypeEnum {
    fn default() -> Self {
        BudgetsActionNotificationTypeEnum::Actual
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum BudgetsActionActionTypeEnum {

    /// APPLY_IAM_POLICY
    #[serde(rename = "APPLY_IAM_POLICY")]
    Applyiampolicy,

    /// APPLY_SCP_POLICY
    #[serde(rename = "APPLY_SCP_POLICY")]
    Applyscppolicy,

    /// RUN_SSM_DOCUMENTS
    #[serde(rename = "RUN_SSM_DOCUMENTS")]
    Runssmdocuments,

}

impl Default for BudgetsActionActionTypeEnum {
    fn default() -> Self {
        BudgetsActionActionTypeEnum::Applyiampolicy
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum BudgetsActionApprovalModelEnum {

    /// AUTOMATIC
    #[serde(rename = "AUTOMATIC")]
    Automatic,

    /// MANUAL
    #[serde(rename = "MANUAL")]
    Manual,

}

impl Default for BudgetsActionApprovalModelEnum {
    fn default() -> Self {
        BudgetsActionApprovalModelEnum::Automatic
    }
}


impl cfn_resources::CfnResource for CfnBudgetsAction {
    fn type_string() -> &'static str {
        "AWS::Budgets::BudgetsAction"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The trigger threshold of the action.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ActionThreshold {


    /// The type of threshold for a notification.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ABSOLUTE_VALUE | PERCENTAGE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: ActionThresholdTypeEnum,


    /// The threshold of a notification.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: f64,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ActionThresholdTypeEnum {

    /// ABSOLUTE_VALUE
    #[serde(rename = "ABSOLUTE_VALUE")]
    Absolutevalue,

    /// PERCENTAGE
    #[serde(rename = "PERCENTAGE")]
    Percentage,

}

impl Default for ActionThresholdTypeEnum {
    fn default() -> Self {
        ActionThresholdTypeEnum::Absolutevalue
    }
}



/// The definition is where you specify all of the type-specific parameters.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Definition {


    /// The AWS Identity and Access Management (IAM) action definition details.
    ///
    /// Required: No
    ///
    /// Type: IamActionDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamActionDefinition")]
    pub iam_action_definition: Option<IamActionDefinition>,


    /// The service control policies (SCP) action definition details.
    ///
    /// Required: No
    ///
    /// Type: ScpActionDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScpActionDefinition")]
    pub scp_action_definition: Option<ScpActionDefinition>,


    /// The Amazon EC2 Systems Manager (SSM) action definition details.
    ///
    /// Required: No
    ///
    /// Type: SsmActionDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "SsmActionDefinition")]
    pub ssm_action_definition: Option<SsmActionDefinition>,

}




/// The AWS Identity and Access Management (IAM) action definition details.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IamActionDefinition {


    /// A list of groups to be attached. There must be at least one group.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Groups")]
    pub groups: Option<Vec<String>>,


    /// The Amazon Resource Name (ARN) of the policy to be attached.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 25
    ///
    /// Maximum: 684
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|us-iso-east-1|us-isob-east-1):iam::(\d{12}|aws):policy(\u002F[\u0021-\u007F]+\u002F|\u002F)[\w+=,.@-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyArn")]
    pub policy_arn: String,


    /// A list of roles to be attached. There must be at least one role.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Roles")]
    pub roles: Option<Vec<String>>,


    /// A list of users to be attached. There must be at least one user.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Users")]
    pub users: Option<Vec<String>>,

}




/// The service control policies (SCP) action definition details.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ScpActionDefinition {


    /// The policy ID attached.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 10
    ///
    /// Maximum: 130
    ///
    /// Pattern: ^p-[0-9a-zA-Z_]{8,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyId")]
    pub policy_id: String,


    /// A list of target IDs.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetIds")]
    pub target_ids: Vec<String>,

}




/// The Amazon EC2 Systems Manager (SSM) action definition details.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SsmActionDefinition {


    /// The EC2 and RDS instance IDs.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,


    /// The Region to run the (SSM) document.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 9
    ///
    /// Maximum: 20
    ///
    /// Pattern: ^\w{2}-\w+(-\w+)?-\d$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    pub region: String,


    /// The action subType.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: STOP_EC2_INSTANCES | STOP_RDS_INSTANCES
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subtype")]
    pub subtype: SsmActionDefinitionSubtypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SsmActionDefinitionSubtypeEnum {

    /// STOP_EC2_INSTANCES
    #[serde(rename = "STOP_EC2_INSTANCES")]
    Stopec2instances,

    /// STOP_RDS_INSTANCES
    #[serde(rename = "STOP_RDS_INSTANCES")]
    Stoprdsinstances,

}

impl Default for SsmActionDefinitionSubtypeEnum {
    fn default() -> Self {
        SsmActionDefinitionSubtypeEnum::Stopec2instances
    }
}



/// The subscriber to a budget notification. The subscriber consists of a subscription type and either an Amazon SNS topic or an email address.
///
/// For example, an email subscriber has the following parameters:
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub address: String,


    /// The type of notification that AWS sends to a subscriber.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

}


