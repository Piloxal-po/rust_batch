use crate::entity::context::JobContext;
use crate::entity::step::listener::ProcessorListener;
use crate::entity::step::step::Step;

pub trait ItemProcessor<T, U> {
    fn process(&self, job_context: &mut JobContext, item: &T) -> U;

    fn run(&self, job_context: &mut JobContext, items: &[T], processor_listener: &ProcessorListener<T, U>) -> Vec<U> {
        job_context.update_current_step(Step::StartStepProcessing);
        if processor_listener.before_chunk.is_some() {
            processor_listener.before_chunk.unwrap()(job_context, items);
        }
        self.run_with_chunk(job_context, items, processor_listener)
    }

    fn run_with_chunk(&self, job_context: &mut JobContext, items: &[T], processor_listener: &ProcessorListener<T, U>) -> Vec<U> {
        let process_items : Vec<U> = items.iter()
            .map(|item| {
                if processor_listener.before_process.is_some() {
                    processor_listener.before_process.unwrap()(job_context, item);
                }
                job_context.update_current_step(Step::StartProcessing);
                let process_item = self.process(job_context, item);
                job_context.update_current_step(Step::EndProcessing);
                if processor_listener.after_process.is_some() {
                    processor_listener.after_process.unwrap()(job_context, &process_item);
                }
                process_item
            })
            .collect();
        if processor_listener.after_chunk.is_some() {
            processor_listener.after_chunk.unwrap()(job_context, process_items.as_slice());
        }
        job_context.update_current_step(Step::EndStepProcessing);
        process_items
    }
}

pub struct Processor<T, U> {
    pub on_process: fn(&JobContext, &T) -> U,
}

impl<T, U> ItemProcessor<T, U> for Processor<T, U> {
    fn process(&self, job_context: &mut JobContext, item: &T) -> U {
        (self.on_process)(job_context, item)
    }
}

impl<T, U> Processor<T, U> {
    pub fn new(on_process: fn(&JobContext, &T) -> U) -> Processor<T, U> {
        Processor { on_process }
    }
}

