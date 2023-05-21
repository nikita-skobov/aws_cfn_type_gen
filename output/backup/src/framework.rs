

/// Creates a framework with one or more controls. A framework is a collection of controls     that you can use to evaluate your backup practices. By using pre-built customizable     controls to define your policies, you can evaluate whether your backup practices comply     with your policies and which resources are not yet in compliance.
///
/// For a sample AWS CloudFormation template, see the AWS Backup Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnFramework {


    /// 
    /// A list of tags with which to tag your framework.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameworkTags")]
    pub framework_tags: Option<Vec<Tag>>,


    /// 
    /// Contains detailed information about all of the controls of a framework. Each framework     must contain at least one control.
    /// 
    /// Required: Yes
    ///
    /// Type: List of FrameworkControl
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameworkControls")]
    pub framework_controls: Vec<FrameworkControl>,


    /// 
    /// An optional description of the framework with a maximum 1,024 characters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "FrameworkDescription")]
    pub framework_description: Option<String>,


    /// 
    /// The unique name of a framework. This name is between 1 and 256 characters, starting with     a letter, and consisting of letters (a-z, A-Z), numbers (0-9), and underscores (_).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z][_a-zA-Z0-9]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "FrameworkName")]
    pub framework_name: Option<String>,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// A framework consists of one or more controls. Each control has its own control scope.     The control scope can include one or more resource types, a combination of a tag key and     value, or a combination of one resource type and one resource ID. If no scope is specified,     evaluations for the rule are triggered when any resource in your recording group changes in     configuration.
#[derive(Default, serde::Serialize)]
pub struct ControlScope {


    /// 
    /// Describes whether the control scope includes one or more types of resources, such as       EFS or RDS.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComplianceResourceTypes")]
    pub compliance_resource_types: Option<Vec<String>>,


    /// 
    /// The ID of the only AWS resource that you want your control scope to     contain.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComplianceResourceIds")]
    pub compliance_resource_ids: Option<Vec<String>>,


    /// 
    /// The tag key-value pair applied to those AWS resources that you want to     trigger an evaluation for a rule. A maximum of one key-value pair can be provided. The tag     value is optional, but it cannot be an empty string. The structure to assign a tag is:       [{"Key":"string","Value":"string"}].
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


/// A list of parameters for a control. A control can have zero, one, or more than one     parameter. An example of a control with two parameters is: "backup plan frequency is at     least daily and the retention period is at least 1 year". The     first parameter is daily. The second parameter is 1 year.
#[derive(Default, serde::Serialize)]
pub struct ControlInputParameter {


    /// 
    /// The value of parameter, for example, hourly.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterValue")]
    pub parameter_value: String,


    /// 
    /// The name of a parameter, for example, BackupPlanFrequency.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterName")]
    pub parameter_name: String,

}


/// Contains detailed information about all of the controls of a framework. Each framework     must contain at least one control.
#[derive(Default, serde::Serialize)]
pub struct FrameworkControl {


    /// 
    /// The name of a control. This name is between 1 and 256 characters.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ControlName")]
    pub control_name: String,


    /// 
    /// A list of ParameterName and ParameterValue pairs.
    /// 
    /// Required: No
    ///
    /// Type: List of ControlInputParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ControlInputParameters")]
    pub control_input_parameters: Option<Vec<ControlInputParameter>>,


    /// 
    /// The scope of a control. The control scope defines what the control will evaluate. Three     examples of control scopes are: a specific backup plan, all backup plans with a specific     tag, or all backup plans. For more information, see ControlScope.
    /// 
    /// Required: No
    ///
    /// Type: ControlScope
    ///
    /// Update requires: No interruption
    #[serde(rename = "ControlScope")]
    pub control_scope: Option<ControlScope>,

}