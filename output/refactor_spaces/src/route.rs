/// Creates an AWS Migration Hub Refactor Spaces route. The account owner of the service resource is always the    environment owner, regardless of which account creates the route. Routes target a service in    the application. If an application does not have any routes, then the first route must be    created as a DEFAULT    RouteType.
///
/// When created, the default route defaults to an active state so state is not a required    input. However, like all other state values the state of the default route can be updated    after creation, but only when all other routes are also inactive. Conversely, no route can be    active without the default route also being active.
///
/// When you create a route, Refactor Spaces configures the Amazon API Gateway to send traffic    to the target service as follows:
///
/// Environments without a network bridge
///
/// When you create environments without a network bridge (CreateEnvironment:NetworkFabricType is NONE) and you use your own    networking infrastructure, you need to configure VPC to VPC connectivity between your network and the application proxy VPC. Route    creation from the application proxy to service endpoints will fail if your network is not    configured to connect to the application proxy VPC. For more information, see Create     a route in the Refactor Spaces User Guide.
///
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRoute {
    ///
    /// The unique identifier of the application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationIdentifier")]
    pub application_identifier: String,

    ///
    /// Configuration for the default route type.
    ///
    /// Required: No
    ///
    /// Type: DefaultRouteInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultRoute")]
    pub default_route: Option<DefaultRouteInput>,

    ///
    /// The unique identifier of the environment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnvironmentIdentifier")]
    pub environment_identifier: String,

    ///
    /// The route type of the route.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RouteType")]
    pub route_type: String,

    ///
    /// The unique identifier of the service.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceIdentifier")]
    pub service_identifier: String,

    ///
    /// The tags assigned to the route.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The configuration for the URI path route type.
    ///
    /// Required: No
    ///
    /// Type: UriPathRouteInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "UriPathRoute")]
    pub uri_path_route: Option<UriPathRouteInput>,
}

impl cfn_resources::CfnResource for CfnRoute {
    fn type_string(&self) -> &'static str {
        "AWS::RefactorSpaces::Route"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.default_route
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.uri_path_route
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The configuration for the default route type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DefaultRouteInput {
    ///
    /// If set to ACTIVE, traffic is forwarded to this route’s service after the    route is created.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActivationState")]
    pub activation_state: String,
}

impl cfn_resources::CfnResource for DefaultRouteInput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The configuration for the URI path route type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UriPathRouteInput {
    ///
    /// If set to ACTIVE, traffic is forwarded to this route’s service after the    route is created.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActivationState")]
    pub activation_state: String,

    ///
    /// Indicates whether to match all subpaths of the given source path. If this value is     false, requests must match the source path exactly before they are forwarded to    this route's service.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "IncludeChildPaths")]
    pub include_child_paths: Option<bool>,

    ///
    /// A list of HTTP methods to match. An empty list matches all values. If a method is present,    only HTTP requests using that method are forwarded to this route’s service.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Methods")]
    pub methods: Option<Vec<String>>,

    ///
    /// The path to use to match traffic. Paths must start with / and are relative to    the base of the application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourcePath")]
    pub source_path: Option<String>,
}

impl cfn_resources::CfnResource for UriPathRouteInput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
