/// Creates a VPC endpoint service configuration to which service consumers (AWS accounts,      users, and IAM roles) can connect.
///
/// To create an endpoint service configuration, you must first create one of the following     for your service:
///
/// For more information, see the AWS PrivateLink User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance_required: Option<bool>,

    ///
    /// Indicates whether to enable the built-in Contributor Insights rules.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContributorInsightsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_insights_enabled: Option<bool>,

    ///
    /// The Amazon Resource Names (ARNs) of the Gateway Load Balancers.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GatewayLoadBalancerArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_load_balancer_arns: Option<Vec<String>>,

    ///
    /// The Amazon Resource Names (ARNs) of the Network Load Balancers.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkLoadBalancerArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_load_balancer_arns: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_responsibility: Option<VPCEndpointServicePayerResponsibilityEnum>,

    #[serde(skip_serializing)]
    pub att_service_id: CfnVPCEndpointServiceserviceid,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum VPCEndpointServicePayerResponsibilityEnum {
    /// ServiceOwner
    #[serde(rename = "ServiceOwner")]
    Serviceowner,
}

impl Default for VPCEndpointServicePayerResponsibilityEnum {
    fn default() -> Self {
        VPCEndpointServicePayerResponsibilityEnum::Serviceowner
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVPCEndpointServiceserviceid;
impl CfnVPCEndpointServiceserviceid {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceId"#
    }
}

impl cfn_resources::CfnResource for CfnVPCEndpointService {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::VPCEndpointService"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
