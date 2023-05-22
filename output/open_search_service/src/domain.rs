/// The AWS::OpenSearchService::Domain resource creates an Amazon OpenSearch Service    domain.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomain {
    ///
    /// An AWS Identity and Access Management (IAM) policy document that specifies who can    access the OpenSearch Service domain and their permissions. For more information, see Configuring access policies in the Amazon OpenSearch Service Developer     Guide.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<serde_json::Value>,

    ///
    /// Additional options to specify for the OpenSearch Service domain. For more information, see     AdvancedOptions in the OpenSearch Service API reference.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdvancedOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_options: Option<std::collections::HashMap<String, String>>,

    /// Specifies options for fine-grained access control and SAML authentication.
    ///
    /// If you specify advanced security options, you must also enable node-to-node encryption (NodeToNodeEncryptionOptions)    and encryption at rest (EncryptionAtRestOptions).    You must also enable EnforceHTTPS within DomainEndpointOptions,    which requires HTTPS for all traffic to the domain.
    ///
    /// Required: No
    ///
    /// Type: AdvancedSecurityOptionsInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdvancedSecurityOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_security_options: Option<AdvancedSecurityOptionsInput>,

    ///
    /// Container for the cluster configuration of a domain.
    ///
    /// Required: No
    ///
    /// Type: ClusterConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_config: Option<ClusterConfig>,

    ///
    /// Configures OpenSearch Service to use Amazon Cognito authentication for OpenSearch    Dashboards.
    ///
    /// Required: No
    ///
    /// Type: CognitoOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CognitoOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_options: Option<CognitoOptions>,

    /// Specifies additional options for the domain endpoint, such as whether to require HTTPS for all traffic or whether to use a custom endpoint rather than the default endpoint.
    ///
    /// Required: No
    ///
    /// Type: DomainEndpointOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainEndpointOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoint_options: Option<DomainEndpointOptions>,

    ///
    /// A name for the OpenSearch Service domain. The name must have a minimum length of 3 and a maximum length of 28. If you don't specify a name, AWS CloudFormation generates a unique    physical ID and uses that ID for the domain name. For more information, see Name     Type.
    ///
    /// Required when creating a new domain.
    ///
    /// ImportantIf you specify a name, you can't perform updates that require replacement of this     resource. You can perform updates that require no or some interruption. If you must replace     the resource, specify a new name.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<cfn_resources::StrVal>,

    ///
    /// The configurations of Amazon Elastic Block Store (Amazon EBS) volumes that are attached to    data nodes in the OpenSearch Service domain. For more information, see EBS volume size limits in the Amazon OpenSearch Service Developer     Guide.
    ///
    /// Required: No
    ///
    /// Type: EBSOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "EBSOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebsoptions: Option<EBSOptions>,

    ///
    /// Whether the domain should encrypt data at rest, and if so, the AWS KMS key to    use. See Encryption of data at rest for Amazon OpenSearch Service.
    ///
    /// Required: No
    ///
    /// Type: EncryptionAtRestOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionAtRestOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_options: Option<EncryptionAtRestOptions>,

    ///
    /// The version of OpenSearch to use. The value must be in the format    OpenSearch_X.Y or Elasticsearch_X.Y. If not specified, the latest version of OpenSearch is used. For    information about the versions that OpenSearch Service supports, see Supported     versions of OpenSearch and Elasticsearch in the Amazon OpenSearch Service     Developer Guide.
    ///
    /// If you set the EnableVersionUpgrade update policy to true, you can update     EngineVersion without interruption. When EnableVersionUpgrade is    set to false, or is not specified, updating EngineVersion results in     replacement.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 14
    ///
    /// Maximum: 18
    ///
    /// Pattern: ^Elasticsearch_[0-9]{1}\.[0-9]{1,2}$|^OpenSearch_[0-9]{1,2}\.[0-9]{1,2}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<cfn_resources::StrVal>,

    ///
    /// An object with one or more of the following keys: SEARCH_SLOW_LOGS,     ES_APPLICATION_LOGS, INDEX_SLOW_LOGS, AUDIT_LOGS,    depending on the types of logs you want to publish. Each key needs a valid     LogPublishingOption value. For the full syntax, see the examples.
    ///
    /// Required: No
    ///
    /// Type: Map of LogPublishingOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogPublishingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_options: Option<std::collections::HashMap<String, LogPublishingOption>>,

    ///
    /// Specifies whether node-to-node encryption is enabled. See Node-to-node encryption for Amazon OpenSearch Service.
    ///
    /// Required: No
    ///
    /// Type: NodeToNodeEncryptionOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_to_node_encryption_options: Option<NodeToNodeEncryptionOptions>,

    ///
    /// Options for a domain's off-peak window, during which OpenSearch Service can perform mandatory configuration changes on the domain.
    ///
    /// Required: No
    ///
    /// Type: OffPeakWindowOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "OffPeakWindowOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_peak_window_options: Option<OffPeakWindowOptions>,

    ///
    /// DEPRECATED. The automated snapshot configuration for the    OpenSearch Service domain indexes.
    ///
    /// Required: No
    ///
    /// Type: SnapshotOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_options: Option<SnapshotOptions>,

    ///
    /// Options for configuring service software updates for a domain.
    ///
    /// Required: No
    ///
    /// Type: SoftwareUpdateOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SoftwareUpdateOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_update_options: Option<SoftwareUpdateOptions>,

    ///
    /// An arbitrary set of tags (key–value pairs) to associate with the OpenSearch Service    domain.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The virtual private cloud (VPC) configuration for the OpenSearch Service domain. For more    information, see Launching your Amazon OpenSearch Service domains within a VPC in the Amazon OpenSearch Service Developer     Guide.
    ///
    /// If you remove this entity altogether, along with its associated properties, it causes a replacement. You might encounter this scenario if you're updating your security configuration from a VPC to a public endpoint.
    ///
    /// Required: No
    ///
    /// Type: VPCOptions
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "VPCOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpcoptions: Option<VPCOptions>,

    #[serde(skip_serializing)]
    pub att_advanced_security_options_anonymous_auth_disable_date:
        CfnDomainadvancedsecurityoptionsanonymousauthdisabledate,

    #[serde(skip_serializing)]
    pub att_arn: CfnDomainarn,

    #[serde(skip_serializing)]
    pub att_domain_arn: CfnDomaindomainarn,

    #[serde(skip_serializing)]
    pub att_domain_endpoint: CfnDomaindomainendpoint,

    #[serde(skip_serializing)]
    pub att_id: CfnDomainid,

    #[serde(skip_serializing)]
    pub att_service_software_options_automated_update_date:
        CfnDomainservicesoftwareoptionsautomatedupdatedate,

    #[serde(skip_serializing)]
    pub att_service_software_options_current_version: CfnDomainservicesoftwareoptionscurrentversion,

    #[serde(skip_serializing)]
    pub att_service_software_options_description: CfnDomainservicesoftwareoptionsdescription,

    #[serde(skip_serializing)]
    pub att_service_software_options_new_version: CfnDomainservicesoftwareoptionsnewversion,

    #[serde(skip_serializing)]
    pub att_service_software_options_update_status: CfnDomainservicesoftwareoptionsupdatestatus,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomainadvancedsecurityoptionsanonymousauthdisabledate;
impl CfnDomainadvancedsecurityoptionsanonymousauthdisabledate {
    pub fn att_name(&self) -> &'static str {
        r#"AdvancedSecurityOptions.AnonymousAuthDisableDate"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomainarn;
impl CfnDomainarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomaindomainarn;
impl CfnDomaindomainarn {
    pub fn att_name(&self) -> &'static str {
        r#"DomainArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomaindomainendpoint;
impl CfnDomaindomainendpoint {
    pub fn att_name(&self) -> &'static str {
        r#"DomainEndpoint"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomainid;
impl CfnDomainid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomainservicesoftwareoptionsautomatedupdatedate;
impl CfnDomainservicesoftwareoptionsautomatedupdatedate {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceSoftwareOptions.AutomatedUpdateDate"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomainservicesoftwareoptionscurrentversion;
impl CfnDomainservicesoftwareoptionscurrentversion {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceSoftwareOptions.CurrentVersion"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomainservicesoftwareoptionsdescription;
impl CfnDomainservicesoftwareoptionsdescription {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceSoftwareOptions.Description"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomainservicesoftwareoptionsnewversion;
impl CfnDomainservicesoftwareoptionsnewversion {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceSoftwareOptions.NewVersion"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomainservicesoftwareoptionsupdatestatus;
impl CfnDomainservicesoftwareoptionsupdatestatus {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceSoftwareOptions.UpdateStatus"#
    }
}

impl cfn_resources::CfnResource for CfnDomain {
    fn type_string(&self) -> &'static str {
        "AWS::OpenSearchService::Domain"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.advanced_security_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.cluster_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.cognito_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.domain_endpoint_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ebsoptions
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.encryption_at_rest_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.engine_version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 18 as _ {
                    return Err(format!(
                        "Max validation failed on field 'engine_version'. {} is greater than 18",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.engine_version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 14 as _ {
                    return Err(format!(
                        "Min validation failed on field 'engine_version'. {} is less than 14",
                        s.len()
                    ));
                }
            }
        }

        self.node_to_node_encryption_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.off_peak_window_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.snapshot_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.software_update_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vpcoptions
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies options for fine-grained access control.
///
/// If you specify advanced security options,    you must also enable node-to-node encryption (NodeToNodeEncryptionOptions) and encryption at rest (EncryptionAtRestOptions). You must also enable EnforceHTTPS within     DomainEndpointOptions, which requires HTTPS for all traffic to the domain.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AdvancedSecurityOptionsInput {
    ///
    /// Date and time when the migration period will be disabled. Only necessary when enabling    fine-grained access control on an existing domain.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnonymousAuthDisableDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_auth_disable_date: Option<cfn_resources::StrVal>,

    ///
    /// True to enable a 30-day migration period during which administrators can create role  mappings. Only necessary when enabling   fine-grained access control on an existing domain.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnonymousAuthEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_auth_enabled: Option<bool>,

    /// True to enable fine-grained access control. You must also enable encryption of data at rest    and node-to-node encryption. See Fine-grained access control in     Amazon OpenSearch Service.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// True to enable the internal user database.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "InternalUserDatabaseEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_user_database_enabled: Option<bool>,

    /// Specifies information about the master user.
    ///
    /// Required: No
    ///
    /// Type: MasterUserOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_options: Option<MasterUserOptions>,

    ///
    /// Container for information about the SAML configuration for OpenSearch Dashboards.
    ///
    /// Required: No
    ///
    /// Type: SAMLOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SAMLOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samloptions: Option<SAMLOptions>,
}

impl cfn_resources::CfnResource for AdvancedSecurityOptionsInput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.master_user_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.samloptions
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The cluster configuration for the OpenSearch Service domain. You can specify options such    as the instance type and the number of instances. For more information, see Creating and managing Amazon OpenSearch Service domains in the Amazon OpenSearch Service     Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClusterConfig {
    ///
    /// The number of instances to use for the master node. If you specify this property, you must    specify true for the DedicatedMasterEnabled property.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DedicatedMasterCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master_count: Option<i64>,

    ///
    /// Indicates whether to use a dedicated master node for the OpenSearch Service domain. A    dedicated master node is a cluster node that performs cluster management tasks, but doesn't    hold data or respond to data upload requests. Dedicated master nodes offload cluster    management tasks to increase the stability of your search clusters. See Dedicated master nodes in Amazon OpenSearch Service.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DedicatedMasterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master_enabled: Option<bool>,

    ///
    /// The hardware configuration of the computer that hosts the dedicated master node, such as     m3.medium.search. If you specify this property, you must specify     true for the DedicatedMasterEnabled property. For valid values,    see Supported     instance types in Amazon OpenSearch Service.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: c4.2xlarge.search | c4.4xlarge.search | c4.8xlarge.search | c4.large.search | c4.xlarge.search | c5.18xlarge.search | c5.2xlarge.search | c5.4xlarge.search | c5.9xlarge.search | c5.large.search | c5.xlarge.search | c6g.12xlarge.search | c6g.2xlarge.search | c6g.4xlarge.search | c6g.8xlarge.search | c6g.large.search | c6g.xlarge.search | d2.2xlarge.search | d2.4xlarge.search | d2.8xlarge.search | d2.xlarge.search | i2.2xlarge.search | i2.xlarge.search | i3.16xlarge.search | i3.2xlarge.search | i3.4xlarge.search | i3.8xlarge.search | i3.large.search | i3.xlarge.search | m3.2xlarge.search | m3.large.search | m3.medium.search | m3.xlarge.search | m4.10xlarge.search | m4.2xlarge.search | m4.4xlarge.search | m4.large.search | m4.xlarge.search | m5.12xlarge.search | m5.24xlarge.search | m5.2xlarge.search | m5.4xlarge.search | m5.large.search | m5.xlarge.search | m6g.12xlarge.search | m6g.2xlarge.search | m6g.4xlarge.search | m6g.8xlarge.search | m6g.large.search | m6g.xlarge.search | r3.2xlarge.search | r3.4xlarge.search | r3.8xlarge.search | r3.large.search | r3.xlarge.search | r4.16xlarge.search | r4.2xlarge.search | r4.4xlarge.search | r4.8xlarge.search | r4.large.search | r4.xlarge.search | r5.12xlarge.search | r5.24xlarge.search | r5.2xlarge.search | r5.4xlarge.search | r5.large.search | r5.xlarge.search | r6g.12xlarge.search | r6g.2xlarge.search | r6g.4xlarge.search | r6g.8xlarge.search | r6g.large.search | r6g.xlarge.search | r6gd.12xlarge.search | r6gd.16xlarge.search | r6gd.2xlarge.search | r6gd.4xlarge.search | r6gd.8xlarge.search | r6gd.large.search | r6gd.xlarge.search | t2.medium.search | t2.micro.search | t2.small.search | t3.2xlarge.search | t3.large.search | t3.medium.search | t3.micro.search | t3.nano.search | t3.small.search | t3.xlarge.search | t4g.medium.search | t4g.small.search | ultrawarm1.large.search | ultrawarm1.medium.search | ultrawarm1.xlarge.search
    ///
    /// Update requires: No interruption
    #[serde(rename = "DedicatedMasterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master_type: Option<ClusterConfigDedicatedMasterTypeEnum>,

    ///
    /// The number of data nodes (instances) to use in the OpenSearch Service domain.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i64>,

    ///
    /// The instance type for your data nodes, such as m3.medium.search. For valid    values, see Supported     instance types in Amazon OpenSearch Service .
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: c4.2xlarge.search | c4.4xlarge.search | c4.8xlarge.search | c4.large.search | c4.xlarge.search | c5.18xlarge.search | c5.2xlarge.search | c5.4xlarge.search | c5.9xlarge.search | c5.large.search | c5.xlarge.search | c6g.12xlarge.search | c6g.2xlarge.search | c6g.4xlarge.search | c6g.8xlarge.search | c6g.large.search | c6g.xlarge.search | d2.2xlarge.search | d2.4xlarge.search | d2.8xlarge.search | d2.xlarge.search | i2.2xlarge.search | i2.xlarge.search | i3.16xlarge.search | i3.2xlarge.search | i3.4xlarge.search | i3.8xlarge.search | i3.large.search | i3.xlarge.search | m3.2xlarge.search | m3.large.search | m3.medium.search | m3.xlarge.search | m4.10xlarge.search | m4.2xlarge.search | m4.4xlarge.search | m4.large.search | m4.xlarge.search | m5.12xlarge.search | m5.24xlarge.search | m5.2xlarge.search | m5.4xlarge.search | m5.large.search | m5.xlarge.search | m6g.12xlarge.search | m6g.2xlarge.search | m6g.4xlarge.search | m6g.8xlarge.search | m6g.large.search | m6g.xlarge.search | r3.2xlarge.search | r3.4xlarge.search | r3.8xlarge.search | r3.large.search | r3.xlarge.search | r4.16xlarge.search | r4.2xlarge.search | r4.4xlarge.search | r4.8xlarge.search | r4.large.search | r4.xlarge.search | r5.12xlarge.search | r5.24xlarge.search | r5.2xlarge.search | r5.4xlarge.search | r5.large.search | r5.xlarge.search | r6g.12xlarge.search | r6g.2xlarge.search | r6g.4xlarge.search | r6g.8xlarge.search | r6g.large.search | r6g.xlarge.search | r6gd.12xlarge.search | r6gd.16xlarge.search | r6gd.2xlarge.search | r6gd.4xlarge.search | r6gd.8xlarge.search | r6gd.large.search | r6gd.xlarge.search | t2.medium.search | t2.micro.search | t2.small.search | t3.2xlarge.search | t3.large.search | t3.medium.search | t3.micro.search | t3.nano.search | t3.small.search | t3.xlarge.search | t4g.medium.search | t4g.small.search | ultrawarm1.large.search | ultrawarm1.medium.search | ultrawarm1.xlarge.search
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<ClusterConfigInstanceTypeEnum>,

    ///
    /// The number of warm nodes in the cluster.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "WarmCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_count: Option<i64>,

    ///
    /// Whether to enable UltraWarm storage for the cluster. See UltraWarm storage for Amazon OpenSearch Service.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "WarmEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_enabled: Option<bool>,

    ///
    /// The instance type for the cluster's warm nodes.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ultrawarm1.large.search | ultrawarm1.medium.search | ultrawarm1.xlarge.search
    ///
    /// Update requires: No interruption
    #[serde(rename = "WarmType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_type: Option<ClusterConfigWarmTypeEnum>,

    ///
    /// Specifies zone awareness configuration options. Only use if     ZoneAwarenessEnabled is true.
    ///
    /// Required: No
    ///
    /// Type: ZoneAwarenessConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ZoneAwarenessConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_awareness_config: Option<ZoneAwarenessConfig>,

    ///
    /// Indicates whether to enable zone awareness for the OpenSearch Service domain. When you    enable zone awareness, OpenSearch Service allocates the nodes and replica index shards that    belong to a cluster across two Availability Zones (AZs) in the same region to prevent data    loss and minimize downtime in the event of node or data center failure. Don't enable zone    awareness if your cluster has no replica index shards or is a single-node cluster. For more    information, see Configuring a multi-AZ domain in Amazon OpenSearch Service.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ZoneAwarenessEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_awareness_enabled: Option<bool>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ClusterConfigDedicatedMasterTypeEnum {
    /// c4.2xlarge.search
    #[serde(rename = "c4.2xlarge.search")]
    C42xlargesearch,

    /// c4.4xlarge.search
    #[serde(rename = "c4.4xlarge.search")]
    C44xlargesearch,

    /// c4.8xlarge.search
    #[serde(rename = "c4.8xlarge.search")]
    C48xlargesearch,

    /// c4.large.search
    #[serde(rename = "c4.large.search")]
    C4largesearch,

    /// c4.xlarge.search
    #[serde(rename = "c4.xlarge.search")]
    C4xlargesearch,

    /// c5.18xlarge.search
    #[serde(rename = "c5.18xlarge.search")]
    C518xlargesearch,

    /// c5.2xlarge.search
    #[serde(rename = "c5.2xlarge.search")]
    C52xlargesearch,

    /// c5.4xlarge.search
    #[serde(rename = "c5.4xlarge.search")]
    C54xlargesearch,

    /// c5.9xlarge.search
    #[serde(rename = "c5.9xlarge.search")]
    C59xlargesearch,

    /// c5.large.search
    #[serde(rename = "c5.large.search")]
    C5largesearch,

    /// c5.xlarge.search
    #[serde(rename = "c5.xlarge.search")]
    C5xlargesearch,

    /// c6g.12xlarge.search
    #[serde(rename = "c6g.12xlarge.search")]
    C6g12xlargesearch,

    /// c6g.2xlarge.search
    #[serde(rename = "c6g.2xlarge.search")]
    C6g2xlargesearch,

    /// c6g.4xlarge.search
    #[serde(rename = "c6g.4xlarge.search")]
    C6g4xlargesearch,

    /// c6g.8xlarge.search
    #[serde(rename = "c6g.8xlarge.search")]
    C6g8xlargesearch,

    /// c6g.large.search
    #[serde(rename = "c6g.large.search")]
    C6glargesearch,

    /// c6g.xlarge.search
    #[serde(rename = "c6g.xlarge.search")]
    C6gxlargesearch,

    /// d2.2xlarge.search
    #[serde(rename = "d2.2xlarge.search")]
    D22xlargesearch,

    /// d2.4xlarge.search
    #[serde(rename = "d2.4xlarge.search")]
    D24xlargesearch,

    /// d2.8xlarge.search
    #[serde(rename = "d2.8xlarge.search")]
    D28xlargesearch,

    /// d2.xlarge.search
    #[serde(rename = "d2.xlarge.search")]
    D2xlargesearch,

    /// i2.2xlarge.search
    #[serde(rename = "i2.2xlarge.search")]
    I22xlargesearch,

    /// i2.xlarge.search
    #[serde(rename = "i2.xlarge.search")]
    I2xlargesearch,

    /// i3.16xlarge.search
    #[serde(rename = "i3.16xlarge.search")]
    I316xlargesearch,

    /// i3.2xlarge.search
    #[serde(rename = "i3.2xlarge.search")]
    I32xlargesearch,

    /// i3.4xlarge.search
    #[serde(rename = "i3.4xlarge.search")]
    I34xlargesearch,

    /// i3.8xlarge.search
    #[serde(rename = "i3.8xlarge.search")]
    I38xlargesearch,

    /// i3.large.search
    #[serde(rename = "i3.large.search")]
    I3largesearch,

    /// i3.xlarge.search
    #[serde(rename = "i3.xlarge.search")]
    I3xlargesearch,

    /// m3.2xlarge.search
    #[serde(rename = "m3.2xlarge.search")]
    M32xlargesearch,

    /// m3.large.search
    #[serde(rename = "m3.large.search")]
    M3largesearch,

    /// m3.medium.search
    #[serde(rename = "m3.medium.search")]
    M3mediumsearch,

    /// m3.xlarge.search
    #[serde(rename = "m3.xlarge.search")]
    M3xlargesearch,

    /// m4.10xlarge.search
    #[serde(rename = "m4.10xlarge.search")]
    M410xlargesearch,

    /// m4.2xlarge.search
    #[serde(rename = "m4.2xlarge.search")]
    M42xlargesearch,

    /// m4.4xlarge.search
    #[serde(rename = "m4.4xlarge.search")]
    M44xlargesearch,

    /// m4.large.search
    #[serde(rename = "m4.large.search")]
    M4largesearch,

    /// m4.xlarge.search
    #[serde(rename = "m4.xlarge.search")]
    M4xlargesearch,

    /// m5.12xlarge.search
    #[serde(rename = "m5.12xlarge.search")]
    M512xlargesearch,

    /// m5.24xlarge.search
    #[serde(rename = "m5.24xlarge.search")]
    M524xlargesearch,

    /// m5.2xlarge.search
    #[serde(rename = "m5.2xlarge.search")]
    M52xlargesearch,

    /// m5.4xlarge.search
    #[serde(rename = "m5.4xlarge.search")]
    M54xlargesearch,

    /// m5.large.search
    #[serde(rename = "m5.large.search")]
    M5largesearch,

    /// m5.xlarge.search
    #[serde(rename = "m5.xlarge.search")]
    M5xlargesearch,

    /// m6g.12xlarge.search
    #[serde(rename = "m6g.12xlarge.search")]
    M6g12xlargesearch,

    /// m6g.2xlarge.search
    #[serde(rename = "m6g.2xlarge.search")]
    M6g2xlargesearch,

    /// m6g.4xlarge.search
    #[serde(rename = "m6g.4xlarge.search")]
    M6g4xlargesearch,

    /// m6g.8xlarge.search
    #[serde(rename = "m6g.8xlarge.search")]
    M6g8xlargesearch,

    /// m6g.large.search
    #[serde(rename = "m6g.large.search")]
    M6glargesearch,

    /// m6g.xlarge.search
    #[serde(rename = "m6g.xlarge.search")]
    M6gxlargesearch,

    /// r3.2xlarge.search
    #[serde(rename = "r3.2xlarge.search")]
    R32xlargesearch,

    /// r3.4xlarge.search
    #[serde(rename = "r3.4xlarge.search")]
    R34xlargesearch,

    /// r3.8xlarge.search
    #[serde(rename = "r3.8xlarge.search")]
    R38xlargesearch,

    /// r3.large.search
    #[serde(rename = "r3.large.search")]
    R3largesearch,

    /// r3.xlarge.search
    #[serde(rename = "r3.xlarge.search")]
    R3xlargesearch,

    /// r4.16xlarge.search
    #[serde(rename = "r4.16xlarge.search")]
    R416xlargesearch,

    /// r4.2xlarge.search
    #[serde(rename = "r4.2xlarge.search")]
    R42xlargesearch,

    /// r4.4xlarge.search
    #[serde(rename = "r4.4xlarge.search")]
    R44xlargesearch,

    /// r4.8xlarge.search
    #[serde(rename = "r4.8xlarge.search")]
    R48xlargesearch,

    /// r4.large.search
    #[serde(rename = "r4.large.search")]
    R4largesearch,

    /// r4.xlarge.search
    #[serde(rename = "r4.xlarge.search")]
    R4xlargesearch,

    /// r5.12xlarge.search
    #[serde(rename = "r5.12xlarge.search")]
    R512xlargesearch,

    /// r5.24xlarge.search
    #[serde(rename = "r5.24xlarge.search")]
    R524xlargesearch,

    /// r5.2xlarge.search
    #[serde(rename = "r5.2xlarge.search")]
    R52xlargesearch,

    /// r5.4xlarge.search
    #[serde(rename = "r5.4xlarge.search")]
    R54xlargesearch,

    /// r5.large.search
    #[serde(rename = "r5.large.search")]
    R5largesearch,

    /// r5.xlarge.search
    #[serde(rename = "r5.xlarge.search")]
    R5xlargesearch,

    /// r6g.12xlarge.search
    #[serde(rename = "r6g.12xlarge.search")]
    R6g12xlargesearch,

    /// r6g.2xlarge.search
    #[serde(rename = "r6g.2xlarge.search")]
    R6g2xlargesearch,

    /// r6g.4xlarge.search
    #[serde(rename = "r6g.4xlarge.search")]
    R6g4xlargesearch,

    /// r6g.8xlarge.search
    #[serde(rename = "r6g.8xlarge.search")]
    R6g8xlargesearch,

    /// r6g.large.search
    #[serde(rename = "r6g.large.search")]
    R6glargesearch,

    /// r6g.xlarge.search
    #[serde(rename = "r6g.xlarge.search")]
    R6gxlargesearch,

    /// r6gd.12xlarge.search
    #[serde(rename = "r6gd.12xlarge.search")]
    R6gd12xlargesearch,

    /// r6gd.16xlarge.search
    #[serde(rename = "r6gd.16xlarge.search")]
    R6gd16xlargesearch,

    /// r6gd.2xlarge.search
    #[serde(rename = "r6gd.2xlarge.search")]
    R6gd2xlargesearch,

    /// r6gd.4xlarge.search
    #[serde(rename = "r6gd.4xlarge.search")]
    R6gd4xlargesearch,

    /// r6gd.8xlarge.search
    #[serde(rename = "r6gd.8xlarge.search")]
    R6gd8xlargesearch,

    /// r6gd.large.search
    #[serde(rename = "r6gd.large.search")]
    R6gdlargesearch,

    /// r6gd.xlarge.search
    #[serde(rename = "r6gd.xlarge.search")]
    R6gdxlargesearch,

    /// t2.medium.search
    #[serde(rename = "t2.medium.search")]
    T2mediumsearch,

    /// t2.micro.search
    #[serde(rename = "t2.micro.search")]
    T2microsearch,

    /// t2.small.search
    #[serde(rename = "t2.small.search")]
    T2smallsearch,

    /// t3.2xlarge.search
    #[serde(rename = "t3.2xlarge.search")]
    T32xlargesearch,

    /// t3.large.search
    #[serde(rename = "t3.large.search")]
    T3largesearch,

    /// t3.medium.search
    #[serde(rename = "t3.medium.search")]
    T3mediumsearch,

    /// t3.micro.search
    #[serde(rename = "t3.micro.search")]
    T3microsearch,

    /// t3.nano.search
    #[serde(rename = "t3.nano.search")]
    T3nanosearch,

    /// t3.small.search
    #[serde(rename = "t3.small.search")]
    T3smallsearch,

    /// t3.xlarge.search
    #[serde(rename = "t3.xlarge.search")]
    T3xlargesearch,

    /// t4g.medium.search
    #[serde(rename = "t4g.medium.search")]
    T4gmediumsearch,

    /// t4g.small.search
    #[serde(rename = "t4g.small.search")]
    T4gsmallsearch,

    /// ultrawarm1.large.search
    #[serde(rename = "ultrawarm1.large.search")]
    Ultrawarm1largesearch,

    /// ultrawarm1.medium.search
    #[serde(rename = "ultrawarm1.medium.search")]
    Ultrawarm1mediumsearch,

    /// ultrawarm1.xlarge.search
    #[serde(rename = "ultrawarm1.xlarge.search")]
    Ultrawarm1xlargesearch,
}

impl Default for ClusterConfigDedicatedMasterTypeEnum {
    fn default() -> Self {
        ClusterConfigDedicatedMasterTypeEnum::C42xlargesearch
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ClusterConfigInstanceTypeEnum {
    /// c4.2xlarge.search
    #[serde(rename = "c4.2xlarge.search")]
    C42xlargesearch,

    /// c4.4xlarge.search
    #[serde(rename = "c4.4xlarge.search")]
    C44xlargesearch,

    /// c4.8xlarge.search
    #[serde(rename = "c4.8xlarge.search")]
    C48xlargesearch,

    /// c4.large.search
    #[serde(rename = "c4.large.search")]
    C4largesearch,

    /// c4.xlarge.search
    #[serde(rename = "c4.xlarge.search")]
    C4xlargesearch,

    /// c5.18xlarge.search
    #[serde(rename = "c5.18xlarge.search")]
    C518xlargesearch,

    /// c5.2xlarge.search
    #[serde(rename = "c5.2xlarge.search")]
    C52xlargesearch,

    /// c5.4xlarge.search
    #[serde(rename = "c5.4xlarge.search")]
    C54xlargesearch,

    /// c5.9xlarge.search
    #[serde(rename = "c5.9xlarge.search")]
    C59xlargesearch,

    /// c5.large.search
    #[serde(rename = "c5.large.search")]
    C5largesearch,

    /// c5.xlarge.search
    #[serde(rename = "c5.xlarge.search")]
    C5xlargesearch,

    /// c6g.12xlarge.search
    #[serde(rename = "c6g.12xlarge.search")]
    C6g12xlargesearch,

    /// c6g.2xlarge.search
    #[serde(rename = "c6g.2xlarge.search")]
    C6g2xlargesearch,

    /// c6g.4xlarge.search
    #[serde(rename = "c6g.4xlarge.search")]
    C6g4xlargesearch,

    /// c6g.8xlarge.search
    #[serde(rename = "c6g.8xlarge.search")]
    C6g8xlargesearch,

    /// c6g.large.search
    #[serde(rename = "c6g.large.search")]
    C6glargesearch,

    /// c6g.xlarge.search
    #[serde(rename = "c6g.xlarge.search")]
    C6gxlargesearch,

    /// d2.2xlarge.search
    #[serde(rename = "d2.2xlarge.search")]
    D22xlargesearch,

    /// d2.4xlarge.search
    #[serde(rename = "d2.4xlarge.search")]
    D24xlargesearch,

    /// d2.8xlarge.search
    #[serde(rename = "d2.8xlarge.search")]
    D28xlargesearch,

    /// d2.xlarge.search
    #[serde(rename = "d2.xlarge.search")]
    D2xlargesearch,

    /// i2.2xlarge.search
    #[serde(rename = "i2.2xlarge.search")]
    I22xlargesearch,

    /// i2.xlarge.search
    #[serde(rename = "i2.xlarge.search")]
    I2xlargesearch,

    /// i3.16xlarge.search
    #[serde(rename = "i3.16xlarge.search")]
    I316xlargesearch,

    /// i3.2xlarge.search
    #[serde(rename = "i3.2xlarge.search")]
    I32xlargesearch,

    /// i3.4xlarge.search
    #[serde(rename = "i3.4xlarge.search")]
    I34xlargesearch,

    /// i3.8xlarge.search
    #[serde(rename = "i3.8xlarge.search")]
    I38xlargesearch,

    /// i3.large.search
    #[serde(rename = "i3.large.search")]
    I3largesearch,

    /// i3.xlarge.search
    #[serde(rename = "i3.xlarge.search")]
    I3xlargesearch,

    /// m3.2xlarge.search
    #[serde(rename = "m3.2xlarge.search")]
    M32xlargesearch,

    /// m3.large.search
    #[serde(rename = "m3.large.search")]
    M3largesearch,

    /// m3.medium.search
    #[serde(rename = "m3.medium.search")]
    M3mediumsearch,

    /// m3.xlarge.search
    #[serde(rename = "m3.xlarge.search")]
    M3xlargesearch,

    /// m4.10xlarge.search
    #[serde(rename = "m4.10xlarge.search")]
    M410xlargesearch,

    /// m4.2xlarge.search
    #[serde(rename = "m4.2xlarge.search")]
    M42xlargesearch,

    /// m4.4xlarge.search
    #[serde(rename = "m4.4xlarge.search")]
    M44xlargesearch,

    /// m4.large.search
    #[serde(rename = "m4.large.search")]
    M4largesearch,

    /// m4.xlarge.search
    #[serde(rename = "m4.xlarge.search")]
    M4xlargesearch,

    /// m5.12xlarge.search
    #[serde(rename = "m5.12xlarge.search")]
    M512xlargesearch,

    /// m5.24xlarge.search
    #[serde(rename = "m5.24xlarge.search")]
    M524xlargesearch,

    /// m5.2xlarge.search
    #[serde(rename = "m5.2xlarge.search")]
    M52xlargesearch,

    /// m5.4xlarge.search
    #[serde(rename = "m5.4xlarge.search")]
    M54xlargesearch,

    /// m5.large.search
    #[serde(rename = "m5.large.search")]
    M5largesearch,

    /// m5.xlarge.search
    #[serde(rename = "m5.xlarge.search")]
    M5xlargesearch,

    /// m6g.12xlarge.search
    #[serde(rename = "m6g.12xlarge.search")]
    M6g12xlargesearch,

    /// m6g.2xlarge.search
    #[serde(rename = "m6g.2xlarge.search")]
    M6g2xlargesearch,

    /// m6g.4xlarge.search
    #[serde(rename = "m6g.4xlarge.search")]
    M6g4xlargesearch,

    /// m6g.8xlarge.search
    #[serde(rename = "m6g.8xlarge.search")]
    M6g8xlargesearch,

    /// m6g.large.search
    #[serde(rename = "m6g.large.search")]
    M6glargesearch,

    /// m6g.xlarge.search
    #[serde(rename = "m6g.xlarge.search")]
    M6gxlargesearch,

    /// r3.2xlarge.search
    #[serde(rename = "r3.2xlarge.search")]
    R32xlargesearch,

    /// r3.4xlarge.search
    #[serde(rename = "r3.4xlarge.search")]
    R34xlargesearch,

    /// r3.8xlarge.search
    #[serde(rename = "r3.8xlarge.search")]
    R38xlargesearch,

    /// r3.large.search
    #[serde(rename = "r3.large.search")]
    R3largesearch,

    /// r3.xlarge.search
    #[serde(rename = "r3.xlarge.search")]
    R3xlargesearch,

    /// r4.16xlarge.search
    #[serde(rename = "r4.16xlarge.search")]
    R416xlargesearch,

    /// r4.2xlarge.search
    #[serde(rename = "r4.2xlarge.search")]
    R42xlargesearch,

    /// r4.4xlarge.search
    #[serde(rename = "r4.4xlarge.search")]
    R44xlargesearch,

    /// r4.8xlarge.search
    #[serde(rename = "r4.8xlarge.search")]
    R48xlargesearch,

    /// r4.large.search
    #[serde(rename = "r4.large.search")]
    R4largesearch,

    /// r4.xlarge.search
    #[serde(rename = "r4.xlarge.search")]
    R4xlargesearch,

    /// r5.12xlarge.search
    #[serde(rename = "r5.12xlarge.search")]
    R512xlargesearch,

    /// r5.24xlarge.search
    #[serde(rename = "r5.24xlarge.search")]
    R524xlargesearch,

    /// r5.2xlarge.search
    #[serde(rename = "r5.2xlarge.search")]
    R52xlargesearch,

    /// r5.4xlarge.search
    #[serde(rename = "r5.4xlarge.search")]
    R54xlargesearch,

    /// r5.large.search
    #[serde(rename = "r5.large.search")]
    R5largesearch,

    /// r5.xlarge.search
    #[serde(rename = "r5.xlarge.search")]
    R5xlargesearch,

    /// r6g.12xlarge.search
    #[serde(rename = "r6g.12xlarge.search")]
    R6g12xlargesearch,

    /// r6g.2xlarge.search
    #[serde(rename = "r6g.2xlarge.search")]
    R6g2xlargesearch,

    /// r6g.4xlarge.search
    #[serde(rename = "r6g.4xlarge.search")]
    R6g4xlargesearch,

    /// r6g.8xlarge.search
    #[serde(rename = "r6g.8xlarge.search")]
    R6g8xlargesearch,

    /// r6g.large.search
    #[serde(rename = "r6g.large.search")]
    R6glargesearch,

    /// r6g.xlarge.search
    #[serde(rename = "r6g.xlarge.search")]
    R6gxlargesearch,

    /// r6gd.12xlarge.search
    #[serde(rename = "r6gd.12xlarge.search")]
    R6gd12xlargesearch,

    /// r6gd.16xlarge.search
    #[serde(rename = "r6gd.16xlarge.search")]
    R6gd16xlargesearch,

    /// r6gd.2xlarge.search
    #[serde(rename = "r6gd.2xlarge.search")]
    R6gd2xlargesearch,

    /// r6gd.4xlarge.search
    #[serde(rename = "r6gd.4xlarge.search")]
    R6gd4xlargesearch,

    /// r6gd.8xlarge.search
    #[serde(rename = "r6gd.8xlarge.search")]
    R6gd8xlargesearch,

    /// r6gd.large.search
    #[serde(rename = "r6gd.large.search")]
    R6gdlargesearch,

    /// r6gd.xlarge.search
    #[serde(rename = "r6gd.xlarge.search")]
    R6gdxlargesearch,

    /// t2.medium.search
    #[serde(rename = "t2.medium.search")]
    T2mediumsearch,

    /// t2.micro.search
    #[serde(rename = "t2.micro.search")]
    T2microsearch,

    /// t2.small.search
    #[serde(rename = "t2.small.search")]
    T2smallsearch,

    /// t3.2xlarge.search
    #[serde(rename = "t3.2xlarge.search")]
    T32xlargesearch,

    /// t3.large.search
    #[serde(rename = "t3.large.search")]
    T3largesearch,

    /// t3.medium.search
    #[serde(rename = "t3.medium.search")]
    T3mediumsearch,

    /// t3.micro.search
    #[serde(rename = "t3.micro.search")]
    T3microsearch,

    /// t3.nano.search
    #[serde(rename = "t3.nano.search")]
    T3nanosearch,

    /// t3.small.search
    #[serde(rename = "t3.small.search")]
    T3smallsearch,

    /// t3.xlarge.search
    #[serde(rename = "t3.xlarge.search")]
    T3xlargesearch,

    /// t4g.medium.search
    #[serde(rename = "t4g.medium.search")]
    T4gmediumsearch,

    /// t4g.small.search
    #[serde(rename = "t4g.small.search")]
    T4gsmallsearch,

    /// ultrawarm1.large.search
    #[serde(rename = "ultrawarm1.large.search")]
    Ultrawarm1largesearch,

    /// ultrawarm1.medium.search
    #[serde(rename = "ultrawarm1.medium.search")]
    Ultrawarm1mediumsearch,

    /// ultrawarm1.xlarge.search
    #[serde(rename = "ultrawarm1.xlarge.search")]
    Ultrawarm1xlargesearch,
}

impl Default for ClusterConfigInstanceTypeEnum {
    fn default() -> Self {
        ClusterConfigInstanceTypeEnum::C42xlargesearch
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ClusterConfigWarmTypeEnum {
    /// ultrawarm1.large.search
    #[serde(rename = "ultrawarm1.large.search")]
    Ultrawarm1largesearch,

    /// ultrawarm1.medium.search
    #[serde(rename = "ultrawarm1.medium.search")]
    Ultrawarm1mediumsearch,

    /// ultrawarm1.xlarge.search
    #[serde(rename = "ultrawarm1.xlarge.search")]
    Ultrawarm1xlargesearch,
}

impl Default for ClusterConfigWarmTypeEnum {
    fn default() -> Self {
        ClusterConfigWarmTypeEnum::Ultrawarm1largesearch
    }
}

impl cfn_resources::CfnResource for ClusterConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.zone_awareness_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Configures OpenSearch Service to use Amazon Cognito authentication for OpenSearch    Dashboards.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CognitoOptions {
    ///
    /// Whether to enable or disable Amazon Cognito authentication for OpenSearch Dashboards. See     Amazon Cognito     authentication for OpenSearch Dashboards.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    ///
    /// The Amazon Cognito identity pool ID that you want OpenSearch Service to use for OpenSearch    Dashboards authentication.
    ///
    /// Required if you enabled Cognito Authentication for OpenSearch Dashboards.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 55
    ///
    /// Pattern: [\w-]+:[0-9a-f-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdentityPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<cfn_resources::StrVal>,

    ///
    /// The AmazonOpenSearchServiceCognitoAccess role that allows OpenSearch Service    to configure your user pool and identity pool.
    ///
    /// Required if you enabled Cognito Authentication for OpenSearch Dashboards.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:(aws|aws\-cn|aws\-us\-gov|aws\-iso|aws\-iso\-b):iam::[0-9]+:role\/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Cognito user pool ID that you want OpenSearch Service to use for OpenSearch    Dashboards authentication.
    ///
    /// Required if you enabled Cognito Authentication for OpenSearch Dashboards.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 55
    ///
    /// Pattern: [\w-]+_[0-9a-zA-Z]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CognitoOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.identity_pool_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 55 as _ {
                    return Err(format!(
                        "Max validation failed on field 'identity_pool_id'. {} is greater than 55",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.identity_pool_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'identity_pool_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'role_arn'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'role_arn'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.user_pool_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 55 as _ {
                    return Err(format!(
                        "Max validation failed on field 'user_pool_id'. {} is greater than 55",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.user_pool_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'user_pool_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Specifies additional options for the domain endpoint, such as whether to require HTTPS for all traffic or whether to use a custom endpoint rather than the default endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DomainEndpointOptions {
    /// The fully qualified URL for your custom endpoint. Required if you enabled a custom endpoint    for the domain.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^(((?!-)[A-Za-z0-9-]{0,62}[A-Za-z0-9])\.)+((?!-)[A-Za-z0-9-]{1,62}[A-Za-z0-9])$
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint: Option<cfn_resources::StrVal>,

    /// The AWS Certificate Manager ARN for your domain's SSL/TLS certificate. Required if you    enabled a custom endpoint for the domain.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomEndpointCertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint_certificate_arn: Option<cfn_resources::StrVal>,

    /// True to enable a custom endpoint for the domain. If enabled, you must also provide values for CustomEndpoint and CustomEndpointCertificateArn.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomEndpointEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint_enabled: Option<bool>,

    /// True to require that all traffic to the domain arrive over HTTPS. Required if you enable    fine-grained access control in AdvancedSecurityOptions.
    ///
    /// Required: Conditional
    ///
    /// Type: Boolean
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "EnforceHTTPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_https: Option<bool>,

    /// The minimum TLS version required for traffic to the domain. Valid values are TLS 1.0 (default) or 1.2:
    ///
    /// Policy-Min-TLS-1-0-2019-07Policy-Min-TLS-1-2-2019-07
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Policy-Min-TLS-1-0-2019-07 | Policy-Min-TLS-1-2-2019-07
    ///
    /// Update requires: No interruption
    #[serde(rename = "TLSSecurityPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tlssecurity_policy: Option<DomainEndpointOptionsTLSSecurityPolicyEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DomainEndpointOptionsTLSSecurityPolicyEnum {
    /// Policy-Min-TLS-1-0-2019-07
    #[serde(rename = "Policy-Min-TLS-1-0-2019-07")]
    Policymintls10201907,

    /// Policy-Min-TLS-1-2-2019-07
    #[serde(rename = "Policy-Min-TLS-1-2-2019-07")]
    Policymintls12201907,
}

impl Default for DomainEndpointOptionsTLSSecurityPolicyEnum {
    fn default() -> Self {
        DomainEndpointOptionsTLSSecurityPolicyEnum::Policymintls10201907
    }
}

impl cfn_resources::CfnResource for DomainEndpointOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.custom_endpoint {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'custom_endpoint'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.custom_endpoint {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'custom_endpoint'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The configurations of Amazon Elastic Block Store (Amazon EBS) volumes that are attached to    data nodes in the OpenSearch Service domain. For more information, see EBS volume size limits in the Amazon OpenSearch Service Developer     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EBSOptions {
    ///
    /// Specifies whether Amazon EBS volumes are attached to data nodes in the OpenSearch Service    domain.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EBSEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebsenabled: Option<bool>,

    ///
    /// The number of I/O operations per second (IOPS) that the volume supports. This property    applies only to the gp3 and provisioned IOPS EBS volume types.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,

    ///
    /// The throughput (in MiB/s) of the EBS volumes attached to data nodes. Applies only to the     gp3 volume type.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Throughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i64>,

    ///
    /// The size (in GiB) of the EBS volume for each data node. The minimum and maximum size of an    EBS volume depends on the EBS volume type and the instance type to which it is attached. For    more information, see EBS volume size     limits in the Amazon OpenSearch Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i64>,

    ///
    /// The EBS volume type to use with the OpenSearch Service domain. If you choose     gp3, you must also specify values for Iops and     Throughput. For more information about each type, see Amazon EBS volume     types in the Amazon EC2 User Guide for Linux Instances.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: gp2 | gp3 | io1 | standard
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<EBSOptionsVolumeTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum EBSOptionsVolumeTypeEnum {
    /// gp2
    #[serde(rename = "gp2")]
    Gp2,

    /// gp3
    #[serde(rename = "gp3")]
    Gp3,

    /// io1
    #[serde(rename = "io1")]
    Io1,

    /// standard
    #[serde(rename = "standard")]
    Standard,
}

impl Default for EBSOptionsVolumeTypeEnum {
    fn default() -> Self {
        EBSOptionsVolumeTypeEnum::Gp2
    }
}

impl cfn_resources::CfnResource for EBSOptions {
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

/// Whether the domain should encrypt data at rest, and if so, the AWS Key Management Service key to use.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EncryptionAtRestOptions {
    ///
    /// Specify true to enable encryption at rest. Required if you enable    fine-grained access control in AdvancedSecurityOptionsInput.
    ///
    /// Required: Conditional
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    ///
    /// The KMS key ID. Takes the form 1a2a3a4-1a2a-3a4a-5a6a-1a2a3a4a5a6a. Required    if you enable encryption at rest.
    ///
    /// You can also use keyAlias as a value.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 500
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for EncryptionAtRestOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 500 as _ {
                    return Err(format!(
                        "Max validation failed on field 'kms_key_id'. {} is greater than 500",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'kms_key_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The SAML Identity Provider's information.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Idp {
    ///
    /// The unique entity ID of the application in the SAML identity provider.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 8
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityId")]
    pub entity_id: cfn_resources::StrVal,

    ///
    /// The metadata of the SAML application, in XML format.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1048576
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetadataContent")]
    pub metadata_content: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Idp {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.entity_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'entity_id'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        let the_val = &self.entity_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 8 as _ {
                return Err(format!(
                    "Min validation failed on field 'entity_id'. {} is less than 8",
                    s.len()
                ));
            }
        }

        let the_val = &self.metadata_content;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1048576 as _ {
                return Err(format!(
                    "Max validation failed on field 'metadata_content'. {} is greater than 1048576",
                    s.len()
                ));
            }
        }

        let the_val = &self.metadata_content;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'metadata_content'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Specifies whether the OpenSearch Service domain publishes application, search slow logs,    or index slow logs to Amazon CloudWatch. Each option must be an object of name     SEARCH_SLOW_LOGS, ES_APPLICATION_LOGS,     INDEX_SLOW_LOGS, or AUDIT_LOGS depending on the type of logs you    want to publish. For the full syntax, see the examples.
///
/// Before you enable log publishing, you need to create a CloudWatch log group and provide    OpenSearch Service the correct permissions to write to it. To learn more, see Enabling log publishing (AWS CloudFormation).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LogPublishingOption {
    ///
    /// Specifies the CloudWatch log group to publish to. Required if you enable log    publishing.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<cfn_resources::StrVal>,

    ///
    /// If true, enables the publishing of logs to CloudWatch.
    ///
    /// Default: false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl cfn_resources::CfnResource for LogPublishingOption {
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

/// Specifies information about the master user.
///
/// Required if if InternalUserDatabaseEnabled is true in AdvancedSecurityOptions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MasterUserOptions {
    /// Amazon Resource Name (ARN) for the master user. The ARN can point to an IAM user or role. This    property is required for Amazon Cognito to work, and it must match the role configured for    Cognito. Only specify if InternalUserDatabaseEnabled is false in AdvancedSecurityOptionsInput.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_arn: Option<cfn_resources::StrVal>,

    /// Username for the master user. Only specify if InternalUserDatabaseEnabled is true    in AdvancedSecurityOptionsInput.
    ///
    /// If you don't want to specify this value directly within the template, you can use a dynamic reference instead.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_name: Option<cfn_resources::StrVal>,

    /// Password for the master user. Only specify if InternalUserDatabaseEnabled is true    in AdvancedSecurityOptionsInput.
    ///
    /// If you don't want to specify this value directly within the template, you can use a dynamic reference instead.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 8
    ///
    /// Maximum: 128
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MasterUserOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.master_user_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!(
                        "Max validation failed on field 'master_user_name'. {} is greater than 64",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.master_user_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'master_user_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.master_user_password {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!("Max validation failed on field 'master_user_password'. {} is greater than 128", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.master_user_password {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 8 as _ {
                    return Err(format!(
                        "Min validation failed on field 'master_user_password'. {} is less than 8",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Specifies options for node-to-node encryption.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NodeToNodeEncryptionOptions {
    ///
    /// Specifies to enable or disable node-to-node encryption on the domain. Required if you    enable fine-grained access control in AdvancedSecurityOptionsInput.
    ///
    /// Required: Conditional
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl cfn_resources::CfnResource for NodeToNodeEncryptionOptions {
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

/// A custom 10-hour, low-traffic window during which OpenSearch Service can perform mandatory configuration changes on the domain.    These actions can include scheduled service software updates and blue/green Auto-Tune enhancements. OpenSearch Service will    schedule these actions during the window that you specify. If you don't specify a window start time, it defaults to 10:00 P.M. local time.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OffPeakWindow {
    ///
    /// The desired start time for an off-peak maintenance window.
    ///
    /// Required: No
    ///
    /// Type: WindowStartTime
    ///
    /// Update requires: No interruption
    #[serde(rename = "WindowStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_start_time: Option<WindowStartTime>,
}

impl cfn_resources::CfnResource for OffPeakWindow {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.window_start_time
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Off-peak window settings for the domain.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OffPeakWindowOptions {
    ///
    /// Specifies whether off-peak window settings are enabled for the domain.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    ///
    /// Off-peak window settings for the domain.
    ///
    /// Required: No
    ///
    /// Type: OffPeakWindow
    ///
    /// Update requires: No interruption
    #[serde(rename = "OffPeakWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_peak_window: Option<OffPeakWindow>,
}

impl cfn_resources::CfnResource for OffPeakWindowOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.off_peak_window
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Container for information about the SAML configuration for OpenSearch Dashboards.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SAMLOptions {
    ///
    /// True to enable SAML authentication for a domain.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    ///
    /// The SAML Identity Provider's information.
    ///
    /// Required: No
    ///
    /// Type: Idp
    ///
    /// Update requires: No interruption
    #[serde(rename = "Idp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp: Option<Idp>,

    ///
    /// The backend role that the SAML master user is mapped to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterBackendRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_backend_role: Option<cfn_resources::StrVal>,

    ///
    /// The SAML master user name, which is stored in the domain's internal user database.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_name: Option<cfn_resources::StrVal>,

    ///
    /// Element of the SAML assertion to use for backend roles. Default is  roles.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RolesKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles_key: Option<cfn_resources::StrVal>,

    ///
    /// The duration, in minutes, after which a user session becomes inactive. Acceptable values are between 1 and 1440,  and the default value is 60.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_timeout_minutes: Option<i64>,

    ///
    /// Element of the SAML assertion to use for the user name. Default is  NameID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubjectKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_key: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for SAMLOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.idp.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.master_backend_role {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!("Max validation failed on field 'master_backend_role'. {} is greater than 256", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.master_backend_role {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'master_backend_role'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.master_user_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!(
                        "Max validation failed on field 'master_user_name'. {} is greater than 64",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.master_user_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'master_user_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The current status of the service software for an Amazon OpenSearch Service domain. For more  information, see Service software updates in   Amazon OpenSearch Service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServiceSoftwareOptions {
    ///
    /// The timestamp, in Epoch time, until which you can manually request a service software update. After this date,  we automatically update your service software.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutomatedUpdateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_update_date: Option<cfn_resources::StrVal>,

    ///
    /// True if you're able to cancel your service software version update. False if you can't  cancel your service software update.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cancellable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellable: Option<bool>,

    ///
    /// The current service software version present on the domain.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CurrentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<cfn_resources::StrVal>,

    ///
    /// A description of the service software update status.
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
    /// The new service software version, if one is available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NewVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_version: Option<cfn_resources::StrVal>,

    ///
    /// True if a service software is never automatically updated. False if a service software is  automatically updated after the automated update date.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptionalDeployment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_deployment: Option<bool>,

    ///
    /// True if you're able to update your service software version. False if you can't update your  service software version.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpdateAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_available: Option<bool>,

    ///
    /// The status of your service software update.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COMPLETED | ELIGIBLE | IN_PROGRESS | NOT_ELIGIBLE | PENDING_UPDATE
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<ServiceSoftwareOptionsUpdateStatusEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ServiceSoftwareOptionsUpdateStatusEnum {
    /// COMPLETED
    #[serde(rename = "COMPLETED")]
    Completed,

    /// ELIGIBLE
    #[serde(rename = "ELIGIBLE")]
    Eligible,

    /// IN_PROGRESS
    #[serde(rename = "IN_PROGRESS")]
    Inprogress,

    /// NOT_ELIGIBLE
    #[serde(rename = "NOT_ELIGIBLE")]
    Noteligible,

    /// PENDING_UPDATE
    #[serde(rename = "PENDING_UPDATE")]
    Pendingupdate,
}

impl Default for ServiceSoftwareOptionsUpdateStatusEnum {
    fn default() -> Self {
        ServiceSoftwareOptionsUpdateStatusEnum::Completed
    }
}

impl cfn_resources::CfnResource for ServiceSoftwareOptions {
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

/// DEPRECATED. This setting is only relevant to domains    running legacy Elasticsearch OSS versions earlier than 5.3. It does not apply to OpenSearch    domains.
///
/// The automated snapshot configuration for the OpenSearch Service domain indexes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SnapshotOptions {
    ///
    /// The hour in UTC during which the service takes an automated daily snapshot of the indexes    in the OpenSearch Service domain. For example, if you specify 0, OpenSearch Service takes an    automated snapshot everyday between midnight and 1 am. You can specify a value between 0 and    23.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutomatedSnapshotStartHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_start_hour: Option<i64>,
}

impl cfn_resources::CfnResource for SnapshotOptions {
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

/// Options for configuring service software updates for a domain.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SoftwareUpdateOptions {
    ///
    /// Specifies whether automatic service software updates are enabled for the domain.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoSoftwareUpdateEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_software_update_enabled: Option<bool>,
}

impl cfn_resources::CfnResource for SoftwareUpdateOptions {
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

/// The virtual private cloud (VPC) configuration for the OpenSearch Service domain. For more    information, see Launching your Amazon OpenSearch     Service domains using a VPC in the Amazon OpenSearch Service Developer     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VPCOptions {
    ///
    /// The list of security group IDs that are associated with the VPC endpoints for the domain.    If you don't provide a security group ID, OpenSearch Service uses the default security group    for the VPC. To learn more, see Security groups for your VPC in    the Amazon VPC User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// Provide one subnet ID for each Availability Zone that your domain uses. For example, you    must specify three subnet IDs for a three-AZ domain. To learn more, see VPCs and subnets in    the Amazon VPC User Guide.
    ///
    /// If you specify more than one subnet, you must also configure     ZoneAwarenessEnabled and ZoneAwarenessConfig within ClusterConfig, otherwise you'll see the error "You must specify exactly one subnet"    during template creation.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for VPCOptions {
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

/// A custom start time for the off-peak window, in Coordinated Universal Time (UTC). The window length will always be 10 hours, so you can't    specify an end time. For example, if you specify 11:00 P.M. UTC as a start time, the end time will automatically be set to 9:00 A.M.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WindowStartTime {
    ///
    /// The start hour of the window in Coordinated Universal Time (UTC), using 24-hour time. For example, 17 refers to 5:00 P.M. UTC.    The minimum value is 0 and the maximum value is 23.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Hours")]
    pub hours: i64,

    ///
    /// The start minute of the window, in UTC. The minimum value is 0 and the maximum value is 59.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Minutes")]
    pub minutes: i64,
}

impl cfn_resources::CfnResource for WindowStartTime {
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

/// Specifies zone awareness configuration options. Only use if     ZoneAwarenessEnabled is true.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ZoneAwarenessConfig {
    ///
    /// If you enabled multiple Availability Zones (AZs), the number of AZs that you want the    domain to use.
    ///
    /// Valid values are 2 and 3. Default is 2.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZoneCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_count: Option<i64>,
}

impl cfn_resources::CfnResource for ZoneAwarenessConfig {
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
