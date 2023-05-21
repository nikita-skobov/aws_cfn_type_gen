

/// Creates a VPC endpoint service configuration to which service consumers (AWS accounts,      users, and IAM roles) can connect.
///
/// To create an endpoint service configuration, you must first create one of the following     for your service:
///
/// For more information, see the AWS PrivateLink User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnVPCEndpointService {


    /// 
    /// Indicates whether requests from service consumers to create an endpoint to your service     must be accepted.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceptanceRequired")]
    pub acceptance_required: Option<bool>,


    /// 
    /// The Amazon Resource Names (ARNs) of the Network Load Balancers.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkLoadBalancerArns")]
    pub network_load_balancer_arns: Option<Vec<String>>,


    /// 
    /// The Amazon Resource Names (ARNs) of the Gateway Load Balancers.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GatewayLoadBalancerArns")]
    pub gateway_load_balancer_arns: Option<Vec<String>>,


    /// 
    /// The entity that is responsible for the endpoint costs. The default is the endpoint owner.       If you set the payer responsibility to the service owner, you cannot set it back to the       endpoint owner.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ServiceOwner
    ///
    /// Update requires: No interruption
    #[serde(rename = "PayerResponsibility")]
    pub payer_responsibility: Option<String>,


    /// 
    /// Indicates whether to enable the built-in Contributor Insights rules.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContributorInsightsEnabled")]
    pub contributor_insights_enabled: Option<bool>,

}