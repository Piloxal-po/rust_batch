use crate::entity::context::JobContext;
use crate::entity::step::listener::{JobListener, Listener, ProcessorListener, ReaderListener, WriterListener};
use crate::entity::step::processor::ItemProcessor;
use crate::entity::step::reader::ItemReader;
use crate::entity::step::step::Step;
use crate::entity::step::writer::ItemWriter;

pub trait OnJob {
    fn on_job(&mut self);
}

pub struct Job<T, U> {
    pub reader: Box<dyn ItemReader<T>>,
    pub processor: Box<dyn ItemProcessor<T, U>>,
    pub writer: Box<dyn ItemWriter<U>>,
    pub chunk: Option<usize>,
    pub listener: Option<Listener<T, U>>,
    pub job_context: JobContext,
}

impl<T, U> Job<T, U> {
    pub fn new(
        reader: Box<dyn ItemReader<T>>,
        processor: Box<dyn ItemProcessor<T, U>>,
        writer: Box<dyn ItemWriter<U>>,
        chunk: Option<usize>,
        listener: Option<Listener<T, U>>,
        name: Option<String>,
    ) -> Self {
        Job {
            reader,
            processor,
            writer,
            chunk,
            listener,
            job_context: JobContext::new(name),
        }
    }

    fn get_reader_listener(&self) -> ReaderListener<T> {
        self.get_listener()
            .reader_listener
            .unwrap_or(ReaderListener::default())
    }

    fn get_processor_listener(&self) -> ProcessorListener<T, U> {
        self.get_listener()
            .processor_listener
            .unwrap_or(ProcessorListener::default())
    }

    fn get_writer_listener(&self) -> WriterListener<U> {
        self.get_listener()
            .writer_listener
            .unwrap_or(WriterListener::default())
    }

    fn get_job_listener(&self) -> JobListener {
        self.get_listener()
            .job_listener
            .unwrap_or(JobListener::default())
    }

    fn get_listener(&self) -> Listener<T, U> {
        self.listener.unwrap_or(Listener::default())
    }
}

impl<T: 'static, U: 'static> OnJob for Job<T, U> {
    fn on_job(&mut self) {
        self.job_context.update_current_step(Step::Start);
        let JobListener {
            before_job,
            after_job,
        } = self.get_job_listener();

        if before_job.is_some() {
            before_job.unwrap()(&self.job_context);
        }

        self.job_context.update_current_step(Step::StartReading);

        let writer_listener = self.get_writer_listener();
        let processor_listener = self.get_processor_listener();
        let reader_listener = self.get_reader_listener();

        let read_data = self
            .reader
            .run(&mut self.job_context, &reader_listener);


        let current_chunk = self.chunk.unwrap_or(0);
        let chunk_item = if current_chunk == 0 { vec![read_data.as_slice()] } else { read_data.chunks(current_chunk).collect() };

        for items in chunk_item {
            self.job_context.update_current_step(Step::StartWriting);
            let process_data = self.processor.run(
                &mut self.job_context,
                items,
                &processor_listener,
            );

            self.writer.run(
                &mut self.job_context,
                process_data.as_slice(),
                &writer_listener,
            );
        }

        self.job_context.update_current_step(Step::Ending);

        self.job_context.context.time_keeper.end();
        if after_job.is_some() {
            after_job.unwrap()(&self.job_context);
        }

        self.job_context.update_current_step(Step::End);
    }
}

pub struct Tasklet {
    pub job_context: JobContext,
    pub job_listener: JobListener,
    pub run: fn(&mut JobContext),
}

impl OnJob for Tasklet {
    fn on_job(&mut self) {
        self.job_context.update_current_step(Step::Start);
        if self.job_listener.before_job.is_some() {
            self.job_listener.before_job.unwrap()(&self.job_context)
        }
        (self.run)(&mut self.job_context);
        self.job_context.update_current_step(Step::Ending);
        self.job_context.context.time_keeper.end();
        if self.job_listener.after_job.is_some() {
            self.job_listener.after_job.unwrap()(&self.job_context)
        }
        self.job_context.update_current_step(Step::End);
    }
}

impl Tasklet {
    pub fn new(name: Option<String>,
               job_listener: Option<JobListener>,
               run: fn(&mut JobContext)
    ) -> Self {
        Tasklet {
            job_context: JobContext::new(name),
            job_listener: job_listener.unwrap_or(JobListener::default()),
            run
        }
    }
}
