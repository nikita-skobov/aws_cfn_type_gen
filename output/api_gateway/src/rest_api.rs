/// The AWS::ApiGateway::RestApi resource creates a REST API. For more information, see restapi:create in the Amazon API Gateway REST API Reference.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnRestApi {
    ///
    /// The source of the API key for metering requests according to a usage plan. Valid values    are: HEADER to read the API key from the X-API-Key header of a    request. AUTHORIZER to read the API key from the UsageIdentifierKey    from a custom authorizer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiKeySourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_source_type: Option<cfn_resources::StrVal>,

    ///
    /// The list of binary media types supported by the RestApi. By default, the RestApi supports only UTF-8-encoded text payloads.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BinaryMediaTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_media_types: Option<Vec<String>>,

    ///
    /// An OpenAPI specification that defines a set of RESTful APIs in JSON format. For YAML templates, you can also provide the specification in YAML format.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,

    ///
    /// The Amazon Simple Storage Service (Amazon S3) location that points to an OpenAPI file, which defines a set of RESTful APIs in JSON or YAML format.
    ///
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "BodyS3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_s3_location: Option<S3Location>,

    ///
    /// The ID of the RestApi that you want to clone from.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloneFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_from: Option<cfn_resources::StrVal>,

    ///
    /// The description of the RestApi.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether clients can invoke your API by using the default execute-api endpoint.    By default, clients can invoke your API with the default    https://{api_id}.execute-api.{region}.amazonaws.com endpoint. To require that clients use a    custom domain name to invoke your API, disable the default endpoint
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableExecuteApiEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_execute_api_endpoint: Option<bool>,

    ///
    /// A list of the endpoint types of the API. Use this property when creating an API. When importing an existing API, specify the endpoint configuration types using the Parameters property.
    ///
    /// Required: No
    ///
    /// Type: EndpointConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfiguration>,

    ///
    /// A query parameter to indicate whether to rollback the API update (true) or not (false)       when a warning is encountered. The default value is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailOnWarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,

    ///
    /// A nullable integer that is used to enable compression (with non-negative between 0 and 10485760 (10M) bytes, inclusive) or disable compression (with a null value) on an API. When compression is enabled, compression or decompression is not applied on the payload if the payload size is smaller than this value. Setting it to zero allows compression for any payload size.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinimumCompressionSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_compression_size: Option<i64>,

    ///
    /// This property applies only when you use OpenAPI to define your REST API. The Mode determines how API Gateway handles resource updates.
    ///
    /// Valid values are overwrite or merge.
    ///
    /// For overwrite, the new API definition replaces the existing one. The existing       API identifier remains unchanged.
    ///
    /// For merge, the new API definition is merged with the existing API.
    ///
    /// If you don't specify this property, a default value is chosen. For REST APIs created before       March 29, 2021, the default is overwrite. For REST APIs created after March 29, 2021, the new API definition takes precedence, but any container types such as endpoint configurations and binary media types are merged with the existing API.
    ///
    /// Use the default mode to define top-level RestApi properties in addition to using OpenAPI. Generally, it's preferred to use API Gateway's OpenAPI extensions to model these properties.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<cfn_resources::StrVal>,

    ///
    /// The name of the RestApi. A name is required if the REST API is not based on an OpenAPI specification.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Custom header parameters as part of the request. For example, to exclude DocumentationParts from an imported API, set ignore=documentation as a parameters value, as in the AWS CLI command of aws apigateway import-rest-api --parameters ignore=documentation --body 'file:///path/to/imported-api-body.json'.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,

    ///
    /// A policy document that contains the permissions for the RestApi resource. To set the ARN for the policy, use the !Join intrinsic function with "" as delimiter and values of "execute-api:/" and "*".
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<serde_json::Value>,

    ///
    /// The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with aws:. The tag value can be up to 256 characters.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_rest_api_id: CfnRestApirestapiid,

    #[serde(skip_serializing)]
    pub att_root_resource_id: CfnRestApirootresourceid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnRestApirestapiid;
impl CfnRestApirestapiid {
    pub fn att_name(&self) -> &'static str {
        r#"RestApiId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnRestApirootresourceid;
impl CfnRestApirootresourceid {
    pub fn att_name(&self) -> &'static str {
        r#"RootResourceId"#
    }
}

impl cfn_resources::CfnResource for CfnRestApi {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGateway::RestApi"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.body_s3_location
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.endpoint_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The EndpointConfiguration property type specifies the endpoint types of a REST API.
///
/// EndpointConfiguration is a property of the AWS::ApiGateway::RestApi resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EndpointConfiguration {
    ///
    /// A list of endpoint types of an API (RestApi) or its custom domain name (DomainName). For an edge-optimized API and its custom domain name, the endpoint type is "EDGE". For a regional API and its custom domain name, the endpoint type is REGIONAL. For a private API, the endpoint type is PRIVATE.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,

    ///
    /// A list of VpcEndpointIds of an API (RestApi) against which to create Route53 ALIASes. It is only supported for PRIVATE endpoint type.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcEndpointIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_ids: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for EndpointConfiguration {
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

/// S3Location is a property of the AWS::ApiGateway::RestApi resource that specifies the Amazon S3 location of a OpenAPI (formerly Swagger) file that defines a set of RESTful APIs in JSON or YAML.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct S3Location {
    ///
    /// The name of the S3 bucket where the OpenAPI file is stored.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon S3 ETag (a file checksum) of the OpenAPI file. If you don't specify a value, API Gateway skips ETag validation of your OpenAPI file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ETag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<cfn_resources::StrVal>,

    ///
    /// The file name of the OpenAPI file (Amazon S3 object name).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<cfn_resources::StrVal>,

    ///
    /// For versioning-enabled buckets, a specific version of the OpenAPI file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for S3Location {
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
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
