/// Describes an endpoint authorization for authorizing Redshift-managed VPC endpoint access to a cluster across AWS accounts.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnEndpointAuthorization {
    ///
    /// The AWS account ID of either the cluster owner (grantor) or grantee.     If Grantee parameter is true, then the Account value is of the grantor.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "Account")]
    pub account: cfn_resources::StrVal,

    ///
    /// The cluster identifier.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: cfn_resources::StrVal,

    ///
    /// Indicates whether to force the revoke action.       If true, the Redshift-managed VPC endpoints associated with the endpoint authorization are also deleted.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub force: Option<bool>,

    ///
    /// The virtual private cloud (VPC) identifiers to grant access to.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcIds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub vpc_ids: Option<Vec<String>>,

    #[serde(skip_serializing)]
    pub att_authorize_time: CfnEndpointAuthorizationauthorizetime,

    #[serde(skip_serializing)]
    pub att_cluster_status: CfnEndpointAuthorizationclusterstatus,

    #[serde(skip_serializing)]
    pub att_grantee: CfnEndpointAuthorizationgrantee,

    #[serde(skip_serializing)]
    pub att_grantor: CfnEndpointAuthorizationgrantor,

    #[serde(skip_serializing)]
    pub att_status: CfnEndpointAuthorizationstatus,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEndpointAuthorizationauthorizetime;
impl CfnEndpointAuthorizationauthorizetime {
    pub fn att_name(&self) -> &'static str {
        r#"AuthorizeTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEndpointAuthorizationclusterstatus;
impl CfnEndpointAuthorizationclusterstatus {
    pub fn att_name(&self) -> &'static str {
        r#"ClusterStatus"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEndpointAuthorizationgrantee;
impl CfnEndpointAuthorizationgrantee {
    pub fn att_name(&self) -> &'static str {
        r#"Grantee"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEndpointAuthorizationgrantor;
impl CfnEndpointAuthorizationgrantor {
    pub fn att_name(&self) -> &'static str {
        r#"Grantor"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEndpointAuthorizationstatus;
impl CfnEndpointAuthorizationstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

impl cfn_resources::CfnResource for CfnEndpointAuthorization {
    fn type_string(&self) -> &'static str {
        "AWS::Redshift::EndpointAuthorization"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.account;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2147483647 as _ {
                return Err(format!(
                    "Max validation failed on field 'account'. {} is greater than 2147483647",
                    s.len()
                ));
            }
        }

        let the_val = &self.cluster_identifier;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'cluster_identifier'. {} is greater than 2147483647", s.len()));
            }
        }

        Ok(())
    }
}
