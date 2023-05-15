
pub mod cfn_pull_through_cache_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnPullThroughCacheRule {
    /// The ECRRepositoryPrefix is a custom alias for upstream registry url.
    #[serde(rename = "EcrRepositoryPrefix")]
    pub ecr_repository_prefix: Option<String>,
    /// The upstreamRegistryUrl is the endpoint of upstream registry url of the public repository to be cached
    #[serde(rename = "UpstreamRegistryUrl")]
    pub upstream_registry_url: Option<String>,

}

pub type EcrRepositoryPrefix = String;
#[derive(serde::Serialize, Default)]
pub struct PullThroughCacheRule {
    #[serde(rename = "RegistryId")]
    pub registry_id: Option<RegistryId>,
    #[serde(rename = "UpstreamRegistryUrl")]
    pub upstream_registry_url: UpstreamRegistryUrl,
    #[serde(rename = "EcrRepositoryPrefix")]
    pub ecr_repository_prefix: EcrRepositoryPrefix,

}
pub type UpstreamRegistryUrl = String;pub type RegistryId = String;

}

pub mod cfn_registry_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnRegistryPolicy {
    /// The JSON policy text to apply to your registry. The policy text follows the same format as IAM policy text. For more information, see Registry permissions (https://docs.aws.amazon.com/AmazonECR/latest/userguide/registry-permissions.html) in the Amazon Elastic Container Registry User Guide.
    #[serde(rename = "PolicyText")]
    pub policy_text: (),

}

pub type RegistryId = String;

}

pub mod cfn_replication_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnReplicationConfiguration {
    /// An object representing the replication configuration for a registry.
    #[serde(rename = "ReplicationConfiguration")]
    pub replication_configuration: ReplicationConfiguration,

}

pub type RegistryId = String;
#[derive(serde::Serialize, Default)]
pub struct RepositoryFilter {
    #[serde(rename = "Filter")]
    pub filter: Filter,
    #[serde(rename = "FilterType")]
    pub filter_type: FilterType,

}
pub type Region = String;
#[derive(serde::Serialize, Default)]
pub struct ReplicationRule {
    #[serde(rename = "RepositoryFilters")]
    pub repository_filters: Option<Vec<RepositoryFilter>>,
    #[serde(rename = "Destinations")]
    pub destinations: Vec<ReplicationDestination>,

}
pub type FilterType = String;pub type Filter = String;
#[derive(serde::Serialize, Default)]
pub struct ReplicationConfiguration {
    #[serde(rename = "Rules")]
    pub rules: Vec<ReplicationRule>,

}

#[derive(serde::Serialize, Default)]
pub struct ReplicationDestination {
    #[serde(rename = "Region")]
    pub region: Region,
    #[serde(rename = "RegistryId")]
    pub registry_id: RegistryId,

}


}

pub mod cfn_repository {

#[derive(serde::Serialize, Default)]
pub struct CfnRepository {
    /// The name to use for the repository. The repository name may be specified on its own (such as nginx-web-app) or it can be prepended with a namespace to group the repository into a category (such as project-a/nginx-web-app). If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the repository name. For more information, see https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-name.html.
    #[serde(rename = "RepositoryName")]
    pub repository_name: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The image tag mutability setting for the repository.
    #[serde(rename = "ImageTagMutability")]
    pub image_tag_mutability: Option<String>,
    /// The encryption configuration for the repository. This determines how the contents of your repository are encrypted at rest.
    /// 
    /// By default, when no encryption configuration is set or the AES256 encryption type is used, Amazon ECR uses server-side encryption with Amazon S3-managed encryption keys which encrypts your data at rest using an AES-256 encryption algorithm. This does not require any action on your part.
    /// 
    /// For more information, see https://docs.aws.amazon.com/AmazonECR/latest/userguide/encryption-at-rest.html
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// The image scanning configuration for the repository. This setting determines whether images are scanned for known vulnerabilities after being pushed to the repository.
    #[serde(rename = "ImageScanningConfiguration")]
    pub image_scanning_configuration: Option<ImageScanningConfiguration>,
    /// The LifecyclePolicy property type specifies a lifecycle policy. For information about lifecycle policy syntax, see https://docs.aws.amazon.com/AmazonECR/latest/userguide/LifecyclePolicies.html
    #[serde(rename = "LifecyclePolicy")]
    pub lifecycle_policy: Option<LifecyclePolicy>,
    /// The JSON repository policy text to apply to the repository. For more information, see https://docs.aws.amazon.com/AmazonECR/latest/userguide/RepositoryPolicyExamples.html in the Amazon Elastic Container Registry User Guide.
    #[serde(rename = "RepositoryPolicyText")]
    pub repository_policy_text: Option<()>,

}

pub type LifecyclePolicyText = String;
#[derive(serde::Serialize, Default)]
pub struct EncryptionConfiguration {
    #[serde(rename = "KmsKey")]
    pub kms_key: Option<KmsKey>,
    #[serde(rename = "EncryptionType")]
    pub encryption_type: EncryptionType,

}

#[derive(serde::Serialize, Default)]
pub struct ImageScanningConfiguration {
    #[serde(rename = "ScanOnPush")]
    pub scan_on_push: Option<ScanOnPush>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type RegistryId = String;
#[derive(serde::Serialize, Default)]
pub struct LifecyclePolicy {
    #[serde(rename = "LifecyclePolicyText")]
    pub lifecycle_policy_text: Option<LifecyclePolicyText>,
    #[serde(rename = "RegistryId")]
    pub registry_id: Option<RegistryId>,

}
pub type EncryptionType = String;pub type ScanOnPush = bool;pub type KmsKey = String;

}
