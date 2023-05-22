/// The AWS::AppSync::DomainNameApiAssociation resource represents the mapping of your custom     domain name to the assigned API URL.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomainNameApiAssociation {
    ///
    /// The API ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiId")]
    pub api_id: cfn_resources::StrVal,

    ///
    /// The domain name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_api_association_identifier: CfnDomainNameApiAssociationapiassociationidentifier,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomainNameApiAssociationapiassociationidentifier;
impl CfnDomainNameApiAssociationapiassociationidentifier {
    pub fn att_name(&self) -> &'static str {
        r#"ApiAssociationIdentifier"#
    }
}

impl cfn_resources::CfnResource for CfnDomainNameApiAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::AppSync::DomainNameApiAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
