use crate::domain::interface;
use crate::domain::service;
use crate::infra;
use crate::serviceclient;
use std::sync::Arc;

#[derive(Clone)]
pub struct Infras {
    pub conn_pool: infra::MySQLConnPool,
    pub hash_manager: Arc<infra::HashManager>,
    pub jwt_handler: Arc<infra::JWTHandler>,
}

pub fn infras(database_url: String, private_key: Vec<u8>) -> Infras {
    Infras {
        conn_pool: infra::MySQLConnPool::new(database_url),
        hash_manager: Arc::new(infra::HashManager::new()),
        jwt_handler: Arc::new(infra::JWTHandler::new(private_key)),
    }
}

#[derive(Clone)]
pub struct ServiceClients {
    pub user_repository: Arc<dyn interface::IUserRepository + Send + Sync>,
    pub login_repository: Arc<dyn interface::IUserLoginRepository + Send + Sync>,
}

pub fn serviceclients(infras: &Infras) -> ServiceClients {
    ServiceClients {
        user_repository: Arc::new(serviceclient::user_repo::UserRepository::new(
            infras.conn_pool.clone(),
        )),
        login_repository: Arc::new(serviceclient::user_login_repo::UserLoginRepository::new(
            infras.conn_pool.clone(),
        )),
    }
}

#[derive(Clone)]
pub struct Services {
    pub user_service: service::UserService,
    pub login_service: service::LoginService,
}

pub fn services(infras: &Infras, serviceclients: &ServiceClients) -> Services {
    Services {
        user_service: service::UserService::new(serviceclients.user_repository.clone()),
        login_service: service::LoginService::new(
            serviceclients.login_repository.clone(),
            infras.hash_manager.clone(),
            infras.jwt_handler.clone(),
        ),
    }
}

#[derive(Clone)]
pub struct AppContext {
    pub infras: Infras,
    pub serviceclients: ServiceClients,
    pub services: Services,
}

pub fn new(database_url: String, private_key: Vec<u8>) -> AppContext {
    let i = infras(database_url, private_key);
    let sc = serviceclients(&i);
    let s = services(&i, &sc);

    AppContext {
        infras: i,
        serviceclients: sc,
        services: s,
    }
}
