/// The AWS::Lambda::LayerVersionPermission resource adds permissions to the resource-based policy of    a version of an Lambda     layer. Use this action to grant layer usage permission to other accounts. You can grant permission to a    single account, all AWS accounts, or all accounts in an organization.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnLayerVersionPermission {
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
    pub action: cfn_resources::StrVal,

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
    pub layer_version_arn: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<cfn_resources::StrVal>,

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
    pub principal: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnLayerVersionPermission {
    fn type_string(&self) -> &'static str {
        "AWS::Lambda::LayerVersionPermission"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.action;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 22 as _ {
                return Err(format!(
                    "Max validation failed on field 'action'. {} is greater than 22",
                    s.len()
                ));
            }
        }

        let the_val = &self.layer_version_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 140 as _ {
                return Err(format!(
                    "Max validation failed on field 'layer_version_arn'. {} is greater than 140",
                    s.len()
                ));
            }
        }

        let the_val = &self.layer_version_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'layer_version_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.organization_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 34 as _ {
                    return Err(format!(
                        "Max validation failed on field 'organization_id'. {} is greater than 34",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
