pub mod mycelink_add_contact;
pub mod mycelink_create_account;

use crate::db::actions::contact_actions::ContactId;
use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::{DBConnector, NoTenant, TenantState};
use crate::model::chat::Chat;
use crate::model::config::Config;
use crate::model::contact::ContactDisplay;
use crate::model::messenger_service::{PollError, PollableService};
use crate::model::protocol_config::{Protocol, ProtocolConfig};
use crate::mycelink::mycelink_account::MycelinkAccount;
use crate::mycelink::mycelink_service::MycelinkService;
use futures::future::join_all;
use futures::{Stream, StreamExt};
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpStream;

pub struct APIConnector<T: TenantState> {
    db_connector: DBConnector<T>,
    fcp_connector: Arc<FCPConnector>,
    messenger_services: Vec<PollableService>,
}

pub trait LoginStatus {}

type NotSignedIn = ();
type SignedIn = MycelinkAccount;

impl LoginStatus for NotSignedIn {}

impl LoginStatus for SignedIn {}

impl APIConnector<NoTenant> {
    pub async fn enter_tenant(self, tenant: Tenant) -> APIConnector<Tenant> {
        let mut res = APIConnector {
            db_connector: self.db_connector.enter_tenant(tenant),
            fcp_connector: self.fcp_connector,
            messenger_services: self.messenger_services,
        };

        let mut protocol_configs = res.db_connector.get_protocol_configs().await;

        while let Some(Ok(protocol_config)) = protocol_configs.next().await {
            match protocol_config {
                ProtocolConfig::Mycelink { account } => {
                    res.messenger_services
                        .push(PollableService::MycelinkService(MycelinkService::new(
                            res.db_connector.clone(),
                            res.fcp_connector.clone(),
                            account,
                        )))
                }
            }
        }

        drop(protocol_configs);

        res
    }

    pub async fn poll_chats(&self) -> Result<(), PollError> {
        let futures = join_all(self.messenger_services.iter().map(|e| e.poll())).await;
        futures.into_iter().fold(Ok(()), |acc, e| acc.and(e))
    }

    pub async fn create_tenant(&self, name: &str) -> sqlx::Result<Tenant> {
        if !self.db_connector.has_tenant(name).await? {
            return self.db_connector.create_tenant(name).await;
        }
        Err(sqlx::error::Error::RowNotFound)
    }

    pub async fn list_tenants(&self) -> impl Stream<Item = sqlx::Result<Tenant>> + '_ {
        self.db_connector.get_tenants().await
    }

    pub async fn init(config: &Config) -> Result<APIConnector<NoTenant>, Box<dyn Error>> {
        let fcp_connector =
            FCPConnector::new(TcpStream::connect(config.fcp_endpoint).await?, "Mycelink").await?;
        let db_connector =
            DBConnector::new(config.database_path.as_os_str().to_str().unwrap()).await?;

        Ok(Self {
            db_connector,
            fcp_connector: Arc::new(fcp_connector),
            messenger_services: Vec::new(),
        })
    }

    pub async fn enter_demo(self) -> sqlx::Result<APIConnector<Tenant>> {
        if !self.db_connector.has_tenant("demo").await? {
            self.db_connector.create_tenant("demo").await?;
        }
        Ok(self.enter_tenant(Tenant::new("demo")).await)
    }
}

impl<T: TenantState> APIConnector<T> {
    pub fn health_check(&self) -> Result<(), ()> {
        Ok(())
    }
}

impl APIConnector<Tenant> {
    pub async fn get_enabled_protocols(
        &self,
    ) -> impl Stream<Item = sqlx::Result<ProtocolConfig>> + '_ {
        self.db_connector.get_protocol_configs().await
    }

    pub async fn create_direct_chat(&self, contact_id: ContactId) -> Result<Chat, ()> {
        todo!()
    }

    pub async fn list_chats(&self) -> impl Stream<Item = sqlx::Result<Chat>> + '_ {
        self.db_connector
            .list_chats(self.messenger_services.as_slice())
            .await
    }

    pub async fn current_mycelink_account_request_key(&self) -> Result<Box<str>, ()> {
        let res = self
            .db_connector
            .get_protocol_config(Protocol::Mycelink)
            .await;

        match res {
            Ok(ok) => ok.ok_or(()).map(|e| {
                TryInto::<MycelinkAccount>::try_into(e)
                    .unwrap()
                    .request_ssk_key()
                    .into()
            }),
            Err(_) => Err(()),
        }
    }

    pub async fn list_contacts(&self) -> impl Stream<Item = sqlx::Result<ContactDisplay>> + '_ {
        self.db_connector.list_contacts().await
    }
}
