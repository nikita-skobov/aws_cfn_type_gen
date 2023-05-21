

/// The AWS::SSM::MaintenanceWindow resource represents general information about    a maintenance window for AWS Systems Manager. Maintenance Windows let you define a schedule    for when to perform potentially disruptive actions on your instances, such as patching an    operating system (OS), updating drivers, or installing software. Each maintenance window has a    schedule, a duration, a set of registered targets, and a set of registered tasks.
///
/// For more information, see Systems Manager     Maintenance Windows in the AWS Systems Manager User Guide and         CreateMaintenanceWindow in the AWS Systems Manager API     Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnMaintenanceWindow {


    /// 
    /// Enables a maintenance window task to run on managed instances, even if you have not    registered those instances as targets. If enabled, then you must specify the unregistered    instances (by instance ID) when you register a task with the maintenance window.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowUnassociatedTargets")]
    pub allow_unassociated_targets: bool,


    /// 
    /// The number of hours before the end of the maintenance window that AWS Systems Manager stops scheduling  new tasks for execution.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 23
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cutoff")]
    pub cutoff: i64,


    /// 
    /// A description of the maintenance window.
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
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The duration of the maintenance window in hours.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 24
    ///
    /// Update requires: No interruption
    #[serde(rename = "Duration")]
    pub duration: i64,


    /// 
    /// The date and time, in ISO-8601 Extended format, for when the maintenance window is scheduled  to become inactive.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndDate")]
    pub end_date: Option<String>,


    /// 
    /// The name of the maintenance window.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9_\-.]{3,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The schedule of the maintenance window in the form of a cron or rate expression.
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
    #[serde(rename = "Schedule")]
    pub schedule: String,


    /// 
    /// The number of days to wait to run a maintenance window after the scheduled cron expression  date and time.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 6
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleOffset")]
    pub schedule_offset: Option<i64>,


    /// 
    /// The time zone that the scheduled maintenance window executions are based on, in Internet  Assigned Numbers Authority (IANA) format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleTimezone")]
    pub schedule_timezone: Option<String>,


    /// 
    /// The date and time, in ISO-8601 Extended format, for when the maintenance window is    scheduled to become active. StartDate allows you to delay activation of the Maintenance Window    until the specified future date.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,


    /// 
    /// Optional metadata that you assign to a resource in the form of an arbitrary set of tags    (key-value pairs). Tags enable you to categorize a resource in different ways, such as by    purpose, owner, or environment. For example, you might want to tag a maintenance window to    identify the type of tasks it will run, the types of targets, and the environment it will run    in.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnMaintenanceWindow {
    fn type_string() -> &'static str {
        "AWS::SSM::MaintenanceWindow"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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


