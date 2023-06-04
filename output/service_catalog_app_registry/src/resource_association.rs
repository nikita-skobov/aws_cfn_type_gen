/// Associates a resource      with an application.      Both the resource and the application can be specified either      by ID or name.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourceAssociation {
    ///
    /// The name or ID      of the application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Application")]
    pub application: cfn_resources::StrVal,

    ///
    /// The name or ID      of the resource      of which the application      will be associated.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Resource")]
    pub resource: cfn_resources::StrVal,

    ///
    /// The type      of resource     of which the application will be associated.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceType")]
    pub resource_type: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_application_arn: CfnResourceAssociationapplicationarn,

    #[serde(skip_serializing)]
    pub att_id: CfnResourceAssociationid,

    #[serde(skip_serializing)]
    pub att_resource_arn: CfnResourceAssociationresourcearn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourceAssociationapplicationarn;
impl CfnResourceAssociationapplicationarn {
    pub fn att_name(&self) -> &'static str {
        r#"ApplicationArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourceAssociationid;
impl CfnResourceAssociationid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourceAssociationresourcearn;
impl CfnResourceAssociationresourcearn {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceArn"#
    }
}

impl cfn_resources::CfnResource for CfnResourceAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceCatalogAppRegistry::ResourceAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
