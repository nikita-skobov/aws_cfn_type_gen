/// Creates a new Capacity Reservation Fleet with the specified attributes. For more     information, see Capacity Reservation Fleets in the Amazon EC2 User     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCapacityReservationFleet {
    ///
    /// The strategy used by the Capacity Reservation Fleet to determine which of the 			specified instance types to use. Currently, only the prioritized 			allocation strategy is supported. For more information, see 				Allocation strategy in the Amazon EC2 User Guide.
    ///
    /// Valid values: prioritized
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AllocationStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_strategy: Option<String>,

    ///
    /// The date and time at which the Capacity Reservation Fleet expires. When the Capacity 			Reservation Fleet expires, its state changes to expired and all of the Capacity 			Reservations in the Fleet expire.
    ///
    /// The Capacity Reservation Fleet expires within an hour after the specified time. For example, 			if you specify 5/31/2019, 13:30:55, the Capacity Reservation Fleet 			is guaranteed to expire between 13:30:55 and 14:30:55 on 			5/31/2019.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,

    ///
    /// Indicates the type of instance launches that the Capacity Reservation Fleet accepts. All 			Capacity Reservations in the Fleet inherit this instance matching criteria.
    ///
    /// Currently, Capacity Reservation Fleets support open instance matching criteria 			only. This means that instances that have matching attributes (instance type, platform, and 			Availability Zone) run in the Capacity Reservations automatically. Instances do not need to 			explicitly target a Capacity Reservation Fleet to use its reserved capacity.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: open
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceMatchCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_match_criteria: Option<CapacityReservationFleetInstanceMatchCriteriaEnum>,

    ///
    /// Information about the instance types for which to reserve the capacity.
    ///
    /// Required: No
    ///
    /// Type: List of InstanceTypeSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceTypeSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type_specifications: Option<Vec<InstanceTypeSpecification>>,

    ///
    /// Used to add an end date to a Capacity Reservation Fleet that has no end date and      time. To add an end date to a Capacity Reservation Fleet, specify true      for this paramater and specify the end date and time (in UTC time format) for the      EndDate parameter.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoRemoveEndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_remove_end_date: Option<bool>,

    ///
    /// Used to remove an end date from a Capacity Reservation Fleet that is configured      to end automatically at a specific date and time. To remove the end date from a      Capacity Reservation Fleet, specify true for this paramater and omit      the EndDate parameter.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveEndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_end_date: Option<bool>,

    ///
    /// The tags to assign to the Capacity Reservation Fleet. The tags are automatically assigned 			to the Capacity Reservations in the Fleet.
    ///
    /// Required: No
    ///
    /// Type: List of TagSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "TagSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_specifications: Option<Vec<TagSpecification>>,

    ///
    /// Indicates the tenancy of the Capacity Reservation Fleet. All Capacity Reservations 			in the Fleet inherit this tenancy. The Capacity Reservation Fleet can have one of 			the following tenancy settings:
    ///
    /// default - The Capacity Reservation Fleet is created on hardware 					that is shared with other AWS accounts.                        dedicated - The Capacity Reservations are created on single-tenant 					hardware that is dedicated to a single AWS account.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: default
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tenancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<CapacityReservationFleetTenancyEnum>,

    ///
    /// The total number of capacity units to be reserved by the Capacity Reservation Fleet. This 			value, together with the instance type weights that you assign to each instance type used by 			the Fleet determine the number of instances for which the Fleet reserves capacity. Both values 			are based on units that make sense for your workload. For more information, see 				Total target capacity in the Amazon EC2 User Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TotalTargetCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_target_capacity: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CapacityReservationFleetInstanceMatchCriteriaEnum {
    /// open
    #[serde(rename = "open")]
    Open,
}

impl Default for CapacityReservationFleetInstanceMatchCriteriaEnum {
    fn default() -> Self {
        CapacityReservationFleetInstanceMatchCriteriaEnum::Open
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CapacityReservationFleetTenancyEnum {
    /// default
    #[serde(rename = "default")]
    Default,
}

impl Default for CapacityReservationFleetTenancyEnum {
    fn default() -> Self {
        CapacityReservationFleetTenancyEnum::Default
    }
}

impl cfn_resources::CfnResource for CfnCapacityReservationFleet {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::CapacityReservationFleet"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies information about an instance type to use in a Capacity Reservation     Fleet.
///
/// InstanceTypeSpecification is a property of the       AWS::EC2::CapacityReservationFleet resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InstanceTypeSpecification {
    ///
    /// The Availability Zone in which the Capacity Reservation Fleet reserves the capacity. A Capacity 			Reservation Fleet can't span Availability Zones. All instance type specifications that you specify 			for the Fleet must use the same Availability Zone.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,

    ///
    /// The ID of the Availability Zone in which the Capacity Reservation Fleet reserves the capacity. A 			Capacity Reservation Fleet can't span Availability Zones. All instance type specifications that you 			specify for the Fleet must use the same Availability Zone.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,

    ///
    /// Indicates whether the Capacity Reservation Fleet supports EBS-optimized instances types. This 			optimization provides dedicated throughput to Amazon EBS and an optimized configuration stack 			to provide optimal I/O performance. This optimization isn't available with all instance types. Additional 			usage charges apply when using EBS-optimized instance types.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EbsOptimized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,

    ///
    /// The type of operating system for which the Capacity Reservation Fleet reserves capacity.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Linux with SQL Server Enterprise | Linux with SQL Server Standard | Linux with SQL Server Web | Linux/UNIX | Red Hat Enterprise Linux | RHEL with HA | RHEL with HA and SQL Server Enterprise | RHEL with HA and SQL Server Standard | RHEL with SQL Server Enterprise | RHEL with SQL Server Standard | RHEL with SQL Server Web | SUSE Linux | Windows | Windows with SQL Server | Windows with SQL Server Enterprise | Windows with SQL Server Standard | Windows with SQL Server Web
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstancePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_platform: Option<InstanceTypeSpecificationInstancePlatformEnum>,

    ///
    /// The instance type for which the Capacity Reservation Fleet reserves capacity.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: a1.2xlarge | a1.4xlarge | a1.large | a1.medium | a1.metal | a1.xlarge | c1.medium | c1.xlarge | c3.2xlarge | c3.4xlarge | c3.8xlarge | c3.large | c3.xlarge | c4.2xlarge | c4.4xlarge | c4.8xlarge | c4.large | c4.xlarge | c5.12xlarge | c5.18xlarge | c5.24xlarge | c5.2xlarge | c5.4xlarge | c5.9xlarge | c5.large | c5.metal | c5.xlarge | c5a.12xlarge | c5a.16xlarge | c5a.24xlarge | c5a.2xlarge | c5a.4xlarge | c5a.8xlarge | c5a.large | c5a.xlarge | c5ad.12xlarge | c5ad.16xlarge | c5ad.24xlarge | c5ad.2xlarge | c5ad.4xlarge | c5ad.8xlarge | c5ad.large | c5ad.xlarge | c5d.12xlarge | c5d.18xlarge | c5d.24xlarge | c5d.2xlarge | c5d.4xlarge | c5d.9xlarge | c5d.large | c5d.metal | c5d.xlarge | c5n.18xlarge | c5n.2xlarge | c5n.4xlarge | c5n.9xlarge | c5n.large | c5n.metal | c5n.xlarge | c6a.12xlarge | c6a.16xlarge | c6a.24xlarge | c6a.2xlarge | c6a.32xlarge | c6a.48xlarge | c6a.4xlarge | c6a.8xlarge | c6a.large | c6a.metal | c6a.xlarge | c6g.12xlarge | c6g.16xlarge | c6g.2xlarge | c6g.4xlarge | c6g.8xlarge | c6g.large | c6g.medium | c6g.metal | c6g.xlarge | c6gd.12xlarge | c6gd.16xlarge | c6gd.2xlarge | c6gd.4xlarge | c6gd.8xlarge | c6gd.large | c6gd.medium | c6gd.metal | c6gd.xlarge | c6gn.12xlarge | c6gn.16xlarge | c6gn.2xlarge | c6gn.4xlarge | c6gn.8xlarge | c6gn.large | c6gn.medium | c6gn.xlarge | c6i.12xlarge | c6i.16xlarge | c6i.24xlarge | c6i.2xlarge | c6i.32xlarge | c6i.4xlarge | c6i.8xlarge | c6i.large | c6i.metal | c6i.xlarge | c6id.12xlarge | c6id.16xlarge | c6id.24xlarge | c6id.2xlarge | c6id.32xlarge | c6id.4xlarge | c6id.8xlarge | c6id.large | c6id.metal | c6id.xlarge | c6in.12xlarge | c6in.16xlarge | c6in.24xlarge | c6in.2xlarge | c6in.32xlarge | c6in.4xlarge | c6in.8xlarge | c6in.large | c6in.metal | c6in.xlarge | c7g.12xlarge | c7g.16xlarge | c7g.2xlarge | c7g.4xlarge | c7g.8xlarge | c7g.large | c7g.medium | c7g.xlarge | cc1.4xlarge | cc2.8xlarge | cg1.4xlarge | cr1.8xlarge | d2.2xlarge | d2.4xlarge | d2.8xlarge | d2.xlarge | d3.2xlarge | d3.4xlarge | d3.8xlarge | d3.xlarge | d3en.12xlarge | d3en.2xlarge | d3en.4xlarge | d3en.6xlarge | d3en.8xlarge | d3en.xlarge | dl1.24xlarge | f1.16xlarge | f1.2xlarge | f1.4xlarge | g2.2xlarge | g2.8xlarge | g3.16xlarge | g3.4xlarge | g3.8xlarge | g3s.xlarge | g4ad.16xlarge | g4ad.2xlarge | g4ad.4xlarge | g4ad.8xlarge | g4ad.xlarge | g4dn.12xlarge | g4dn.16xlarge | g4dn.2xlarge | g4dn.4xlarge | g4dn.8xlarge | g4dn.metal | g4dn.xlarge | g5.12xlarge | g5.16xlarge | g5.24xlarge | g5.2xlarge | g5.48xlarge | g5.4xlarge | g5.8xlarge | g5.xlarge | g5g.16xlarge | g5g.2xlarge | g5g.4xlarge | g5g.8xlarge | g5g.metal | g5g.xlarge | h1.16xlarge | h1.2xlarge | h1.4xlarge | h1.8xlarge | hi1.4xlarge | hpc6a.48xlarge | hpc6id.32xlarge | hs1.8xlarge | i2.2xlarge | i2.4xlarge | i2.8xlarge | i2.xlarge | i3.16xlarge | i3.2xlarge | i3.4xlarge | i3.8xlarge | i3.large | i3.metal | i3.xlarge | i3en.12xlarge | i3en.24xlarge | i3en.2xlarge | i3en.3xlarge | i3en.6xlarge | i3en.large | i3en.metal | i3en.xlarge | i4g.16xlarge | i4g.2xlarge | i4g.4xlarge | i4g.8xlarge | i4g.large | i4g.xlarge | i4i.16xlarge | i4i.2xlarge | i4i.32xlarge | i4i.4xlarge | i4i.8xlarge | i4i.large | i4i.metal | i4i.xlarge | im4gn.16xlarge | im4gn.2xlarge | im4gn.4xlarge | im4gn.8xlarge | im4gn.large | im4gn.xlarge | inf1.24xlarge | inf1.2xlarge | inf1.6xlarge | inf1.xlarge | inf2.24xlarge | inf2.48xlarge | inf2.8xlarge | inf2.xlarge | is4gen.2xlarge | is4gen.4xlarge | is4gen.8xlarge | is4gen.large | is4gen.medium | is4gen.xlarge | m1.large | m1.medium | m1.small | m1.xlarge | m2.2xlarge | m2.4xlarge | m2.xlarge | m3.2xlarge | m3.large | m3.medium | m3.xlarge | m4.10xlarge | m4.16xlarge | m4.2xlarge | m4.4xlarge | m4.large | m4.xlarge | m5.12xlarge | m5.16xlarge | m5.24xlarge | m5.2xlarge | m5.4xlarge | m5.8xlarge | m5.large | m5.metal | m5.xlarge | m5a.12xlarge | m5a.16xlarge | m5a.24xlarge | m5a.2xlarge | m5a.4xlarge | m5a.8xlarge | m5a.large | m5a.xlarge | m5ad.12xlarge | m5ad.16xlarge | m5ad.24xlarge | m5ad.2xlarge | m5ad.4xlarge | m5ad.8xlarge | m5ad.large | m5ad.xlarge | m5d.12xlarge | m5d.16xlarge | m5d.24xlarge | m5d.2xlarge | m5d.4xlarge | m5d.8xlarge | m5d.large | m5d.metal | m5d.xlarge | m5dn.12xlarge | m5dn.16xlarge | m5dn.24xlarge | m5dn.2xlarge | m5dn.4xlarge | m5dn.8xlarge | m5dn.large | m5dn.metal | m5dn.xlarge | m5n.12xlarge | m5n.16xlarge | m5n.24xlarge | m5n.2xlarge | m5n.4xlarge | m5n.8xlarge | m5n.large | m5n.metal | m5n.xlarge | m5zn.12xlarge | m5zn.2xlarge | m5zn.3xlarge | m5zn.6xlarge | m5zn.large | m5zn.metal | m5zn.xlarge | m6a.12xlarge | m6a.16xlarge | m6a.24xlarge | m6a.2xlarge | m6a.32xlarge | m6a.48xlarge | m6a.4xlarge | m6a.8xlarge | m6a.large | m6a.metal | m6a.xlarge | m6g.12xlarge | m6g.16xlarge | m6g.2xlarge | m6g.4xlarge | m6g.8xlarge | m6g.large | m6g.medium | m6g.metal | m6g.xlarge | m6gd.12xlarge | m6gd.16xlarge | m6gd.2xlarge | m6gd.4xlarge | m6gd.8xlarge | m6gd.large | m6gd.medium | m6gd.metal | m6gd.xlarge | m6i.12xlarge | m6i.16xlarge | m6i.24xlarge | m6i.2xlarge | m6i.32xlarge | m6i.4xlarge | m6i.8xlarge | m6i.large | m6i.metal | m6i.xlarge | m6id.12xlarge | m6id.16xlarge | m6id.24xlarge | m6id.2xlarge | m6id.32xlarge | m6id.4xlarge | m6id.8xlarge | m6id.large | m6id.metal | m6id.xlarge | m6idn.12xlarge | m6idn.16xlarge | m6idn.24xlarge | m6idn.2xlarge | m6idn.32xlarge | m6idn.4xlarge | m6idn.8xlarge | m6idn.large | m6idn.metal | m6idn.xlarge | m6in.12xlarge | m6in.16xlarge | m6in.24xlarge | m6in.2xlarge | m6in.32xlarge | m6in.4xlarge | m6in.8xlarge | m6in.large | m6in.metal | m6in.xlarge | mac1.metal | mac2.metal | p2.16xlarge | p2.8xlarge | p2.xlarge | p3.16xlarge | p3.2xlarge | p3.8xlarge | p3dn.24xlarge | p4d.24xlarge | p4de.24xlarge | r3.2xlarge | r3.4xlarge | r3.8xlarge | r3.large | r3.xlarge | r4.16xlarge | r4.2xlarge | r4.4xlarge | r4.8xlarge | r4.large | r4.xlarge | r5.12xlarge | r5.16xlarge | r5.24xlarge | r5.2xlarge | r5.4xlarge | r5.8xlarge | r5.large | r5.metal | r5.xlarge | r5a.12xlarge | r5a.16xlarge | r5a.24xlarge | r5a.2xlarge | r5a.4xlarge | r5a.8xlarge | r5a.large | r5a.xlarge | r5ad.12xlarge | r5ad.16xlarge | r5ad.24xlarge | r5ad.2xlarge | r5ad.4xlarge | r5ad.8xlarge | r5ad.large | r5ad.xlarge | r5b.12xlarge | r5b.16xlarge | r5b.24xlarge | r5b.2xlarge | r5b.4xlarge | r5b.8xlarge | r5b.large | r5b.metal | r5b.xlarge | r5d.12xlarge | r5d.16xlarge | r5d.24xlarge | r5d.2xlarge | r5d.4xlarge | r5d.8xlarge | r5d.large | r5d.metal | r5d.xlarge | r5dn.12xlarge | r5dn.16xlarge | r5dn.24xlarge | r5dn.2xlarge | r5dn.4xlarge | r5dn.8xlarge | r5dn.large | r5dn.metal | r5dn.xlarge | r5n.12xlarge | r5n.16xlarge | r5n.24xlarge | r5n.2xlarge | r5n.4xlarge | r5n.8xlarge | r5n.large | r5n.metal | r5n.xlarge | r6a.12xlarge | r6a.16xlarge | r6a.24xlarge | r6a.2xlarge | r6a.32xlarge | r6a.48xlarge | r6a.4xlarge | r6a.8xlarge | r6a.large | r6a.metal | r6a.xlarge | r6g.12xlarge | r6g.16xlarge | r6g.2xlarge | r6g.4xlarge | r6g.8xlarge | r6g.large | r6g.medium | r6g.metal | r6g.xlarge | r6gd.12xlarge | r6gd.16xlarge | r6gd.2xlarge | r6gd.4xlarge | r6gd.8xlarge | r6gd.large | r6gd.medium | r6gd.metal | r6gd.xlarge | r6i.12xlarge | r6i.16xlarge | r6i.24xlarge | r6i.2xlarge | r6i.32xlarge | r6i.4xlarge | r6i.8xlarge | r6i.large | r6i.metal | r6i.xlarge | r6id.12xlarge | r6id.16xlarge | r6id.24xlarge | r6id.2xlarge | r6id.32xlarge | r6id.4xlarge | r6id.8xlarge | r6id.large | r6id.metal | r6id.xlarge | r6idn.12xlarge | r6idn.16xlarge | r6idn.24xlarge | r6idn.2xlarge | r6idn.32xlarge | r6idn.4xlarge | r6idn.8xlarge | r6idn.large | r6idn.metal | r6idn.xlarge | r6in.12xlarge | r6in.16xlarge | r6in.24xlarge | r6in.2xlarge | r6in.32xlarge | r6in.4xlarge | r6in.8xlarge | r6in.large | r6in.metal | r6in.xlarge | t1.micro | t2.2xlarge | t2.large | t2.medium | t2.micro | t2.nano | t2.small | t2.xlarge | t3.2xlarge | t3.large | t3.medium | t3.micro | t3.nano | t3.small | t3.xlarge | t3a.2xlarge | t3a.large | t3a.medium | t3a.micro | t3a.nano | t3a.small | t3a.xlarge | t4g.2xlarge | t4g.large | t4g.medium | t4g.micro | t4g.nano | t4g.small | t4g.xlarge | trn1.2xlarge | trn1.32xlarge | trn1n.32xlarge | u-12tb1.112xlarge | u-12tb1.metal | u-18tb1.112xlarge | u-18tb1.metal | u-24tb1.112xlarge | u-24tb1.metal | u-3tb1.56xlarge | u-6tb1.112xlarge | u-6tb1.56xlarge | u-6tb1.metal | u-9tb1.112xlarge | u-9tb1.metal | vt1.24xlarge | vt1.3xlarge | vt1.6xlarge | x1.16xlarge | x1.32xlarge | x1e.16xlarge | x1e.2xlarge | x1e.32xlarge | x1e.4xlarge | x1e.8xlarge | x1e.xlarge | x2gd.12xlarge | x2gd.16xlarge | x2gd.2xlarge | x2gd.4xlarge | x2gd.8xlarge | x2gd.large | x2gd.medium | x2gd.metal | x2gd.xlarge | x2idn.16xlarge | x2idn.24xlarge | x2idn.32xlarge | x2idn.metal | x2iedn.16xlarge | x2iedn.24xlarge | x2iedn.2xlarge | x2iedn.32xlarge | x2iedn.4xlarge | x2iedn.8xlarge | x2iedn.metal | x2iedn.xlarge | x2iezn.12xlarge | x2iezn.2xlarge | x2iezn.4xlarge | x2iezn.6xlarge | x2iezn.8xlarge | x2iezn.metal | z1d.12xlarge | z1d.2xlarge | z1d.3xlarge | z1d.6xlarge | z1d.large | z1d.metal | z1d.xlarge
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<InstanceTypeSpecificationInstanceTypeEnum>,

    ///
    /// The priority to assign to the instance type. This value is used to determine which of the instance types 			specified for the Fleet should be prioritized for use. A lower value indicates a high priority. For more 			information, see Instance type priority 			in the Amazon EC2 User Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: Replacement
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,

    ///
    /// The number of capacity units provided by the specified instance type. This value,     together with the total target capacity that you specify for the Fleet determine the number     of instances for which the Fleet reserves capacity. Both values are based on units that     make sense for your workload. For more information, see Total target       capacity in the Amazon EC2 User Guide.
    ///
    /// Valid Range: Minimum value of 0.001. Maximum value of     99.999.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum InstanceTypeSpecificationInstancePlatformEnum {
    /// Linux with SQL Server Enterprise
    #[serde(rename = "Linux with SQL Server Enterprise")]
    Linuxwithsqlserverenterprise,

    /// Linux with SQL Server Standard
    #[serde(rename = "Linux with SQL Server Standard")]
    Linuxwithsqlserverstandard,

    /// Linux with SQL Server Web
    #[serde(rename = "Linux with SQL Server Web")]
    Linuxwithsqlserverweb,

    /// Linux/UNIX
    #[serde(rename = "Linux/UNIX")]
    Linuxunix,

    /// Red Hat Enterprise Linux
    #[serde(rename = "Red Hat Enterprise Linux")]
    Redhatenterpriselinux,

    /// RHEL with HA
    #[serde(rename = "RHEL with HA")]
    Rhelwithha,

    /// RHEL with HA and SQL Server Enterprise
    #[serde(rename = "RHEL with HA and SQL Server Enterprise")]
    Rhelwithhaandsqlserverenterprise,

    /// RHEL with HA and SQL Server Standard
    #[serde(rename = "RHEL with HA and SQL Server Standard")]
    Rhelwithhaandsqlserverstandard,

    /// RHEL with SQL Server Enterprise
    #[serde(rename = "RHEL with SQL Server Enterprise")]
    Rhelwithsqlserverenterprise,

    /// RHEL with SQL Server Standard
    #[serde(rename = "RHEL with SQL Server Standard")]
    Rhelwithsqlserverstandard,

    /// RHEL with SQL Server Web
    #[serde(rename = "RHEL with SQL Server Web")]
    Rhelwithsqlserverweb,

    /// SUSE Linux
    #[serde(rename = "SUSE Linux")]
    Suselinux,

    /// Windows
    #[serde(rename = "Windows")]
    Windows,

    /// Windows with SQL Server
    #[serde(rename = "Windows with SQL Server")]
    Windowswithsqlserver,

    /// Windows with SQL Server Enterprise
    #[serde(rename = "Windows with SQL Server Enterprise")]
    Windowswithsqlserverenterprise,

    /// Windows with SQL Server Standard
    #[serde(rename = "Windows with SQL Server Standard")]
    Windowswithsqlserverstandard,

    /// Windows with SQL Server Web
    #[serde(rename = "Windows with SQL Server Web")]
    Windowswithsqlserverweb,
}

impl Default for InstanceTypeSpecificationInstancePlatformEnum {
    fn default() -> Self {
        InstanceTypeSpecificationInstancePlatformEnum::Linuxwithsqlserverenterprise
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum InstanceTypeSpecificationInstanceTypeEnum {
    /// a1.2xlarge
    #[serde(rename = "a1.2xlarge")]
    A12xlarge,

    /// a1.4xlarge
    #[serde(rename = "a1.4xlarge")]
    A14xlarge,

    /// a1.large
    #[serde(rename = "a1.large")]
    A1large,

    /// a1.medium
    #[serde(rename = "a1.medium")]
    A1medium,

    /// a1.metal
    #[serde(rename = "a1.metal")]
    A1metal,

    /// a1.xlarge
    #[serde(rename = "a1.xlarge")]
    A1xlarge,

    /// c1.medium
    #[serde(rename = "c1.medium")]
    C1medium,

    /// c1.xlarge
    #[serde(rename = "c1.xlarge")]
    C1xlarge,

    /// c3.2xlarge
    #[serde(rename = "c3.2xlarge")]
    C32xlarge,

    /// c3.4xlarge
    #[serde(rename = "c3.4xlarge")]
    C34xlarge,

    /// c3.8xlarge
    #[serde(rename = "c3.8xlarge")]
    C38xlarge,

    /// c3.large
    #[serde(rename = "c3.large")]
    C3large,

    /// c3.xlarge
    #[serde(rename = "c3.xlarge")]
    C3xlarge,

    /// c4.2xlarge
    #[serde(rename = "c4.2xlarge")]
    C42xlarge,

    /// c4.4xlarge
    #[serde(rename = "c4.4xlarge")]
    C44xlarge,

    /// c4.8xlarge
    #[serde(rename = "c4.8xlarge")]
    C48xlarge,

    /// c4.large
    #[serde(rename = "c4.large")]
    C4large,

    /// c4.xlarge
    #[serde(rename = "c4.xlarge")]
    C4xlarge,

    /// c5.12xlarge
    #[serde(rename = "c5.12xlarge")]
    C512xlarge,

    /// c5.18xlarge
    #[serde(rename = "c5.18xlarge")]
    C518xlarge,

    /// c5.24xlarge
    #[serde(rename = "c5.24xlarge")]
    C524xlarge,

    /// c5.2xlarge
    #[serde(rename = "c5.2xlarge")]
    C52xlarge,

    /// c5.4xlarge
    #[serde(rename = "c5.4xlarge")]
    C54xlarge,

    /// c5.9xlarge
    #[serde(rename = "c5.9xlarge")]
    C59xlarge,

    /// c5.large
    #[serde(rename = "c5.large")]
    C5large,

    /// c5.metal
    #[serde(rename = "c5.metal")]
    C5metal,

    /// c5.xlarge
    #[serde(rename = "c5.xlarge")]
    C5xlarge,

    /// c5a.12xlarge
    #[serde(rename = "c5a.12xlarge")]
    C5a12xlarge,

    /// c5a.16xlarge
    #[serde(rename = "c5a.16xlarge")]
    C5a16xlarge,

    /// c5a.24xlarge
    #[serde(rename = "c5a.24xlarge")]
    C5a24xlarge,

    /// c5a.2xlarge
    #[serde(rename = "c5a.2xlarge")]
    C5a2xlarge,

    /// c5a.4xlarge
    #[serde(rename = "c5a.4xlarge")]
    C5a4xlarge,

    /// c5a.8xlarge
    #[serde(rename = "c5a.8xlarge")]
    C5a8xlarge,

    /// c5a.large
    #[serde(rename = "c5a.large")]
    C5alarge,

    /// c5a.xlarge
    #[serde(rename = "c5a.xlarge")]
    C5axlarge,

    /// c5ad.12xlarge
    #[serde(rename = "c5ad.12xlarge")]
    C5ad12xlarge,

    /// c5ad.16xlarge
    #[serde(rename = "c5ad.16xlarge")]
    C5ad16xlarge,

    /// c5ad.24xlarge
    #[serde(rename = "c5ad.24xlarge")]
    C5ad24xlarge,

    /// c5ad.2xlarge
    #[serde(rename = "c5ad.2xlarge")]
    C5ad2xlarge,

    /// c5ad.4xlarge
    #[serde(rename = "c5ad.4xlarge")]
    C5ad4xlarge,

    /// c5ad.8xlarge
    #[serde(rename = "c5ad.8xlarge")]
    C5ad8xlarge,

    /// c5ad.large
    #[serde(rename = "c5ad.large")]
    C5adlarge,

    /// c5ad.xlarge
    #[serde(rename = "c5ad.xlarge")]
    C5adxlarge,

    /// c5d.12xlarge
    #[serde(rename = "c5d.12xlarge")]
    C5d12xlarge,

    /// c5d.18xlarge
    #[serde(rename = "c5d.18xlarge")]
    C5d18xlarge,

    /// c5d.24xlarge
    #[serde(rename = "c5d.24xlarge")]
    C5d24xlarge,

    /// c5d.2xlarge
    #[serde(rename = "c5d.2xlarge")]
    C5d2xlarge,

    /// c5d.4xlarge
    #[serde(rename = "c5d.4xlarge")]
    C5d4xlarge,

    /// c5d.9xlarge
    #[serde(rename = "c5d.9xlarge")]
    C5d9xlarge,

    /// c5d.large
    #[serde(rename = "c5d.large")]
    C5dlarge,

    /// c5d.metal
    #[serde(rename = "c5d.metal")]
    C5dmetal,

    /// c5d.xlarge
    #[serde(rename = "c5d.xlarge")]
    C5dxlarge,

    /// c5n.18xlarge
    #[serde(rename = "c5n.18xlarge")]
    C5n18xlarge,

    /// c5n.2xlarge
    #[serde(rename = "c5n.2xlarge")]
    C5n2xlarge,

    /// c5n.4xlarge
    #[serde(rename = "c5n.4xlarge")]
    C5n4xlarge,

    /// c5n.9xlarge
    #[serde(rename = "c5n.9xlarge")]
    C5n9xlarge,

    /// c5n.large
    #[serde(rename = "c5n.large")]
    C5nlarge,

    /// c5n.metal
    #[serde(rename = "c5n.metal")]
    C5nmetal,

    /// c5n.xlarge
    #[serde(rename = "c5n.xlarge")]
    C5nxlarge,

    /// c6a.12xlarge
    #[serde(rename = "c6a.12xlarge")]
    C6a12xlarge,

    /// c6a.16xlarge
    #[serde(rename = "c6a.16xlarge")]
    C6a16xlarge,

    /// c6a.24xlarge
    #[serde(rename = "c6a.24xlarge")]
    C6a24xlarge,

    /// c6a.2xlarge
    #[serde(rename = "c6a.2xlarge")]
    C6a2xlarge,

    /// c6a.32xlarge
    #[serde(rename = "c6a.32xlarge")]
    C6a32xlarge,

    /// c6a.48xlarge
    #[serde(rename = "c6a.48xlarge")]
    C6a48xlarge,

    /// c6a.4xlarge
    #[serde(rename = "c6a.4xlarge")]
    C6a4xlarge,

    /// c6a.8xlarge
    #[serde(rename = "c6a.8xlarge")]
    C6a8xlarge,

    /// c6a.large
    #[serde(rename = "c6a.large")]
    C6alarge,

    /// c6a.metal
    #[serde(rename = "c6a.metal")]
    C6ametal,

    /// c6a.xlarge
    #[serde(rename = "c6a.xlarge")]
    C6axlarge,

    /// c6g.12xlarge
    #[serde(rename = "c6g.12xlarge")]
    C6g12xlarge,

    /// c6g.16xlarge
    #[serde(rename = "c6g.16xlarge")]
    C6g16xlarge,

    /// c6g.2xlarge
    #[serde(rename = "c6g.2xlarge")]
    C6g2xlarge,

    /// c6g.4xlarge
    #[serde(rename = "c6g.4xlarge")]
    C6g4xlarge,

    /// c6g.8xlarge
    #[serde(rename = "c6g.8xlarge")]
    C6g8xlarge,

    /// c6g.large
    #[serde(rename = "c6g.large")]
    C6glarge,

    /// c6g.medium
    #[serde(rename = "c6g.medium")]
    C6gmedium,

    /// c6g.metal
    #[serde(rename = "c6g.metal")]
    C6gmetal,

    /// c6g.xlarge
    #[serde(rename = "c6g.xlarge")]
    C6gxlarge,

    /// c6gd.12xlarge
    #[serde(rename = "c6gd.12xlarge")]
    C6gd12xlarge,

    /// c6gd.16xlarge
    #[serde(rename = "c6gd.16xlarge")]
    C6gd16xlarge,

    /// c6gd.2xlarge
    #[serde(rename = "c6gd.2xlarge")]
    C6gd2xlarge,

    /// c6gd.4xlarge
    #[serde(rename = "c6gd.4xlarge")]
    C6gd4xlarge,

    /// c6gd.8xlarge
    #[serde(rename = "c6gd.8xlarge")]
    C6gd8xlarge,

    /// c6gd.large
    #[serde(rename = "c6gd.large")]
    C6gdlarge,

    /// c6gd.medium
    #[serde(rename = "c6gd.medium")]
    C6gdmedium,

    /// c6gd.metal
    #[serde(rename = "c6gd.metal")]
    C6gdmetal,

    /// c6gd.xlarge
    #[serde(rename = "c6gd.xlarge")]
    C6gdxlarge,

    /// c6gn.12xlarge
    #[serde(rename = "c6gn.12xlarge")]
    C6gn12xlarge,

    /// c6gn.16xlarge
    #[serde(rename = "c6gn.16xlarge")]
    C6gn16xlarge,

    /// c6gn.2xlarge
    #[serde(rename = "c6gn.2xlarge")]
    C6gn2xlarge,

    /// c6gn.4xlarge
    #[serde(rename = "c6gn.4xlarge")]
    C6gn4xlarge,

    /// c6gn.8xlarge
    #[serde(rename = "c6gn.8xlarge")]
    C6gn8xlarge,

    /// c6gn.large
    #[serde(rename = "c6gn.large")]
    C6gnlarge,

    /// c6gn.medium
    #[serde(rename = "c6gn.medium")]
    C6gnmedium,

    /// c6gn.xlarge
    #[serde(rename = "c6gn.xlarge")]
    C6gnxlarge,

    /// c6i.12xlarge
    #[serde(rename = "c6i.12xlarge")]
    C6i12xlarge,

    /// c6i.16xlarge
    #[serde(rename = "c6i.16xlarge")]
    C6i16xlarge,

    /// c6i.24xlarge
    #[serde(rename = "c6i.24xlarge")]
    C6i24xlarge,

    /// c6i.2xlarge
    #[serde(rename = "c6i.2xlarge")]
    C6i2xlarge,

    /// c6i.32xlarge
    #[serde(rename = "c6i.32xlarge")]
    C6i32xlarge,

    /// c6i.4xlarge
    #[serde(rename = "c6i.4xlarge")]
    C6i4xlarge,

    /// c6i.8xlarge
    #[serde(rename = "c6i.8xlarge")]
    C6i8xlarge,

    /// c6i.large
    #[serde(rename = "c6i.large")]
    C6ilarge,

    /// c6i.metal
    #[serde(rename = "c6i.metal")]
    C6imetal,

    /// c6i.xlarge
    #[serde(rename = "c6i.xlarge")]
    C6ixlarge,

    /// c6id.12xlarge
    #[serde(rename = "c6id.12xlarge")]
    C6id12xlarge,

    /// c6id.16xlarge
    #[serde(rename = "c6id.16xlarge")]
    C6id16xlarge,

    /// c6id.24xlarge
    #[serde(rename = "c6id.24xlarge")]
    C6id24xlarge,

    /// c6id.2xlarge
    #[serde(rename = "c6id.2xlarge")]
    C6id2xlarge,

    /// c6id.32xlarge
    #[serde(rename = "c6id.32xlarge")]
    C6id32xlarge,

    /// c6id.4xlarge
    #[serde(rename = "c6id.4xlarge")]
    C6id4xlarge,

    /// c6id.8xlarge
    #[serde(rename = "c6id.8xlarge")]
    C6id8xlarge,

    /// c6id.large
    #[serde(rename = "c6id.large")]
    C6idlarge,

    /// c6id.metal
    #[serde(rename = "c6id.metal")]
    C6idmetal,

    /// c6id.xlarge
    #[serde(rename = "c6id.xlarge")]
    C6idxlarge,

    /// c6in.12xlarge
    #[serde(rename = "c6in.12xlarge")]
    C6in12xlarge,

    /// c6in.16xlarge
    #[serde(rename = "c6in.16xlarge")]
    C6in16xlarge,

    /// c6in.24xlarge
    #[serde(rename = "c6in.24xlarge")]
    C6in24xlarge,

    /// c6in.2xlarge
    #[serde(rename = "c6in.2xlarge")]
    C6in2xlarge,

    /// c6in.32xlarge
    #[serde(rename = "c6in.32xlarge")]
    C6in32xlarge,

    /// c6in.4xlarge
    #[serde(rename = "c6in.4xlarge")]
    C6in4xlarge,

    /// c6in.8xlarge
    #[serde(rename = "c6in.8xlarge")]
    C6in8xlarge,

    /// c6in.large
    #[serde(rename = "c6in.large")]
    C6inlarge,

    /// c6in.metal
    #[serde(rename = "c6in.metal")]
    C6inmetal,

    /// c6in.xlarge
    #[serde(rename = "c6in.xlarge")]
    C6inxlarge,

    /// c7g.12xlarge
    #[serde(rename = "c7g.12xlarge")]
    C7g12xlarge,

    /// c7g.16xlarge
    #[serde(rename = "c7g.16xlarge")]
    C7g16xlarge,

    /// c7g.2xlarge
    #[serde(rename = "c7g.2xlarge")]
    C7g2xlarge,

    /// c7g.4xlarge
    #[serde(rename = "c7g.4xlarge")]
    C7g4xlarge,

    /// c7g.8xlarge
    #[serde(rename = "c7g.8xlarge")]
    C7g8xlarge,

    /// c7g.large
    #[serde(rename = "c7g.large")]
    C7glarge,

    /// c7g.medium
    #[serde(rename = "c7g.medium")]
    C7gmedium,

    /// c7g.xlarge
    #[serde(rename = "c7g.xlarge")]
    C7gxlarge,

    /// cc1.4xlarge
    #[serde(rename = "cc1.4xlarge")]
    Cc14xlarge,

    /// cc2.8xlarge
    #[serde(rename = "cc2.8xlarge")]
    Cc28xlarge,

    /// cg1.4xlarge
    #[serde(rename = "cg1.4xlarge")]
    Cg14xlarge,

    /// cr1.8xlarge
    #[serde(rename = "cr1.8xlarge")]
    Cr18xlarge,

    /// d2.2xlarge
    #[serde(rename = "d2.2xlarge")]
    D22xlarge,

    /// d2.4xlarge
    #[serde(rename = "d2.4xlarge")]
    D24xlarge,

    /// d2.8xlarge
    #[serde(rename = "d2.8xlarge")]
    D28xlarge,

    /// d2.xlarge
    #[serde(rename = "d2.xlarge")]
    D2xlarge,

    /// d3.2xlarge
    #[serde(rename = "d3.2xlarge")]
    D32xlarge,

    /// d3.4xlarge
    #[serde(rename = "d3.4xlarge")]
    D34xlarge,

    /// d3.8xlarge
    #[serde(rename = "d3.8xlarge")]
    D38xlarge,

    /// d3.xlarge
    #[serde(rename = "d3.xlarge")]
    D3xlarge,

    /// d3en.12xlarge
    #[serde(rename = "d3en.12xlarge")]
    D3en12xlarge,

    /// d3en.2xlarge
    #[serde(rename = "d3en.2xlarge")]
    D3en2xlarge,

    /// d3en.4xlarge
    #[serde(rename = "d3en.4xlarge")]
    D3en4xlarge,

    /// d3en.6xlarge
    #[serde(rename = "d3en.6xlarge")]
    D3en6xlarge,

    /// d3en.8xlarge
    #[serde(rename = "d3en.8xlarge")]
    D3en8xlarge,

    /// d3en.xlarge
    #[serde(rename = "d3en.xlarge")]
    D3enxlarge,

    /// dl1.24xlarge
    #[serde(rename = "dl1.24xlarge")]
    Dl124xlarge,

    /// f1.16xlarge
    #[serde(rename = "f1.16xlarge")]
    F116xlarge,

    /// f1.2xlarge
    #[serde(rename = "f1.2xlarge")]
    F12xlarge,

    /// f1.4xlarge
    #[serde(rename = "f1.4xlarge")]
    F14xlarge,

    /// g2.2xlarge
    #[serde(rename = "g2.2xlarge")]
    G22xlarge,

    /// g2.8xlarge
    #[serde(rename = "g2.8xlarge")]
    G28xlarge,

    /// g3.16xlarge
    #[serde(rename = "g3.16xlarge")]
    G316xlarge,

    /// g3.4xlarge
    #[serde(rename = "g3.4xlarge")]
    G34xlarge,

    /// g3.8xlarge
    #[serde(rename = "g3.8xlarge")]
    G38xlarge,

    /// g3s.xlarge
    #[serde(rename = "g3s.xlarge")]
    G3sxlarge,

    /// g4ad.16xlarge
    #[serde(rename = "g4ad.16xlarge")]
    G4ad16xlarge,

    /// g4ad.2xlarge
    #[serde(rename = "g4ad.2xlarge")]
    G4ad2xlarge,

    /// g4ad.4xlarge
    #[serde(rename = "g4ad.4xlarge")]
    G4ad4xlarge,

    /// g4ad.8xlarge
    #[serde(rename = "g4ad.8xlarge")]
    G4ad8xlarge,

    /// g4ad.xlarge
    #[serde(rename = "g4ad.xlarge")]
    G4adxlarge,

    /// g4dn.12xlarge
    #[serde(rename = "g4dn.12xlarge")]
    G4dn12xlarge,

    /// g4dn.16xlarge
    #[serde(rename = "g4dn.16xlarge")]
    G4dn16xlarge,

    /// g4dn.2xlarge
    #[serde(rename = "g4dn.2xlarge")]
    G4dn2xlarge,

    /// g4dn.4xlarge
    #[serde(rename = "g4dn.4xlarge")]
    G4dn4xlarge,

    /// g4dn.8xlarge
    #[serde(rename = "g4dn.8xlarge")]
    G4dn8xlarge,

    /// g4dn.metal
    #[serde(rename = "g4dn.metal")]
    G4dnmetal,

    /// g4dn.xlarge
    #[serde(rename = "g4dn.xlarge")]
    G4dnxlarge,

    /// g5.12xlarge
    #[serde(rename = "g5.12xlarge")]
    G512xlarge,

    /// g5.16xlarge
    #[serde(rename = "g5.16xlarge")]
    G516xlarge,

    /// g5.24xlarge
    #[serde(rename = "g5.24xlarge")]
    G524xlarge,

    /// g5.2xlarge
    #[serde(rename = "g5.2xlarge")]
    G52xlarge,

    /// g5.48xlarge
    #[serde(rename = "g5.48xlarge")]
    G548xlarge,

    /// g5.4xlarge
    #[serde(rename = "g5.4xlarge")]
    G54xlarge,

    /// g5.8xlarge
    #[serde(rename = "g5.8xlarge")]
    G58xlarge,

    /// g5.xlarge
    #[serde(rename = "g5.xlarge")]
    G5xlarge,

    /// g5g.16xlarge
    #[serde(rename = "g5g.16xlarge")]
    G5g16xlarge,

    /// g5g.2xlarge
    #[serde(rename = "g5g.2xlarge")]
    G5g2xlarge,

    /// g5g.4xlarge
    #[serde(rename = "g5g.4xlarge")]
    G5g4xlarge,

    /// g5g.8xlarge
    #[serde(rename = "g5g.8xlarge")]
    G5g8xlarge,

    /// g5g.metal
    #[serde(rename = "g5g.metal")]
    G5gmetal,

    /// g5g.xlarge
    #[serde(rename = "g5g.xlarge")]
    G5gxlarge,

    /// h1.16xlarge
    #[serde(rename = "h1.16xlarge")]
    H116xlarge,

    /// h1.2xlarge
    #[serde(rename = "h1.2xlarge")]
    H12xlarge,

    /// h1.4xlarge
    #[serde(rename = "h1.4xlarge")]
    H14xlarge,

    /// h1.8xlarge
    #[serde(rename = "h1.8xlarge")]
    H18xlarge,

    /// hi1.4xlarge
    #[serde(rename = "hi1.4xlarge")]
    Hi14xlarge,

    /// hpc6a.48xlarge
    #[serde(rename = "hpc6a.48xlarge")]
    Hpc6a48xlarge,

    /// hpc6id.32xlarge
    #[serde(rename = "hpc6id.32xlarge")]
    Hpc6id32xlarge,

    /// hs1.8xlarge
    #[serde(rename = "hs1.8xlarge")]
    Hs18xlarge,

    /// i2.2xlarge
    #[serde(rename = "i2.2xlarge")]
    I22xlarge,

    /// i2.4xlarge
    #[serde(rename = "i2.4xlarge")]
    I24xlarge,

    /// i2.8xlarge
    #[serde(rename = "i2.8xlarge")]
    I28xlarge,

    /// i2.xlarge
    #[serde(rename = "i2.xlarge")]
    I2xlarge,

    /// i3.16xlarge
    #[serde(rename = "i3.16xlarge")]
    I316xlarge,

    /// i3.2xlarge
    #[serde(rename = "i3.2xlarge")]
    I32xlarge,

    /// i3.4xlarge
    #[serde(rename = "i3.4xlarge")]
    I34xlarge,

    /// i3.8xlarge
    #[serde(rename = "i3.8xlarge")]
    I38xlarge,

    /// i3.large
    #[serde(rename = "i3.large")]
    I3large,

    /// i3.metal
    #[serde(rename = "i3.metal")]
    I3metal,

    /// i3.xlarge
    #[serde(rename = "i3.xlarge")]
    I3xlarge,

    /// i3en.12xlarge
    #[serde(rename = "i3en.12xlarge")]
    I3en12xlarge,

    /// i3en.24xlarge
    #[serde(rename = "i3en.24xlarge")]
    I3en24xlarge,

    /// i3en.2xlarge
    #[serde(rename = "i3en.2xlarge")]
    I3en2xlarge,

    /// i3en.3xlarge
    #[serde(rename = "i3en.3xlarge")]
    I3en3xlarge,

    /// i3en.6xlarge
    #[serde(rename = "i3en.6xlarge")]
    I3en6xlarge,

    /// i3en.large
    #[serde(rename = "i3en.large")]
    I3enlarge,

    /// i3en.metal
    #[serde(rename = "i3en.metal")]
    I3enmetal,

    /// i3en.xlarge
    #[serde(rename = "i3en.xlarge")]
    I3enxlarge,

    /// i4g.16xlarge
    #[serde(rename = "i4g.16xlarge")]
    I4g16xlarge,

    /// i4g.2xlarge
    #[serde(rename = "i4g.2xlarge")]
    I4g2xlarge,

    /// i4g.4xlarge
    #[serde(rename = "i4g.4xlarge")]
    I4g4xlarge,

    /// i4g.8xlarge
    #[serde(rename = "i4g.8xlarge")]
    I4g8xlarge,

    /// i4g.large
    #[serde(rename = "i4g.large")]
    I4glarge,

    /// i4g.xlarge
    #[serde(rename = "i4g.xlarge")]
    I4gxlarge,

    /// i4i.16xlarge
    #[serde(rename = "i4i.16xlarge")]
    I4i16xlarge,

    /// i4i.2xlarge
    #[serde(rename = "i4i.2xlarge")]
    I4i2xlarge,

    /// i4i.32xlarge
    #[serde(rename = "i4i.32xlarge")]
    I4i32xlarge,

    /// i4i.4xlarge
    #[serde(rename = "i4i.4xlarge")]
    I4i4xlarge,

    /// i4i.8xlarge
    #[serde(rename = "i4i.8xlarge")]
    I4i8xlarge,

    /// i4i.large
    #[serde(rename = "i4i.large")]
    I4ilarge,

    /// i4i.metal
    #[serde(rename = "i4i.metal")]
    I4imetal,

    /// i4i.xlarge
    #[serde(rename = "i4i.xlarge")]
    I4ixlarge,

    /// im4gn.16xlarge
    #[serde(rename = "im4gn.16xlarge")]
    Im4gn16xlarge,

    /// im4gn.2xlarge
    #[serde(rename = "im4gn.2xlarge")]
    Im4gn2xlarge,

    /// im4gn.4xlarge
    #[serde(rename = "im4gn.4xlarge")]
    Im4gn4xlarge,

    /// im4gn.8xlarge
    #[serde(rename = "im4gn.8xlarge")]
    Im4gn8xlarge,

    /// im4gn.large
    #[serde(rename = "im4gn.large")]
    Im4gnlarge,

    /// im4gn.xlarge
    #[serde(rename = "im4gn.xlarge")]
    Im4gnxlarge,

    /// inf1.24xlarge
    #[serde(rename = "inf1.24xlarge")]
    Inf124xlarge,

    /// inf1.2xlarge
    #[serde(rename = "inf1.2xlarge")]
    Inf12xlarge,

    /// inf1.6xlarge
    #[serde(rename = "inf1.6xlarge")]
    Inf16xlarge,

    /// inf1.xlarge
    #[serde(rename = "inf1.xlarge")]
    Inf1xlarge,

    /// inf2.24xlarge
    #[serde(rename = "inf2.24xlarge")]
    Inf224xlarge,

    /// inf2.48xlarge
    #[serde(rename = "inf2.48xlarge")]
    Inf248xlarge,

    /// inf2.8xlarge
    #[serde(rename = "inf2.8xlarge")]
    Inf28xlarge,

    /// inf2.xlarge
    #[serde(rename = "inf2.xlarge")]
    Inf2xlarge,

    /// is4gen.2xlarge
    #[serde(rename = "is4gen.2xlarge")]
    Is4gen2xlarge,

    /// is4gen.4xlarge
    #[serde(rename = "is4gen.4xlarge")]
    Is4gen4xlarge,

    /// is4gen.8xlarge
    #[serde(rename = "is4gen.8xlarge")]
    Is4gen8xlarge,

    /// is4gen.large
    #[serde(rename = "is4gen.large")]
    Is4genlarge,

    /// is4gen.medium
    #[serde(rename = "is4gen.medium")]
    Is4genmedium,

    /// is4gen.xlarge
    #[serde(rename = "is4gen.xlarge")]
    Is4genxlarge,

    /// m1.large
    #[serde(rename = "m1.large")]
    M1large,

    /// m1.medium
    #[serde(rename = "m1.medium")]
    M1medium,

    /// m1.small
    #[serde(rename = "m1.small")]
    M1small,

    /// m1.xlarge
    #[serde(rename = "m1.xlarge")]
    M1xlarge,

    /// m2.2xlarge
    #[serde(rename = "m2.2xlarge")]
    M22xlarge,

    /// m2.4xlarge
    #[serde(rename = "m2.4xlarge")]
    M24xlarge,

    /// m2.xlarge
    #[serde(rename = "m2.xlarge")]
    M2xlarge,

    /// m3.2xlarge
    #[serde(rename = "m3.2xlarge")]
    M32xlarge,

    /// m3.large
    #[serde(rename = "m3.large")]
    M3large,

    /// m3.medium
    #[serde(rename = "m3.medium")]
    M3medium,

    /// m3.xlarge
    #[serde(rename = "m3.xlarge")]
    M3xlarge,

    /// m4.10xlarge
    #[serde(rename = "m4.10xlarge")]
    M410xlarge,

    /// m4.16xlarge
    #[serde(rename = "m4.16xlarge")]
    M416xlarge,

    /// m4.2xlarge
    #[serde(rename = "m4.2xlarge")]
    M42xlarge,

    /// m4.4xlarge
    #[serde(rename = "m4.4xlarge")]
    M44xlarge,

    /// m4.large
    #[serde(rename = "m4.large")]
    M4large,

    /// m4.xlarge
    #[serde(rename = "m4.xlarge")]
    M4xlarge,

    /// m5.12xlarge
    #[serde(rename = "m5.12xlarge")]
    M512xlarge,

    /// m5.16xlarge
    #[serde(rename = "m5.16xlarge")]
    M516xlarge,

    /// m5.24xlarge
    #[serde(rename = "m5.24xlarge")]
    M524xlarge,

    /// m5.2xlarge
    #[serde(rename = "m5.2xlarge")]
    M52xlarge,

    /// m5.4xlarge
    #[serde(rename = "m5.4xlarge")]
    M54xlarge,

    /// m5.8xlarge
    #[serde(rename = "m5.8xlarge")]
    M58xlarge,

    /// m5.large
    #[serde(rename = "m5.large")]
    M5large,

    /// m5.metal
    #[serde(rename = "m5.metal")]
    M5metal,

    /// m5.xlarge
    #[serde(rename = "m5.xlarge")]
    M5xlarge,

    /// m5a.12xlarge
    #[serde(rename = "m5a.12xlarge")]
    M5a12xlarge,

    /// m5a.16xlarge
    #[serde(rename = "m5a.16xlarge")]
    M5a16xlarge,

    /// m5a.24xlarge
    #[serde(rename = "m5a.24xlarge")]
    M5a24xlarge,

    /// m5a.2xlarge
    #[serde(rename = "m5a.2xlarge")]
    M5a2xlarge,

    /// m5a.4xlarge
    #[serde(rename = "m5a.4xlarge")]
    M5a4xlarge,

    /// m5a.8xlarge
    #[serde(rename = "m5a.8xlarge")]
    M5a8xlarge,

    /// m5a.large
    #[serde(rename = "m5a.large")]
    M5alarge,

    /// m5a.xlarge
    #[serde(rename = "m5a.xlarge")]
    M5axlarge,

    /// m5ad.12xlarge
    #[serde(rename = "m5ad.12xlarge")]
    M5ad12xlarge,

    /// m5ad.16xlarge
    #[serde(rename = "m5ad.16xlarge")]
    M5ad16xlarge,

    /// m5ad.24xlarge
    #[serde(rename = "m5ad.24xlarge")]
    M5ad24xlarge,

    /// m5ad.2xlarge
    #[serde(rename = "m5ad.2xlarge")]
    M5ad2xlarge,

    /// m5ad.4xlarge
    #[serde(rename = "m5ad.4xlarge")]
    M5ad4xlarge,

    /// m5ad.8xlarge
    #[serde(rename = "m5ad.8xlarge")]
    M5ad8xlarge,

    /// m5ad.large
    #[serde(rename = "m5ad.large")]
    M5adlarge,

    /// m5ad.xlarge
    #[serde(rename = "m5ad.xlarge")]
    M5adxlarge,

    /// m5d.12xlarge
    #[serde(rename = "m5d.12xlarge")]
    M5d12xlarge,

    /// m5d.16xlarge
    #[serde(rename = "m5d.16xlarge")]
    M5d16xlarge,

    /// m5d.24xlarge
    #[serde(rename = "m5d.24xlarge")]
    M5d24xlarge,

    /// m5d.2xlarge
    #[serde(rename = "m5d.2xlarge")]
    M5d2xlarge,

    /// m5d.4xlarge
    #[serde(rename = "m5d.4xlarge")]
    M5d4xlarge,

    /// m5d.8xlarge
    #[serde(rename = "m5d.8xlarge")]
    M5d8xlarge,

    /// m5d.large
    #[serde(rename = "m5d.large")]
    M5dlarge,

    /// m5d.metal
    #[serde(rename = "m5d.metal")]
    M5dmetal,

    /// m5d.xlarge
    #[serde(rename = "m5d.xlarge")]
    M5dxlarge,

    /// m5dn.12xlarge
    #[serde(rename = "m5dn.12xlarge")]
    M5dn12xlarge,

    /// m5dn.16xlarge
    #[serde(rename = "m5dn.16xlarge")]
    M5dn16xlarge,

    /// m5dn.24xlarge
    #[serde(rename = "m5dn.24xlarge")]
    M5dn24xlarge,

    /// m5dn.2xlarge
    #[serde(rename = "m5dn.2xlarge")]
    M5dn2xlarge,

    /// m5dn.4xlarge
    #[serde(rename = "m5dn.4xlarge")]
    M5dn4xlarge,

    /// m5dn.8xlarge
    #[serde(rename = "m5dn.8xlarge")]
    M5dn8xlarge,

    /// m5dn.large
    #[serde(rename = "m5dn.large")]
    M5dnlarge,

    /// m5dn.metal
    #[serde(rename = "m5dn.metal")]
    M5dnmetal,

    /// m5dn.xlarge
    #[serde(rename = "m5dn.xlarge")]
    M5dnxlarge,

    /// m5n.12xlarge
    #[serde(rename = "m5n.12xlarge")]
    M5n12xlarge,

    /// m5n.16xlarge
    #[serde(rename = "m5n.16xlarge")]
    M5n16xlarge,

    /// m5n.24xlarge
    #[serde(rename = "m5n.24xlarge")]
    M5n24xlarge,

    /// m5n.2xlarge
    #[serde(rename = "m5n.2xlarge")]
    M5n2xlarge,

    /// m5n.4xlarge
    #[serde(rename = "m5n.4xlarge")]
    M5n4xlarge,

    /// m5n.8xlarge
    #[serde(rename = "m5n.8xlarge")]
    M5n8xlarge,

    /// m5n.large
    #[serde(rename = "m5n.large")]
    M5nlarge,

    /// m5n.metal
    #[serde(rename = "m5n.metal")]
    M5nmetal,

    /// m5n.xlarge
    #[serde(rename = "m5n.xlarge")]
    M5nxlarge,

    /// m5zn.12xlarge
    #[serde(rename = "m5zn.12xlarge")]
    M5zn12xlarge,

    /// m5zn.2xlarge
    #[serde(rename = "m5zn.2xlarge")]
    M5zn2xlarge,

    /// m5zn.3xlarge
    #[serde(rename = "m5zn.3xlarge")]
    M5zn3xlarge,

    /// m5zn.6xlarge
    #[serde(rename = "m5zn.6xlarge")]
    M5zn6xlarge,

    /// m5zn.large
    #[serde(rename = "m5zn.large")]
    M5znlarge,

    /// m5zn.metal
    #[serde(rename = "m5zn.metal")]
    M5znmetal,

    /// m5zn.xlarge
    #[serde(rename = "m5zn.xlarge")]
    M5znxlarge,

    /// m6a.12xlarge
    #[serde(rename = "m6a.12xlarge")]
    M6a12xlarge,

    /// m6a.16xlarge
    #[serde(rename = "m6a.16xlarge")]
    M6a16xlarge,

    /// m6a.24xlarge
    #[serde(rename = "m6a.24xlarge")]
    M6a24xlarge,

    /// m6a.2xlarge
    #[serde(rename = "m6a.2xlarge")]
    M6a2xlarge,

    /// m6a.32xlarge
    #[serde(rename = "m6a.32xlarge")]
    M6a32xlarge,

    /// m6a.48xlarge
    #[serde(rename = "m6a.48xlarge")]
    M6a48xlarge,

    /// m6a.4xlarge
    #[serde(rename = "m6a.4xlarge")]
    M6a4xlarge,

    /// m6a.8xlarge
    #[serde(rename = "m6a.8xlarge")]
    M6a8xlarge,

    /// m6a.large
    #[serde(rename = "m6a.large")]
    M6alarge,

    /// m6a.metal
    #[serde(rename = "m6a.metal")]
    M6ametal,

    /// m6a.xlarge
    #[serde(rename = "m6a.xlarge")]
    M6axlarge,

    /// m6g.12xlarge
    #[serde(rename = "m6g.12xlarge")]
    M6g12xlarge,

    /// m6g.16xlarge
    #[serde(rename = "m6g.16xlarge")]
    M6g16xlarge,

    /// m6g.2xlarge
    #[serde(rename = "m6g.2xlarge")]
    M6g2xlarge,

    /// m6g.4xlarge
    #[serde(rename = "m6g.4xlarge")]
    M6g4xlarge,

    /// m6g.8xlarge
    #[serde(rename = "m6g.8xlarge")]
    M6g8xlarge,

    /// m6g.large
    #[serde(rename = "m6g.large")]
    M6glarge,

    /// m6g.medium
    #[serde(rename = "m6g.medium")]
    M6gmedium,

    /// m6g.metal
    #[serde(rename = "m6g.metal")]
    M6gmetal,

    /// m6g.xlarge
    #[serde(rename = "m6g.xlarge")]
    M6gxlarge,

    /// m6gd.12xlarge
    #[serde(rename = "m6gd.12xlarge")]
    M6gd12xlarge,

    /// m6gd.16xlarge
    #[serde(rename = "m6gd.16xlarge")]
    M6gd16xlarge,

    /// m6gd.2xlarge
    #[serde(rename = "m6gd.2xlarge")]
    M6gd2xlarge,

    /// m6gd.4xlarge
    #[serde(rename = "m6gd.4xlarge")]
    M6gd4xlarge,

    /// m6gd.8xlarge
    #[serde(rename = "m6gd.8xlarge")]
    M6gd8xlarge,

    /// m6gd.large
    #[serde(rename = "m6gd.large")]
    M6gdlarge,

    /// m6gd.medium
    #[serde(rename = "m6gd.medium")]
    M6gdmedium,

    /// m6gd.metal
    #[serde(rename = "m6gd.metal")]
    M6gdmetal,

    /// m6gd.xlarge
    #[serde(rename = "m6gd.xlarge")]
    M6gdxlarge,

    /// m6i.12xlarge
    #[serde(rename = "m6i.12xlarge")]
    M6i12xlarge,

    /// m6i.16xlarge
    #[serde(rename = "m6i.16xlarge")]
    M6i16xlarge,

    /// m6i.24xlarge
    #[serde(rename = "m6i.24xlarge")]
    M6i24xlarge,

    /// m6i.2xlarge
    #[serde(rename = "m6i.2xlarge")]
    M6i2xlarge,

    /// m6i.32xlarge
    #[serde(rename = "m6i.32xlarge")]
    M6i32xlarge,

    /// m6i.4xlarge
    #[serde(rename = "m6i.4xlarge")]
    M6i4xlarge,

    /// m6i.8xlarge
    #[serde(rename = "m6i.8xlarge")]
    M6i8xlarge,

    /// m6i.large
    #[serde(rename = "m6i.large")]
    M6ilarge,

    /// m6i.metal
    #[serde(rename = "m6i.metal")]
    M6imetal,

    /// m6i.xlarge
    #[serde(rename = "m6i.xlarge")]
    M6ixlarge,

    /// m6id.12xlarge
    #[serde(rename = "m6id.12xlarge")]
    M6id12xlarge,

    /// m6id.16xlarge
    #[serde(rename = "m6id.16xlarge")]
    M6id16xlarge,

    /// m6id.24xlarge
    #[serde(rename = "m6id.24xlarge")]
    M6id24xlarge,

    /// m6id.2xlarge
    #[serde(rename = "m6id.2xlarge")]
    M6id2xlarge,

    /// m6id.32xlarge
    #[serde(rename = "m6id.32xlarge")]
    M6id32xlarge,

    /// m6id.4xlarge
    #[serde(rename = "m6id.4xlarge")]
    M6id4xlarge,

    /// m6id.8xlarge
    #[serde(rename = "m6id.8xlarge")]
    M6id8xlarge,

    /// m6id.large
    #[serde(rename = "m6id.large")]
    M6idlarge,

    /// m6id.metal
    #[serde(rename = "m6id.metal")]
    M6idmetal,

    /// m6id.xlarge
    #[serde(rename = "m6id.xlarge")]
    M6idxlarge,

    /// m6idn.12xlarge
    #[serde(rename = "m6idn.12xlarge")]
    M6idn12xlarge,

    /// m6idn.16xlarge
    #[serde(rename = "m6idn.16xlarge")]
    M6idn16xlarge,

    /// m6idn.24xlarge
    #[serde(rename = "m6idn.24xlarge")]
    M6idn24xlarge,

    /// m6idn.2xlarge
    #[serde(rename = "m6idn.2xlarge")]
    M6idn2xlarge,

    /// m6idn.32xlarge
    #[serde(rename = "m6idn.32xlarge")]
    M6idn32xlarge,

    /// m6idn.4xlarge
    #[serde(rename = "m6idn.4xlarge")]
    M6idn4xlarge,

    /// m6idn.8xlarge
    #[serde(rename = "m6idn.8xlarge")]
    M6idn8xlarge,

    /// m6idn.large
    #[serde(rename = "m6idn.large")]
    M6idnlarge,

    /// m6idn.metal
    #[serde(rename = "m6idn.metal")]
    M6idnmetal,

    /// m6idn.xlarge
    #[serde(rename = "m6idn.xlarge")]
    M6idnxlarge,

    /// m6in.12xlarge
    #[serde(rename = "m6in.12xlarge")]
    M6in12xlarge,

    /// m6in.16xlarge
    #[serde(rename = "m6in.16xlarge")]
    M6in16xlarge,

    /// m6in.24xlarge
    #[serde(rename = "m6in.24xlarge")]
    M6in24xlarge,

    /// m6in.2xlarge
    #[serde(rename = "m6in.2xlarge")]
    M6in2xlarge,

    /// m6in.32xlarge
    #[serde(rename = "m6in.32xlarge")]
    M6in32xlarge,

    /// m6in.4xlarge
    #[serde(rename = "m6in.4xlarge")]
    M6in4xlarge,

    /// m6in.8xlarge
    #[serde(rename = "m6in.8xlarge")]
    M6in8xlarge,

    /// m6in.large
    #[serde(rename = "m6in.large")]
    M6inlarge,

    /// m6in.metal
    #[serde(rename = "m6in.metal")]
    M6inmetal,

    /// m6in.xlarge
    #[serde(rename = "m6in.xlarge")]
    M6inxlarge,

    /// mac1.metal
    #[serde(rename = "mac1.metal")]
    Mac1metal,

    /// mac2.metal
    #[serde(rename = "mac2.metal")]
    Mac2metal,

    /// p2.16xlarge
    #[serde(rename = "p2.16xlarge")]
    P216xlarge,

    /// p2.8xlarge
    #[serde(rename = "p2.8xlarge")]
    P28xlarge,

    /// p2.xlarge
    #[serde(rename = "p2.xlarge")]
    P2xlarge,

    /// p3.16xlarge
    #[serde(rename = "p3.16xlarge")]
    P316xlarge,

    /// p3.2xlarge
    #[serde(rename = "p3.2xlarge")]
    P32xlarge,

    /// p3.8xlarge
    #[serde(rename = "p3.8xlarge")]
    P38xlarge,

    /// p3dn.24xlarge
    #[serde(rename = "p3dn.24xlarge")]
    P3dn24xlarge,

    /// p4d.24xlarge
    #[serde(rename = "p4d.24xlarge")]
    P4d24xlarge,

    /// p4de.24xlarge
    #[serde(rename = "p4de.24xlarge")]
    P4de24xlarge,

    /// r3.2xlarge
    #[serde(rename = "r3.2xlarge")]
    R32xlarge,

    /// r3.4xlarge
    #[serde(rename = "r3.4xlarge")]
    R34xlarge,

    /// r3.8xlarge
    #[serde(rename = "r3.8xlarge")]
    R38xlarge,

    /// r3.large
    #[serde(rename = "r3.large")]
    R3large,

    /// r3.xlarge
    #[serde(rename = "r3.xlarge")]
    R3xlarge,

    /// r4.16xlarge
    #[serde(rename = "r4.16xlarge")]
    R416xlarge,

    /// r4.2xlarge
    #[serde(rename = "r4.2xlarge")]
    R42xlarge,

    /// r4.4xlarge
    #[serde(rename = "r4.4xlarge")]
    R44xlarge,

    /// r4.8xlarge
    #[serde(rename = "r4.8xlarge")]
    R48xlarge,

    /// r4.large
    #[serde(rename = "r4.large")]
    R4large,

    /// r4.xlarge
    #[serde(rename = "r4.xlarge")]
    R4xlarge,

    /// r5.12xlarge
    #[serde(rename = "r5.12xlarge")]
    R512xlarge,

    /// r5.16xlarge
    #[serde(rename = "r5.16xlarge")]
    R516xlarge,

    /// r5.24xlarge
    #[serde(rename = "r5.24xlarge")]
    R524xlarge,

    /// r5.2xlarge
    #[serde(rename = "r5.2xlarge")]
    R52xlarge,

    /// r5.4xlarge
    #[serde(rename = "r5.4xlarge")]
    R54xlarge,

    /// r5.8xlarge
    #[serde(rename = "r5.8xlarge")]
    R58xlarge,

    /// r5.large
    #[serde(rename = "r5.large")]
    R5large,

    /// r5.metal
    #[serde(rename = "r5.metal")]
    R5metal,

    /// r5.xlarge
    #[serde(rename = "r5.xlarge")]
    R5xlarge,

    /// r5a.12xlarge
    #[serde(rename = "r5a.12xlarge")]
    R5a12xlarge,

    /// r5a.16xlarge
    #[serde(rename = "r5a.16xlarge")]
    R5a16xlarge,

    /// r5a.24xlarge
    #[serde(rename = "r5a.24xlarge")]
    R5a24xlarge,

    /// r5a.2xlarge
    #[serde(rename = "r5a.2xlarge")]
    R5a2xlarge,

    /// r5a.4xlarge
    #[serde(rename = "r5a.4xlarge")]
    R5a4xlarge,

    /// r5a.8xlarge
    #[serde(rename = "r5a.8xlarge")]
    R5a8xlarge,

    /// r5a.large
    #[serde(rename = "r5a.large")]
    R5alarge,

    /// r5a.xlarge
    #[serde(rename = "r5a.xlarge")]
    R5axlarge,

    /// r5ad.12xlarge
    #[serde(rename = "r5ad.12xlarge")]
    R5ad12xlarge,

    /// r5ad.16xlarge
    #[serde(rename = "r5ad.16xlarge")]
    R5ad16xlarge,

    /// r5ad.24xlarge
    #[serde(rename = "r5ad.24xlarge")]
    R5ad24xlarge,

    /// r5ad.2xlarge
    #[serde(rename = "r5ad.2xlarge")]
    R5ad2xlarge,

    /// r5ad.4xlarge
    #[serde(rename = "r5ad.4xlarge")]
    R5ad4xlarge,

    /// r5ad.8xlarge
    #[serde(rename = "r5ad.8xlarge")]
    R5ad8xlarge,

    /// r5ad.large
    #[serde(rename = "r5ad.large")]
    R5adlarge,

    /// r5ad.xlarge
    #[serde(rename = "r5ad.xlarge")]
    R5adxlarge,

    /// r5b.12xlarge
    #[serde(rename = "r5b.12xlarge")]
    R5b12xlarge,

    /// r5b.16xlarge
    #[serde(rename = "r5b.16xlarge")]
    R5b16xlarge,

    /// r5b.24xlarge
    #[serde(rename = "r5b.24xlarge")]
    R5b24xlarge,

    /// r5b.2xlarge
    #[serde(rename = "r5b.2xlarge")]
    R5b2xlarge,

    /// r5b.4xlarge
    #[serde(rename = "r5b.4xlarge")]
    R5b4xlarge,

    /// r5b.8xlarge
    #[serde(rename = "r5b.8xlarge")]
    R5b8xlarge,

    /// r5b.large
    #[serde(rename = "r5b.large")]
    R5blarge,

    /// r5b.metal
    #[serde(rename = "r5b.metal")]
    R5bmetal,

    /// r5b.xlarge
    #[serde(rename = "r5b.xlarge")]
    R5bxlarge,

    /// r5d.12xlarge
    #[serde(rename = "r5d.12xlarge")]
    R5d12xlarge,

    /// r5d.16xlarge
    #[serde(rename = "r5d.16xlarge")]
    R5d16xlarge,

    /// r5d.24xlarge
    #[serde(rename = "r5d.24xlarge")]
    R5d24xlarge,

    /// r5d.2xlarge
    #[serde(rename = "r5d.2xlarge")]
    R5d2xlarge,

    /// r5d.4xlarge
    #[serde(rename = "r5d.4xlarge")]
    R5d4xlarge,

    /// r5d.8xlarge
    #[serde(rename = "r5d.8xlarge")]
    R5d8xlarge,

    /// r5d.large
    #[serde(rename = "r5d.large")]
    R5dlarge,

    /// r5d.metal
    #[serde(rename = "r5d.metal")]
    R5dmetal,

    /// r5d.xlarge
    #[serde(rename = "r5d.xlarge")]
    R5dxlarge,

    /// r5dn.12xlarge
    #[serde(rename = "r5dn.12xlarge")]
    R5dn12xlarge,

    /// r5dn.16xlarge
    #[serde(rename = "r5dn.16xlarge")]
    R5dn16xlarge,

    /// r5dn.24xlarge
    #[serde(rename = "r5dn.24xlarge")]
    R5dn24xlarge,

    /// r5dn.2xlarge
    #[serde(rename = "r5dn.2xlarge")]
    R5dn2xlarge,

    /// r5dn.4xlarge
    #[serde(rename = "r5dn.4xlarge")]
    R5dn4xlarge,

    /// r5dn.8xlarge
    #[serde(rename = "r5dn.8xlarge")]
    R5dn8xlarge,

    /// r5dn.large
    #[serde(rename = "r5dn.large")]
    R5dnlarge,

    /// r5dn.metal
    #[serde(rename = "r5dn.metal")]
    R5dnmetal,

    /// r5dn.xlarge
    #[serde(rename = "r5dn.xlarge")]
    R5dnxlarge,

    /// r5n.12xlarge
    #[serde(rename = "r5n.12xlarge")]
    R5n12xlarge,

    /// r5n.16xlarge
    #[serde(rename = "r5n.16xlarge")]
    R5n16xlarge,

    /// r5n.24xlarge
    #[serde(rename = "r5n.24xlarge")]
    R5n24xlarge,

    /// r5n.2xlarge
    #[serde(rename = "r5n.2xlarge")]
    R5n2xlarge,

    /// r5n.4xlarge
    #[serde(rename = "r5n.4xlarge")]
    R5n4xlarge,

    /// r5n.8xlarge
    #[serde(rename = "r5n.8xlarge")]
    R5n8xlarge,

    /// r5n.large
    #[serde(rename = "r5n.large")]
    R5nlarge,

    /// r5n.metal
    #[serde(rename = "r5n.metal")]
    R5nmetal,

    /// r5n.xlarge
    #[serde(rename = "r5n.xlarge")]
    R5nxlarge,

    /// r6a.12xlarge
    #[serde(rename = "r6a.12xlarge")]
    R6a12xlarge,

    /// r6a.16xlarge
    #[serde(rename = "r6a.16xlarge")]
    R6a16xlarge,

    /// r6a.24xlarge
    #[serde(rename = "r6a.24xlarge")]
    R6a24xlarge,

    /// r6a.2xlarge
    #[serde(rename = "r6a.2xlarge")]
    R6a2xlarge,

    /// r6a.32xlarge
    #[serde(rename = "r6a.32xlarge")]
    R6a32xlarge,

    /// r6a.48xlarge
    #[serde(rename = "r6a.48xlarge")]
    R6a48xlarge,

    /// r6a.4xlarge
    #[serde(rename = "r6a.4xlarge")]
    R6a4xlarge,

    /// r6a.8xlarge
    #[serde(rename = "r6a.8xlarge")]
    R6a8xlarge,

    /// r6a.large
    #[serde(rename = "r6a.large")]
    R6alarge,

    /// r6a.metal
    #[serde(rename = "r6a.metal")]
    R6ametal,

    /// r6a.xlarge
    #[serde(rename = "r6a.xlarge")]
    R6axlarge,

    /// r6g.12xlarge
    #[serde(rename = "r6g.12xlarge")]
    R6g12xlarge,

    /// r6g.16xlarge
    #[serde(rename = "r6g.16xlarge")]
    R6g16xlarge,

    /// r6g.2xlarge
    #[serde(rename = "r6g.2xlarge")]
    R6g2xlarge,

    /// r6g.4xlarge
    #[serde(rename = "r6g.4xlarge")]
    R6g4xlarge,

    /// r6g.8xlarge
    #[serde(rename = "r6g.8xlarge")]
    R6g8xlarge,

    /// r6g.large
    #[serde(rename = "r6g.large")]
    R6glarge,

    /// r6g.medium
    #[serde(rename = "r6g.medium")]
    R6gmedium,

    /// r6g.metal
    #[serde(rename = "r6g.metal")]
    R6gmetal,

    /// r6g.xlarge
    #[serde(rename = "r6g.xlarge")]
    R6gxlarge,

    /// r6gd.12xlarge
    #[serde(rename = "r6gd.12xlarge")]
    R6gd12xlarge,

    /// r6gd.16xlarge
    #[serde(rename = "r6gd.16xlarge")]
    R6gd16xlarge,

    /// r6gd.2xlarge
    #[serde(rename = "r6gd.2xlarge")]
    R6gd2xlarge,

    /// r6gd.4xlarge
    #[serde(rename = "r6gd.4xlarge")]
    R6gd4xlarge,

    /// r6gd.8xlarge
    #[serde(rename = "r6gd.8xlarge")]
    R6gd8xlarge,

    /// r6gd.large
    #[serde(rename = "r6gd.large")]
    R6gdlarge,

    /// r6gd.medium
    #[serde(rename = "r6gd.medium")]
    R6gdmedium,

    /// r6gd.metal
    #[serde(rename = "r6gd.metal")]
    R6gdmetal,

    /// r6gd.xlarge
    #[serde(rename = "r6gd.xlarge")]
    R6gdxlarge,

    /// r6i.12xlarge
    #[serde(rename = "r6i.12xlarge")]
    R6i12xlarge,

    /// r6i.16xlarge
    #[serde(rename = "r6i.16xlarge")]
    R6i16xlarge,

    /// r6i.24xlarge
    #[serde(rename = "r6i.24xlarge")]
    R6i24xlarge,

    /// r6i.2xlarge
    #[serde(rename = "r6i.2xlarge")]
    R6i2xlarge,

    /// r6i.32xlarge
    #[serde(rename = "r6i.32xlarge")]
    R6i32xlarge,

    /// r6i.4xlarge
    #[serde(rename = "r6i.4xlarge")]
    R6i4xlarge,

    /// r6i.8xlarge
    #[serde(rename = "r6i.8xlarge")]
    R6i8xlarge,

    /// r6i.large
    #[serde(rename = "r6i.large")]
    R6ilarge,

    /// r6i.metal
    #[serde(rename = "r6i.metal")]
    R6imetal,

    /// r6i.xlarge
    #[serde(rename = "r6i.xlarge")]
    R6ixlarge,

    /// r6id.12xlarge
    #[serde(rename = "r6id.12xlarge")]
    R6id12xlarge,

    /// r6id.16xlarge
    #[serde(rename = "r6id.16xlarge")]
    R6id16xlarge,

    /// r6id.24xlarge
    #[serde(rename = "r6id.24xlarge")]
    R6id24xlarge,

    /// r6id.2xlarge
    #[serde(rename = "r6id.2xlarge")]
    R6id2xlarge,

    /// r6id.32xlarge
    #[serde(rename = "r6id.32xlarge")]
    R6id32xlarge,

    /// r6id.4xlarge
    #[serde(rename = "r6id.4xlarge")]
    R6id4xlarge,

    /// r6id.8xlarge
    #[serde(rename = "r6id.8xlarge")]
    R6id8xlarge,

    /// r6id.large
    #[serde(rename = "r6id.large")]
    R6idlarge,

    /// r6id.metal
    #[serde(rename = "r6id.metal")]
    R6idmetal,

    /// r6id.xlarge
    #[serde(rename = "r6id.xlarge")]
    R6idxlarge,

    /// r6idn.12xlarge
    #[serde(rename = "r6idn.12xlarge")]
    R6idn12xlarge,

    /// r6idn.16xlarge
    #[serde(rename = "r6idn.16xlarge")]
    R6idn16xlarge,

    /// r6idn.24xlarge
    #[serde(rename = "r6idn.24xlarge")]
    R6idn24xlarge,

    /// r6idn.2xlarge
    #[serde(rename = "r6idn.2xlarge")]
    R6idn2xlarge,

    /// r6idn.32xlarge
    #[serde(rename = "r6idn.32xlarge")]
    R6idn32xlarge,

    /// r6idn.4xlarge
    #[serde(rename = "r6idn.4xlarge")]
    R6idn4xlarge,

    /// r6idn.8xlarge
    #[serde(rename = "r6idn.8xlarge")]
    R6idn8xlarge,

    /// r6idn.large
    #[serde(rename = "r6idn.large")]
    R6idnlarge,

    /// r6idn.metal
    #[serde(rename = "r6idn.metal")]
    R6idnmetal,

    /// r6idn.xlarge
    #[serde(rename = "r6idn.xlarge")]
    R6idnxlarge,

    /// r6in.12xlarge
    #[serde(rename = "r6in.12xlarge")]
    R6in12xlarge,

    /// r6in.16xlarge
    #[serde(rename = "r6in.16xlarge")]
    R6in16xlarge,

    /// r6in.24xlarge
    #[serde(rename = "r6in.24xlarge")]
    R6in24xlarge,

    /// r6in.2xlarge
    #[serde(rename = "r6in.2xlarge")]
    R6in2xlarge,

    /// r6in.32xlarge
    #[serde(rename = "r6in.32xlarge")]
    R6in32xlarge,

    /// r6in.4xlarge
    #[serde(rename = "r6in.4xlarge")]
    R6in4xlarge,

    /// r6in.8xlarge
    #[serde(rename = "r6in.8xlarge")]
    R6in8xlarge,

    /// r6in.large
    #[serde(rename = "r6in.large")]
    R6inlarge,

    /// r6in.metal
    #[serde(rename = "r6in.metal")]
    R6inmetal,

    /// r6in.xlarge
    #[serde(rename = "r6in.xlarge")]
    R6inxlarge,

    /// t1.micro
    #[serde(rename = "t1.micro")]
    T1micro,

    /// t2.2xlarge
    #[serde(rename = "t2.2xlarge")]
    T22xlarge,

    /// t2.large
    #[serde(rename = "t2.large")]
    T2large,

    /// t2.medium
    #[serde(rename = "t2.medium")]
    T2medium,

    /// t2.micro
    #[serde(rename = "t2.micro")]
    T2micro,

    /// t2.nano
    #[serde(rename = "t2.nano")]
    T2nano,

    /// t2.small
    #[serde(rename = "t2.small")]
    T2small,

    /// t2.xlarge
    #[serde(rename = "t2.xlarge")]
    T2xlarge,

    /// t3.2xlarge
    #[serde(rename = "t3.2xlarge")]
    T32xlarge,

    /// t3.large
    #[serde(rename = "t3.large")]
    T3large,

    /// t3.medium
    #[serde(rename = "t3.medium")]
    T3medium,

    /// t3.micro
    #[serde(rename = "t3.micro")]
    T3micro,

    /// t3.nano
    #[serde(rename = "t3.nano")]
    T3nano,

    /// t3.small
    #[serde(rename = "t3.small")]
    T3small,

    /// t3.xlarge
    #[serde(rename = "t3.xlarge")]
    T3xlarge,

    /// t3a.2xlarge
    #[serde(rename = "t3a.2xlarge")]
    T3a2xlarge,

    /// t3a.large
    #[serde(rename = "t3a.large")]
    T3alarge,

    /// t3a.medium
    #[serde(rename = "t3a.medium")]
    T3amedium,

    /// t3a.micro
    #[serde(rename = "t3a.micro")]
    T3amicro,

    /// t3a.nano
    #[serde(rename = "t3a.nano")]
    T3anano,

    /// t3a.small
    #[serde(rename = "t3a.small")]
    T3asmall,

    /// t3a.xlarge
    #[serde(rename = "t3a.xlarge")]
    T3axlarge,

    /// t4g.2xlarge
    #[serde(rename = "t4g.2xlarge")]
    T4g2xlarge,

    /// t4g.large
    #[serde(rename = "t4g.large")]
    T4glarge,

    /// t4g.medium
    #[serde(rename = "t4g.medium")]
    T4gmedium,

    /// t4g.micro
    #[serde(rename = "t4g.micro")]
    T4gmicro,

    /// t4g.nano
    #[serde(rename = "t4g.nano")]
    T4gnano,

    /// t4g.small
    #[serde(rename = "t4g.small")]
    T4gsmall,

    /// t4g.xlarge
    #[serde(rename = "t4g.xlarge")]
    T4gxlarge,

    /// trn1.2xlarge
    #[serde(rename = "trn1.2xlarge")]
    Trn12xlarge,

    /// trn1.32xlarge
    #[serde(rename = "trn1.32xlarge")]
    Trn132xlarge,

    /// trn1n.32xlarge
    #[serde(rename = "trn1n.32xlarge")]
    Trn1n32xlarge,

    /// u-12tb1.112xlarge
    #[serde(rename = "u-12tb1.112xlarge")]
    U12tb1112xlarge,

    /// u-12tb1.metal
    #[serde(rename = "u-12tb1.metal")]
    U12tb1metal,

    /// u-18tb1.112xlarge
    #[serde(rename = "u-18tb1.112xlarge")]
    U18tb1112xlarge,

    /// u-18tb1.metal
    #[serde(rename = "u-18tb1.metal")]
    U18tb1metal,

    /// u-24tb1.112xlarge
    #[serde(rename = "u-24tb1.112xlarge")]
    U24tb1112xlarge,

    /// u-24tb1.metal
    #[serde(rename = "u-24tb1.metal")]
    U24tb1metal,

    /// u-3tb1.56xlarge
    #[serde(rename = "u-3tb1.56xlarge")]
    U3tb156xlarge,

    /// u-6tb1.112xlarge
    #[serde(rename = "u-6tb1.112xlarge")]
    U6tb1112xlarge,

    /// u-6tb1.56xlarge
    #[serde(rename = "u-6tb1.56xlarge")]
    U6tb156xlarge,

    /// u-6tb1.metal
    #[serde(rename = "u-6tb1.metal")]
    U6tb1metal,

    /// u-9tb1.112xlarge
    #[serde(rename = "u-9tb1.112xlarge")]
    U9tb1112xlarge,

    /// u-9tb1.metal
    #[serde(rename = "u-9tb1.metal")]
    U9tb1metal,

    /// vt1.24xlarge
    #[serde(rename = "vt1.24xlarge")]
    Vt124xlarge,

    /// vt1.3xlarge
    #[serde(rename = "vt1.3xlarge")]
    Vt13xlarge,

    /// vt1.6xlarge
    #[serde(rename = "vt1.6xlarge")]
    Vt16xlarge,

    /// x1.16xlarge
    #[serde(rename = "x1.16xlarge")]
    X116xlarge,

    /// x1.32xlarge
    #[serde(rename = "x1.32xlarge")]
    X132xlarge,

    /// x1e.16xlarge
    #[serde(rename = "x1e.16xlarge")]
    X1e16xlarge,

    /// x1e.2xlarge
    #[serde(rename = "x1e.2xlarge")]
    X1e2xlarge,

    /// x1e.32xlarge
    #[serde(rename = "x1e.32xlarge")]
    X1e32xlarge,

    /// x1e.4xlarge
    #[serde(rename = "x1e.4xlarge")]
    X1e4xlarge,

    /// x1e.8xlarge
    #[serde(rename = "x1e.8xlarge")]
    X1e8xlarge,

    /// x1e.xlarge
    #[serde(rename = "x1e.xlarge")]
    X1exlarge,

    /// x2gd.12xlarge
    #[serde(rename = "x2gd.12xlarge")]
    X2gd12xlarge,

    /// x2gd.16xlarge
    #[serde(rename = "x2gd.16xlarge")]
    X2gd16xlarge,

    /// x2gd.2xlarge
    #[serde(rename = "x2gd.2xlarge")]
    X2gd2xlarge,

    /// x2gd.4xlarge
    #[serde(rename = "x2gd.4xlarge")]
    X2gd4xlarge,

    /// x2gd.8xlarge
    #[serde(rename = "x2gd.8xlarge")]
    X2gd8xlarge,

    /// x2gd.large
    #[serde(rename = "x2gd.large")]
    X2gdlarge,

    /// x2gd.medium
    #[serde(rename = "x2gd.medium")]
    X2gdmedium,

    /// x2gd.metal
    #[serde(rename = "x2gd.metal")]
    X2gdmetal,

    /// x2gd.xlarge
    #[serde(rename = "x2gd.xlarge")]
    X2gdxlarge,

    /// x2idn.16xlarge
    #[serde(rename = "x2idn.16xlarge")]
    X2idn16xlarge,

    /// x2idn.24xlarge
    #[serde(rename = "x2idn.24xlarge")]
    X2idn24xlarge,

    /// x2idn.32xlarge
    #[serde(rename = "x2idn.32xlarge")]
    X2idn32xlarge,

    /// x2idn.metal
    #[serde(rename = "x2idn.metal")]
    X2idnmetal,

    /// x2iedn.16xlarge
    #[serde(rename = "x2iedn.16xlarge")]
    X2iedn16xlarge,

    /// x2iedn.24xlarge
    #[serde(rename = "x2iedn.24xlarge")]
    X2iedn24xlarge,

    /// x2iedn.2xlarge
    #[serde(rename = "x2iedn.2xlarge")]
    X2iedn2xlarge,

    /// x2iedn.32xlarge
    #[serde(rename = "x2iedn.32xlarge")]
    X2iedn32xlarge,

    /// x2iedn.4xlarge
    #[serde(rename = "x2iedn.4xlarge")]
    X2iedn4xlarge,

    /// x2iedn.8xlarge
    #[serde(rename = "x2iedn.8xlarge")]
    X2iedn8xlarge,

    /// x2iedn.metal
    #[serde(rename = "x2iedn.metal")]
    X2iednmetal,

    /// x2iedn.xlarge
    #[serde(rename = "x2iedn.xlarge")]
    X2iednxlarge,

    /// x2iezn.12xlarge
    #[serde(rename = "x2iezn.12xlarge")]
    X2iezn12xlarge,

    /// x2iezn.2xlarge
    #[serde(rename = "x2iezn.2xlarge")]
    X2iezn2xlarge,

    /// x2iezn.4xlarge
    #[serde(rename = "x2iezn.4xlarge")]
    X2iezn4xlarge,

    /// x2iezn.6xlarge
    #[serde(rename = "x2iezn.6xlarge")]
    X2iezn6xlarge,

    /// x2iezn.8xlarge
    #[serde(rename = "x2iezn.8xlarge")]
    X2iezn8xlarge,

    /// x2iezn.metal
    #[serde(rename = "x2iezn.metal")]
    X2ieznmetal,

    /// z1d.12xlarge
    #[serde(rename = "z1d.12xlarge")]
    Z1d12xlarge,

    /// z1d.2xlarge
    #[serde(rename = "z1d.2xlarge")]
    Z1d2xlarge,

    /// z1d.3xlarge
    #[serde(rename = "z1d.3xlarge")]
    Z1d3xlarge,

    /// z1d.6xlarge
    #[serde(rename = "z1d.6xlarge")]
    Z1d6xlarge,

    /// z1d.large
    #[serde(rename = "z1d.large")]
    Z1dlarge,

    /// z1d.metal
    #[serde(rename = "z1d.metal")]
    Z1dmetal,

    /// z1d.xlarge
    #[serde(rename = "z1d.xlarge")]
    Z1dxlarge,
}

impl Default for InstanceTypeSpecificationInstanceTypeEnum {
    fn default() -> Self {
        InstanceTypeSpecificationInstanceTypeEnum::A12xlarge
    }
}

impl cfn_resources::CfnResource for InstanceTypeSpecification {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.priority {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'priority'. {} is less than 0",
                    the_val
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

/// The tags to apply to a resource when the resource is being created. When you specify a tag, you must     specify the resource type to tag, otherwise the request will fail.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagSpecification {
    ///
    /// The type of resource to tag on creation. Specify     capacity-reservation-fleet.
    ///
    /// To tag a resource after it has been created, see CreateTags.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,

    ///
    /// The tags to apply to the resource.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for TagSpecification {
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
