

/// The AWS::Lambda::LayerVersionPermission resource adds permissions to the resource-based policy of    a version of an Lambda     layer. Use this action to grant layer usage permission to other accounts. You can grant permission to a    single account, all AWS accounts, or all accounts in an organization.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLayerVersionPermission {


    /// 
    /// With the principal set to *, grant permission to all accounts in the specified    organization.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 34
    ///
    /// Pattern: o-[a-z0-9]{10,32}
    ///
    /// Update requires: Replacement
    #[serde(rename = "OrganizationId")]
    pub organization_id: Option<String>,


    /// 
    /// The API action that grants access to the layer. For example, lambda:GetLayerVersion.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 22
    ///
    /// Pattern: lambda:GetLayerVersion
    ///
    /// Update requires: Replacement
    #[serde(rename = "Action")]
    pub action: String,


    /// 
    /// An account ID, or * to grant layer usage permission to all    accounts in an organization, or all AWS accounts (if organizationId is not specified).    For the last case, make sure that you really do want all AWS accounts to have usage permission to this layer.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: \d{12}|\*|arn:(aws[a-zA-Z-]*):iam::\d{12}:root
    ///
    /// Update requires: Replacement
    #[serde(rename = "Principal")]
    pub principal: String,


    /// 
    /// The name or Amazon Resource Name (ARN) of the layer.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 140
    ///
    /// Pattern: (arn:[a-zA-Z0-9-]+:lambda:[a-zA-Z0-9-]+:\d{12}:layer:[a-zA-Z0-9-_]+)|[a-zA-Z0-9-_]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "LayerVersionArn")]
    pub layer_version_arn: String,

}



impl cfn_resources::CfnResource for CfnLayerVersionPermission {
    fn type_string() -> &'static str {
        "AWS::Lambda::LayerVersionPermission"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
