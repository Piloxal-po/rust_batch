use std::collections::HashSet;
use std::hash::RandomState;

use chrono::Local;
use uuid::Uuid;

use crate::entity::step::step::Step;
use crate::entity::utils::times::TimeKeeper;

pub struct Context {
    uid: u128,
    pub env: HashSet<String, RandomState>,
    pub time_keeper: TimeKeeper,
    pub name: String,
}

impl Context {
    pub fn new(name: Option<String>) -> Self {
        let uuid = Uuid::new_v4().as_u128();
        Context {
            uid: uuid,
            name: name.unwrap_or(uuid.to_string()),
            env: HashSet::new(),
            time_keeper: *TimeKeeper::new().start(),
        }
    }
}

pub struct JobContext {
    pub context: Context,
    pub current_step: Step
}

impl JobContext {
    pub fn new(name: Option<String>) -> Self {
        JobContext {
            context: Context::new(name),
            current_step: Step::Starting
        }
    }

    pub fn update_current_step(&mut self, step: Step) {
        self.current_step = step;
        self.context.time_keeper.update(Local::now());
    }
}

pub struct BatchContext {
    pub context: Context,
    pub current_job: Option<JobContext>
}

impl BatchContext {
    pub fn new(name: Option<String>) -> Self {
        BatchContext {
            context: Context::new(name),
            current_job: None
        }
    }

    pub fn update_current_job(&mut self, job_context: JobContext)  {
        self.current_job = Some(job_context);
        self.context.time_keeper.update(Local::now());
    }
}
