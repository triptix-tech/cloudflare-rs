use crate::endpoints::load_balancing::Pool;
use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiResult;

/// List Load Balancers
/// <https://api.cloudflare.com/#load-balancers-list-load-balancers>
#[derive(Debug)]
pub struct ListPools<'a> {
    /// The Zone to list Load Balancers from.
    pub zone_identifier: &'a str,
}

impl<'a> EndpointSpec<Vec<Pool>> for ListPools<'a> {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("accounts/{}/load_balancers/pools", self.zone_identifier)
    }
}

impl ApiResult for Vec<Pool> {}
