

/// The AWS::SSM::PatchBaseline resource defines the basic information for an     AWS Systems Manager patch baseline. A patch baseline defines which patches are approved for     installation on your instances.
///
/// For more information, see CreatePatchBaseline in the AWS Systems Manager API       Reference.
#[derive(Default, serde::Serialize)]
pub struct CfnPatchBaseline {


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
    pub sources: Option<Vec<PatchSource>>,


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
    pub patch_groups: Option<Vec<String>>,


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
    pub description: Option<String>,


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
    pub tags: Option<Vec<Tag>>,


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
    pub approved_patches_compliance_level: Option<String>,


    /// 
    /// A set of global filters used to include patches in the baseline.
    /// 
    /// Required: No
    ///
    /// Type: PatchFilterGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlobalFilters")]
    pub global_filters: Option<PatchFilterGroup>,


    /// 
    /// Indicates whether the list of approved patches includes non-security updates that should be  applied to the managed nodes. The default value is false. Applies to Linux managed  nodes only.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    pub approved_patches_enable_non_security: Option<bool>,


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
    pub operating_system: Option<String>,


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
    pub rejected_patches: Option<Vec<String>>,


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
    pub name: String,


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
    pub rejected_patches_action: Option<String>,


    /// 
    /// A set of rules used to include patches in the baseline.
    /// 
    /// Required: No
    ///
    /// Type: RuleGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApprovalRules")]
    pub approval_rules: Option<RuleGroup>,

}


/// The date for ApproveUntilDate, as a String in the format     YYYY-MM-DD. For example, 2020-12-31.
#[derive(Default, serde::Serialize)]
pub struct PatchStringDate {

}


/// The PatchFilterGroup property type specifies a set of patch filters for an    AWS Systems Manager patch baseline, typically used for approval rules for a Systems Manager    patch baseline.
///
/// PatchFilterGroup is the property type for the GlobalFilters property    of the AWS::SSM::PatchBaseline resource and the PatchFilterGroup property of    the Rule property type.
#[derive(Default, serde::Serialize)]
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
    pub patch_filters: Option<Vec<PatchFilter>>,

}


/// The PatchFilter property type defines a patch filter for an AWS Systems Manager patch baseline.
///
/// The PatchFilters property of the PatchFilterGroup property type contains a list of PatchFilter property    types.
///
/// You can view lists of valid values for the patch properties by running the   DescribePatchProperties command. For more information, see DescribePatchProperties in the AWS Systems Manager API Reference.
#[derive(Default, serde::Serialize)]
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
    pub key: Option<String>,


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
    pub values: Option<Vec<String>>,

}


/// The RuleGroup property type specifies a set of rules that define the approval    rules for an AWS Systems Manager patch baseline.
///
/// RuleGroup is the property type for the ApprovalRules property of the     AWS::SSM::PatchBaseline resource.
#[derive(Default, serde::Serialize)]
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
    pub patch_rules: Option<Vec<Rule>>,

}


/// The Rule property type specifies an approval rule for a Systems Manager patch    baseline.
///
/// The PatchRules property of the RuleGroup property type contains a list of Rule property types.
#[derive(Default, serde::Serialize)]
pub struct Rule {


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
    pub approve_until_date: Option<PatchStringDate>,


    /// 
    /// The patch filter group that defines the criteria for the rule.
    /// 
    /// Required: No
    ///
    /// Type: PatchFilterGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatchFilterGroup")]
    pub patch_filter_group: Option<PatchFilterGroup>,


    /// 
    /// For managed nodes identified by the approval rule filters, enables a patch baseline to apply  non-security updates available in the specified repository. The default value is   false. Applies to Linux managed nodes only.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableNonSecurity")]
    pub enable_non_security: Option<bool>,


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
    pub approve_after_days: Option<i64>,


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
    pub compliance_level: Option<String>,

}


/// PatchSource is the property type for the Sources resource of the     AWS::SSM::PatchBaseline resource.
///
/// The AWS CloudFormation AWS::SSM::PatchSource resource is used to provide    information about the patches to use to update target instances, including target operating    systems and source repository. Applies to Linux instances only.
#[derive(Default, serde::Serialize)]
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
    pub configuration: Option<String>,


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
    pub products: Option<Vec<String>>,


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
    pub name: Option<String>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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
