

/// Contains information about a returned CloudTrail channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnChannel {


    /// 
    /// One or more event data stores to which events arriving through a channel will be logged.
    /// 
    /// Required: No
    ///
    /// Type: List of Destination
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destinations")]
    pub destinations: Option<Vec<Destination>>,


    /// 
    /// The name of the channel.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9._\-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The name of the partner or external event source. You cannot change this name after you create the      channel. A maximum of one channel is allowed per source.
    /// 
    /// A source can be either Custom for all valid non-AWS     events, or the name of a partner event source. For information about the source names for available partners, see Additional information about integration partners in the CloudTrail User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Source")]
    pub source: Option<String>,


    /// 
    /// A list of tags.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnChannel {
    fn type_string() -> &'static str {
        "AWS::CloudTrail::Channel"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Contains information about the destination receiving events.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Destination {


    /// 
    /// For channels used for a CloudTrail Lake integration, the location is the ARN of an event data store that receives events from a channel.      For service-linked channels, the location is the name of the AWS service.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^[a-zA-Z0-9._/\-:]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: String,


    /// 
    /// The type of destination for events arriving from a channel. For channels used for a CloudTrail Lake integration, the value is EventDataStore. For service-linked channels,      the value is AWS_SERVICE.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AWS_SERVICE | EVENT_DATA_STORE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: DestinationTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum DestinationTypeEnum {

    /// AWS_SERVICE
    #[serde(rename = "AWS_SERVICE")]
    Awsservice,

    /// EVENT_DATA_STORE
    #[serde(rename = "EVENT_DATA_STORE")]
    Eventdatastore,

}

impl Default for DestinationTypeEnum {
    fn default() -> Self {
        DestinationTypeEnum::Awsservice
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


