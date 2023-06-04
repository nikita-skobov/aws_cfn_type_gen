/// In the response to an 			AssociateResolverRule, 			DisassociateResolverRule, 			or 			ListResolverRuleAssociations 			request, provides information about an association between a resolver rule and a VPC. The association determines which 			DNS queries that originate in the VPC are forwarded to your network.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnResolverRuleAssociation {
    ///
    /// The name of an association between a Resolver rule and a VPC.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Pattern: (?!^[0-9]+$)([a-zA-Z0-9\-_' ']+)
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the Resolver rule that you associated with the VPC that is specified by VPCId.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResolverRuleId")]
    pub resolver_rule_id: cfn_resources::StrVal,

    ///
    /// The ID of the VPC that you associated the Resolver rule with.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "VPCId")]
    pub vpcid: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_name: CfnResolverRuleAssociationname,

    #[serde(skip_serializing)]
    pub att_resolver_rule_association_id: CfnResolverRuleAssociationresolverruleassociationid,

    #[serde(skip_serializing)]
    pub att_resolver_rule_id: CfnResolverRuleAssociationresolverruleid,

    #[serde(skip_serializing)]
    pub att_vpcid: CfnResolverRuleAssociationvpcid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverRuleAssociationname;
impl CfnResolverRuleAssociationname {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverRuleAssociationresolverruleassociationid;
impl CfnResolverRuleAssociationresolverruleassociationid {
    pub fn att_name(&self) -> &'static str {
        r#"ResolverRuleAssociationId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverRuleAssociationresolverruleid;
impl CfnResolverRuleAssociationresolverruleid {
    pub fn att_name(&self) -> &'static str {
        r#"ResolverRuleId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverRuleAssociationvpcid;
impl CfnResolverRuleAssociationvpcid {
    pub fn att_name(&self) -> &'static str {
        r#"VPCId"#
    }
}

impl cfn_resources::CfnResource for CfnResolverRuleAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::Route53Resolver::ResolverRuleAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
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

        let the_val = &self.resolver_rule_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'resolver_rule_id'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        let the_val = &self.resolver_rule_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'resolver_rule_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.vpcid;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'vpcid'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        let the_val = &self.vpcid;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'vpcid'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
