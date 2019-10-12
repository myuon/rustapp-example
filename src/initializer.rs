use crate::domain::interface;
use crate::domain::service;
use crate::infra::connection_pool;
use std::sync::Arc;

#[derive(Clone)]
pub struct Infras {
    pub ConnPool: connection_pool::MySQLConnPool,
}

pub fn infras() -> Infras {
    unimplemented!()
}

#[derive(Clone)]
pub struct ServiceClients {
    pub userRepository: Arc<dyn interface::IUserRepository + Send + Sync>,
}

pub fn serviceclients() -> ServiceClients {
    unimplemented!()
}

#[derive(Clone)]
pub struct Services {
    pub userService: service::UserService,
}

pub fn services() -> Services {
    unimplemented!()
}

#[derive(Clone)]
pub struct Initializer {
    pub infras: Infras,
    pub serviceclients: ServiceClients,
    pub services: Services,
}
