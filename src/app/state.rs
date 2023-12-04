use crate::app::database::Database;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub database: Database,
}
