/// Adds or updates an AWS Config rule for your entire organization to evaluate if your AWS resources comply with your       desired configurations. For information on how many organization AWS Config rules you can have per account,       see Service Limits in the AWS Config Developer Guide.
///
/// Only a management account and a delegated administrator can create or update an organization AWS Config rule.       When calling the OrganizationConfigRule resource with a delegated administrator, you must ensure AWS Organizations       ListDelegatedAdministrator permissions are added. An organization can have up to 3 delegated administrators.
///
/// The OrganizationConfigRule resource enables organization service access through the EnableAWSServiceAccess action and creates a service-linked       role AWSServiceRoleForConfigMultiAccountSetup in the management or delegated administrator account of your organization.       The service-linked role is created only when the role does not exist in the caller account.       AWS Config verifies the existence of role with GetRole action.
///
/// To use the OrganizationConfigRule resource with delegated administrator, register a delegated administrator by calling AWS Organization       register-delegated-administrator for config-multiaccountsetup.amazonaws.com.
///
/// There are two types of rules: AWS Config Managed Rules and AWS Config Custom Rules.       You can use PutOrganizationConfigRule to create both AWS Config Managed Rules and AWS Config Custom Rules.
///
/// AWS Config Managed Rules are predefined,       customizable rules created by AWS Config. For a list of managed rules, see       List of AWS Config         Managed Rules. If you are adding an AWS Config managed rule, you must specify the rule's identifier for the RuleIdentifier key.
///
/// AWS Config Custom Rules are rules that you create from scratch. There are two ways to create AWS Config custom rules: with Lambda functions       (AWS Lambda Developer Guide) and with Guard (Guard GitHub           Repository), a policy-as-code language.             AWS Config custom rules created with AWS Lambda       are called AWS Config Custom Lambda Rules and AWS Config custom rules created with       Guard are called AWS Config Custom Policy Rules.
///
/// If you are adding a new AWS Config Custom Lambda rule, you first need to create an AWS Lambda function in the management account or a delegated       administrator that the rule invokes to evaluate your resources. You also need to create an IAM role in the managed account that can be assumed by the Lambda function.       When you use PutOrganizationConfigRule to add a Custom Lambda rule to AWS Config, you must       specify the Amazon Resource Name (ARN) that AWS Lambda assigns to the function.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnOrganizationConfigRule {
    ///
    /// A comma-separated list of accounts excluded from organization AWS Config rule.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludedAccounts")]
    pub excluded_accounts: Option<Vec<String>>,

    ///
    /// The name that you assign to organization AWS Config rule.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "OrganizationConfigRuleName")]
    pub organization_config_rule_name: String,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: OrganizationCustomPolicyRuleMetadata
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationCustomPolicyRuleMetadata")]
    pub organization_custom_policy_rule_metadata: Option<OrganizationCustomPolicyRuleMetadata>,

    ///
    /// An OrganizationCustomRuleMetadata object.
    ///
    /// Required: No
    ///
    /// Type: OrganizationCustomRuleMetadata
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationCustomRuleMetadata")]
    pub organization_custom_rule_metadata: Option<OrganizationCustomRuleMetadata>,

    ///
    /// An OrganizationManagedRuleMetadata object.
    ///
    /// Required: No
    ///
    /// Type: OrganizationManagedRuleMetadata
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationManagedRuleMetadata")]
    pub organization_managed_rule_metadata: Option<OrganizationManagedRuleMetadata>,
}

impl cfn_resources::CfnResource for CfnOrganizationConfigRule {
    fn type_string(&self) -> &'static str {
        "AWS::Config::OrganizationConfigRule"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.excluded_accounts {
            if the_val.len() > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'excluded_accounts'. {} is greater than 1000",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.organization_config_rule_name;

        if the_val.len() > 64 as _ {
            return Err(format!("Max validation failed on field 'organization_config_rule_name'. {} is greater than 64", the_val.len()));
        }

        let the_val = &self.organization_config_rule_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'organization_config_rule_name'. {} is less than 1",
                the_val.len()
            ));
        }

        self.organization_custom_policy_rule_metadata
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.organization_custom_rule_metadata
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.organization_managed_rule_metadata
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The OrganizationCustomPolicyRuleMetadata property type specifies Property description not available. for an AWS::Config::OrganizationConfigRule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OrganizationCustomPolicyRuleMetadata {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DebugLogDeliveryAccounts")]
    pub debug_log_delivery_accounts: Option<Vec<String>>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputParameters")]
    pub input_parameters: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumExecutionFrequency")]
    pub maximum_execution_frequency: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationConfigRuleTriggerTypes")]
    pub organization_config_rule_trigger_types: Option<Vec<String>>,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyText")]
    pub policy_text: String,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceIdScope")]
    pub resource_id_scope: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceTypesScope")]
    pub resource_types_scope: Option<Vec<String>>,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Runtime")]
    pub runtime: String,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagKeyScope")]
    pub tag_key_scope: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagValueScope")]
    pub tag_value_scope: Option<String>,
}

impl cfn_resources::CfnResource for OrganizationCustomPolicyRuleMetadata {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// An object that specifies organization custom rule metadata such as resource type, resource ID of AWS resource, Lambda function ARN, 			and organization trigger types that trigger AWS Config to evaluate your AWS resources against a rule. 			It also provides the frequency with which you want AWS Config to run evaluations for the rule if the trigger type is periodic.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OrganizationCustomRuleMetadata {
    ///
    /// The description that you provide for your organization AWS Config rule.
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
    /// A string, in JSON format, that is passed to your organization AWS Config rule Lambda function.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputParameters")]
    pub input_parameters: Option<String>,

    ///
    /// The lambda function ARN.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaFunctionArn")]
    pub lambda_function_arn: String,

    ///
    /// The maximum frequency with which AWS Config runs evaluations for a rule. 			Your custom rule is triggered when AWS Config delivers the configuration snapshot. For more information, see ConfigSnapshotDeliveryProperties.
    ///
    /// NoteBy default, rules with a periodic trigger are evaluated every 24 hours. To change the frequency, specify a valid 			value for the MaximumExecutionFrequency parameter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: One_Hour | Six_Hours | Three_Hours | Twelve_Hours | TwentyFour_Hours
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumExecutionFrequency")]
    pub maximum_execution_frequency:
        Option<OrganizationCustomRuleMetadataMaximumExecutionFrequencyEnum>,

    ///
    /// The type of notification that triggers AWS Config to run an evaluation for a rule. You can specify the following notification types:
    ///
    /// ConfigurationItemChangeNotification - Triggers an evaluation when AWS Config delivers a configuration item as a result of a resource change.                        OversizedConfigurationItemChangeNotification - Triggers an evaluation when AWS Config delivers an oversized configuration item. 			     	AWS Config may generate this notification type when a resource changes and the notification exceeds the maximum size allowed by Amazon SNS.                        ScheduledNotification - Triggers a periodic evaluation at the frequency specified for MaximumExecutionFrequency.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationConfigRuleTriggerTypes")]
    pub organization_config_rule_trigger_types: Vec<String>,

    ///
    /// The ID of the AWS resource that was evaluated.
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
    #[serde(rename = "ResourceIdScope")]
    pub resource_id_scope: Option<String>,

    ///
    /// The type of the AWS resource that was evaluated.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceTypesScope")]
    pub resource_types_scope: Option<Vec<String>>,

    ///
    /// One part of a key-value pair that make up a tag. 			A key is a general label that acts like a category for more specific tag values.
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
    #[serde(rename = "TagKeyScope")]
    pub tag_key_scope: Option<String>,

    ///
    /// The optional part of a key-value pair that make up a tag. 			A value acts as a descriptor within a tag category (key).
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
    #[serde(rename = "TagValueScope")]
    pub tag_value_scope: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OrganizationCustomRuleMetadataMaximumExecutionFrequencyEnum {
    /// One_Hour
    #[serde(rename = "One_Hour")]
    Onehour,

    /// Six_Hours
    #[serde(rename = "Six_Hours")]
    Sixhours,

    /// Three_Hours
    #[serde(rename = "Three_Hours")]
    Threehours,

    /// Twelve_Hours
    #[serde(rename = "Twelve_Hours")]
    Twelvehours,

    /// TwentyFour_Hours
    #[serde(rename = "TwentyFour_Hours")]
    Twentyfourhours,
}

impl Default for OrganizationCustomRuleMetadataMaximumExecutionFrequencyEnum {
    fn default() -> Self {
        OrganizationCustomRuleMetadataMaximumExecutionFrequencyEnum::Onehour
    }
}

impl cfn_resources::CfnResource for OrganizationCustomRuleMetadata {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'description'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.input_parameters {
            if the_val.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'input_parameters'. {} is greater than 2048",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.input_parameters {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'input_parameters'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.lambda_function_arn;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'lambda_function_arn'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.lambda_function_arn;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'lambda_function_arn'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.resource_id_scope {
            if the_val.len() > 768 as _ {
                return Err(format!(
                    "Max validation failed on field 'resource_id_scope'. {} is greater than 768",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.resource_id_scope {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'resource_id_scope'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.resource_types_scope {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'resource_types_scope'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.tag_key_scope {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'tag_key_scope'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.tag_key_scope {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'tag_key_scope'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.tag_value_scope {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'tag_value_scope'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.tag_value_scope {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'tag_value_scope'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// An object that specifies organization managed rule metadata such as resource type and ID of AWS resource along with the rule identifier. 			It also provides the frequency with which you want AWS Config to run evaluations for the rule if the trigger type is periodic.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OrganizationManagedRuleMetadata {
    ///
    /// The description that you provide for your organization AWS Config rule.
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
    /// A string, in JSON format, that is passed to your organization AWS Config rule Lambda function.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputParameters")]
    pub input_parameters: Option<String>,

    ///
    /// The maximum frequency with which AWS Config runs evaluations for a rule. This is for an AWS Config managed rule that is triggered at a periodic frequency.
    ///
    /// NoteBy default, rules with a periodic trigger are evaluated every 24 hours. To change the frequency, specify a valid 			value for the MaximumExecutionFrequency parameter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: One_Hour | Six_Hours | Three_Hours | Twelve_Hours | TwentyFour_Hours
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumExecutionFrequency")]
    pub maximum_execution_frequency:
        Option<OrganizationManagedRuleMetadataMaximumExecutionFrequencyEnum>,

    ///
    /// The ID of the AWS resource that was evaluated.
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
    #[serde(rename = "ResourceIdScope")]
    pub resource_id_scope: Option<String>,

    ///
    /// The type of the AWS resource that was evaluated.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceTypesScope")]
    pub resource_types_scope: Option<Vec<String>>,

    ///
    /// For organization config managed rules, a predefined identifier from a 			list. For example, IAM_PASSWORD_POLICY is a managed 			rule. To reference a managed rule, see Using AWS Config managed rules.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleIdentifier")]
    pub rule_identifier: String,

    ///
    /// One part of a key-value pair that make up a tag. 			A key is a general label that acts like a category for more specific tag values.
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
    #[serde(rename = "TagKeyScope")]
    pub tag_key_scope: Option<String>,

    ///
    /// The optional part of a key-value pair that make up a tag. 			A value acts as a descriptor within a tag category (key).
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
    #[serde(rename = "TagValueScope")]
    pub tag_value_scope: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OrganizationManagedRuleMetadataMaximumExecutionFrequencyEnum {
    /// One_Hour
    #[serde(rename = "One_Hour")]
    Onehour,

    /// Six_Hours
    #[serde(rename = "Six_Hours")]
    Sixhours,

    /// Three_Hours
    #[serde(rename = "Three_Hours")]
    Threehours,

    /// Twelve_Hours
    #[serde(rename = "Twelve_Hours")]
    Twelvehours,

    /// TwentyFour_Hours
    #[serde(rename = "TwentyFour_Hours")]
    Twentyfourhours,
}

impl Default for OrganizationManagedRuleMetadataMaximumExecutionFrequencyEnum {
    fn default() -> Self {
        OrganizationManagedRuleMetadataMaximumExecutionFrequencyEnum::Onehour
    }
}

impl cfn_resources::CfnResource for OrganizationManagedRuleMetadata {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'description'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.input_parameters {
            if the_val.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'input_parameters'. {} is greater than 2048",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.input_parameters {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'input_parameters'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.resource_id_scope {
            if the_val.len() > 768 as _ {
                return Err(format!(
                    "Max validation failed on field 'resource_id_scope'. {} is greater than 768",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.resource_id_scope {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'resource_id_scope'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.resource_types_scope {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'resource_types_scope'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.rule_identifier;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'rule_identifier'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.rule_identifier;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'rule_identifier'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.tag_key_scope {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'tag_key_scope'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.tag_key_scope {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'tag_key_scope'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.tag_value_scope {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'tag_value_scope'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.tag_value_scope {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'tag_value_scope'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
