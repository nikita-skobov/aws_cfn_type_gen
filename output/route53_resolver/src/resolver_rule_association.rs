

/// In the response to an 			AssociateResolverRule, 			DisassociateResolverRule, 			or 			ListResolverRuleAssociations 			request, provides information about an association between a resolver rule and a VPC. The association determines which 			DNS queries that originate in the VPC are forwarded to your network.
#[derive(Default, serde::Serialize)]
pub struct CfnResolverRuleAssociation {


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
    pub vpcid: String,


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
    pub resolver_rule_id: String,


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
    pub name: Option<String>,

}