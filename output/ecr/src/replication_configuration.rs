/// The AWS::ECR::ReplicationConfiguration resource creates or updates the       replication configuration for a private registry. The first time a replication       configuration is applied to a private registry, a service-linked IAM role       is created in your account for the replication process. For more information, see Using         Service-Linked Roles for Amazon ECR in the Amazon Elastic         Container Registry User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnReplicationConfiguration {
    ///
    /// The replication configuration for a registry.
    ///
    /// Required: Yes
    ///
    /// Type: ReplicationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationConfiguration")]
    pub replication_configuration: Box<ReplicationConfiguration>,

    #[serde(skip_serializing)]
    pub att_registry_id: CfnReplicationConfigurationregistryid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnReplicationConfigurationregistryid;
impl CfnReplicationConfigurationregistryid {
    pub fn att_name(&self) -> &'static str {
        r#"RegistryId"#
    }
}

impl cfn_resources::CfnResource for CfnReplicationConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::ECR::ReplicationConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.replication_configuration.validate()?;

        Ok(())
    }
}

/// The replication configuration for a registry.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ReplicationConfiguration {
    ///
    /// An array of objects representing the replication destinations and repository filters       for a replication configuration.
    ///
    /// Required: Yes
    ///
    /// Type: List of ReplicationRule
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: Vec<ReplicationRule>,
}

impl cfn_resources::CfnResource for ReplicationConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.rules;

        if the_val.len() > 10 as _ {
            return Err(format!(
                "Max validation failed on field 'rules'. {} is greater than 10",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// An array of objects representing the destination for a replication rule.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ReplicationDestination {
    ///
    /// The Region to replicate to.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 2
    ///
    /// Maximum: 25
    ///
    /// Pattern: [0-9a-z-]{2,25}
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    pub region: cfn_resources::StrVal,

    ///
    /// The AWS account ID of the Amazon ECR private registry to replicate to. When configuring       cross-Region replication within your own registry, specify your own account ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [0-9]{12}
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegistryId")]
    pub registry_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ReplicationDestination {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.region;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 25 as _ {
                return Err(format!(
                    "Max validation failed on field 'region'. {} is greater than 25",
                    s.len()
                ));
            }
        }

        let the_val = &self.region;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 2 as _ {
                return Err(format!(
                    "Min validation failed on field 'region'. {} is less than 2",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// An array of objects representing the replication destinations and repository filters       for a replication configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ReplicationRule {
    ///
    /// An array of objects representing the destination for a replication rule.
    ///
    /// Required: Yes
    ///
    /// Type: List of ReplicationDestination
    ///
    /// Maximum: 25
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destinations")]
    pub destinations: Vec<ReplicationDestination>,

    ///
    /// An array of objects representing the filters for a replication rule. Specifying a       repository filter for a replication rule provides a method for controlling which       repositories in a private registry are replicated.
    ///
    /// Required: No
    ///
    /// Type: List of RepositoryFilter
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "RepositoryFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_filters: Option<Vec<RepositoryFilter>>,
}

impl cfn_resources::CfnResource for ReplicationRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.destinations;

        if the_val.len() > 25 as _ {
            return Err(format!(
                "Max validation failed on field 'destinations'. {} is greater than 25",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.repository_filters {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'repository_filters'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The filter settings used with image replication. Specifying a repository filter to a       replication rule provides a method for controlling which repositories in a private       registry are replicated. If no filters are added, the contents of all repositories are       replicated.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct RepositoryFilter {
    ///
    /// The repository filter details. When the PREFIX_MATCH filter type is       specified, this value is required and should be the repository name prefix to configure       replication for.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 2
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^(?:[a-z0-9]+(?:[._-][a-z0-9]*)*/)*[a-z0-9]*(?:[._-][a-z0-9]*)*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filter")]
    pub filter: cfn_resources::StrVal,

    ///
    /// The repository filter type. The only supported value is PREFIX_MATCH,       which is a repository name prefix specified with the filter       parameter.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: PREFIX_MATCH
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterType")]
    pub filter_type: RepositoryFilterFilterTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum RepositoryFilterFilterTypeEnum {
    /// PREFIX_MATCH
    #[serde(rename = "PREFIX_MATCH")]
    Prefixmatch,
}

impl Default for RepositoryFilterFilterTypeEnum {
    fn default() -> Self {
        RepositoryFilterFilterTypeEnum::Prefixmatch
    }
}

impl cfn_resources::CfnResource for RepositoryFilter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.filter;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'filter'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.filter;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 2 as _ {
                return Err(format!(
                    "Min validation failed on field 'filter'. {} is less than 2",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
