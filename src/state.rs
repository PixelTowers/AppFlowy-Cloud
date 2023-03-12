use crate::component::auth::LoggedUser;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct State {
    pub pg_pool: PgPool,
    pub cache: Arc<RwLock<Cache>>,
}

impl State {
    pub async fn load_users(_pool: &PgPool) {
        todo!()
    }
}

#[derive(Clone, Debug, Copy)]
enum AuthStatus {
    Authorized(DateTime<Utc>),
    NotAuthorized,
}

pub const EXPIRED_DURATION_DAYS: i64 = 30;

#[derive(Debug, Default)]
pub struct Cache {
    user: BTreeMap<LoggedUser, AuthStatus>,
}

impl Cache {
    pub fn new() -> Self {
        Cache::default()
    }

    pub fn is_authorized(&self, user: &LoggedUser) -> bool {
        match self.user.get(user) {
            None => {
                tracing::debug!("user not login yet or server was reboot");
                false
            }
            Some(status) => match *status {
                AuthStatus::Authorized(last_time) => {
                    let current_time = Utc::now();
                    let days = (current_time - last_time).num_days();
                    days < EXPIRED_DURATION_DAYS
                }
                AuthStatus::NotAuthorized => {
                    tracing::debug!("user logout already");
                    false
                }
            },
        }
    }

    pub fn authorized(&mut self, user: LoggedUser) {
        self.user.insert(user, AuthStatus::Authorized(Utc::now()));
    }

    pub fn unauthorized(&mut self, user: LoggedUser) {
        self.user.insert(user, AuthStatus::NotAuthorized);
    }
}
