

/// Attaches a resource-based permission policy to a secret. A resource-based policy is    optional. For more information, see Authentication and access control     for Secrets Manager
///
/// For information about attaching a policy in the console, see Attach a     permissions policy to a secret.
///
/// Required permissions:    secretsmanager:PutResourcePolicy. For more information, see IAM policy actions for Secrets Manager and Authentication and access control     in Secrets Manager.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResourcePolicy {


    /// 
    /// Specifies whether to block resource-based policies that allow broad access to the secret.    By default, Secrets Manager blocks policies that allow broad access, for example those that    use a wildcard for the principal.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockPublicPolicy")]
    pub block_public_policy: Option<bool>,


    /// 
    /// A JSON-formatted string for an AWS    resource-based policy. For example policies, see Permissions     policy examples.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourcePolicy")]
    pub resource_policy: serde_json::Value,


    /// 
    /// The ARN or name of the secret to attach the resource-based policy.
    /// 
    /// For an ARN, we recommend that you specify a complete ARN rather    than a partial ARN.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecretId")]
    pub secret_id: String,

}



impl cfn_resources::CfnResource for CfnResourcePolicy {
    fn type_string(&self) -> &'static str {
        "AWS::SecretsManager::ResourcePolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}