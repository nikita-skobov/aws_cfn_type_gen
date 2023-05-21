

/// Manages an event type. An event is a business activity that is evaluated for fraud risk. With Amazon Fraud Detector, you generate fraud predictions for events.       An event type defines the structure for an event sent to Amazon Fraud Detector. This includes the variables sent as part of the event, the entity performing the event       (such as a customer), and the labels that classify the event. Example event types include online payment transactions, account registrations, and authentications.
#[derive(Default, serde::Serialize)]
pub struct CfnEventType {


    /// 
    /// The event type entity types.
    /// 
    /// Required: Yes
    ///
    /// Type: List of EntityType
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityTypes")]
    pub entity_types: Vec<EntityType>,


    /// 
    /// The event type name.
    /// 
    /// Pattern : ^[0-9a-z_-]+$
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The event type event variables.
    /// 
    /// Required: Yes
    ///
    /// Type: List of EventVariable
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventVariables")]
    pub event_variables: Vec<EventVariable>,


    /// 
    /// The event type description.
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
    /// The event type labels.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Label
    ///
    /// Update requires: No interruption
    #[serde(rename = "Labels")]
    pub labels: Vec<Label>,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


/// The entity type details.
#[derive(Default, serde::Serialize)]
pub struct EntityType {


    /// 
    /// Timestamp of when the entity type was last updated.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 11
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,


    /// 
    /// The entity type ARN.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn\:aws[a-z-]{0,15}\:frauddetector\:[a-z0-9-]{3,20}\:[0-9]{12}\:[^\s]{2,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: Option<String>,


    /// 
    /// The entity type name.
    /// 
    /// ^[0-9a-z_-]+$
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// Indicates whether the resource is defined within this CloudFormation template and impacts the create, update, and delete behavior of the stack. If the value is true,     CloudFormation will create/update/delete the resource when creating/updating/deleting the stack. If the value is false, CloudFormation will validate that the object exists and      then use it within the resource without making changes to the object.
    /// 
    /// For example, when creating AWS::FraudDetector::EventType you must define at least two variables. You can set Inline=true for these variables and CloudFormation will      create/update/delete the variables as part of stack operations. However, if you set Inline=false, CloudFormation will associate the variables to your event type but not execute any      changes to the variables.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,


    /// 
    /// The entity type description.
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
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Timestamp of when the entity type was created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 11
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,

}


/// The variables associated with this event type.
#[derive(Default, serde::Serialize)]
pub struct EventVariable {


    /// 
    /// The source of the event variable.
    /// 
    /// Valid values: EVENT | EXTERNAL_MODEL_SCORE
    /// 
    /// When defining a variable within a event type, you can only use the EVENT value for DataSource when the Inline property is set to true.      If the Inline property is set false, you can use either EVENT or MODEL_SCORE for DataSource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSource")]
    pub data_source: Option<String>,


    /// 
    /// Timestamp for when event variable was created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,


    /// 
    /// The type of event variable. For more information, see Variable types.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VariableType")]
    pub variable_type: Option<String>,


    /// 
    /// The default value of the event variable
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,


    /// 
    /// Timestamp for when the event variable was last updated.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,


    /// 
    /// The data type of the event variable. For more information, see Data types.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataType")]
    pub data_type: Option<String>,


    /// 
    /// The event variable ARN.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: Option<String>,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The name of the event variable.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The event variable description.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// Indicates whether the resource is defined within this CloudFormation template and impacts the create, update, and delete behavior of the stack. If the value is true,     CloudFormation will create/update/delete the resource when creating/updating/deleting the stack. If the value is false, CloudFormation will validate that the object exists and      then use it within the resource without making changes to the object.
    /// 
    /// For example, when creating AWS::FraudDetector::EventType you must define at least two variables. You can set Inline=true for these variables and CloudFormation will      create/update/delete the Variables as part of stack operations. However, if you set Inline=false, CloudFormation will associate the variables to your event type but not execute any      changes to the variables.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,

}


/// The label associated with the event type.
#[derive(Default, serde::Serialize)]
pub struct Label {


    /// 
    /// The label ARN.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn\:aws[a-z-]{0,15}\:frauddetector\:[a-z0-9-]{3,20}\:[0-9]{12}\:[^\s]{2,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: Option<String>,


    /// 
    /// Timestamp of when the label was last updated.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 11
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,


    /// 
    /// Timestamp of when the event type was created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 11
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The label description.
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
    /// The label name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// Indicates whether the resource is defined within this CloudFormation template and impacts the create, update, and delete behavior of the stack. If the value is true,     CloudFormation will create/update/delete the resource when creating/updating/deleting the stack. If the value is false, CloudFormation will validate that the object exists and      then use it within the resource without making changes to the object.
    /// 
    /// For example, when creating AWS::FraudDetector::EventType you must define at least two variables. You can set Inline=true for these variables and CloudFormation will      create/update/delete the variables as part of stack operations. However, if you set Inline=false, CloudFormation will associate the variables to your EventType but not execute any      changes to the variables.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,

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