use crate::entity::context::BatchContext;
use crate::entity::job::OnJob;

pub struct Batch {
    batch_context: BatchContext,
    jobs: Vec<Box<dyn OnJob>>
}

impl Batch {
    pub fn new(name: Option<String>, jobs: Vec<Box<dyn OnJob>>) -> Self {
        Batch {
            batch_context: BatchContext::new(name),
            jobs
        }
    }

    pub fn run(&mut self) {
        for job in &mut self.jobs {
            job.on_job();
        }
    }
}
