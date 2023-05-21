

/// Use a web ACL association to define an association between a web ACL and a regional application resource, to protect the resource.      A regional application can be an Application Load Balancer (ALB), an Amazon API Gateway REST API, an AWS AppSync GraphQL API,      an Amazon Cognito user pool, or an AWS App Runner service.
///
/// For Amazon CloudFront, don't use this resource. Instead, use your CloudFront distribution configuration. To associate a web ACL with a distribution, provide the Amazon Resource Name (ARN) of the AWS::WAFv2::WebACL to your CloudFront distribution configuration. To disassociate a web ACL, provide an empty ARN. For information, see AWS::CloudFront::Distribution.
///
/// When you create a web ACL or make changes to a web ACL or web ACL components, like rules and rule groups, AWS WAF propagates the changes everywhere that the web ACL and its components are stored and used. Your changes are applied within seconds, but there might be a brief period of inconsistency when the changes have arrived in some places and not in others. So, for example, if you change a rule action setting, the action might be the old action in one area and the new action in another area. Or if you add an IP address to an IP set used in a blocking rule, the new address might briefly be blocked in one area while still allowed in another. This temporary inconsistency can occur when you first associate a web ACL with an AWS resource and when you change a web ACL that is already associated with a resource. Generally, any inconsistencies of this type last only a few seconds.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnWebACLAssociation {


    /// 
    /// The Amazon Resource Name (ARN) of the web ACL that you want to associate with the     resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "WebACLArn")]
    pub web_aclarn: String,


    /// 
    /// The Amazon Resource Name (ARN) of the resource to associate with the web ACL.
    /// 
    /// The ARN must be in one of the following formats:
    /// 
    /// For an Application Load Balancer: arn:aws:elasticloadbalancing:region:account-id:loadbalancer/app/load-balancer-name/load-balancer-id                                For an Amazon API Gateway REST API: arn:aws:apigateway:region::/restapis/api-id/stages/stage-name                                For an AWS AppSync GraphQL API: arn:aws:appsync:region:account-id:apis/GraphQLApiId                                For an Amazon Cognito user pool: arn:aws:cognito-idp:region:account-id:userpool/user-pool-id                                For an AWS App Runner service: arn:aws:apprunner:region:account-id:service/apprunner-service-name/apprunner-service-id
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,

}



impl cfn_resources::CfnResource for CfnWebACLAssociation {
    fn type_string() -> &'static str {
        "AWS::WAFv2::WebACLAssociation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
