/// The AWS::Route53Resolver::ResolverQueryLoggingConfig resource is a complex type that contains settings for one query logging configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnResolverQueryLoggingConfig {
    ///
    /// The ARN of the resource that you want Resolver to send query logs: an Amazon S3 bucket, a CloudWatch Logs log group, or 			a Kinesis Data Firehose delivery stream.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 600
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub destination_arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of the query logging configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: (?!^[0-9]+$)([a-zA-Z0-9\-_' ']+)
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub name: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_arn: CfnResolverQueryLoggingConfigarn,

    #[serde(skip_serializing)]
    pub att_creation_time: CfnResolverQueryLoggingConfigcreationtime,

    #[serde(skip_serializing)]
    pub att_creator_request_id: CfnResolverQueryLoggingConfigcreatorrequestid,

    #[serde(skip_serializing)]
    pub att_id: CfnResolverQueryLoggingConfigid,

    #[serde(skip_serializing)]
    pub att_owner_id: CfnResolverQueryLoggingConfigownerid,

    #[serde(skip_serializing)]
    pub att_share_status: CfnResolverQueryLoggingConfigsharestatus,

    #[serde(skip_serializing)]
    pub att_status: CfnResolverQueryLoggingConfigstatus,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverQueryLoggingConfigarn;
impl CfnResolverQueryLoggingConfigarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverQueryLoggingConfigcreationtime;
impl CfnResolverQueryLoggingConfigcreationtime {
    pub fn att_name(&self) -> &'static str {
        r#"CreationTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverQueryLoggingConfigcreatorrequestid;
impl CfnResolverQueryLoggingConfigcreatorrequestid {
    pub fn att_name(&self) -> &'static str {
        r#"CreatorRequestId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverQueryLoggingConfigid;
impl CfnResolverQueryLoggingConfigid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverQueryLoggingConfigownerid;
impl CfnResolverQueryLoggingConfigownerid {
    pub fn att_name(&self) -> &'static str {
        r#"OwnerId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverQueryLoggingConfigsharestatus;
impl CfnResolverQueryLoggingConfigsharestatus {
    pub fn att_name(&self) -> &'static str {
        r#"ShareStatus"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverQueryLoggingConfigstatus;
impl CfnResolverQueryLoggingConfigstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

impl cfn_resources::CfnResource for CfnResolverQueryLoggingConfig {
    fn type_string(&self) -> &'static str {
        "AWS::Route53Resolver::ResolverQueryLoggingConfig"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.destination_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 600 as _ {
                    return Err(format!(
                        "Max validation failed on field 'destination_arn'. {} is greater than 600",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.destination_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'destination_arn'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 64",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
