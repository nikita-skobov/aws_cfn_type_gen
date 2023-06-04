/// The AWS::Route53Resolver::ResolverQueryLoggingConfigAssociation resource is a configuration for DNS query logging. After you create a query logging configuration, Amazon Route 53 begins to publish log data to an Amazon CloudWatch Logs log group.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnResolverQueryLoggingConfigAssociation {
    ///
    /// The ID of the query logging configuration that a VPC is associated with.
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
    #[serde(rename = "ResolverQueryLogConfigId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub resolver_query_log_config_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the Amazon VPC that is associated with the query logging configuration.
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub resource_id: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_creation_time: CfnResolverQueryLoggingConfigAssociationcreationtime,

    #[serde(skip_serializing)]
    pub att_error: CfnResolverQueryLoggingConfigAssociationerror,

    #[serde(skip_serializing)]
    pub att_error_message: CfnResolverQueryLoggingConfigAssociationerrormessage,

    #[serde(skip_serializing)]
    pub att_id: CfnResolverQueryLoggingConfigAssociationid,

    #[serde(skip_serializing)]
    pub att_status: CfnResolverQueryLoggingConfigAssociationstatus,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverQueryLoggingConfigAssociationcreationtime;
impl CfnResolverQueryLoggingConfigAssociationcreationtime {
    pub fn att_name(&self) -> &'static str {
        r#"CreationTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverQueryLoggingConfigAssociationerror;
impl CfnResolverQueryLoggingConfigAssociationerror {
    pub fn att_name(&self) -> &'static str {
        r#"Error"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverQueryLoggingConfigAssociationerrormessage;
impl CfnResolverQueryLoggingConfigAssociationerrormessage {
    pub fn att_name(&self) -> &'static str {
        r#"ErrorMessage"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverQueryLoggingConfigAssociationid;
impl CfnResolverQueryLoggingConfigAssociationid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverQueryLoggingConfigAssociationstatus;
impl CfnResolverQueryLoggingConfigAssociationstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

impl cfn_resources::CfnResource for CfnResolverQueryLoggingConfigAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::Route53Resolver::ResolverQueryLoggingConfigAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.resolver_query_log_config_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!("Max validation failed on field 'resolver_query_log_config_id'. {} is greater than 64", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.resolver_query_log_config_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'resolver_query_log_config_id'. {} is less than 1", s.len()));
                }
            }
        }

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
