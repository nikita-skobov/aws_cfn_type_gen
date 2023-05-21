

/// The AWS::IoTAnalytics::Pipeline resource consumes messages from one or more channels and allows      you to process the messages before storing them in a data store. You must specify both a      channel and a datastore activity and, optionally, as many      as 23 additional activities in the pipelineActivities array. For more information, see            How to Use AWS IoT Analytics in the AWS IoT Analytics User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnPipeline {


    /// 
    /// A list of "PipelineActivity" objects. Activities perform transformations on your messages,      such as removing, renaming or adding message attributes; filtering messages based on attribute      values; invoking your Lambda functions on messages for advanced processing; or performing      mathematical transformations to normalize device data.
    /// 
    /// The list can be 2-25 PipelineActivity objects and must      contain both a channel and a datastore activity. Each entry in the      list must contain only one activity, for example:
    /// 
    /// pipelineActivities = [  {   "channel": { ... }  },  {   "lambda": { ... }  },  ...    ]
    /// 
    /// Required: Yes
    ///
    /// Type: List of Activity
    ///
    /// Maximum: 25
    ///
    /// Update requires: No interruption
    #[serde(rename = "PipelineActivities")]
    pub pipeline_activities: Vec<Activity>,


    /// 
    /// The name of the pipeline.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: (^(?!_{2}))(^[a-zA-Z0-9_]+$)
    ///
    /// Update requires: Replacement
    #[serde(rename = "PipelineName")]
    pub pipeline_name: Option<String>,


    /// 
    /// Metadata which can be used to manage the pipeline.
    /// 
    /// For more information, see Tag.
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

}


/// An activity that performs a transformation on a message.
#[derive(Default, serde::Serialize)]
pub struct Activity {


    /// 
    /// Adds other attributes based on existing attributes in the message.
    /// 
    /// Required: No
    ///
    /// Type: AddAttributes
    ///
    /// Update requires: No interruption
    #[serde(rename = "AddAttributes")]
    pub add_attributes: Option<AddAttributes>,


    /// 
    /// Adds data from the AWS IoT device registry to your message.
    /// 
    /// Required: No
    ///
    /// Type: DeviceRegistryEnrich
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceRegistryEnrich")]
    pub device_registry_enrich: Option<DeviceRegistryEnrich>,


    /// 
    /// Adds information from the AWS IoT Device Shadows service to a message.
    /// 
    /// Required: No
    ///
    /// Type: DeviceShadowEnrich
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceShadowEnrich")]
    pub device_shadow_enrich: Option<DeviceShadowEnrich>,


    /// 
    /// Creates a new message using only the specified attributes from the original message.
    /// 
    /// Required: No
    ///
    /// Type: SelectAttributes
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectAttributes")]
    pub select_attributes: Option<SelectAttributes>,


    /// 
    /// Runs a Lambda function to modify the message.
    /// 
    /// Required: No
    ///
    /// Type: Lambda
    ///
    /// Update requires: No interruption
    #[serde(rename = "Lambda")]
    pub lambda: Option<Lambda>,


    /// 
    /// Removes attributes from a message.
    /// 
    /// Required: No
    ///
    /// Type: RemoveAttributes
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveAttributes")]
    pub remove_attributes: Option<RemoveAttributes>,


    /// 
    /// Specifies where to store the processed message data.
    /// 
    /// Required: No
    ///
    /// Type: Datastore
    ///
    /// Update requires: No interruption
    #[serde(rename = "Datastore")]
    pub datastore: Option<Datastore>,


    /// 
    /// Filters a message based on its attributes.
    /// 
    /// Required: No
    ///
    /// Type: Filter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filter")]
    pub filter: Option<Filter>,


    /// 
    /// Determines the source of the messages to be processed.
    /// 
    /// Required: No
    ///
    /// Type: Channel
    ///
    /// Update requires: No interruption
    #[serde(rename = "Channel")]
    pub channel: Option<Channel>,


    /// 
    /// Computes an arithmetic expression using the message's attributes and adds    it to the message.
    /// 
    /// Required: No
    ///
    /// Type: Math
    ///
    /// Update requires: No interruption
    #[serde(rename = "Math")]
    pub math: Option<Math>,

}


/// An activity that adds other attributes based on existing attributes in the message.
#[derive(Default, serde::Serialize)]
pub struct AddAttributes {


    /// 
    /// A list of 1-50 "AttributeNameMapping"     objects that map an existing attribute to a new attribute.
    /// 
    /// NoteThe existing attributes remain in the message,   so if you want to remove the originals,   use "RemoveAttributeActivity".
    /// 
    /// Required: Yes
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    pub attributes: std::collections::HashMap<String, String>,


    /// 
    /// The next activity in the pipeline.
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
    #[serde(rename = "Next")]
    pub next: Option<String>,


    /// 
    /// The name of the 'addAttributes' activity.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// An activity that adds data from the AWS IoT device registry to your message.
#[derive(Default, serde::Serialize)]
pub struct DeviceRegistryEnrich {


    /// 
    /// The name of the attribute that is added to the message.
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
    #[serde(rename = "Attribute")]
    pub attribute: String,


    /// 
    /// The name of the IoT device whose registry information is added to the message.
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
    #[serde(rename = "ThingName")]
    pub thing_name: String,


    /// 
    /// The next activity in the pipeline.
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
    #[serde(rename = "Next")]
    pub next: Option<String>,


    /// 
    /// The ARN of the role that allows access to the device's registry information.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The name of the 'deviceRegistryEnrich' activity.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// An activity that filters a message based on its attributes.
#[derive(Default, serde::Serialize)]
pub struct Filter {


    /// 
    /// An expression that looks like an SQL WHERE clause that must return a Boolean value.
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
    #[serde(rename = "Filter")]
    pub filter: String,


    /// 
    /// The name of the 'filter' activity.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The next activity in the pipeline.
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
    #[serde(rename = "Next")]
    pub next: Option<String>,

}


/// An activity that computes an arithmetic expression using the message's attributes.
#[derive(Default, serde::Serialize)]
pub struct Math {


    /// 
    /// The name of the 'math' activity.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The next activity in the pipeline.
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
    #[serde(rename = "Next")]
    pub next: Option<String>,


    /// 
    /// The name of the attribute that contains the result of the math operation.
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
    #[serde(rename = "Attribute")]
    pub attribute: String,


    /// 
    /// An expression that uses one or more existing attributes and must return an integer value.
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
    #[serde(rename = "Math")]
    pub math: String,

}


/// The datastore activity that specifies where to store the processed data.
#[derive(Default, serde::Serialize)]
pub struct Datastore {


    /// 
    /// The name of the datastore activity.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The name of the data store where processed messages are stored.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: (^(?!_{2}))(^[a-zA-Z0-9_]+$)
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatastoreName")]
    pub datastore_name: String,

}


/// An activity that runs a Lambda function to modify the message.
#[derive(Default, serde::Serialize)]
pub struct Lambda {


    /// 
    /// The number of messages passed to the Lambda function for processing.
    /// 
    /// The AWS Lambda function must be able to process all of these messages within     five minutes, which is the maximum timeout duration for Lambda functions.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchSize")]
    pub batch_size: i64,


    /// 
    /// The next activity in the pipeline.
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
    #[serde(rename = "Next")]
    pub next: Option<String>,


    /// 
    /// The name of the 'lambda' activity.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The name of the Lambda function that is run on the message.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^[a-zA-Z0-9_-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaName")]
    pub lambda_name: String,

}


/// An activity that adds information from the AWS IoT Device Shadows service to a message.
#[derive(Default, serde::Serialize)]
pub struct DeviceShadowEnrich {


    /// 
    /// The name of the IoT device whose shadow information is added to      the message.
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
    #[serde(rename = "ThingName")]
    pub thing_name: String,


    /// 
    /// The name of the attribute that is added to the message.
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
    #[serde(rename = "Attribute")]
    pub attribute: String,


    /// 
    /// The ARN of the role that allows access to the device's shadow.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The next activity in the pipeline.
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
    #[serde(rename = "Next")]
    pub next: Option<String>,


    /// 
    /// The name of the 'deviceShadowEnrich' activity.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// Creates a new message using only the specified attributes     from the original message.
#[derive(Default, serde::Serialize)]
pub struct SelectAttributes {


    /// 
    /// The name of the 'selectAttributes' activity.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The next activity in the pipeline.
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
    #[serde(rename = "Next")]
    pub next: Option<String>,


    /// 
    /// A list of the attributes to select from the message.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    pub attributes: Vec<String>,

}


/// An activity that removes attributes from a message.
#[derive(Default, serde::Serialize)]
pub struct RemoveAttributes {


    /// 
    /// The next activity in the pipeline.
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
    #[serde(rename = "Next")]
    pub next: Option<String>,


    /// 
    /// The name of the 'removeAttributes' activity.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// A list of 1-50 attributes to remove from the message.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    pub attributes: Vec<String>,

}


/// Determines the source of the messages to be processed.
#[derive(Default, serde::Serialize)]
pub struct Channel {


    /// 
    /// The name of the channel from which the messages are processed.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: (^(?!_{2}))(^[a-zA-Z0-9_]+$)
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelName")]
    pub channel_name: String,


    /// 
    /// The name of the 'channel' activity.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The next activity in the pipeline.
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
    #[serde(rename = "Next")]
    pub next: Option<String>,

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
