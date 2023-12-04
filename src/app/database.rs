use crate::app::model::{FilterType, Status};
use chrono::NaiveDateTime;
use image::DynamicImage;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub type Database = Arc<RwLock<DatabasePrivate>>;

impl DatabasePrivate {
    // Create a new job
    pub fn create_job(&mut self, id: Uuid, filter: FilterType) -> Uuid {
        let job = Job {
            id,
            filter,
            status: Status::Processing,
            start_time: chrono::Utc::now().naive_utc(),
            end_time: None,
            result: None,
        };
        self.0.insert(id, job);
        id
    }

    pub fn update_result(&mut self, id: Uuid, status: Status, result: Option<DynamicImage>) {
        let job = self.0.get_mut(&id).unwrap();
        job.status = status;
        job.result = result;
        job.end_time = Some(chrono::Utc::now().naive_utc());
    }
}

#[derive(Debug, Default)]
pub struct DatabasePrivate(pub HashMap<Uuid, Job>);

impl Deref for DatabasePrivate {
    type Target = HashMap<Uuid, Job>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Job {
    pub id: Uuid,
    pub filter: FilterType,
    pub status: Status,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub result: Option<DynamicImage>,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Job {}

impl Hash for Job {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Into<crate::app::model::Job> for Job {
    fn into(self) -> crate::app::model::Job {
        crate::app::model::Job {
            id: self.id,
            filter: self.filter,
            status: self.status,
            start_time: self.start_time,
            end_time: self.end_time,
        }
    }
}
