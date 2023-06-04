/// Associates an attribute group      with an application      to augment the application's metadata      with the group's attributes.      This feature enables applications      to be described      with user-defined details     that are machine-readable,      such as third-party integrations.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAttributeGroupAssociation {
    ///
    /// The name or ID     of the application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Application")]
    pub application: cfn_resources::StrVal,

    ///
    /// The name or ID      of the attribute group      that holds the attributes      to describe the application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AttributeGroup")]
    pub attribute_group: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_application_arn: CfnAttributeGroupAssociationapplicationarn,

    #[serde(skip_serializing)]
    pub att_attribute_group_arn: CfnAttributeGroupAssociationattributegrouparn,

    #[serde(skip_serializing)]
    pub att_id: CfnAttributeGroupAssociationid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAttributeGroupAssociationapplicationarn;
impl CfnAttributeGroupAssociationapplicationarn {
    pub fn att_name(&self) -> &'static str {
        r#"ApplicationArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAttributeGroupAssociationattributegrouparn;
impl CfnAttributeGroupAssociationattributegrouparn {
    pub fn att_name(&self) -> &'static str {
        r#"AttributeGroupArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAttributeGroupAssociationid;
impl CfnAttributeGroupAssociationid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnAttributeGroupAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceCatalogAppRegistry::AttributeGroupAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
