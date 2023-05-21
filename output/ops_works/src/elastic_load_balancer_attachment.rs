/// Attaches an Elastic Load Balancing load balancer to an AWS OpsWorks layer that you     specify.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnElasticLoadBalancerAttachment {
    ///
    /// The Elastic Load Balancing instance name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElasticLoadBalancerName")]
    pub elastic_load_balancer_name: cfn_resources::StrVal,

    ///
    /// The AWS OpsWorks layer ID to which the Elastic Load Balancing load balancer is attached.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LayerId")]
    pub layer_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnElasticLoadBalancerAttachment {
    fn type_string(&self) -> &'static str {
        "AWS::OpsWorks::ElasticLoadBalancerAttachment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
