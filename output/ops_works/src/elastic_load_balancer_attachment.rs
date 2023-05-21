

/// Attaches an Elastic Load Balancing load balancer to an AWS OpsWorks layer that you     specify.
#[derive(Default, serde::Serialize)]
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
    pub elastic_load_balancer_name: String,


    /// 
    /// The AWS OpsWorks layer ID to which the Elastic Load Balancing load balancer is attached.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LayerId")]
    pub layer_id: String,

}
