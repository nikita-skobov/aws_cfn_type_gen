/// The AWS::ApiGateway::Resource resource creates a resource in an API.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResource {
    ///
    /// The parent resource's identifier.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ParentId")]
    pub parent_id: cfn_resources::StrVal,

    ///
    /// The last path segment for this resource.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PathPart")]
    pub path_part: cfn_resources::StrVal,

    ///
    /// The string identifier of the associated RestApi.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RestApiId")]
    pub rest_api_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_resource_id: CfnResourceresourceid,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResourceresourceid;
impl CfnResourceresourceid {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceId"#
    }
}

impl cfn_resources::CfnResource for CfnResource {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGateway::Resource"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
