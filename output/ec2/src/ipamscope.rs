/// In IPAM, a scope is the highest-level container within IPAM. An IPAM contains two default scopes. Each scope represents the IP space for a single network. The private scope is intended for all private IP address space. The public scope is intended for all public IP address space. Scopes enable you to reuse IP addresses across multiple unconnected networks without causing IP address overlap or conflict.
///
/// For more information, see How IPAM works in the Amazon VPC IPAM User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMScope {
    ///
    /// The description of the scope.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the IPAM for which you're creating this scope.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IpamId")]
    pub ipam_id: cfn_resources::StrVal,

    ///
    /// The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value.   For example, to find all resources that have a tag with the key Owner and the value TeamA, specify tag:Owner for the filter name and TeamA for the filter value.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnIPAMScopearn,

    #[serde(skip_serializing)]
    pub att_ipam_arn: CfnIPAMScopeipamarn,

    #[serde(skip_serializing)]
    pub att_ipam_scope_id: CfnIPAMScopeipamscopeid,

    #[serde(skip_serializing)]
    pub att_ipam_scope_type: CfnIPAMScopeipamscopetype,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMScopearn;
impl CfnIPAMScopearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMScopeipamarn;
impl CfnIPAMScopeipamarn {
    pub fn att_name(&self) -> &'static str {
        r#"IpamArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMScopeipamscopeid;
impl CfnIPAMScopeipamscopeid {
    pub fn att_name(&self) -> &'static str {
        r#"IpamScopeId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMScopeipamscopetype;
impl CfnIPAMScopeipamscopetype {
    pub fn att_name(&self) -> &'static str {
        r#"IpamScopeType"#
    }
}

impl cfn_resources::CfnResource for CfnIPAMScope {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::IPAMScope"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
