/// Creates a private namespace based on DNS, which is visible only inside a specified Amazon  VPC. The namespace defines your service naming scheme. For example, if you name your namespace   example.com and name your service backend, the resulting DNS name for  the service is backend.example.com. Service instances that are registered using a  private DNS namespace can be discovered using either a DiscoverInstances request or  using DNS. For the current quota on the number of namespaces that you can create using the same   AWS account, see AWS Cloud Map quotas in the           AWS Cloud Map Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnPrivateDnsNamespace {
    ///
    /// A description for the namespace.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name that you want to assign to this namespace. When you create a private DNS namespace,  AWS Cloud Map automatically creates an Amazon Route 53 private hosted zone that has the same name as the  namespace.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 253
    ///
    /// Pattern: ^[!-~]{1,253}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Properties for the  private DNS namespace.
    ///
    /// Required: No
    ///
    /// Type: Properties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Properties>,

    ///
    /// The tags for the namespace. Each tag consists of a key and an optional value, both of which you define. Tag keys  can have a maximum character length of 128 characters, and tag values can have a maximum length of 256  characters.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The ID of the Amazon VPC that you want to associate the namespace with.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "Vpc")]
    pub vpc: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_arn: CfnPrivateDnsNamespacearn,

    #[serde(skip_serializing)]
    pub att_hosted_zone_id: CfnPrivateDnsNamespacehostedzoneid,

    #[serde(skip_serializing)]
    pub att_id: CfnPrivateDnsNamespaceid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPrivateDnsNamespacearn;
impl CfnPrivateDnsNamespacearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPrivateDnsNamespacehostedzoneid;
impl CfnPrivateDnsNamespacehostedzoneid {
    pub fn att_name(&self) -> &'static str {
        r#"HostedZoneId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPrivateDnsNamespaceid;
impl CfnPrivateDnsNamespaceid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnPrivateDnsNamespace {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceDiscovery::PrivateDnsNamespace"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 253 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 253",
                    s.len()
                ));
            }
        }

        self.properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.tags {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.vpc;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'vpc'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// DNS properties for  the private DNS namespace.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PrivateDnsPropertiesMutable {
    ///
    /// Fields for the Start  of Authority (SOA) record for the hosted zone for the private DNS  namespace.
    ///
    /// Required: No
    ///
    /// Type: SOA
    ///
    /// Update requires: No interruption
    #[serde(rename = "SOA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soa: Option<SOA>,
}

impl cfn_resources::CfnResource for PrivateDnsPropertiesMutable {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.soa.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Properties for the  private DNS namespace.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Properties {
    ///
    /// DNS properties for  the private DNS namespace.
    ///
    /// Required: No
    ///
    /// Type: PrivateDnsPropertiesMutable
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_properties: Option<PrivateDnsPropertiesMutable>,
}

impl cfn_resources::CfnResource for Properties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.dns_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Start of Authority  (SOA) properties for a public or private DNS namespace.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SOA {
    ///
    /// The time to live  (TTL) for purposes of negative caching.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "TTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<f64>,
}

impl cfn_resources::CfnResource for SOA {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
