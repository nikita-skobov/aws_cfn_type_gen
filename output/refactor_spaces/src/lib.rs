
pub mod cfn_application {

#[derive(serde::Serialize, Default)]
pub struct CfnApplication {
    /// Metadata that you can assign to help organize the frameworks that you create. Each tag is a key-value pair.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "ApiGatewayProxy")]
    pub api_gateway_proxy: Option<ApiGatewayProxyInput>,
    /// No documentation provided by AWS
    #[serde(rename = "ProxyType")]
    pub proxy_type: ProxyType,
    /// No documentation provided by AWS
    #[serde(rename = "EnvironmentIdentifier")]
    pub environment_identifier: String,
    /// No documentation provided by AWS
    #[serde(rename = "VpcId")]
    pub vpc_id: String,

}

pub type ApiGatewayEndpointType = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type ProxyType = String;
#[derive(serde::Serialize, Default)]
pub struct ApiGatewayProxyInput {
    #[serde(rename = "StageName")]
    pub stage_name: Option<String>,
    #[serde(rename = "EndpointType")]
    pub endpoint_type: Option<ApiGatewayEndpointType>,

}


}

pub mod cfn_environment {

#[derive(serde::Serialize, Default)]
pub struct CfnEnvironment {
    /// Metadata that you can assign to help organize the frameworks that you create. Each tag is a key-value pair.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "NetworkFabricType")]
    pub network_fabric_type: NetworkFabricType,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}

pub type NetworkFabricType = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_route {

#[derive(serde::Serialize, Default)]
pub struct CfnRoute {
    /// No documentation provided by AWS
    #[serde(rename = "ServiceIdentifier")]
    pub service_identifier: String,
    /// No documentation provided by AWS
    #[serde(rename = "RouteType")]
    pub route_type: RouteType,
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationIdentifier")]
    pub application_identifier: String,
    /// No documentation provided by AWS
    #[serde(rename = "EnvironmentIdentifier")]
    pub environment_identifier: String,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultRoute")]
    pub default_route: Option<DefaultRouteInput>,
    /// Metadata that you can assign to help organize the frameworks that you create. Each tag is a key-value pair.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "UriPathRoute")]
    pub uri_path_route: Option<UriPathRouteInput>,

}

pub type RouteType = String;pub type RouteActivationState = String;pub type Method = String;
#[derive(serde::Serialize, Default)]
pub struct DefaultRouteInput {
    #[serde(rename = "ActivationState")]
    pub activation_state: RouteActivationState,

}

#[derive(serde::Serialize, Default)]
pub struct UriPathRouteInput {
    #[serde(rename = "ActivationState")]
    pub activation_state: RouteActivationState,
    #[serde(rename = "SourcePath")]
    pub source_path: Option<String>,
    #[serde(rename = "Methods")]
    pub methods: Option<Vec<Method>>,
    #[serde(rename = "IncludeChildPaths")]
    pub include_child_paths: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_service {

#[derive(serde::Serialize, Default)]
pub struct CfnService {
    /// No documentation provided by AWS
    #[serde(rename = "EnvironmentIdentifier")]
    pub environment_identifier: String,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "UrlEndpoint")]
    pub url_endpoint: Option<UrlEndpointInput>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LambdaEndpoint")]
    pub lambda_endpoint: Option<LambdaEndpointInput>,
    /// Metadata that you can assign to help organize the frameworks that you create. Each tag is a key-value pair.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationIdentifier")]
    pub application_identifier: String,
    /// No documentation provided by AWS
    #[serde(rename = "EndpointType")]
    pub endpoint_type: ServiceEndpointType,

}


#[derive(serde::Serialize, Default)]
pub struct UrlEndpointInput {
    #[serde(rename = "HealthUrl")]
    pub health_url: Option<String>,
    #[serde(rename = "Url")]
    pub url: String,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaEndpointInput {
    #[serde(rename = "Arn")]
    pub arn: String,

}
pub type ServiceEndpointType = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
