use crate::entity::context::JobContext;
use crate::entity::step::listener::ReaderListener;
use crate::entity::step::step::Step;

pub trait ItemReader<T> {
    fn read(&self, job_context: &mut JobContext) -> Vec<T>;

    fn run(&self, job_context: &mut JobContext, reader_listener: &ReaderListener<T>) -> Vec<T> {
        job_context.update_current_step(Step::StartStepReading);
        if reader_listener.before_read.is_some() {
            reader_listener.before_read.unwrap()(job_context);
        }

        job_context.update_current_step(Step::StartReading);
        let ret = self.read(job_context);
        job_context.update_current_step(Step::EndReading);

        if reader_listener.after_read.is_some() {
            reader_listener.after_read.unwrap()(job_context, &ret);
        }
        job_context.update_current_step(Step::EndStepReading);
        ret
    }
}

pub struct Reader<T> {
    pub on_read: fn(&mut JobContext) -> Vec<T>,
}

impl<T> Reader<T> {
    pub fn new(on_read: fn(&mut JobContext) -> Vec<T>) -> Reader<T> {
        Reader { on_read }
    }
}

impl<T> ItemReader<T> for Reader<T> {
    fn read(&self, job_context: &mut JobContext) -> Vec<T> {
        (self.on_read)(job_context)
    }
}
