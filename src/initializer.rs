use crate::domain::interface;
use crate::domain::service;
use crate::infra::connection_pool;
use crate::serviceclient;
use std::sync::Arc;

#[derive(Clone)]
pub struct Infras {
    pub ConnPool: connection_pool::MySQLConnPool,
}

pub fn infras(database_url: String) -> Infras {
    Infras {
        ConnPool: connection_pool::MySQLConnPool::new(database_url),
    }
}

#[derive(Clone)]
pub struct ServiceClients {
    pub userRepository: Arc<dyn interface::IUserRepository + Send + Sync>,
}

pub fn serviceclients(infras: &Infras) -> ServiceClients {
    ServiceClients {
        userRepository: Arc::new(serviceclient::user_repo::UserRepository::new(
            infras.ConnPool.clone(),
        )),
    }
}

#[derive(Clone)]
pub struct Services {
    pub userService: service::UserService,
}

pub fn services(serviceclients: &ServiceClients) -> Services {
    Services {
        userService: service::UserService::new(serviceclients.userRepository.clone()),
    }
}

#[derive(Clone)]
pub struct AppContext {
    pub infras: Infras,
    pub serviceclients: ServiceClients,
    pub services: Services,
}

pub fn new(database_url: String) -> AppContext {
    let i = infras(database_url);
    let sc = serviceclients(&i);
    let s = services(&sc);

    AppContext {
        infras: i,
        serviceclients: sc,
        services: s,
    }
}
