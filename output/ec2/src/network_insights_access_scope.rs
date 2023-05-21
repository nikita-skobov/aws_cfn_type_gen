

/// Describes a Network Access Scope. A Network Access Scope defines outbound (egress) and inbound (ingress)      traffic patterns, including sources, destinations, paths, and traffic types.
///
/// Network Access Analyzer identifies unintended network access to your resources on       AWS. When you start an analysis on a Network Access Scope, Network     Access Analyzer produces findings. For more information, see the Network Access Analyzer       User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnNetworkInsightsAccessScope {


    /// 
    /// The paths to exclude.
    /// 
    /// Required: No
    ///
    /// Type: List of AccessScopePathRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExcludePaths")]
    pub exclude_paths: Option<Vec<AccessScopePathRequest>>,


    /// 
    /// The paths to match.
    /// 
    /// Required: No
    ///
    /// Type: List of AccessScopePathRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "MatchPaths")]
    pub match_paths: Option<Vec<AccessScopePathRequest>>,


    /// 
    /// The tags.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnNetworkInsightsAccessScope {
    fn type_string() -> &'static str {
        "AWS::EC2::NetworkInsightsAccessScope"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Describes a path.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessScopePathRequest {


    /// 
    /// The destination.
    /// 
    /// Required: No
    ///
    /// Type: PathStatementRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "Destination")]
    pub destination: Option<PathStatementRequest>,


    /// 
    /// The source.
    /// 
    /// Required: No
    ///
    /// Type: PathStatementRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "Source")]
    pub source: Option<PathStatementRequest>,


    /// 
    /// The through resources.
    /// 
    /// Required: No
    ///
    /// Type: List of ThroughResourcesStatementRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThroughResources")]
    pub through_resources: Option<Vec<ThroughResourcesStatementRequest>>,

}



impl cfn_resources::CfnResource for AccessScopePathRequest {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.destination.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.source.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes a packet header statement.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PacketHeaderStatementRequest {


    /// 
    /// The destination addresses.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationAddresses")]
    pub destination_addresses: Option<Vec<String>>,


    /// 
    /// The destination ports.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationPorts")]
    pub destination_ports: Option<Vec<String>>,


    /// 
    /// The destination prefix lists.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationPrefixLists")]
    pub destination_prefix_lists: Option<Vec<String>>,


    /// 
    /// The protocols.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Protocols")]
    pub protocols: Option<Vec<String>>,


    /// 
    /// The source addresses.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceAddresses")]
    pub source_addresses: Option<Vec<String>>,


    /// 
    /// The source ports.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourcePorts")]
    pub source_ports: Option<Vec<String>>,


    /// 
    /// The source prefix lists.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourcePrefixLists")]
    pub source_prefix_lists: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for PacketHeaderStatementRequest {
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

/// Describes a path statement.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PathStatementRequest {


    /// 
    /// The packet header statement.
    /// 
    /// Required: No
    ///
    /// Type: PacketHeaderStatementRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "PacketHeaderStatement")]
    pub packet_header_statement: Option<PacketHeaderStatementRequest>,


    /// 
    /// The resource statement.
    /// 
    /// Required: No
    ///
    /// Type: ResourceStatementRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceStatement")]
    pub resource_statement: Option<ResourceStatementRequest>,

}



impl cfn_resources::CfnResource for PathStatementRequest {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.packet_header_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.resource_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes a resource statement.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourceStatementRequest {


    /// 
    /// The resource types.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceTypes")]
    pub resource_types: Option<Vec<String>>,


    /// 
    /// The resources.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Resources")]
    pub resources: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for ResourceStatementRequest {
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

/// Describes a through resource statement.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ThroughResourcesStatementRequest {


    /// 
    /// The resource statement.
    /// 
    /// Required: No
    ///
    /// Type: ResourceStatementRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceStatement")]
    pub resource_statement: Option<ResourceStatementRequest>,

}



impl cfn_resources::CfnResource for ThroughResourcesStatementRequest {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.resource_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}