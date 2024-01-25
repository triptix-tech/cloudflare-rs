use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiResult;

use serde::Deserialize;

/// Delete Pool
/// <https://api.cloudflare.com/#account-load-balancer-pools-delete-pool>
#[derive(Debug)]
pub struct DeletePool<'a> {
    /// Which pool to delete.
    pub identifier: &'a str,
}

impl<'a> EndpointSpec<Response> for DeletePool<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!("user/load_balancers/pools/{}", self.identifier)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Response {
    pub id: String,
}
impl ApiResult for Response {}
