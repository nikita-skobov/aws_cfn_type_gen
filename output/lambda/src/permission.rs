/// The AWS::Lambda::Permission resource grants an AWS service or another account permission to use a    function. You can apply the policy at the function level, or specify a qualifier to restrict access to a single    version or alias. If you use a qualifier, the invoker must use the full Amazon Resource Name (ARN) of that version    or alias to invoke the function.
///
/// To grant permission to another account, specify the account ID as the Principal. To grant permission to an organization    defined in AWS Organizations, specify the organization ID as the PrincipalOrgID. For AWS    services, the principal is a domain-style identifier defined by the service, like s3.amazonaws.com or    sns.amazonaws.com. For AWS services, you can also specify the ARN of the associated resource as the     SourceArn. If you grant permission to a service principal without specifying the source, other    accounts could potentially configure resources in their account to invoke your Lambda function.
///
/// If your function has a function URL, you can specify the FunctionUrlAuthType parameter. This adds a condition to your    permission that only applies when your function URL's AuthType matches the specified FunctionUrlAuthType. For    more information about the AuthType parameter, see     Security and auth model for Lambda function URLs.
///
/// This resource adds a statement to a resource-based permission policy for the function. For more information    about function policies, see Lambda Function Policies.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPermission {
    ///
    /// The action that the principal can use on the function. For example, lambda:InvokeFunction or     lambda:GetFunction.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: (lambda:[*]|lambda:[a-zA-Z]+|[*])
    ///
    /// Update requires: Replacement
    #[serde(rename = "Action")]
    pub action: cfn_resources::StrVal,

    ///
    /// For Alexa Smart Home functions, a token that the invoker must supply.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z0-9._\-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "EventSourceToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_token: Option<cfn_resources::StrVal>,

    ///
    /// The name of the Lambda function, version, or alias.
    ///
    /// Name formats                                            Function name – my-function (name-only), my-function:v1 (with alias).                        Function ARN – arn:aws:lambda:us-west-2:123456789012:function:my-function.                        Partial ARN – 123456789012:function:my-function.
    ///
    /// You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN.    If you specify only the function name, it is limited to 64 characters in length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 140
    ///
    /// Pattern: (arn:(aws[a-zA-Z-]*)?:lambda:)?([a-z]{2}(-gov)?-[a-z]+-\d{1}:)?(\d{12}:)?(function:)?([a-zA-Z0-9-_]+)(:(\$LATEST|[a-zA-Z0-9-_]+))?
    ///
    /// Update requires: Replacement
    #[serde(rename = "FunctionName")]
    pub function_name: cfn_resources::StrVal,

    ///
    /// The type of authentication that your function URL uses. Set to AWS_IAM if you want to restrict access to authenticated  users only. Set to NONE if you want to bypass IAM authentication to create a public endpoint. For more information,  see Security and auth model for Lambda function URLs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AWS_IAM | NONE
    ///
    /// Update requires: Replacement
    #[serde(rename = "FunctionUrlAuthType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_url_auth_type: Option<PermissionFunctionUrlAuthTypeEnum>,

    ///
    /// The AWS service or AWS account that invokes the function. If you specify a    service, use SourceArn or SourceAccount to limit who can invoke the function through    that service.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [^\s]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Principal")]
    pub principal: cfn_resources::StrVal,

    ///
    /// The identifier for your organization in AWS Organizations. Use this to grant permissions to all the     AWS accounts under this organization.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 34
    ///
    /// Pattern: ^o-[a-z0-9]{10,32}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "PrincipalOrgID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_org_id: Option<cfn_resources::StrVal>,

    ///
    /// For AWS service, the ID of the AWS account that owns the resource. Use this    together with SourceArn to ensure that the specified account owns the resource. It is possible for an     Amazon S3 bucket to be deleted by its owner and recreated by another account.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 12
    ///
    /// Pattern: \d{12}
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account: Option<cfn_resources::StrVal>,

    ///
    /// For AWS services, the ARN of the AWS resource that invokes the function. For    example, an Amazon S3 bucket or Amazon SNS topic.
    ///
    /// Note that Lambda configures the comparison using the StringLike operator.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: arn:(aws[a-zA-Z0-9-]*):([a-zA-Z0-9\-])+:([a-z]{2}(-gov)?-[a-z]+-\d{1})?:(\d{12})?:(.*)
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PermissionFunctionUrlAuthTypeEnum {
    /// AWS_IAM
    #[serde(rename = "AWS_IAM")]
    Awsiam,

    /// NONE
    #[serde(rename = "NONE")]
    None,
}

impl Default for PermissionFunctionUrlAuthTypeEnum {
    fn default() -> Self {
        PermissionFunctionUrlAuthTypeEnum::Awsiam
    }
}

impl cfn_resources::CfnResource for CfnPermission {
    fn type_string(&self) -> &'static str {
        "AWS::Lambda::Permission"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.event_source_token {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!("Max validation failed on field 'event_source_token'. {} is greater than 256", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.event_source_token {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'event_source_token'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.function_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 140 as _ {
                return Err(format!(
                    "Max validation failed on field 'function_name'. {} is greater than 140",
                    s.len()
                ));
            }
        }

        let the_val = &self.function_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'function_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.principal_org_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 34 as _ {
                    return Err(format!(
                        "Max validation failed on field 'principal_org_id'. {} is greater than 34",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.principal_org_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 12 as _ {
                    return Err(format!(
                        "Min validation failed on field 'principal_org_id'. {} is less than 12",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.source_account {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 12 as _ {
                    return Err(format!(
                        "Max validation failed on field 'source_account'. {} is greater than 12",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
