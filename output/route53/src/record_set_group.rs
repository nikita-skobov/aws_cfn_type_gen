/// A complex type that contains an optional comment, the name and ID of the hosted zone that you want to make changes in, 			and values for the records that you want to create.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRecordSetGroup {
    ///
    /// Optional: Any comments you want to include about a change batch 			request.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    ///
    /// The ID of the hosted zone that you want to create records in.
    ///
    /// Specify either HostedZoneName or HostedZoneId, but not both. If you have multiple hosted zones 			with the same domain name, you must specify the hosted zone using HostedZoneId.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 32
    ///
    /// Update requires: Replacement
    #[serde(rename = "HostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,

    ///
    /// The name of the hosted zone that you want to create records in. You must include a trailing dot (for example, www.example.com.) as part       of the HostedZoneName.
    ///
    /// When you create a stack using an AWS::Route53::RecordSet that specifies HostedZoneName, 			AWS CloudFormation attempts to find a hosted zone whose name matches the HostedZoneName. If AWS CloudFormation 			can't find a hosted zone with a matching domain name, or if there is more than one hosted zone with the specified domain name, 			AWS CloudFormation will not create the stack.
    ///
    /// Specify either HostedZoneName or HostedZoneId, but not both. If you have multiple hosted zones 			with the same domain name, you must specify the hosted zone using HostedZoneId.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "HostedZoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_name: Option<String>,

    ///
    /// A complex type that contains one RecordSet element for each record that you want to create.
    ///
    /// Required: No
    ///
    /// Type: List of RecordSet
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_sets: Option<Vec<RecordSet>>,
}

impl cfn_resources::CfnResource for CfnRecordSetGroup {
    fn type_string(&self) -> &'static str {
        "AWS::Route53::RecordSetGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.comment {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'comment'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.hosted_zone_id {
            if the_val.len() > 32 as _ {
                return Err(format!(
                    "Max validation failed on field 'hosted_zone_id'. {} is greater than 32",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.hosted_zone_name {
            if the_val.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'hosted_zone_name'. {} is greater than 1024",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Alias records only: Information about the AWS resource, such as a CloudFront distribution or 			an Amazon S3 bucket, that you want to route traffic to.
///
/// When creating records for a private hosted zone, note the following:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AliasTarget {
    ///
    /// Alias records only: The value that you specify depends on where you want to route queries:
    ///
    /// Amazon API Gateway custom regional APIs and edge-optimized APIs 				 					Specify the applicable domain name for your API. You can get the applicable value using the AWS CLI command 						get-domain-names: 						 							 							 						 								For regional APIs, specify the value of regionalDomainName. 							 								For edge-optimized APIs, specify the value of distributionDomainName. This is the name of the 									associated CloudFront distribution, such as da1b2c3d4e5.cloudfront.net. 							 					NoteThe name of the record that you're creating must match a custom domain name for your API, such as 							api.example.com. 				 			 				Amazon Virtual Private Cloud interface VPC endpoint 					 						Enter the API endpoint for the interface endpoint, such as 						vpce-123456789abcdef01-example-us-east-1a.elasticloadbalancing.us-east-1.vpce.amazonaws.com. For edge-optimized APIs, 						this is the domain name for the corresponding CloudFront distribution. You can get the value of DnsName using the AWS CLI command 						describe-vpc-endpoints. 					 			 				CloudFront distribution 				 					Specify the domain name that CloudFront assigned when you created your distribution. 					Your CloudFront distribution must include an alternate domain name that matches the name of the record. 						For example, if the name of the record is acme.example.com, your CloudFront distribution must 						include acme.example.com as one of the alternate domain names. For more information, see 						Using Alternate Domain Names (CNAMEs) in the Amazon CloudFront Developer Guide. 					You can't create a record in a private hosted zone to route traffic to a CloudFront distribution. 					NoteFor failover alias records, you can't specify a CloudFront distribution for both the primary and secondary records. 							A distribution must include an alternate domain name that matches the name of the record. However, the primary and secondary records 							have the same name, and you can't include the same alternate domain name in more than one distribution. 				 			 				Elastic Beanstalk environment 				 					If the domain name for your Elastic Beanstalk environment includes the region that you deployed the environment in, 						you can create an alias record that routes traffic to the environment. For example, the domain name 						my-environment.us-west-2.elasticbeanstalk.com is a regionalized domain name. 					ImportantFor environments that were created before early 2016, the domain name doesn't include the region. To route traffic 							to these environments, you must create a CNAME record instead of an alias record. Note that you can't create a 							CNAME record for the root domain name. For example, if your domain name is example.com, you can create a record 							that routes traffic for acme.example.com to your Elastic Beanstalk environment, but you can't create a record 							that routes traffic for example.com to your Elastic Beanstalk environment. 					For Elastic Beanstalk environments that have regionalized subdomains, specify the CNAME attribute for the environment. 						You can use the following methods to get the value of the CNAME attribute: 						 							 							 							 						 								AWS Management Console: For information about how to get the value by using the console, 									see Using Custom Domains with AWS Elastic Beanstalk in the 									AWS Elastic Beanstalk Developer Guide. 							 								Elastic Beanstalk API: Use the DescribeEnvironments action to get 									the value of the CNAME attribute. For more information, see 									DescribeEnvironments 									in the AWS Elastic Beanstalk API Reference. 							 								AWS CLI: Use the describe-environments command to get the value of the 									CNAME attribute. For more information, see 									describe-environments in the 									AWS CLI. 							 				 			 				ELB load balancer 				 					Specify the DNS name that is associated with the load balancer. Get the DNS name by using the AWS Management Console, 						the ELB API, or the AWS CLI. 						 							 							 							 							 					 								AWS Management Console: Go to the EC2 page, choose Load Balancers 									in the navigation pane, choose the load balancer, choose the Description tab, and get the value 									of the DNS name field. 								If you're routing traffic to a Classic Load Balancer, get the value that begins with dualstack. 									If you're routing traffic to another type of load balancer, get the value that applies to the record type, A or AAAA. 							 								Elastic Load Balancing API: Use DescribeLoadBalancers to get the value 									of DNSName. For more information, see the applicable guide: 								 									 									 								 										Classic Load Balancers: 											DescribeLoadBalancers 										 									 										Application and Network Load Balancers: 											DescribeLoadBalancers 										 									 							 								CloudFormation Fn::GetAtt intrinsic function: Use the 									Fn::GetAtt 									intrinsic function to get the value of DNSName: 								 									 									 								 										Classic Load Balancers. 									 									 										Application and Network Load Balancers. 									 									 							 								AWS CLI: Use describe-load-balancers to get the value of DNSName. 									For more information, see the applicable guide: 								 									 									 								 										Classic Load Balancers: 											describe-load-balancers 										 									 										Application and Network Load Balancers: 											describe-load-balancers 										 									 							 				 			 				Global Accelerator accelerator 				 					Specify the DNS name for your accelerator: 					 						 						 					Global Accelerator API: To get the DNS name, use 							DescribeAccelerator. AWS CLI: To get the DNS name, use 							describe-accelerator. 				 			 				Amazon S3 bucket that is configured as a static website 				 					Specify the domain name of the Amazon S3 website endpoint that you created the bucket in, for example, 						s3-website.us-east-2.amazonaws.com. For more information about valid values, see the table 						Amazon S3 Website Endpoints 						in the Amazon Web Services General Reference. For more information about using S3 buckets for websites, 						see Getting Started with Amazon Route 53 						in the Amazon Route 53 Developer Guide. 					 				 			 				Another Route 53 record 				 					Specify the value of the Name element for a record in the current hosted zone. 					 					NoteIf you're creating an alias record that has the same name as the hosted zone (known as the zone apex), 							you can't specify the domain name for a record for which the value of Type is CNAME. This is because 							the alias record must have the same type as the record that you're routing traffic to, and creating a CNAME record for the 							zone apex isn't supported even for an alias record.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "DNSName")]
    pub dnsname: String,

    ///
    /// Applies only to alias, failover alias, geolocation alias, latency alias, and 				weighted alias resource record sets: When 				EvaluateTargetHealth is true, an alias resource record set 			inherits the health of the referenced AWS resource, such as an ELB load 			balancer or another resource record set in the hosted zone.
    ///
    /// Note the following:
    ///
    /// CloudFront distributions                  You can't set EvaluateTargetHealth to true when 						the alias target is a CloudFront distribution.                       Elastic Beanstalk environments that have regionalized subdomains                  If you specify an Elastic Beanstalk environment in DNSName 						and the environment contains an ELB load balancer, Elastic Load Balancing 						routes queries only to the healthy Amazon EC2 instances that are registered 						with the load balancer. (An environment automatically contains an ELB load 						balancer if it includes more than one Amazon EC2 instance.) If you set 							EvaluateTargetHealth to true and either no 						Amazon EC2 instances are healthy or the load balancer itself is unhealthy, 						Route 53 routes queries to other available resources that are healthy, if 						any.          If the environment contains a single Amazon EC2 instance, there are no 						special requirements.                       ELB load balancers                  Health checking behavior depends on the type of load balancer:                                                                     Classic Load Balancers: If you 								specify an ELB Classic Load Balancer in DNSName, 								Elastic Load Balancing routes queries only to the healthy Amazon EC2 								instances that are registered with the load balancer. If you set 									EvaluateTargetHealth to true and 								either no EC2 instances are healthy or the load balancer itself is 								unhealthy, Route 53 routes queries to other resources.                                      Application and Network Load 									Balancers: If you specify an ELB Application or 								Network Load Balancer and you set EvaluateTargetHealth 								to true, Route 53 routes queries to the load balancer 								based on the health of the target groups that are associated with 								the load balancer:                                                                      For an Application or Network Load Balancer to be 										considered healthy, every target group that contains targets 										must contain at least one healthy target. If any target 										group contains only unhealthy targets, the load balancer is 										considered unhealthy, and Route 53 routes queries to other 										resources.                              A target group that has no registered targets is 										considered unhealthy.                                   NoteWhen you create a load balancer, you configure settings for Elastic 							Load Balancing health checks; they're not Route 53 health checks, but 							they perform a similar function. Do not create Route 53 health checks 							for the EC2 instances that you register with an ELB load balancer. 						                       S3 buckets                  There are no special requirements for setting 							EvaluateTargetHealth to true when the alias 						target is an S3 bucket.                       Other records in the same hosted zone                  If the AWS resource that you specify in 							DNSName is a record or a group of records (for example, a 						group of weighted records) but is not another alias record, we recommend 						that you associate a health check with all of the records in the alias 						target. For more information, see What Happens When You Omit Health Checks? in the 							Amazon Route 53 Developer Guide.
    ///
    /// For more information and examples, see Amazon Route 53 Health Checks 				and DNS Failover in the Amazon Route 53 Developer 			Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluateTargetHealth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_target_health: Option<bool>,

    ///
    /// Alias resource records sets only: The value used depends on where you want to route traffic:
    ///
    /// Amazon API Gateway custom regional APIs and edge-optimized APIs 				Specify the hosted zone ID for your API. You can get the applicable value using the AWS CLI command 					get-domain-names: 					 						 						 					 							For regional APIs, specify the value of regionalHostedZoneId. 						 							For edge-optimized APIs, specify the value of distributionHostedZoneId. 						 				 			 				Amazon Virtual Private Cloud interface VPC endpoint 				 					Specify the hosted zone ID for your interface endpoint. You can get the value of HostedZoneId 						using the AWS CLI command 						describe-vpc-endpoints. 				 			 				CloudFront distribution 				 					Specify Z2FDTNDATAQYW2. This is always the hosted zone ID when you create an alias record that 						routes traffic to a CloudFront distribution. 					NoteAlias records for CloudFront can't be created in a private zone. 				 			 				Elastic Beanstalk environment 				 					Specify the hosted zone ID for the region that you created the environment in. The environment 					  must have a regionalized subdomain. For a list of regions and the corresponding hosted zone IDs, see 					  AWS Elastic Beanstalk endpoints and quotas in the Amazon Web Services General Reference. 				 			 				ELB load balancer 				 					Specify the value of the hosted zone ID for the load balancer. Use the following methods to get the hosted zone ID: 						 							 							 							 							 						 					 								Service Endpoints table 									in the "Elastic Load Balancing Endpoints and Quotas" topic in the 									Amazon Web Services General Reference: Use the value that corresponds with 									the region that you created your load balancer in. Note that there are separate columns for 									Application and Classic Load Balancers and for Network Load Balancers. 							 								AWS Management Console: Go to the Amazon EC2 page, choose 									Load Balancers in the navigation pane, select the load balancer, and get the value of the 									Hosted zone field on the Description tab. 							 								Elastic Load Balancing API: Use DescribeLoadBalancers to get the applicable value. 									For more information, see the applicable guide: 								 									 									 								Classic Load Balancers: Use 										DescribeLoadBalancers 										to get the value of CanonicalHostedZoneNameID. 									Application and Network Load Balancers: Use 										DescribeLoadBalancers 										to get the value of CanonicalHostedZoneID. 									 							 								CloudFormation Fn::GetAtt intrinsic function: Use the 									Fn::GetAtt 									intrinsic function to get the applicable value: 								 									 									 								Classic Load Balancers: Get 										CanonicalHostedZoneNameID. 									 									Application and Network Load Balancers: Get 										CanonicalHostedZoneID. 									 									 						 							AWS CLI: Use describe-load-balancers to get the applicable value. 								For more information, see the applicable guide: 								 									 									 								 										Classic Load Balancers: Use 										describe-load-balancers 										to get the value of CanonicalHostedZoneNameID. 									 										Application and Network Load Balancers: Use 											describe-load-balancers 											to get the value of CanonicalHostedZoneID. 									 						 				 			 				Global Accelerator accelerator 				 					Specify Z2BJ6XQ5FK7U4H. 				 			 				An Amazon S3 bucket configured as a static website 				 					Specify the hosted zone ID for the region that you created the bucket in. For more information about 						valid values, see the table 						Amazon S3 Website Endpoints 						in the Amazon Web Services General Reference. 				 			 				Another Route 53 record in your hosted zone 				 					Specify the hosted zone ID of your hosted zone. (An alias record can't reference a record 						in a different hosted zone.)
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 32
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: String,
}

impl cfn_resources::CfnResource for AliasTarget {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.dnsname;

        if the_val.len() > 1024 as _ {
            return Err(format!(
                "Max validation failed on field 'dnsname'. {} is greater than 1024",
                the_val.len()
            ));
        }

        let the_val = &self.hosted_zone_id;

        if the_val.len() > 32 as _ {
            return Err(format!(
                "Max validation failed on field 'hosted_zone_id'. {} is greater than 32",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The object that is specified in resource record set object when you are linking a 			resource record set to a CIDR location.
///
/// A LocationName with an asterisk “*” can be used to create a default CIDR 			record. CollectionId is still required for default record.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CidrRoutingConfig {
    ///
    /// The CIDR collection ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [0-9a-f]{8}-(?:[0-9a-f]{4}-){3}[0-9a-f]{12}
    ///
    /// Update requires: No interruption
    #[serde(rename = "CollectionId")]
    pub collection_id: String,

    ///
    /// The CIDR collection location name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 16
    ///
    /// Pattern: [0-9A-Za-z_\-\*]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocationName")]
    pub location_name: String,
}

impl cfn_resources::CfnResource for CidrRoutingConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.location_name;

        if the_val.len() > 16 as _ {
            return Err(format!(
                "Max validation failed on field 'location_name'. {} is greater than 16",
                the_val.len()
            ));
        }

        let the_val = &self.location_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'location_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// A complex type that contains information about a geographic location.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GeoLocation {
    ///
    /// For geolocation resource record sets, a two-letter abbreviation that identifies a continent. Route 53 supports the following continent codes:
    ///
    /// AF: AfricaAN: AntarcticaAS: AsiaEU: EuropeOC: OceaniaNA: North AmericaSA: South America
    ///
    /// Constraint: Specifying ContinentCode with either CountryCode or SubdivisionCode returns an 			InvalidInput error.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 2
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContinentCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_code: Option<String>,

    ///
    /// For geolocation resource record sets, the two-letter code for a country.
    ///
    /// Route 53 uses the two-letter country codes that are specified in 			ISO standard 3166-1 alpha-2.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "CountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,

    ///
    /// For geolocation resource record sets, the two-letter code for a state of the United States. 			Route 53 doesn't support any other values for SubdivisionCode. For a list of state abbreviations, see 			Appendix B: Two–Letter State and Possession Abbreviations 			on the United States Postal Service website.
    ///
    /// If you specify subdivisioncode, you must also specify US for CountryCode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 3
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubdivisionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdivision_code: Option<String>,
}

impl cfn_resources::CfnResource for GeoLocation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.continent_code {
            if the_val.len() > 2 as _ {
                return Err(format!(
                    "Max validation failed on field 'continent_code'. {} is greater than 2",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.continent_code {
            if the_val.len() < 2 as _ {
                return Err(format!(
                    "Min validation failed on field 'continent_code'. {} is less than 2",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.country_code {
            if the_val.len() > 2 as _ {
                return Err(format!(
                    "Max validation failed on field 'country_code'. {} is greater than 2",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.country_code {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'country_code'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.subdivision_code {
            if the_val.len() > 3 as _ {
                return Err(format!(
                    "Max validation failed on field 'subdivision_code'. {} is greater than 3",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.subdivision_code {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'subdivision_code'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Information about the record that you want to create.
///
/// The AWS::Route53::RecordSet type can be used as a standalone resource or as an embedded property in the 			AWS::Route53::RecordSetGroup type. Note that some AWS::Route53::RecordSet properties are valid 			only when used within AWS::Route53::RecordSetGroup.
///
/// For more information, see ChangeResourceRecordSets 			in the Amazon Route 53 API Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RecordSet {
    ///
    /// Alias resource record sets only: Information about the AWS resource, such as a CloudFront distribution or an Amazon S3 bucket, that 			you want to route traffic to.
    ///
    /// If you're creating resource records sets for a private hosted zone, note the 			following:
    ///
    /// You can't create an alias resource record set in a private hosted zone to 					route traffic to a CloudFront distribution.               For information about creating failover resource record sets in a private 					hosted zone, see Configuring Failover in a Private Hosted Zone in the 						Amazon Route 53 Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: AliasTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "AliasTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_target: Option<AliasTarget>,

    ///
    /// The object that is specified in resource record set object when you are linking a 			resource record set to a CIDR location.
    ///
    /// A LocationName with an asterisk “*” can be used to create a default CIDR 			record. CollectionId is still required for default record.
    ///
    /// Required: No
    ///
    /// Type: CidrRoutingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "CidrRoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_routing_config: Option<CidrRoutingConfig>,

    ///
    /// Failover resource record sets only: To configure failover, you 			add the Failover element to two resource record sets. For one resource 			record set, you specify PRIMARY as the value for Failover; for 			the other resource record set, you specify SECONDARY. In addition, you 			include the HealthCheckId element and specify the health check that you 			want Amazon Route 53 to perform for each resource record set.
    ///
    /// Except where noted, the following failover behaviors assume that you have included the 				HealthCheckId element in both resource record sets:
    ///
    /// When the primary resource record set is healthy, Route 53 responds to DNS 					queries with the applicable value from the primary resource record set 					regardless of the health of the secondary resource record set.               When the primary resource record set is unhealthy and the secondary resource 					record set is healthy, Route 53 responds to DNS queries with the applicable 					value from the secondary resource record set.               When the secondary resource record set is unhealthy, Route 53 responds to DNS 					queries with the applicable value from the primary resource record set 					regardless of the health of the primary resource record set.               If you omit the HealthCheckId element for the secondary resource 					record set, and if the primary resource record set is unhealthy, Route 53 always 					responds to DNS queries with the applicable value from the secondary resource 					record set. This is true regardless of the health of the associated 					endpoint.
    ///
    /// You can't create non-failover resource record sets that have the same values for the 				Name and Type elements as failover resource record 			sets.
    ///
    /// For failover alias resource record sets, you must also include the 				EvaluateTargetHealth element and set the value to true.
    ///
    /// For more information about configuring failover for Route 53, see the following topics 			in the Amazon Route 53 Developer Guide:
    ///
    /// Route 53 Health Checks 						and DNS Failover                                Configuring Failover in a Private Hosted Zone
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PRIMARY | SECONDARY
    ///
    /// Update requires: No interruption
    #[serde(rename = "Failover")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover: Option<RecordSetFailoverEnum>,

    ///
    /// Geolocation resource record sets only: A complex type that lets 			you control how Amazon Route 53 responds to DNS queries based on the geographic origin 			of the query. For example, if you want all queries from Africa to be routed to a web 			server with an IP address of 192.0.2.111, create a resource record set with 			a Type of A and a ContinentCode of 				AF.
    ///
    /// NoteAlthough creating geolocation and geolocation alias resource record sets in a 				private hosted zone is allowed, it's not supported.
    ///
    /// If you create separate resource record sets for overlapping geographic regions (for 			example, one resource record set for a continent and one for a country on the same 			continent), priority goes to the smallest geographic region. This allows you to route 			most queries for a continent to one resource and to route queries for a country on that 			continent to a different resource.
    ///
    /// You can't create two geolocation resource record sets that specify the same geographic 			location.
    ///
    /// The value * in the CountryCode element matches all 			geographic locations that aren't specified in other geolocation resource record sets 			that have the same values for the Name and Type 			elements.
    ///
    /// ImportantGeolocation works by mapping IP addresses to locations. However, some IP addresses 				aren't mapped to geographic locations, so even if you create geolocation resource 				record sets that cover all seven continents, Route 53 will receive some DNS queries 				from locations that it can't identify. We recommend that you create a resource 				record set for which the value of CountryCode is *. Two 				groups of queries are routed to the resource that you specify in this record: 				queries that come from locations for which you haven't created geolocation resource 				record sets and queries from IP addresses that aren't mapped to a location. If you 				don't create a * resource record set, Route 53 returns a "no answer" 				response for queries from those locations.
    ///
    /// You can't create non-geolocation resource record sets that have the same values for 			the Name and Type elements as geolocation resource record 			sets.
    ///
    /// Required: No
    ///
    /// Type: GeoLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "GeoLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_location: Option<GeoLocation>,

    ///
    /// If you want Amazon Route 53 to return this resource record set in response to a DNS 			query only when the status of a health check is healthy, include the 				HealthCheckId element and specify the ID of the applicable health 			check.
    ///
    /// Route 53 determines whether a resource record set is healthy based on one of the 			following:
    ///
    /// By periodically sending a request to the endpoint that is specified in the 					health check               By aggregating the status of a specified group of health checks (calculated 					health checks)               By determining the current state of a CloudWatch alarm (CloudWatch metric 					health checks)
    ///
    /// ImportantRoute 53 doesn't check the health of the endpoint that is specified in the 				resource record set, for example, the endpoint specified by the IP address in the 					Value element. When you add a HealthCheckId element to 				a resource record set, Route 53 checks the health of the endpoint that you specified 				in the health check.
    ///
    /// For more information, see the following topics in the Amazon Route 53 				Developer Guide:
    ///
    /// How Amazon Route 53 Determines Whether an Endpoint Is 					Healthy                                Route 53 Health Checks 						and DNS Failover                                Configuring Failover in a Private Hosted Zone
    ///
    /// When to Specify HealthCheckId
    ///
    /// Specifying a value for HealthCheckId is useful only when Route 53 is 			choosing between two or more resource record sets to respond to a DNS query, and you 			want Route 53 to base the choice in part on the status of a health check. Configuring 			health checks makes sense only in the following configurations:
    ///
    /// Non-alias resource record sets: You're 					checking the health of a group of non-alias resource record sets that have the 					same routing policy, name, and type (such as multiple weighted records named 					www.example.com with a type of A) and you specify health check IDs for all the 					resource record sets.         If the health check status for a resource record set is healthy, Route 53 					includes the record among the records that it responds to DNS queries 					with.        If the health check status for a resource record set is unhealthy, Route 53 					stops responding to DNS queries using the value for that resource record 					set.        If the health check status for all resource record sets in the group is 					unhealthy, Route 53 considers all resource record sets in the group healthy and 					responds to DNS queries accordingly.                         Alias resource record sets: You specify the 					following settings:                                                         You set EvaluateTargetHealth to true for an alias 							resource record set in a group of resource record sets that have the 							same routing policy, name, and type (such as multiple weighted records 							named www.example.com with a type of A).                     You configure the alias resource record set to route traffic to a 							non-alias resource record set in the same hosted zone.                     You specify a health check ID for the non-alias resource record set. 						                  If the health check status is healthy, Route 53 considers the alias resource 					record set to be healthy and includes the alias record among the records that it 					responds to DNS queries with.        If the health check status is unhealthy, Route 53 stops responding to DNS 					queries using the alias resource record set.        NoteThe alias resource record set can also route traffic to a 							group of non-alias resource record sets that have 						the same routing policy, name, and type. In that configuration, associate 						health checks with all of the resource record sets in the group of non-alias 						resource record sets.
    ///
    /// Geolocation Routing
    ///
    /// For geolocation resource record sets, if an endpoint is unhealthy, Route 53 looks for 			a resource record set for the larger, associated geographic region. For example, suppose 			you have resource record sets for a state in the United States, for the entire United 			States, for North America, and a resource record set that has * for 				CountryCode is *, which applies to all locations. If the 			endpoint for the state resource record set is unhealthy, Route 53 checks for healthy 			resource record sets in the following order until it finds a resource record set for 			which the endpoint is healthy:
    ///
    /// The United States               North America               The default resource record set
    ///
    /// Specifying the Health Check Endpoint by Domain 			Name
    ///
    /// If your health checks specify the endpoint only by domain name, we recommend that you 			create a separate health check for each endpoint. For example, create a health check for 			each HTTP server that is serving content for www.example.com. 			For the value of FullyQualifiedDomainName, specify the domain name of the 			server (such as us-east-2-www.example.com), not the name of the resource 			record sets (www.example.com).
    ///
    /// ImportantHealth check results will be unpredictable if you do the following:                                 Create a health check that has the same value for 							FullyQualifiedDomainName as the name of a resource record 						set.                  Associate that health check with the resource record set.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_id: Option<String>,

    ///
    /// The ID of the hosted zone that you want to create records in.
    ///
    /// Specify either HostedZoneName or HostedZoneId, but not both. If you have multiple hosted zones 			with the same domain name, you must specify the hosted zone using HostedZoneId.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 32
    ///
    /// Update requires: Replacement
    #[serde(rename = "HostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,

    ///
    /// The name of the hosted zone that you want to create records in. You must include a trailing dot (for example, www.example.com.) as part of       the HostedZoneName.
    ///
    /// When you create a stack using an AWS::Route53::RecordSet that specifies HostedZoneName, AWS CloudFormation attempts to find a hosted zone       whose name matches the HostedZoneName. If AWS CloudFormation cannot find a hosted zone with a matching domain name, or if there is more than one       hosted zone with the specified domain name, AWS CloudFormation will not create the stack.
    ///
    /// Specify either HostedZoneName or HostedZoneId, but not both. If you have multiple hosted zones 			with the same domain name, you must specify the hosted zone using HostedZoneId.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 32
    ///
    /// Update requires: Replacement
    #[serde(rename = "HostedZoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_name: Option<String>,

    ///
    /// Multivalue answer resource record sets only: To route traffic 			approximately randomly to multiple resources, such as web servers, create one multivalue 			answer record for each resource and specify true for 				MultiValueAnswer. Note the following:
    ///
    /// If you associate a health check with a multivalue answer resource record set, 					Amazon Route 53 responds to DNS queries with the corresponding IP address only 					when the health check is healthy.               If you don't associate a health check with a multivalue answer record, Route 					53 always considers the record to be healthy.               Route 53 responds to DNS queries with up to eight healthy records; if you have 					eight or fewer healthy records, Route 53 responds to all DNS queries with all 					the healthy records.               If you have more than eight healthy records, Route 53 responds to different 					DNS resolvers with different combinations of healthy records.               When all records are unhealthy, Route 53 responds to DNS queries with up to 					eight unhealthy records.               If a resource becomes unavailable after a resolver caches a response, client 					software typically tries another of the IP addresses in the response.
    ///
    /// You can't create multivalue answer alias records.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "MultiValueAnswer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_value_answer: Option<bool>,

    ///
    /// For ChangeResourceRecordSets requests, the name of the record that you 			want to create, update, or delete. For ListResourceRecordSets responses, 			the name of a record in the specified hosted zone.
    ///
    /// ChangeResourceRecordSets Only
    ///
    /// Enter a fully qualified domain name, for example, www.example.com. You 			can optionally include a trailing dot. If you omit the trailing dot, Amazon Route 53 			assumes that the domain name that you specify is fully qualified. This means that Route 			53 treats www.example.com (without a trailing dot) and 				www.example.com. (with a trailing dot) as identical.
    ///
    /// For information about how to specify characters other than a-z, 				0-9, and - (hyphen) and how to specify internationalized 			domain names, see DNS Domain Name 				Format in the Amazon Route 53 Developer Guide.
    ///
    /// You can use the asterisk (*) wildcard to replace the leftmost label in a domain name, 			for example, *.example.com. Note the following:
    ///
    /// The * must replace the entire label. For example, you can't specify 						*prod.example.com or prod*.example.com.               The * can't replace any of the middle labels, for example, 					marketing.*.example.com.               If you include * in any position other than the leftmost label in a domain 					name, DNS treats it as an * character (ASCII 42), not as a wildcard.        ImportantYou can't use the * wildcard for resource records sets that have a type of 						NS.
    ///
    /// You can use the * wildcard as the leftmost label in a domain name, for example, 				*.example.com. You can't use an * for one of the middle labels, for 			example, marketing.*.example.com. In addition, the * must replace the 			entire label; for example, you can't specify prod*.example.com.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// Latency-based resource record sets only: The Amazon EC2 Region 			where you created the resource that this resource record set refers to. The resource 			typically is an AWS resource, such as an EC2 instance or an ELB load 			balancer, and is referred to by an IP address or a DNS domain name, depending on the 			record type.
    ///
    /// When Amazon Route 53 receives a DNS query for a domain name and type for which you 			have created latency resource record sets, Route 53 selects the latency resource record 			set that has the lowest latency between the end user and the associated Amazon EC2 			Region. Route 53 then returns the value that is associated with the selected resource 			record set.
    ///
    /// Note the following:
    ///
    /// You can only specify one ResourceRecord per latency resource 					record set.               You can only create one latency resource record set for each Amazon EC2 					Region.               You aren't required to create latency resource record sets for all Amazon EC2 					Regions. Route 53 will choose the region with the best latency from among the 					regions that you create latency resource record sets for.               You can't create non-latency resource record sets that have the same values 					for the Name and Type elements as latency resource 					record sets.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: af-south-1 | ap-east-1 | ap-northeast-1 | ap-northeast-2 | ap-northeast-3 | ap-south-1 | ap-southeast-1 | ap-southeast-2 | ap-southeast-3 | ca-central-1 | cn-north-1 | cn-northwest-1 | eu-central-1 | eu-north-1 | eu-south-1 | eu-west-1 | eu-west-2 | eu-west-3 | me-south-1 | sa-east-1 | us-east-1 | us-east-2 | us-west-1 | us-west-2
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<RecordSetRegionEnum>,

    ///
    /// One or more values that correspond with the value that you specified for the Type property. For example, if you specified 			A for Type, you specify one or more IP addresses in IPv4 format for ResourceRecords. 			For information about the format of values for each record type, see 			Supported DNS Resource Record Types 			in the Amazon Route 53 Developer Guide.
    ///
    /// Note the following:
    ///
    /// You can specify more than one value for all record types except CNAME and SOA.The maximum length of a value is 4000 characters.If you're creating an alias record, omit ResourceRecords.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_records: Option<Vec<String>>,

    ///
    /// Resource record sets that have a routing policy other than 				simple: An identifier that differentiates among multiple resource record 			sets that have the same combination of name and type, such as multiple weighted resource 			record sets named acme.example.com that have a type of A. In a group of resource record 			sets that have the same name and type, the value of SetIdentifier must be 			unique for each resource record set.
    ///
    /// For information about routing policies, see Choosing a Routing 				Policy in the Amazon Route 53 Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "SetIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_identifier: Option<String>,

    ///
    /// The resource record cache time to live (TTL), in seconds. Note the following:
    ///
    /// If you're creating or updating an alias resource record set, omit 						TTL. Amazon Route 53 uses the value of TTL for the 					alias target.               If you're associating this resource record set with a health check (if you're 					adding a HealthCheckId element), we recommend that you specify a 						TTL of 60 seconds or less so clients respond quickly to changes 					in health status.               All of the resource record sets in a group of weighted resource record sets 					must have the same value for TTL.               If a group of weighted resource record sets includes one or more weighted 					alias resource record sets for which the alias target is an ELB load balancer, 					we recommend that you specify a TTL of 60 seconds for all of the 					non-alias weighted resource record sets that have the same name and type. Values 					other than 60 seconds (the TTL for load balancers) will change the effect of the 					values that you specify for Weight.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,

    ///
    /// The DNS record type. For information about different record types and how data is 			encoded for them, see Supported DNS Resource 				Record Types in the Amazon Route 53 Developer 			Guide.
    ///
    /// Valid values for basic resource record sets: A | AAAA | 				CAA | CNAME | DS |MX | 				NAPTR | NS | PTR | SOA | 				SPF | SRV | TXT
    ///
    /// Values for weighted, latency, geolocation, and failover resource record sets: 				A | AAAA | CAA | CNAME | 				MX | NAPTR | PTR | SPF | 				SRV | TXT. When creating a group of weighted, latency, 			geolocation, or failover resource record sets, specify the same value for all of the 			resource record sets in the group.
    ///
    /// Valid values for multivalue answer resource record sets: A | 				AAAA | MX | NAPTR | PTR | 				SPF | SRV | TXT
    ///
    /// NoteSPF records were formerly used to verify the identity of the sender of email 				messages. However, we no longer recommend that you create resource record sets for 				which the value of Type is SPF. RFC 7208, Sender 					Policy Framework (SPF) for Authorizing Use of Domains in Email, Version 					1, has been updated to say, "...[I]ts existence and mechanism defined 				in [RFC4408] have led to some interoperability issues. Accordingly, its use is no 				longer appropriate for SPF version 1; implementations are not to use it." In RFC 				7208, see section 14.1, The SPF DNS Record Type.
    ///
    /// Values for alias resource record sets:
    ///
    /// Amazon API Gateway custom regional APIs and 						edge-optimized APIs:          A                                CloudFront distributions:          A                If IPv6 is enabled for the distribution, create two resource record sets to 					route traffic to your distribution, one with a value of A and one 					with a value of AAAA.                         Amazon API Gateway environment that has a regionalized 						subdomain: A                                ELB load balancers:          A | AAAA                                Amazon S3 buckets:          A                                Amazon Virtual Private Cloud interface VPC 						endpoints          A                                Another resource record set in this hosted 						zone: Specify the type of the resource record set that you're 					creating the alias for. All values are supported except NS and 						SOA.        NoteIf you're creating an alias record that has the same name as the hosted 						zone (known as the zone apex), you can't route traffic to a record for which 						the value of Type is CNAME. This is because the 						alias record must have the same type as the record you're routing traffic 						to, and creating a CNAME record for the zone apex isn't supported even for 						an alias record.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: A | AAAA | CAA | CNAME | DS | MX | NAPTR | NS | PTR | SOA | SPF | SRV | TXT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: RecordSetTypeEnum,

    ///
    /// Weighted resource record sets only: Among resource record sets 			that have the same combination of DNS name and type, a value that determines the 			proportion of DNS queries that Amazon Route 53 responds to using the current resource 			record set. Route 53 calculates the sum of the weights for the resource record sets that 			have the same combination of DNS name and type. Route 53 then responds to queries based 			on the ratio of a resource's weight to the total. Note the following:
    ///
    /// You must specify a value for the Weight element for every 					weighted resource record set.               You can only specify one ResourceRecord per weighted resource 					record set.               You can't create latency, failover, or geolocation resource record sets that 					have the same values for the Name and Type elements as 					weighted resource record sets.               You can create a maximum of 100 weighted resource record sets that have the 					same values for the Name and Type elements.               For weighted (but not weighted alias) resource record sets, if you set 						Weight to 0 for a resource record set, Route 53 					never responds to queries with the applicable value for that resource record 					set. However, if you set Weight to 0 for all resource 					record sets that have the same combination of DNS name and type, traffic is 					routed to all resources with equal probability.        The effect of setting Weight to 0 is different when 					you associate health checks with weighted resource record sets. For more 					information, see Options for Configuring Route 53 Active-Active and Active-Passive 						Failover in the Amazon Route 53 Developer 					Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RecordSetFailoverEnum {
    /// PRIMARY
    #[serde(rename = "PRIMARY")]
    Primary,

    /// SECONDARY
    #[serde(rename = "SECONDARY")]
    Secondary,
}

impl Default for RecordSetFailoverEnum {
    fn default() -> Self {
        RecordSetFailoverEnum::Primary
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RecordSetRegionEnum {
    /// af-south-1
    #[serde(rename = "af-south-1")]
    Afsouth1,

    /// ap-east-1
    #[serde(rename = "ap-east-1")]
    Apeast1,

    /// ap-northeast-1
    #[serde(rename = "ap-northeast-1")]
    Apnortheast1,

    /// ap-northeast-2
    #[serde(rename = "ap-northeast-2")]
    Apnortheast2,

    /// ap-northeast-3
    #[serde(rename = "ap-northeast-3")]
    Apnortheast3,

    /// ap-south-1
    #[serde(rename = "ap-south-1")]
    Apsouth1,

    /// ap-southeast-1
    #[serde(rename = "ap-southeast-1")]
    Apsoutheast1,

    /// ap-southeast-2
    #[serde(rename = "ap-southeast-2")]
    Apsoutheast2,

    /// ap-southeast-3
    #[serde(rename = "ap-southeast-3")]
    Apsoutheast3,

    /// ca-central-1
    #[serde(rename = "ca-central-1")]
    Cacentral1,

    /// cn-north-1
    #[serde(rename = "cn-north-1")]
    Cnnorth1,

    /// cn-northwest-1
    #[serde(rename = "cn-northwest-1")]
    Cnnorthwest1,

    /// eu-central-1
    #[serde(rename = "eu-central-1")]
    Eucentral1,

    /// eu-north-1
    #[serde(rename = "eu-north-1")]
    Eunorth1,

    /// eu-south-1
    #[serde(rename = "eu-south-1")]
    Eusouth1,

    /// eu-west-1
    #[serde(rename = "eu-west-1")]
    Euwest1,

    /// eu-west-2
    #[serde(rename = "eu-west-2")]
    Euwest2,

    /// eu-west-3
    #[serde(rename = "eu-west-3")]
    Euwest3,

    /// me-south-1
    #[serde(rename = "me-south-1")]
    Mesouth1,

    /// sa-east-1
    #[serde(rename = "sa-east-1")]
    Saeast1,

    /// us-east-1
    #[serde(rename = "us-east-1")]
    Useast1,

    /// us-east-2
    #[serde(rename = "us-east-2")]
    Useast2,

    /// us-west-1
    #[serde(rename = "us-west-1")]
    Uswest1,

    /// us-west-2
    #[serde(rename = "us-west-2")]
    Uswest2,
}

impl Default for RecordSetRegionEnum {
    fn default() -> Self {
        RecordSetRegionEnum::Afsouth1
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RecordSetTypeEnum {
    /// A
    #[serde(rename = "A")]
    A,

    /// AAAA
    #[serde(rename = "AAAA")]
    Aaaa,

    /// CAA
    #[serde(rename = "CAA")]
    Caa,

    /// CNAME
    #[serde(rename = "CNAME")]
    Cname,

    /// DS
    #[serde(rename = "DS")]
    Ds,

    /// MX
    #[serde(rename = "MX")]
    Mx,

    /// NAPTR
    #[serde(rename = "NAPTR")]
    Naptr,

    /// NS
    #[serde(rename = "NS")]
    Ns,

    /// PTR
    #[serde(rename = "PTR")]
    Ptr,

    /// SOA
    #[serde(rename = "SOA")]
    Soa,

    /// SPF
    #[serde(rename = "SPF")]
    Spf,

    /// SRV
    #[serde(rename = "SRV")]
    Srv,

    /// TXT
    #[serde(rename = "TXT")]
    Txt,
}

impl Default for RecordSetTypeEnum {
    fn default() -> Self {
        RecordSetTypeEnum::A
    }
}

impl cfn_resources::CfnResource for RecordSet {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.alias_target
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.cidr_routing_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.geo_location
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.health_check_id {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'health_check_id'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.hosted_zone_id {
            if the_val.len() > 32 as _ {
                return Err(format!(
                    "Max validation failed on field 'hosted_zone_id'. {} is greater than 32",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.hosted_zone_name {
            if the_val.len() > 32 as _ {
                return Err(format!(
                    "Max validation failed on field 'hosted_zone_name'. {} is greater than 32",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.name;

        if the_val.len() > 1024 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 1024",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.set_identifier {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'set_identifier'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.set_identifier {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'set_identifier'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
