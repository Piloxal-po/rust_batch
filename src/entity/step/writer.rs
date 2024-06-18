use crate::entity::context::JobContext;
use crate::entity::step::listener::WriterListener;
use crate::entity::step::step::Step;

pub trait ItemWriter<T> {
    fn write(
        &self,
        job_context: &mut JobContext,
        item: &T,
    );

    fn run(&self, job_context: &mut JobContext, items: &[T], writer_listener: &WriterListener<T>) {
        job_context.update_current_step(Step::StartStepWriting);
        if writer_listener.before_chunk.is_some() {
            writer_listener.before_chunk.unwrap()(job_context, items);
        }
        self.run_with_chunk(job_context, items, writer_listener);
        if writer_listener.after_chunk.is_some() {
            writer_listener.after_chunk.unwrap()(job_context, items);
        }
        job_context.update_current_step(Step::EndStepWriting);
    }

    fn run_with_chunk(&self, job_context: &mut JobContext, items: &[T], writer_listener: &WriterListener<T>) {
        for item in items {
            if writer_listener.before_write.is_some() {
                writer_listener.before_write.unwrap()(job_context, item);
            }
            job_context.update_current_step(Step::StartWriting);
            self.write(job_context, item);
            job_context.update_current_step(Step::EndWriting);
            if writer_listener.after_write.is_some() {
                writer_listener.after_write.unwrap()(job_context, item);
            }
        }
    }
}

pub struct Writer<T> {
    pub on_write: fn(&JobContext, &T),
}

impl<T> ItemWriter<T> for Writer<T> {
    fn write(&self, job_context: &mut JobContext, item: &T) {
        (self.on_write)(job_context, item);
    }
}

impl<T> Writer<T> {
    pub fn new(on_write: fn(&JobContext, &T)) -> Writer<T> {
        Writer { on_write }
    }
}

pub struct ManyWriter<T> {
    pub writers: Vec<Box<dyn ItemWriter<T>>>,
}

impl<T> ItemWriter<T> for ManyWriter<T> {
    fn write(&self, job_context: &mut JobContext, item: &T) {
        for writer in &self.writers {
            writer.write(job_context, item);
        }
    }
}

impl<T> ManyWriter<T> {
    pub fn new(writers: Vec<Box<dyn ItemWriter<T>>>) -> ManyWriter<T> {
        ManyWriter { writers }
    }
}
