use crate::endpoints::load_balancing::{
    LbPoolId, LbPoolMapping, LoadBalancer, SessionAffinity, SessionAffinityAttributes,
    SteeringPolicy,
};
use crate::framework::endpoint::{EndpointSpec, Method};

use serde::Serialize;

/// Create Load Balancer
/// <https://api.cloudflare.com/#load-balancers-create-load-balancer>
#[derive(Debug)]
pub struct PatchLoadBalancer<'a> {
    /// The Zone to which this Load Balancer belongs.
    pub zone_identifier: &'a str,
    /// Which load balancer to patch.
    pub identifier: &'a str,
    /// Parameters for the API call
    pub params: Params<'a>,
}

/// Mandatory parameters for creating a Load Balancer.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct Params<'a> {
    /// The DNS hostname to associate with your Load Balancer.
    /// If this hostname already exists as a DNS record in Cloudflare's DNS,
    /// the Load Balancer will take precedence and the DNS record will not be used.
    pub name: Option<&'a str>,
    /// The list of LB Pools (by their IDs) ordered by their failover priority.
    pub default_pools: Option<&'a [LbPoolId]>,
    /// The LB Pool ID to use when all other pools are detected as unhealthy.
    pub fallback_pool: Option<&'a LbPoolId>,
    /// Object description.
    /// Example: Load Balancer for www.example.com
    pub description: Option<&'a str>,
    /// Time to live (TTL) of the DNS entry for the IP address returned by this load balancer. This
    /// only applies to gray-clouded (unproxied) load balancers.
    pub ttl: Option<u32>,
    /// A mapping of Cloudflare PoP identifiers to a list of pool IDs (ordered by their failover
    /// priority) for the PoP (datacenter). Any PoPs not explicitly defined will fall back to using
    /// default_pools.
    pub pop_pools: Option<LbPoolMapping>,
    /// A mapping of region/country codes to a list of pool IDs (ordered by their failover priority)
    /// for the given region. Any regions not explicitly defined will fall back to using
    /// default_pools.
    pub region_pools: Option<LbPoolMapping>,
    pub proxied: Option<bool>,
    pub steering_policy: Option<SteeringPolicy>,
    pub session_affinity: Option<SessionAffinity>,
    pub session_affinity_attributes: Option<SessionAffinityAttributes>,
    pub session_affinity_ttl: Option<u32>,
}

impl<'a> EndpointSpec<LoadBalancer> for PatchLoadBalancer<'a> {
    fn method(&self) -> Method {
        Method::PATCH
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/load_balancers/{}",
            self.zone_identifier, self.identifier
        )
    }
    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}
