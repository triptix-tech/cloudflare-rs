use crate::endpoints::load_balancing::Pool;
use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiResult;

/// List Load Balancers
/// <https://api.cloudflare.com/#load-balancers-list-load-balancers>
#[derive(Debug)]
pub struct ListPools {}

impl EndpointSpec<Vec<Pool>> for ListPools {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("user/load_balancers/pools")
    }
}

impl ApiResult for Vec<Pool> {}
