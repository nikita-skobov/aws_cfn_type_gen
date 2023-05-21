/// Specifies an ingress authorization rule to add to a Client VPN endpoint. Ingress     authorization rules act as firewall rules that grant access to networks. You must configure     ingress authorization rules to enable clients to access resources in AWS     or on-premises networks.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnClientVpnAuthorizationRule {
    ///
    /// The ID of the group to grant access to, for example, the Active Directory group or identity provider (IdP) group. Required if AuthorizeAllGroups is false or not specified.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AccessGroupId")]
    pub access_group_id: Option<String>,

    ///
    /// Indicates whether to grant access to all clients. Specify true to grant all       clients who successfully establish a VPN connection access to the network. Must be set       to true if AccessGroupId is not specified.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "AuthorizeAllGroups")]
    pub authorize_all_groups: Option<bool>,

    ///
    /// The ID of the Client VPN endpoint.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientVpnEndpointId")]
    pub client_vpn_endpoint_id: String,

    ///
    /// A brief description of the authorization rule.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The IPv4 address range, in CIDR notation, of the network for which access is being authorized.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetNetworkCidr")]
    pub target_network_cidr: String,
}

impl cfn_resources::CfnResource for CfnClientVpnAuthorizationRule {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::ClientVpnAuthorizationRule"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
