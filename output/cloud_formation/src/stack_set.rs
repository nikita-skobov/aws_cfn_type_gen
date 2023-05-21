

/// The AWS::CloudFormation::StackSet enables you to provision stacks into AWS accounts and across Regions by using a single CloudFormation template. In the stack set, you specify the template  to use, in addition to any parameters and capabilities that the template requires.
#[derive(Default, serde::Serialize)]
pub struct CfnStackSet {


    /// 
    /// The key-value pairs to associate with this stack set and the stacks created from it. AWS CloudFormation  also propagates these tags to supported resources that are created in the stacks. A maximum number of 50 tags can be  specified.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The input parameters for the stack set template.
    /// 
    /// Required: No
    ///
    /// Type: List of Parameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<Parameter>>,


    /// 
    /// Describes how the IAM roles required for stack set operations are created.
    /// 
    /// With SELF_MANAGED permissions, you must create the administrator and execution roles required to   deploy to target accounts. For more information, see Grant Self-Managed Stack Set    Permissions.     With SERVICE_MANAGED permissions, StackSets automatically creates the IAM roles   required to deploy to accounts managed by AWS Organizations. For more information, see Grant    Service-Managed Stack Set Permissions.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: SELF_MANAGED | SERVICE_MANAGED
    ///
    /// Update requires: Replacement
    #[serde(rename = "PermissionModel")]
    pub permission_model: String,


    /// 
    /// [Service-managed permissions] Specifies whether you are acting as an account administrator in the organization's  management account or as a delegated administrator in a member account.
    /// 
    /// By default, SELF is specified. Use SELF for stack sets with self-managed  permissions.
    /// 
    /// To create a stack set with service-managed permissions while signed in to the management account, specify    SELF.     To create a stack set with service-managed permissions while signed in to a delegated administrator account,   specify DELEGATED_ADMIN.   Your AWS account must be registered as a delegated admin in the management account. For more   information, see Register a delegated    administrator in the AWS CloudFormation User Guide.
    /// 
    /// Stack sets with service-managed permissions are created in the management account, including stack sets that are  created by delegated administrators.
    /// 
    /// Valid Values: SELF | DELEGATED_ADMIN
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CallAs")]
    pub call_as: Option<String>,


    /// 
    /// The name of the IAM execution role to use to create the stack set. If you don't specify an  execution role, AWS CloudFormation uses the AWSCloudFormationStackSetExecutionRole role for the  stack set operation.
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 64
    /// 
    /// Pattern: [a-zA-Z_0-9+=,.@-]+
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionRoleName")]
    pub execution_role_name: Option<String>,


    /// 
    /// A description of the stack set.
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 1024
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// Describes whether StackSets performs non-conflicting operations concurrently and queues conflicting  operations.
    /// 
    /// When active, StackSets performs non-conflicting operations concurrently and queues conflicting operations. After  conflicting operations finish, StackSets starts queued operations in request order.
    /// 
    /// NoteIf there are already running or queued operations, StackSets queues all incoming operations even if they are   non-conflicting.You can't modify your stack set's execution configuration while there are running or queued operations for that   stack set.
    /// 
    /// When inactive (default), StackSets performs one operation at a time in request order.
    /// 
    /// Required: No
    ///
    /// Type: ManagedExecution
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManagedExecution")]
    pub managed_execution: Option<ManagedExecution>,


    /// 
    /// Location of file containing the template body. The URL must point to a template (max size: 460,800 bytes) that's  located in an Amazon S3 bucket.
    /// 
    /// You must include either TemplateURL or TemplateBody in a StackSet, but you can't use  both.
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 1024
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateURL")]
    pub template_url: Option<String>,


    /// 
    /// The Amazon Resource Number (ARN) of the IAM role to use to create this stack set. Specify an   IAM role only if you are using customized administrator roles to control which users or groups can  manage specific stack sets within the same administrator account.
    /// 
    /// Use customized administrator roles to control which users or groups can manage specific stack sets within the  same administrator account. For more information, see Prerequisites: Granting Permissions for Stack Set   Operations in the AWS CloudFormation User Guide.
    /// 
    /// Minimum: 20
    /// 
    /// Maximum: 2048
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdministrationRoleARN")]
    pub administration_role_arn: Option<String>,


    /// 
    /// The structure that contains the template body, with a minimum length of 1 byte and a maximum length of 51,200  bytes.
    /// 
    /// You must include either TemplateURL or TemplateBody in a StackSet, but you can't use  both. Dynamic references in the TemplateBody may not work correctly in all cases. It's recommended to  pass templates containing dynamic references through TemplateUrl instead.
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 51200
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateBody")]
    pub template_body: Option<String>,


    /// 
    /// The capabilities that are allowed in the stack set. Some stack set templates might include resources that can  affect permissions in your AWS account—for example, by creating new AWS Identity and Access Management   (IAM) users. For more information, see Acknowledging IAM   Resources in AWS CloudFormation Templates.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Capabilities")]
    pub capabilities: Option<Vec<String>>,


    /// 
    /// [Service-managed permissions] Describes whether StackSets automatically deploys to AWS Organizations accounts that are added to a target organization or organizational unit (OU).
    /// 
    /// Required: No
    ///
    /// Type: AutoDeployment
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoDeployment")]
    pub auto_deployment: Option<AutoDeployment>,


    /// 
    /// The name to associate with the stack set. The name must be unique in the Region where you create your stack  set.
    /// 
    /// Maximum: 128
    /// 
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9-]{0,127}$
    /// 
    /// NoteThe StackSetName property is required.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StackSetName")]
    pub stack_set_name: String,


    /// 
    /// The user-specified preferences for how AWS CloudFormation performs a stack set operation.
    /// 
    /// Required: No
    ///
    /// Type: OperationPreferences
    ///
    /// Update requires: No interruption
    #[serde(rename = "OperationPreferences")]
    pub operation_preferences: Option<OperationPreferences>,


    /// 
    /// A group of stack instances with parameters in some specific accounts and Regions.
    /// 
    /// Required: No
    ///
    /// Type: List of StackInstances
    ///
    /// Update requires: No interruption
    #[serde(rename = "StackInstancesGroup")]
    pub stack_instances_group: Option<Vec<StackInstances>>,

}


/// Stack instances in some specific accounts and Regions.
#[derive(Default, serde::Serialize)]
pub struct StackInstances {


    /// 
    /// The names of one or more Regions where you want to create stack instances using the specified AWS accounts.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Regions")]
    pub regions: Vec<String>,


    /// 
    /// A list of stack set parameters whose values you want to override in the selected stack instances.
    /// 
    /// Required: No
    ///
    /// Type: List of Parameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterOverrides")]
    pub parameter_overrides: Option<Vec<Parameter>>,


    /// 
    /// The AWS  OrganizationalUnitIds or Accounts for which to create stack instances in the specified  Regions.
    /// 
    /// Required: Yes
    ///
    /// Type: DeploymentTargets
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeploymentTargets")]
    pub deployment_targets: DeploymentTargets,

}


/// The Parameter data type.
#[derive(Default, serde::Serialize)]
pub struct Parameter {


    /// 
    /// The key associated with the parameter. If you don't specify a key and value for a particular parameter, AWS CloudFormation uses the default value that's specified in your template.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterKey")]
    pub parameter_key: String,


    /// 
    /// The input value associated with the parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterValue")]
    pub parameter_value: String,

}


/// Describes whether StackSets performs non-conflicting operations concurrently and queues conflicting  operations.
#[derive(Default, serde::Serialize)]
pub struct ManagedExecution {


    /// 
    /// When true, StackSets performs non-conflicting operations concurrently and queues conflicting  operations. After conflicting operations finish, StackSets starts queued operations in request order.
    /// 
    /// NoteIf there are already running or queued operations, StackSets queues all incoming operations even if they are   non-conflicting.You can't modify your stack set's execution configuration while there are running or queued operations for that   stack set.
    /// 
    /// When false (default), StackSets performs one operation at a time in request order.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Active")]
    pub active: Option<bool>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}


/// The AWS OrganizationalUnitIds or Accounts for which to create stack instances in the specified  Regions.
#[derive(Default, serde::Serialize)]
pub struct DeploymentTargets {


    /// 
    /// The names of one or more AWS accounts for which you want to deploy stack set updates.
    /// 
    /// Pattern: ^[0-9]{12}$
    /// 
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Accounts")]
    pub accounts: Option<Vec<String>>,


    /// 
    /// The organization root ID or organizational unit (OU) IDs to which StackSets deploys.
    /// 
    /// Pattern: ^(ou-[a-z0-9]{4,32}-[a-z0-9]{8,32}|r-[a-z0-9]{4,32})$
    /// 
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationalUnitIds")]
    pub organizational_unit_ids: Option<Vec<String>>,


    /// 
    /// Limit deployment targets to individual accounts or include additional accounts with provided OUs.
    /// 
    /// The following is a list of possible values for the AccountFilterType operation.
    /// 
    /// INTERSECTION: StackSets deploys to the accounts specified in Accounts parameter.                           DIFFERENCE: StackSets excludes the accounts specified in Accounts parameter. This   enables user to avoid certain accounts within an OU such as suspended accounts.                        UNION: StackSets includes additional accounts deployment targets.         This is the default value if AccountFilterType is not provided. This enables user to update an   entire OU and individual accounts from a different OU in one request, which used to be two separate   requests.                        NONE: Deploys to all the accounts in specified organizational units (OU).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DIFFERENCE | INTERSECTION | NONE | UNION
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountFilterType")]
    pub account_filter_type: Option<String>,

}


/// [Service-managed permissions] Describes whether StackSets automatically deploys to AWS Organizations accounts that are added to a target organizational unit (OU).
#[derive(Default, serde::Serialize)]
pub struct AutoDeployment {


    /// 
    /// If set to true, StackSets automatically deploys additional stack instances to AWS Organizations accounts that are added to a target organization or organizational unit (OU) in the specified Regions. If an  account is removed from a target organization or OU, StackSets deletes stack instances from the account in the  specified Regions.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// If set to true, stack resources are retained when an account is removed from a target organization  or OU. If set to false, stack resources are deleted. Specify only if Enabled is set to   True.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetainStacksOnAccountRemoval")]
    pub retain_stacks_on_account_removal: Option<bool>,

}


/// The user-specified preferences for how AWS CloudFormation performs a stack set operation. For more  information on maximum concurrent accounts and failure tolerance, see Stack set operation   options.
#[derive(Default, serde::Serialize)]
pub struct OperationPreferences {


    /// 
    /// The number of accounts, per Region, for which this operation can fail before AWS CloudFormation stops the  operation in that Region. If the operation is stopped in a Region, AWS CloudFormation doesn't attempt the  operation in any subsequent Regions.
    /// 
    /// Conditional: You must specify either FailureToleranceCount or   FailureTolerancePercentage (but not both).
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailureToleranceCount")]
    pub failure_tolerance_count: Option<i64>,


    /// 
    /// The concurrency type of deploying StackSets operations in Regions, could be in parallel or one Region at a  time.
    /// 
    /// Allowed values: SEQUENTIAL | PARALLEL
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PARALLEL | SEQUENTIAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegionConcurrencyType")]
    pub region_concurrency_type: Option<String>,


    /// 
    /// The order of the Regions where you want to perform the stack operation.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegionOrder")]
    pub region_order: Option<Vec<String>>,


    /// 
    /// The percentage of accounts, per Region, for which this stack operation can fail before AWS CloudFormation  stops the operation in that Region. If the operation is stopped in a Region, AWS CloudFormation doesn't attempt  the operation in any subsequent Regions.
    /// 
    /// When calculating the number of accounts based on the specified percentage, AWS CloudFormation rounds   down to the next whole number.
    /// 
    /// Conditional: You must specify either FailureToleranceCount or   FailureTolerancePercentage, but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailureTolerancePercentage")]
    pub failure_tolerance_percentage: Option<i64>,


    /// 
    /// The maximum number of accounts in which to perform this operation at one time. This is dependent on the value of   FailureToleranceCount. MaxConcurrentCount is at most one more than the   FailureToleranceCount.
    /// 
    /// Note that this setting lets you specify the maximum for operations. For large deployments,  under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service  throttling.
    /// 
    /// Conditional: You must specify either MaxConcurrentCount or MaxConcurrentPercentage,  but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxConcurrentCount")]
    pub max_concurrent_count: Option<i64>,


    /// 
    /// The maximum percentage of accounts in which to perform this operation at one time.
    /// 
    /// When calculating the number of accounts based on the specified percentage, AWS CloudFormation rounds down  to the next whole number. This is true except in cases where rounding down would result is zero. In this case,   CloudFormation sets the number as one instead.
    /// 
    /// Note that this setting lets you specify the maximum for operations. For large deployments,  under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service  throttling.
    /// 
    /// Conditional: You must specify either MaxConcurrentCount or MaxConcurrentPercentage,  but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxConcurrentPercentage")]
    pub max_concurrent_percentage: Option<i64>,

}
