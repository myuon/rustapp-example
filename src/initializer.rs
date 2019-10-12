use crate::domain::interface;
use crate::domain::service;
use crate::infra::connection_pool;
use crate::serviceclient;
use std::sync::Arc;

#[derive(Clone)]
pub struct Infras {
    pub conn_pool: connection_pool::MySQLConnPool,
}

pub fn infras(database_url: String) -> Infras {
    Infras {
        conn_pool: connection_pool::MySQLConnPool::new(database_url),
    }
}

#[derive(Clone)]
pub struct ServiceClients {
    pub user_repository: Arc<dyn interface::IUserRepository + Send + Sync>,
}

pub fn serviceclients(infras: &Infras) -> ServiceClients {
    ServiceClients {
        user_repository: Arc::new(serviceclient::user_repo::UserRepository::new(
            infras.conn_pool.clone(),
        )),
    }
}

#[derive(Clone)]
pub struct Services {
    pub user_service: service::UserService,
}

pub fn services(serviceclients: &ServiceClients) -> Services {
    Services {
        user_service: service::UserService::new(serviceclients.user_repository.clone()),
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
