use crate::domain::interface;
use crate::domain::service;
use crate::infra;
use crate::serviceclient;
use std::sync::Arc;

#[derive(Clone)]
pub struct Infras {
    pub db: infra::DBConnector,
    pub hash_manager: Arc<infra::HashManager>,
    pub jwt_handler: Arc<infra::JWTHandler>,
}

pub fn infras(database_url: String, private_key: &str) -> Infras {
    let db = actix::SyncArbiter::start(3, move || infra::DBExecutor::new(database_url.clone()));

    Infras {
        db: infra::DBConnector::new(db),
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
            infras.db.clone(),
        )),
        login_repository: Arc::new(serviceclient::user_login_repo::UserLoginRepository::new(
            infras.db.clone(),
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

pub fn new(database_url: String, private_key: &str) -> AppContext {
    let i = infras(database_url, private_key);
    let sc = serviceclients(&i);
    let s = services(&i, &sc);

    AppContext {
        infras: i,
        serviceclients: sc,
        services: s,
    }
}
