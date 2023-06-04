/// Use the AWS::IoT::AccountAuditConfiguration resource to configure or     reconfigure the Device Defender audit settings for your account. Settings include how audit     notifications are sent and which audit checks are enabled or disabled. For API reference,     see UpdateAccountAuditConfiguration and for detailed information on all available audit checks, see Audit checks.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnAccountAuditConfiguration {
    ///
    /// The ID of the account. You can use the expression !Sub "${AWS::AccountId}" to use your account ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AccountId")]
    pub account_id: cfn_resources::StrVal,

    ///
    /// Specifies which audit checks are enabled and disabled for this account.
    ///
    /// Some data collection might start immediately when certain checks are enabled. When a check is disabled, any data collected so far in relation to the check is deleted. To disable a check, set the value of the Enabled: key to false.
    ///
    /// If an enabled check is removed from the template, it will also be disabled.
    ///
    /// You can't disable a check if it's used by any scheduled audit. You must delete the check     from the scheduled audit or delete the scheduled audit itself to disable the check.
    ///
    /// For more information on avialbe auidt checks see AWS::IoT::AccountAuditConfiguration AuditCheckConfigurations
    ///
    /// Required: Yes
    ///
    /// Type: AuditCheckConfigurations
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuditCheckConfigurations")]
    pub audit_check_configurations: AuditCheckConfigurations,

    ///
    /// Information about the targets to which audit notifications are sent.
    ///
    /// Required: No
    ///
    /// Type: AuditNotificationTargetConfigurations
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuditNotificationTargetConfigurations")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub audit_notification_target_configurations: Option<AuditNotificationTargetConfigurations>,

    ///
    /// The Amazon Resource Name (ARN) of the role that grants permission to AWS IoT to access information about your devices, policies, certificates, and other items as required when performing an audit.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnAccountAuditConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::AccountAuditConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.audit_check_configurations.validate()?;

        self.audit_notification_target_configurations
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Which audit checks are enabled and disabled for this account.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AuditCheckConfiguration {
    ///
    /// True if this audit check is enabled for this account.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub enabled: Option<bool>,
}

impl cfn_resources::CfnResource for AuditCheckConfiguration {
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

/// The types of audit checks that can be performed.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AuditCheckConfigurations {
    ///
    /// Checks the permissiveness of an authenticated Amazon Cognito identity pool role. For     this check, AWS IoT Device Defender audits all Amazon Cognito identity pools that have been     used to connect to the AWS IoT message broker during the 31 days before the audit is     performed.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticatedCognitoRoleOverlyPermissiveCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub authenticated_cognito_role_overly_permissive_check: Option<AuditCheckConfiguration>,

    ///
    /// Checks if a CA certificate is expiring. This check applies to CA certificates expiring within 30 days or that have expired.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaCertificateExpiringCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub ca_certificate_expiring_check: Option<AuditCheckConfiguration>,

    ///
    /// Checks the quality of the CA certificate key. The quality checks if the key is in a valid format, not expired, and if the key meets a minimum required size. This check applies to CA certificates that are ACTIVE or PENDING_TRANSFER.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaCertificateKeyQualityCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub ca_certificate_key_quality_check: Option<AuditCheckConfiguration>,

    ///
    /// Checks if multiple devices connect using the same client ID.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConflictingClientIdsCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub conflicting_client_ids_check: Option<AuditCheckConfiguration>,

    ///
    /// Checks if a device certificate is expiring. This check applies to device certificates expiring within 30 days or that have expired.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceCertificateExpiringCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub device_certificate_expiring_check: Option<AuditCheckConfiguration>,

    ///
    /// Checks the quality of the device certificate key. The quality checks if the key is in a valid format, not expired, signed by a registered certificate authority, and if the key meets a minimum required size.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceCertificateKeyQualityCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub device_certificate_key_quality_check: Option<AuditCheckConfiguration>,

    ///
    /// Checks if multiple concurrent connections use the same X.509 certificate to authenticate with AWS IoT.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceCertificateSharedCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub device_certificate_shared_check: Option<AuditCheckConfiguration>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntermediateCaRevokedForActiveDeviceCertificatesCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub intermediate_ca_revoked_for_active_device_certificates_check:
        Option<AuditCheckConfiguration>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "IoTPolicyPotentialMisConfigurationCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub io_tpolicy_potential_mis_configuration_check: Option<AuditCheckConfiguration>,

    ///
    /// Checks the permissiveness of a policy attached to an authenticated Amazon Cognito     identity pool role.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotPolicyOverlyPermissiveCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub iot_policy_overly_permissive_check: Option<AuditCheckConfiguration>,

    ///
    /// Checks if a role alias has access to services that haven't been used for the AWS IoT device in the last year.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotRoleAliasAllowsAccessToUnusedServicesCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub iot_role_alias_allows_access_to_unused_services_check: Option<AuditCheckConfiguration>,

    ///
    /// Checks if the temporary credentials provided by AWS IoT role aliases are overly permissive.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotRoleAliasOverlyPermissiveCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub iot_role_alias_overly_permissive_check: Option<AuditCheckConfiguration>,

    ///
    /// Checks if AWS IoT logs are disabled.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingDisabledCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub logging_disabled_check: Option<AuditCheckConfiguration>,

    ///
    /// Checks if a revoked CA certificate is still active.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "RevokedCaCertificateStillActiveCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub revoked_ca_certificate_still_active_check: Option<AuditCheckConfiguration>,

    ///
    /// Checks if a revoked device certificate is still active.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "RevokedDeviceCertificateStillActiveCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub revoked_device_certificate_still_active_check: Option<AuditCheckConfiguration>,

    ///
    /// Checks if policy attached to an unauthenticated Amazon Cognito identity pool role is too permissive.
    ///
    /// Required: No
    ///
    /// Type: AuditCheckConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnauthenticatedCognitoRoleOverlyPermissiveCheck")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub unauthenticated_cognito_role_overly_permissive_check: Option<AuditCheckConfiguration>,
}

impl cfn_resources::CfnResource for AuditCheckConfigurations {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.authenticated_cognito_role_overly_permissive_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ca_certificate_expiring_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ca_certificate_key_quality_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.conflicting_client_ids_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.device_certificate_expiring_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.device_certificate_key_quality_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.device_certificate_shared_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.intermediate_ca_revoked_for_active_device_certificates_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.io_tpolicy_potential_mis_configuration_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.iot_policy_overly_permissive_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.iot_role_alias_allows_access_to_unused_services_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.iot_role_alias_overly_permissive_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.logging_disabled_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.revoked_ca_certificate_still_active_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.revoked_device_certificate_still_active_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.unauthenticated_cognito_role_overly_permissive_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information about the targets to which audit notifications are sent.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AuditNotificationTarget {
    ///
    /// True if notifications to the target are enabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub enabled: Option<bool>,

    ///
    /// The ARN of the role that grants permission to send notifications to the target.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the target (SNS topic) to which audit notifications are sent.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub target_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AuditNotificationTarget {
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

/// The configuration of the audit notification target.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AuditNotificationTargetConfigurations {
    ///
    /// The Sns notification target.
    ///
    /// Required: No
    ///
    /// Type: AuditNotificationTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sns")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub sns: Option<AuditNotificationTarget>,
}

impl cfn_resources::CfnResource for AuditNotificationTargetConfigurations {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.sns.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
