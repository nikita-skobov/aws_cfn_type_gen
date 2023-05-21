

/// The AWS::GlobalAccelerator::EndpointGroup resource is a Global Accelerator resource type that contains information about 		how you create an endpoint group for the specified listener. An endpoint group is a collection of endpoints in one AWS Region.
#[derive(Default, serde::Serialize)]
pub struct CfnEndpointGroup {


    /// 
    /// Allows you to override the destination ports used to route traffic to an endpoint. 			Using a port override lets you map a list of external destination ports (that your 			users send traffic to) to a list of internal destination ports that you want an application 			endpoint to receive traffic on.
    /// 
    /// Required: No
    ///
    /// Type: List of PortOverride
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortOverrides")]
    pub port_overrides: Option<Vec<PortOverride>>,


    /// 
    /// The protocol that Global Accelerator uses to perform health checks on endpoints that are part of this endpoint group. The default 			value is TCP.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HTTP | HTTPS | TCP
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckProtocol")]
    pub health_check_protocol: Option<String>,


    /// 
    /// The percentage of traffic to send to an AWS Regions. Additional traffic is distributed to other endpoint groups for 			this listener.
    /// 
    /// Use this action to increase (dial up) or decrease (dial down) traffic to a specific Region. The percentage is 			applied to the traffic that would otherwise have been routed to the Region based on optimal routing.
    /// 
    /// The default value is 100.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrafficDialPercentage")]
    pub traffic_dial_percentage: Option<f64>,


    /// 
    /// The port that Global Accelerator uses to perform health checks on endpoints that are part of this endpoint group.
    /// 
    /// The default port is the port for the listener that this endpoint group is associated with. If the listener port is a 		  list, Global Accelerator uses the first specified port in the list of ports.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckPort")]
    pub health_check_port: Option<i64>,


    /// 
    /// The Amazon Resource Name (ARN) of the listener.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "ListenerArn")]
    pub listener_arn: String,


    /// 
    /// If the protocol is HTTP/S, then this value provides the ping path that Global Accelerator uses for the destination on the 			endpoints for health checks. The default is slash (/).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^/[-a-zA-Z0-9@:%_\\+.~#?&/=]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckPath")]
    pub health_check_path: Option<String>,


    /// 
    /// The time—10 seconds or 30 seconds—between health checks for each endpoint. The default value is 30.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 10
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckIntervalSeconds")]
    pub health_check_interval_seconds: Option<i64>,


    /// 
    /// The number of consecutive health checks required to set the state of a healthy endpoint to unhealthy, or to set an 			unhealthy endpoint to healthy. The default value is 3.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThresholdCount")]
    pub threshold_count: Option<i64>,


    /// 
    /// The AWS Regions where the endpoint group is located.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndpointGroupRegion")]
    pub endpoint_group_region: String,


    /// 
    /// The list of endpoint objects.
    /// 
    /// Required: No
    ///
    /// Type: List of EndpointConfiguration
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointConfigurations")]
    pub endpoint_configurations: Option<Vec<EndpointConfiguration>>,

}


/// Override specific listener ports used to route traffic to endpoints that are part of an endpoint group. 			For example, you can create a port override in which the listener 			receives user traffic on ports 80 and 443, but your accelerator routes that traffic to ports 1080 			and 1443, respectively, on the endpoints.
///
/// For more information, see 			Port overrides in the AWS Global Accelerator Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct PortOverride {


    /// 
    /// The listener port that you want to map to a specific endpoint port. This is the port that user traffic 		  arrives to the Global Accelerator on.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "ListenerPort")]
    pub listener_port: i64,


    /// 
    /// The endpoint port that you want a listener port to be mapped to. This is the port on the endpoint, 		  such as the Application Load Balancer or Amazon EC2 instance.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointPort")]
    pub endpoint_port: i64,

}


/// A complex type for endpoints. A resource must be valid and active when you add it as an endpoint.
#[derive(Default, serde::Serialize)]
pub struct EndpointConfiguration {


    /// 
    /// The weight associated with the endpoint. When you add weights to endpoints, you configure Global Accelerator to route traffic 			based on proportions that you specify. For example, you might specify endpoint weights of 4, 5, 5, and 6 (sum=20). The 			result is that 4/20 of your traffic, on average, is routed to the first endpoint, 5/20 is routed both to the second 			and third endpoints, and 6/20 is routed to the last endpoint. For more information, see Endpoint Weights in the 		    	AWS Global Accelerator Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Weight")]
    pub weight: Option<i64>,


    /// 
    /// An ID for the endpoint. If the endpoint is a Network Load Balancer or Application Load Balancer, this is the Amazon 			Resource Name (ARN) of the resource. If the endpoint is an Elastic IP address, this is the Elastic IP address 			allocation ID. For Amazon EC2 instances, this is the EC2 instance ID. A resource must be valid and active 			when you add it as an endpoint.
    /// 
    /// An Application Load Balancer can be either internal or internet-facing.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointId")]
    pub endpoint_id: String,


    /// 
    /// Indicates whether client IP address preservation is enabled for an Application Load Balancer endpoint. 			The value is true or false. The default value is true for new accelerators.
    /// 
    /// If the value is set to true, the client's IP address is preserved in the X-Forwarded-For request header as 			traffic travels to applications on the Application Load Balancer endpoint fronted by the accelerator.
    /// 
    /// For more information, see 			Preserve Client IP Addresses in the AWS Global Accelerator Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientIPPreservationEnabled")]
    pub client_ippreservation_enabled: Option<bool>,

}