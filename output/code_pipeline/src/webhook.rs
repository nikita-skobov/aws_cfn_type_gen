

/// The AWS::CodePipeline::Webhook resource creates and registers your    webhook. After the webhook is created and registered, it triggers your pipeline to start every    time an external event occurs. For more information, see Migrate polling pipelines to use event-based change detection in the AWS CodePipeline     User Guide.
///
/// We strongly recommend that you use AWS Secrets Manager to store your credentials. If you    use Secrets Manager, you must have already configured and stored your secret parameters in    Secrets Manager. For more information, see Using Dynamic References to Specify Template Values.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnWebhook {


    /// 
    /// Properties that configure the authentication applied to incoming webhook trigger       requests. The required properties depend on the authentication type. For GITHUB_HMAC,       only the SecretToken property must be set. For IP, only the         AllowedIPRange property must be set to a valid CIDR range. For       UNAUTHENTICATED, no properties can be set.
    /// 
    /// Required: Yes
    ///
    /// Type: WebhookAuthConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationConfiguration")]
    pub authentication_configuration: WebhookAuthConfiguration,


    /// 
    /// A list of rules applied to the body/payload sent in the POST request to a webhook       URL. All defined rules must pass for the request to be accepted and the pipeline       started.
    /// 
    /// Required: Yes
    ///
    /// Type: List of WebhookFilterRule
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filters")]
    pub filters: Vec<WebhookFilterRule>,


    /// 
    /// Configures a connection between the webhook that was created and the external tool       with events to be detected.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegisterWithThirdParty")]
    pub register_with_third_party: Option<bool>,


    /// 
    /// The name of the pipeline you want to connect to the webhook.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [A-Za-z0-9.@\-_]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetPipeline")]
    pub target_pipeline: String,


    /// 
    /// The version number of the pipeline to be connected to the trigger request.
    /// 
    /// Required: Yes
    /// 
    /// Type: Integer
    /// 
    /// Update requires: No interruption
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetPipelineVersion")]
    pub target_pipeline_version: i64,


    /// 
    /// The name of the webhook.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [A-Za-z0-9.@\-_]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The name of the action in a pipeline you want to connect to the webhook. The action       must be from the source (first) stage of the pipeline.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [A-Za-z0-9.@\-_]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetAction")]
    pub target_action: String,


    /// 
    /// Supported options are GITHUB_HMAC, IP, and UNAUTHENTICATED.
    /// 
    /// For information about the authentication scheme implemented by GITHUB_HMAC,           see Securing your             webhooks on the GitHub Developer website.               IP rejects webhooks trigger requests unless they originate from an IP           address in the IP range whitelisted in the authentication           configuration.               UNAUTHENTICATED accepts all webhook trigger requests regardless of           origin.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: GITHUB_HMAC | IP | UNAUTHENTICATED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Authentication")]
    pub authentication: WebhookAuthenticationEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum WebhookAuthenticationEnum {

    /// GITHUB_HMAC
    #[serde(rename = "GITHUB_HMAC")]
    Githubhmac,

    /// IP
    #[serde(rename = "IP")]
    Ip,

    /// UNAUTHENTICATED
    #[serde(rename = "UNAUTHENTICATED")]
    Unauthenticated,

}

impl Default for WebhookAuthenticationEnum {
    fn default() -> Self {
        WebhookAuthenticationEnum::Githubhmac
    }
}


impl cfn_resources::CfnResource for CfnWebhook {
    fn type_string() -> &'static str {
        "AWS::CodePipeline::Webhook"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The authentication applied to incoming webhook trigger requests.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WebhookAuthConfiguration {


    /// 
    /// The property used to configure acceptance of webhooks in an IP address range. For       IP, only the AllowedIPRange property must be set. This property must be set       to a valid CIDR range.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedIPRange")]
    pub allowed_iprange: Option<String>,


    /// 
    /// The property used to configure GitHub authentication. For GITHUB_HMAC, only the         SecretToken property must be set.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretToken")]
    pub secret_token: Option<String>,

}




/// The event criteria that specify when a webhook notification is sent to your       URL.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WebhookFilterRule {


    /// 
    /// A JsonPath expression that is applied to the body/payload of the webhook. The value       selected by the JsonPath expression must match the value specified in the         MatchEquals field. Otherwise, the request is ignored. For more       information, see Java JsonPath         implementation in GitHub.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 150
    ///
    /// Update requires: No interruption
    #[serde(rename = "JsonPath")]
    pub json_path: String,


    /// 
    /// The value selected by the JsonPath expression must match what is       supplied in the MatchEquals field. Otherwise, the request is ignored.       Properties from the target action configuration can be included as placeholders in this       value by surrounding the action configuration key with curly brackets. For example, if       the value supplied here is "refs/heads/{Branch}" and the target action has an action       configuration property called "Branch" with a value of "main", the         MatchEquals value is evaluated as "refs/heads/main". For a list of       action configuration properties for built-in action types, see Pipeline Structure Reference Action Requirements.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 150
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchEquals")]
    pub match_equals: Option<String>,

}


