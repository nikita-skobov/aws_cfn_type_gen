

/// Describes a local gateway route table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLocalGatewayRouteTable {


    /// 
    /// The ID of the local gateway.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocalGatewayId")]
    pub local_gateway_id: String,


    /// 
    /// The mode of the local gateway route table.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: coip | direct-vpc-routing
    ///
    /// Update requires: Replacement
    #[serde(rename = "Mode")]
    pub mode: Option<LocalGatewayRouteTableModeEnum>,


    /// 
    /// The tags assigned to the local gateway route table.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum LocalGatewayRouteTableModeEnum {

    /// coip
    #[serde(rename = "coip")]
    Coip,

    /// direct-vpc-routing
    #[serde(rename = "direct-vpc-routing")]
    Directvpcrouting,

}

impl Default for LocalGatewayRouteTableModeEnum {
    fn default() -> Self {
        LocalGatewayRouteTableModeEnum::Coip
    }
}


impl cfn_resources::CfnResource for CfnLocalGatewayRouteTable {
    fn type_string() -> &'static str {
        "AWS::EC2::LocalGatewayRouteTable"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
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



impl cfn_resources::CfnResource for Tag {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}