/// The AWS::Route53Resolver::ResolverDNSSECConfig resource is a complex type that contains information about a configuration for DNSSEC validation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnResolverDNSSECConfig {
    ///
    /// The ID of the virtual private cloud (VPC) that you're configuring the DNSSEC validation status for.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_id: CfnResolverDNSSECConfigid,

    #[serde(skip_serializing)]
    pub att_owner_id: CfnResolverDNSSECConfigownerid,

    #[serde(skip_serializing)]
    pub att_validation_status: CfnResolverDNSSECConfigvalidationstatus,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverDNSSECConfigid;
impl CfnResolverDNSSECConfigid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverDNSSECConfigownerid;
impl CfnResolverDNSSECConfigownerid {
    pub fn att_name(&self) -> &'static str {
        r#"OwnerId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverDNSSECConfigvalidationstatus;
impl CfnResolverDNSSECConfigvalidationstatus {
    pub fn att_name(&self) -> &'static str {
        r#"ValidationStatus"#
    }
}

impl cfn_resources::CfnResource for CfnResolverDNSSECConfig {
    fn type_string(&self) -> &'static str {
        "AWS::Route53Resolver::ResolverDNSSECConfig"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.resource_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!(
                        "Max validation failed on field 'resource_id'. {} is greater than 64",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.resource_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'resource_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
