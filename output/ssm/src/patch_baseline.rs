/// The AWS::SSM::PatchBaseline resource defines the basic information for an     AWS Systems Manager patch baseline. A patch baseline defines which patches are approved for     installation on your instances.
///
/// For more information, see CreatePatchBaseline in the AWS Systems Manager API       Reference.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPatchBaseline {
    ///
    /// A set of rules used to include patches in the baseline.
    ///
    /// Required: No
    ///
    /// Type: RuleGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApprovalRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<RuleGroup>,

    ///
    /// A list of explicitly approved patches for the baseline.
    ///
    /// For information about accepted formats for lists of approved patches and rejected patches,             see About             package name formats for approved and rejected patch lists in the         AWS Systems Manager User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApprovedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<Vec<String>>,

    ///
    /// Defines the compliance level for approved patches. When an approved patch is reported as  missing, this value describes the severity of the compliance violation. The default value is   UNSPECIFIED.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CRITICAL | HIGH | INFORMATIONAL | LOW | MEDIUM | UNSPECIFIED
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<PatchBaselineApprovedPatchesComplianceLevelEnum>,

    ///
    /// Indicates whether the list of approved patches includes non-security updates that should be  applied to the managed nodes. The default value is false. Applies to Linux managed  nodes only.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<bool>,

    ///
    /// A description of the patch baseline.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// A set of global filters used to include patches in the baseline.
    ///
    /// Required: No
    ///
    /// Type: PatchFilterGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlobalFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<PatchFilterGroup>,

    ///
    /// The name of the patch baseline.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9_\-.]{3,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Defines the operating system the patch baseline applies to. The default value is   WINDOWS.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AMAZON_LINUX | AMAZON_LINUX_2 | AMAZON_LINUX_2022 | AMAZON_LINUX_2023 | CENTOS | DEBIAN | MACOS | ORACLE_LINUX | RASPBIAN | REDHAT_ENTERPRISE_LINUX | ROCKY_LINUX | SUSE | UBUNTU | WINDOWS
    ///
    /// Update requires: Replacement
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<PatchBaselineOperatingSystemEnum>,

    ///
    /// The name of the patch group to be registered with the patch baseline.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^([\p{L}\p{Z}\p{N}_.:/=+\-@]*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatchGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_groups: Option<Vec<String>>,

    ///
    /// A list of explicitly rejected patches for the baseline.
    ///
    /// For information about accepted formats for lists of approved patches and rejected patches,             see About             package name formats for approved and rejected patch lists in the         AWS Systems Manager User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "RejectedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<Vec<String>>,

    ///
    /// The action for Patch Manager to take on patches included in the   RejectedPackages list.
    ///
    /// ALLOW_AS_DEPENDENCY          : A package in the    Rejected patches list is installed only if it is a dependency of another package.   It is considered compliant with the patch baseline, and its status is reported as    InstalledOther. This is the default action if no option is specified.                                   BLOCK          : Packages in the    RejectedPatches list, and packages that include them as dependencies, aren't   installed under any circumstances. If a package was installed before it was added to the   Rejected patches list, it is considered non-compliant with the patch baseline, and its status   is reported as InstalledRejected.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ALLOW_AS_DEPENDENCY | BLOCK
    ///
    /// Update requires: No interruption
    #[serde(rename = "RejectedPatchesAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches_action: Option<PatchBaselineRejectedPatchesActionEnum>,

    ///
    /// Information about the patches to use to update the managed nodes, including target operating  systems and source repositories. Applies to Linux managed nodes only.
    ///
    /// Required: No
    ///
    /// Type: List of PatchSource
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<PatchSource>>,

    ///
    /// Optional metadata that you assign to a resource. Tags enable you to categorize a resource    in different ways, such as by purpose, owner, or environment. For example, you might want to    tag a patch baseline to identify the severity level of patches it specifies and the operating    system family it applies to.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PatchBaselineApprovedPatchesComplianceLevelEnum {
    /// CRITICAL
    #[serde(rename = "CRITICAL")]
    Critical,

    /// HIGH
    #[serde(rename = "HIGH")]
    High,

    /// INFORMATIONAL
    #[serde(rename = "INFORMATIONAL")]
    Informational,

    /// LOW
    #[serde(rename = "LOW")]
    Low,

    /// MEDIUM
    #[serde(rename = "MEDIUM")]
    Medium,

    /// UNSPECIFIED
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,
}

impl Default for PatchBaselineApprovedPatchesComplianceLevelEnum {
    fn default() -> Self {
        PatchBaselineApprovedPatchesComplianceLevelEnum::Critical
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PatchBaselineOperatingSystemEnum {
    /// AMAZON_LINUX
    #[serde(rename = "AMAZON_LINUX")]
    Amazonlinux,

    /// AMAZON_LINUX_2
    #[serde(rename = "AMAZON_LINUX_2")]
    Amazonlinux2,

    /// AMAZON_LINUX_2022
    #[serde(rename = "AMAZON_LINUX_2022")]
    Amazonlinux2022,

    /// AMAZON_LINUX_2023
    #[serde(rename = "AMAZON_LINUX_2023")]
    Amazonlinux2023,

    /// CENTOS
    #[serde(rename = "CENTOS")]
    Centos,

    /// DEBIAN
    #[serde(rename = "DEBIAN")]
    Debian,

    /// MACOS
    #[serde(rename = "MACOS")]
    Macos,

    /// ORACLE_LINUX
    #[serde(rename = "ORACLE_LINUX")]
    Oraclelinux,

    /// RASPBIAN
    #[serde(rename = "RASPBIAN")]
    Raspbian,

    /// REDHAT_ENTERPRISE_LINUX
    #[serde(rename = "REDHAT_ENTERPRISE_LINUX")]
    Redhatenterpriselinux,

    /// ROCKY_LINUX
    #[serde(rename = "ROCKY_LINUX")]
    Rockylinux,

    /// SUSE
    #[serde(rename = "SUSE")]
    Suse,

    /// UBUNTU
    #[serde(rename = "UBUNTU")]
    Ubuntu,

    /// WINDOWS
    #[serde(rename = "WINDOWS")]
    Windows,
}

impl Default for PatchBaselineOperatingSystemEnum {
    fn default() -> Self {
        PatchBaselineOperatingSystemEnum::Amazonlinux
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PatchBaselineRejectedPatchesActionEnum {
    /// ALLOW_AS_DEPENDENCY
    #[serde(rename = "ALLOW_AS_DEPENDENCY")]
    Allowasdependency,

    /// BLOCK
    #[serde(rename = "BLOCK")]
    Block,
}

impl Default for PatchBaselineRejectedPatchesActionEnum {
    fn default() -> Self {
        PatchBaselineRejectedPatchesActionEnum::Allowasdependency
    }
}

impl cfn_resources::CfnResource for CfnPatchBaseline {
    fn type_string(&self) -> &'static str {
        "AWS::SSM::PatchBaseline"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.approval_rules
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.approved_patches {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'approved_patches'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.global_filters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 3 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 3",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.patch_groups {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'patch_groups'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.patch_groups {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'patch_groups'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.rejected_patches {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'rejected_patches'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.sources {
            if the_val.len() > 20 as _ {
                return Err(format!(
                    "Max validation failed on field 'sources'. {} is greater than 20",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 1000",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The PatchFilter property type defines a patch filter for an AWS Systems Manager patch baseline.
///
/// The PatchFilters property of the PatchFilterGroup property type contains a list of PatchFilter property    types.
///
/// You can view lists of valid values for the patch properties by running the   DescribePatchProperties command. For more information, see DescribePatchProperties in the AWS Systems Manager API Reference.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PatchFilter {
    ///
    /// The key for the filter.
    ///
    /// For information about valid keys, see PatchFilter in the     AWS Systems Manager API Reference.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADVISORY_ID | ARCH | BUGZILLA_ID | CLASSIFICATION | CVE_ID | EPOCH | MSRC_SEVERITY | NAME | PATCH_ID | PATCH_SET | PRIORITY | PRODUCT | PRODUCT_FAMILY | RELEASE | REPOSITORY | SECTION | SECURITY | SEVERITY | VERSION
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<PatchFilterKeyEnum>,

    ///
    /// The value for the filter key.
    ///
    /// For information about valid values for each key based on operating system type, see PatchFilter in the AWS Systems Manager API Reference.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PatchFilterKeyEnum {
    /// ADVISORY_ID
    #[serde(rename = "ADVISORY_ID")]
    Advisoryid,

    /// ARCH
    #[serde(rename = "ARCH")]
    Arch,

    /// BUGZILLA_ID
    #[serde(rename = "BUGZILLA_ID")]
    Bugzillaid,

    /// CLASSIFICATION
    #[serde(rename = "CLASSIFICATION")]
    Classification,

    /// CVE_ID
    #[serde(rename = "CVE_ID")]
    Cveid,

    /// EPOCH
    #[serde(rename = "EPOCH")]
    Epoch,

    /// MSRC_SEVERITY
    #[serde(rename = "MSRC_SEVERITY")]
    Msrcseverity,

    /// NAME
    #[serde(rename = "NAME")]
    Name,

    /// PATCH_ID
    #[serde(rename = "PATCH_ID")]
    Patchid,

    /// PATCH_SET
    #[serde(rename = "PATCH_SET")]
    Patchset,

    /// PRIORITY
    #[serde(rename = "PRIORITY")]
    Priority,

    /// PRODUCT
    #[serde(rename = "PRODUCT")]
    Product,

    /// PRODUCT_FAMILY
    #[serde(rename = "PRODUCT_FAMILY")]
    Productfamily,

    /// RELEASE
    #[serde(rename = "RELEASE")]
    Release,

    /// REPOSITORY
    #[serde(rename = "REPOSITORY")]
    Repository,

    /// SECTION
    #[serde(rename = "SECTION")]
    Section,

    /// SECURITY
    #[serde(rename = "SECURITY")]
    Security,

    /// SEVERITY
    #[serde(rename = "SEVERITY")]
    Severity,

    /// VERSION
    #[serde(rename = "VERSION")]
    Version,
}

impl Default for PatchFilterKeyEnum {
    fn default() -> Self {
        PatchFilterKeyEnum::Advisoryid
    }
}

impl cfn_resources::CfnResource for PatchFilter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.values {
            if the_val.len() > 20 as _ {
                return Err(format!(
                    "Max validation failed on field 'values'. {} is greater than 20",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The PatchFilterGroup property type specifies a set of patch filters for an    AWS Systems Manager patch baseline, typically used for approval rules for a Systems Manager    patch baseline.
///
/// PatchFilterGroup is the property type for the GlobalFilters property    of the AWS::SSM::PatchBaseline resource and the PatchFilterGroup property of    the Rule property type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PatchFilterGroup {
    ///
    /// The set of patch filters that make up the group.
    ///
    /// Required: No
    ///
    /// Type: List of PatchFilter
    ///
    /// Maximum: 4
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatchFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_filters: Option<Vec<PatchFilter>>,
}

impl cfn_resources::CfnResource for PatchFilterGroup {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.patch_filters {
            if the_val.len() > 4 as _ {
                return Err(format!(
                    "Max validation failed on field 'patch_filters'. {} is greater than 4",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// PatchSource is the property type for the Sources resource of the     AWS::SSM::PatchBaseline resource.
///
/// The AWS CloudFormation AWS::SSM::PatchSource resource is used to provide    information about the patches to use to update target instances, including target operating    systems and source repository. Applies to Linux instances only.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PatchSource {
    ///
    /// The value of the yum repo configuration. For example:
    ///
    /// [main]
    ///
    /// name=MyCustomRepository
    ///
    /// baseurl=https://my-custom-repository
    ///
    /// enabled=1
    ///
    /// NoteFor information about other options available for your yum repository configuration, see   dnf.conf(5).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<cfn_resources::StrVal>,

    ///
    /// The name specified to identify the patch source.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9_\-.]{3,50}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The specific operating system versions a patch repository applies to, such as    "Ubuntu16.04", "AmazonLinux2016.09", "RedhatEnterpriseLinux7.2" or "Suse12.7". For lists of    supported product values, see PatchFilter in the     AWS Systems Manager API Reference.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "Products")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for PatchSource {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.configuration {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'configuration'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.configuration {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'configuration'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.products {
            if the_val.len() > 20 as _ {
                return Err(format!(
                    "Max validation failed on field 'products'. {} is greater than 20",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The date for ApproveUntilDate, as a String in the format     YYYY-MM-DD. For example, 2020-12-31.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PatchStringDate {}

impl cfn_resources::CfnResource for PatchStringDate {
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

/// The Rule property type specifies an approval rule for a Systems Manager patch    baseline.
///
/// The PatchRules property of the RuleGroup property type contains a list of Rule property types.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Rule {
    ///
    /// The number of days after the release date of each patch matched by the rule that the patch    is marked as approved in the patch baseline. For example, a value of 7 means that    patches are approved seven days after they are released.
    ///
    /// You must specify a value for ApproveAfterDays.
    ///
    /// Exception: Not supported on Debian Server or Ubuntu Server.
    ///
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 360
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApproveAfterDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approve_after_days: Option<i64>,

    ///
    /// The cutoff date for auto approval of released patches. Any patches released on or before  this date are installed automatically. Not supported on Debian Server or Ubuntu Server.
    ///
    /// Enter dates in the format YYYY-MM-DD. For example,  2021-12-31.
    ///
    /// Required: No
    ///
    /// Type: PatchStringDate
    ///
    /// Minimum: 1
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApproveUntilDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approve_until_date: Option<PatchStringDate>,

    ///
    /// A compliance severity level for all approved patches in a patch baseline. Valid compliance    severity levels include the following: UNSPECIFIED, CRITICAL,     HIGH, MEDIUM, LOW, and    INFORMATIONAL.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CRITICAL | HIGH | INFORMATIONAL | LOW | MEDIUM | UNSPECIFIED
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComplianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_level: Option<RuleComplianceLevelEnum>,

    ///
    /// For managed nodes identified by the approval rule filters, enables a patch baseline to apply  non-security updates available in the specified repository. The default value is   false. Applies to Linux managed nodes only.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableNonSecurity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_non_security: Option<bool>,

    ///
    /// The patch filter group that defines the criteria for the rule.
    ///
    /// Required: No
    ///
    /// Type: PatchFilterGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatchFilterGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_filter_group: Option<PatchFilterGroup>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum RuleComplianceLevelEnum {
    /// CRITICAL
    #[serde(rename = "CRITICAL")]
    Critical,

    /// HIGH
    #[serde(rename = "HIGH")]
    High,

    /// INFORMATIONAL
    #[serde(rename = "INFORMATIONAL")]
    Informational,

    /// LOW
    #[serde(rename = "LOW")]
    Low,

    /// MEDIUM
    #[serde(rename = "MEDIUM")]
    Medium,

    /// UNSPECIFIED
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,
}

impl Default for RuleComplianceLevelEnum {
    fn default() -> Self {
        RuleComplianceLevelEnum::Critical
    }
}

impl cfn_resources::CfnResource for Rule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.approve_after_days {
            if *the_val > 360 as _ {
                return Err(format!(
                    "Max validation failed on field 'approve_after_days'. {} is greater than 360",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.approve_after_days {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'approve_after_days'. {} is less than 0",
                    the_val
                ));
            }
        }

        self.approve_until_date
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.patch_filter_group
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The RuleGroup property type specifies a set of rules that define the approval    rules for an AWS Systems Manager patch baseline.
///
/// RuleGroup is the property type for the ApprovalRules property of the     AWS::SSM::PatchBaseline resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct RuleGroup {
    ///
    /// The rules that make up the rule group.
    ///
    /// Required: No
    ///
    /// Type: List of Rule
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatchRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_rules: Option<Vec<Rule>>,
}

impl cfn_resources::CfnResource for RuleGroup {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.patch_rules {
            if the_val.len() > 10 as _ {
                return Err(format!(
                    "Max validation failed on field 'patch_rules'. {} is greater than 10",
                    the_val.len()
                ));
            }
        }

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
