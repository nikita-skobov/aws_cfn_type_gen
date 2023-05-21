

/// Creates a Connect attachment from a specified transit gateway attachment. A Connect attachment is a GRE-based tunnel attachment that you can use to establish a connection between a transit gateway and an appliance.
///
/// A Connect attachment uses an existing VPC or AWS Direct Connect attachment as the underlying transport mechanism.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGatewayConnect {


    /// 
    /// The Connect attachment options.
    /// 
    /// protocol (gre)
    /// 
    /// Required: Yes
    ///
    /// Type: TransitGatewayConnectOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "Options")]
    pub options: TransitGatewayConnectOptions,


    /// 
    /// The ID of the attachment from which the Connect attachment was created.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransportTransitGatewayAttachmentId")]
    pub transport_transit_gateway_attachment_id: String,


    /// 
    /// The tags for the attachment.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnTransitGatewayConnect {
    fn type_string() -> &'static str {
        "AWS::EC2::TransitGatewayConnect"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Describes the Connect attachment options.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TransitGatewayConnectOptions {


    /// 
    /// The tunnel protocol.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: gre
    ///
    /// Update requires: Replacement
    #[serde(rename = "Protocol")]
    pub protocol: Option<TransitGatewayConnectOptionsProtocolEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum TransitGatewayConnectOptionsProtocolEnum {

    /// gre
    #[serde(rename = "gre")]
    Gre,

}

impl Default for TransitGatewayConnectOptionsProtocolEnum {
    fn default() -> Self {
        TransitGatewayConnectOptionsProtocolEnum::Gre
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


