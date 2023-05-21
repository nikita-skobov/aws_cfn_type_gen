

/// Specifies a permission set within a specified IAM Identity Center instance.
#[derive(Default, serde::Serialize)]
pub struct CfnPermissionSet {


    /// 
    /// Specifies the configuration of the AWS managed or customer managed policy that you want    to set as a permissions boundary. Specify either CustomerManagedPolicyReference    to use the name and path of a customer managed policy, or ManagedPolicyArn to use    the ARN of an AWS managed policy. A permissions boundary represents the maximum permissions    that any policy can grant your role. For more information, see Permissions boundaries for IAM     entities in the IAM User Guide.
    /// 
    /// ImportantPolicies used as permissions boundaries don't provide permissions. You must also attach     an IAM policy to the role. To learn how the effective permissions for a role are     evaluated, see IAM JSON policy      evaluation logic in the IAM User Guide.
    /// 
    /// Required: No
    ///
    /// Type: PermissionsBoundary
    ///
    /// Update requires: No interruption
    #[serde(rename = "PermissionsBoundary")]
    pub permissions_boundary: Option<PermissionsBoundary>,


    /// 
    /// The inline policy that is attached to the permission set.
    /// 
    /// NoteFor Length Constraints, if a valid ARN is provided for a permission set, it is possible for an empty inline policy to be returned.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32768
    ///
    /// Pattern: [\u0009\u000A\u000D\u0020-\u00FF]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "InlinePolicy")]
    pub inline_policy: Option<serde_json::Value>,


    /// 
    /// The name of the permission set.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Pattern: [\w+=,.@-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Used to redirect users within the application during the federation authentication    process.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 240
    ///
    /// Pattern: [a-zA-Z0-9&$@#\\\/%?=~\-_'"|!:,.;*+\[\]\ \(\)\{\}]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "RelayStateType")]
    pub relay_state_type: Option<String>,


    /// 
    /// The tags to attach to the new AWS::SSO::PermissionSet.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Specifies the names and paths of the customer managed policies that you have attached to    your permission set.
    /// 
    /// Required: No
    ///
    /// Type: List of CustomerManagedPolicyReference
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomerManagedPolicyReferences")]
    pub customer_managed_policy_references: Option<Vec<CustomerManagedPolicyReference>>,


    /// 
    /// The description of the AWS::SSO::PermissionSet.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 700
    ///
    /// Pattern: [\u0009\u000A\u000D\u0020-\u007E\u00A1-\u00FF]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// A structure that stores the details of the AWS managed policy.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManagedPolicies")]
    pub managed_policies: Option<Vec<String>>,


    /// 
    /// The length of time that the application user sessions are valid for in the ISO-8601    standard.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^(-?)P(?=\d|T\d)(?:(\d+)Y)?(?:(\d+)M)?(?:(\d+)([DW]))?(?:T(?:(\d+)H)?(?:(\d+)M)?(?:(\d+(?:\.\d+)?)S)?)?$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionDuration")]
    pub session_duration: Option<String>,


    /// 
    /// The ARN of the IAM Identity Center instance under which the operation will be executed. For more     information about ARNs, see Amazon Resource Names (ARNs) and       AWS Service Namespaces in the AWS General Reference.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 10
    ///
    /// Maximum: 1224
    ///
    /// Pattern: arn:(aws|aws-us-gov|aws-cn|aws-iso|aws-iso-b):sso:::instance/(sso)?ins-[a-zA-Z0-9-.]{16}
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceArn")]
    pub instance_arn: String,

}


/// Specifies the configuration of the AWS managed or customer managed policy that you want    to set as a permissions boundary. Specify either CustomerManagedPolicyReference    to use the name and path of a customer managed policy, or ManagedPolicyArn to use    the ARN of an AWS managed policy. A permissions boundary represents the maximum permissions    that any policy can grant your role. For more information, see Permissions boundaries for IAM     entities in the IAM User Guide.
#[derive(Default, serde::Serialize)]
pub struct PermissionsBoundary {


    /// 
    /// The AWS managed policy ARN that you want to attach to a permission set as a permissions    boundary.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:(aws|aws-us-gov|aws-cn|aws-iso|aws-iso-b):iam::aws:policy/[\p{L}\p{M}\p{Z}\p{S}\p{N}\p{P}]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManagedPolicyArn")]
    pub managed_policy_arn: Option<String>,


    /// 
    /// Specifies the name and path of a customer managed policy. You must have an IAM policy that matches the name and path in each AWS account where you want to deploy your permission set.
    /// 
    /// Required: No
    ///
    /// Type: CustomerManagedPolicyReference
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomerManagedPolicyReference")]
    pub customer_managed_policy_reference: Option<CustomerManagedPolicyReference>,

}


/// Specifies the name and path of a customer managed policy. You must have an IAM policy that matches the name and path in each AWS account where you want to deploy your permission set.
#[derive(Default, serde::Serialize)]
pub struct CustomerManagedPolicyReference {


    /// 
    /// The path to the IAM policy that you have configured in each account where you want to deploy your permission set. The default is /. For more information, see Friendly    names and paths in the IAM User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: ((/[A-Za-z0-9\.,\+@=_-]+)*)/
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    pub path: Option<String>,


    /// 
    /// The name of the IAM policy that you have configured in each account where you want to deploy your permission set.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w+=,.@-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

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