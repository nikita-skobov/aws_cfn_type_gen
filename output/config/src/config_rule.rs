

/// Adds or updates an AWS Config rule to evaluate if your      AWS resources comply with your desired configurations. For information on how many AWS Config rules you can have per account,       see Service Limits in the AWS Config Developer Guide.
///
/// There are two types of rules: AWS Config Managed Rules and AWS Config Custom Rules.      You can use the ConfigRule resource to create both AWS Config Managed Rules and AWS Config Custom Rules.
///
/// AWS Config Managed Rules are predefined,      customizable rules created by AWS Config. For a list of managed rules, see      List of AWS Config        Managed Rules. If you are adding an AWS Config managed rule, you must specify the      rule's identifier for the SourceIdentifier key.
///
/// AWS Config Custom Rules are rules that you create from scratch. There are two ways to create AWS Config custom rules: with Lambda functions      (AWS Lambda Developer Guide) and with Guard (Guard GitHub          Repository), a policy-as-code language.            AWS Config custom rules created with AWS Lambda      are called AWS Config Custom Lambda Rules and AWS Config custom rules created with      Guard are called AWS Config Custom Policy Rules.
///
/// If you are adding a newAWS Config Custom Lambda rule,      you first need to create an AWS Lambda function that the rule invokes to evaluate      your resources. When you use the ConfigRule resource to add a Custom Lambda rule to AWS Config, you must specify the Amazon Resource      Name (ARN) that AWS Lambda assigns to the function. You specify the ARN      in the SourceIdentifier key. This key is part of the      Source object, which is part of the      ConfigRule object.
///
/// For any new AWS Config rule that you add, specify the      ConfigRuleName in the ConfigRule      object. Do not specify the ConfigRuleArn or the      ConfigRuleId. These values are generated by AWS Config for new rules.
///
/// If you are updating a rule that you added previously, you can      specify the rule by ConfigRuleName,      ConfigRuleId, or ConfigRuleArn in the      ConfigRule data type that you use in this      request.
///
/// For more information about developing and using AWS Config      rules, see Evaluating Resources with AWS Config Rules      in the AWS Config Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnConfigRule {


    /// 
    /// Defines which resources can trigger an evaluation for the rule. 			The scope can include one or more resource types, a combination of 			one resource type and one resource ID, or a combination of a tag key 			and value. Specify a scope to constrain the resources that can 			trigger an evaluation for the rule. If you do not specify a scope, 			evaluations are triggered when any resource in the recording group 			changes.
    /// 
    /// NoteThe scope can be empty.
    /// 
    /// Required: No
    ///
    /// Type: Scope
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scope")]
    pub scope: Option<Scope>,


    /// 
    /// The description that you provide for the AWS Config 			rule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The maximum frequency with which AWS Config runs evaluations 			for a rule. You can specify a value for 				MaximumExecutionFrequency when:
    /// 
    /// You are using an AWS managed rule that is triggered at 					a periodic frequency. 			     				      Your custom rule is triggered when AWS Config delivers 				        the configuration snapshot. For more information, see ConfigSnapshotDeliveryProperties.
    /// 
    /// NoteBy default, rules with a periodic trigger are evaluated 				every 24 hours. To change the frequency, specify a valid value 				for the MaximumExecutionFrequency 				parameter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: One_Hour | Six_Hours | Three_Hours | Twelve_Hours | TwentyFour_Hours
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumExecutionFrequency")]
    pub maximum_execution_frequency: Option<String>,


    /// 
    /// Provides the rule owner (        AWS       for managed rules, CUSTOM_POLICY for Custom Policy rules, and CUSTOM_LAMBDA for Custom Lambda rules), the rule identifier, 			and the notifications that cause the function to evaluate your AWS 			resources.
    /// 
    /// Required: Yes
    ///
    /// Type: Source
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source")]
    pub source: Source,


    /// 
    /// A name for the AWS Config rule. If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the rule name.       For more information, see Name Type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: Option<String>,


    /// 
    /// A string, in JSON format, that is passed to the AWS Config rule 			Lambda function.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputParameters")]
    pub input_parameters: Option<serde_json::Value>,

}


/// Provides the runtime system, policy definition, and whether debug logging enabled. You can 			specify the following CustomPolicyDetails parameter values 			only 			for AWS Config Custom Policy rules.
#[derive(Default, serde::Serialize)]
pub struct CustomPolicyDetails {


    /// 
    /// The runtime system for your AWS Config Custom Policy rule. Guard is a policy-as-code language that allows you to write policies that are enforced by AWS Config Custom Policy rules. For more information about Guard, see the Guard GitHub 					Repository.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: guard\-2\.x\.x
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyRuntime")]
    pub policy_runtime: Option<String>,


    /// 
    /// The boolean expression for enabling debug logging for your AWS Config Custom Policy rule. The default value is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableDebugLogDelivery")]
    pub enable_debug_log_delivery: Option<bool>,


    /// 
    /// The policy definition containing the logic for your AWS Config Custom Policy rule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10000
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyText")]
    pub policy_text: Option<String>,

}


/// Provides the CustomPolicyDetails, the rule owner (        AWS       for managed rules, CUSTOM_POLICY for Custom Policy rules, and CUSTOM_LAMBDA for Custom Lambda rules), the rule 			identifier, and the events that cause the evaluation of your AWS 			resources.
#[derive(Default, serde::Serialize)]
pub struct Source {


    /// 
    /// For AWS Config Managed rules, a predefined identifier from a 			list. For example, IAM_PASSWORD_POLICY is a managed 			rule. To reference a managed rule, see List of AWS Config Managed Rules.
    /// 
    /// For AWS Config Custom Lambda rules, the identifier is the Amazon Resource Name 			(ARN) of the rule's AWS Lambda function, such as 			arn:aws:lambda:us-east-2:123456789012:function:custom_rule_name.
    /// 
    /// For AWS Config Custom Policy rules, this field will be ignored.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceIdentifier")]
    pub source_identifier: Option<String>,


    /// 
    /// Provides the runtime system, policy definition, and whether debug logging is enabled. Required when owner is set to CUSTOM_POLICY.
    /// 
    /// Required: No
    ///
    /// Type: CustomPolicyDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomPolicyDetails")]
    pub custom_policy_details: Option<CustomPolicyDetails>,


    /// 
    /// Indicates whether AWS or the customer owns and manages the AWS Config rule.
    /// 
    /// AWS Config Managed Rules are predefined rules owned by AWS. For more information, see AWS Config Managed Rules in the         AWS Config developer guide.
    /// 
    /// AWS Config Custom Rules are rules that you can develop either with Guard (CUSTOM_POLICY) or AWS Lambda (CUSTOM_LAMBDA). For more information, see AWS Config Custom Rules in the         AWS Config developer guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AWS | CUSTOM_LAMBDA | CUSTOM_POLICY
    ///
    /// Update requires: No interruption
    #[serde(rename = "Owner")]
    pub owner: String,


    /// 
    /// Provides the source and the message types that cause AWS Config to evaluate your AWS resources against a rule. It also provides the frequency with which you want AWS Config to run evaluations for the rule if the trigger type is periodic.
    /// 
    /// If the owner is set to CUSTOM_POLICY, the only acceptable values for the AWS Config rule trigger message type are ConfigurationItemChangeNotification and OversizedConfigurationItemChangeNotification.
    /// 
    /// Required: No
    ///
    /// Type: List of SourceDetail
    ///
    /// Maximum: 25
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceDetails")]
    pub source_details: Option<Vec<SourceDetail>>,

}


/// Defines which resources trigger an evaluation for an AWS Config 			rule. The scope can include one or more resource types, a 			combination of a tag key and value, or a combination of one resource 			type and one resource ID. Specify a scope to constrain which 			resources trigger an evaluation for a rule. Otherwise, evaluations 			for the rule are triggered when any resource in your recording group 			changes in configuration.
#[derive(Default, serde::Serialize)]
pub struct Scope {


    /// 
    /// The tag value applied to only those AWS resources that you want 			to trigger an evaluation for the rule. If you specify a value for 				TagValue, you must also specify a value for 				TagKey.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagValue")]
    pub tag_value: Option<String>,


    /// 
    /// The tag key that is applied to only those AWS resources that 			you want to trigger an evaluation for the rule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagKey")]
    pub tag_key: Option<String>,


    /// 
    /// The ID of the only AWS resource that you want to trigger an 			evaluation for the rule. If you specify a resource ID, you must 			specify one resource type for 			ComplianceResourceTypes.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 768
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComplianceResourceId")]
    pub compliance_resource_id: Option<String>,


    /// 
    /// The resource types of only those AWS resources that you want to 			trigger an evaluation for the rule. You can only specify one type if 			you also specify a resource ID for 			ComplianceResourceId.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComplianceResourceTypes")]
    pub compliance_resource_types: Option<Vec<String>>,

}


/// Provides the source and the message types that trigger AWS Config to evaluate your AWS resources against a rule. It also 			provides the frequency with which you want AWS Config to run 			evaluations for the rule if the trigger type is periodic. You can 			specify the parameter values for SourceDetail only for 			custom rules.
#[derive(Default, serde::Serialize)]
pub struct SourceDetail {


    /// 
    /// The type of notification that triggers AWS Config to run an 			evaluation for a rule. You can specify the following notification 			types:
    /// 
    /// ConfigurationItemChangeNotification - Triggers 					an evaluation when AWS Config delivers a configuration item 					as a result of a resource change.                        OversizedConfigurationItemChangeNotification 					- Triggers an evaluation when AWS Config delivers an 					oversized configuration item. AWS Config may generate this 					notification type when a resource changes and the 					notification exceeds the maximum size allowed by Amazon 					SNS.                        ScheduledNotification - Triggers a 					periodic evaluation at the frequency specified for 						MaximumExecutionFrequency.                        ConfigurationSnapshotDeliveryCompleted - 					Triggers a periodic evaluation when AWS Config delivers a 					configuration snapshot.
    /// 
    /// If you want your custom rule to be triggered by configuration 			changes, specify two SourceDetail objects, one for 				ConfigurationItemChangeNotification and one for 				OversizedConfigurationItemChangeNotification.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ConfigurationItemChangeNotification | ConfigurationSnapshotDeliveryCompleted | OversizedConfigurationItemChangeNotification | ScheduledNotification
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageType")]
    pub message_type: String,


    /// 
    /// The source of the event, such as an AWS service, that triggers 			AWS Config to evaluate your AWS resources.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: aws.config
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventSource")]
    pub event_source: String,


    /// 
    /// The frequency at which you want AWS Config to run evaluations 			for a custom rule with a periodic trigger. If you specify a value 			for MaximumExecutionFrequency, then 				MessageType must use the 				ScheduledNotification value.
    /// 
    /// NoteBy default, rules with a periodic trigger are evaluated 				every 24 hours. To change the frequency, specify a valid value 				for the MaximumExecutionFrequency 				parameter.Based on the valid value you choose, AWS Config runs 				evaluations once for each valid value. For example, if you 				choose Three_Hours, AWS Config runs evaluations 				once every three hours. In this case, Three_Hours 				is the frequency of this rule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: One_Hour | Six_Hours | Three_Hours | Twelve_Hours | TwentyFour_Hours
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumExecutionFrequency")]
    pub maximum_execution_frequency: Option<String>,

}
