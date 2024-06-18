use crate::entity::context::JobContext;

pub struct JobListener {
    pub before_job: Option<fn(&JobContext)>,
    pub after_job: Option<fn(&JobContext)>,
}

pub struct WriterListener<T> {
    pub before_chunk: Option<fn(&JobContext, &[T])>,
    pub before_write: Option<fn(&JobContext, &T)>,
    pub after_write: Option<fn(&JobContext, &T)>,
    pub after_chunk: Option<fn(&JobContext, &[T])>,
}

pub struct ProcessorListener<T, U> {
    pub before_chunk: Option<fn(&JobContext, &[T])>,
    pub before_process: Option<fn(&JobContext, &T)>,
    pub after_process: Option<fn(&JobContext, &U)>,
    pub after_chunk: Option<fn(&JobContext, &[U])>,
}

pub struct ReaderListener<T> {
    pub before_read: Option<fn(&JobContext)>,
    pub after_read: Option<fn(&JobContext, &Vec<T>)>,
}

pub struct Listener<T, U> {
    pub job_listener: Option<JobListener>,
    pub reader_listener: Option<ReaderListener<T>>,
    pub processor_listener: Option<ProcessorListener<T, U>>,
    pub writer_listener: Option<WriterListener<U>>,
}

impl<T, U> Listener<T, U> {
    pub fn new(
        job_listener: Option<JobListener>,
        reader_listener: Option<ReaderListener<T>>,
        processor_listener: Option<ProcessorListener<T, U>>,
        writer_listener: Option<WriterListener<U>>,
    ) -> Listener<T, U> {
        Listener {
            job_listener,
            reader_listener,
            processor_listener,
            writer_listener,
        }
    }

    pub fn default() -> Listener<T, U> {
        Listener::new(None, None, None, None)
    }
}

impl<T, U> Clone for Listener<T, U> {
    fn clone(&self) -> Self {
        Listener {
            job_listener: self.job_listener,
            reader_listener: self.reader_listener,
            processor_listener: self.processor_listener,
            writer_listener: self.writer_listener,
        }
    }
}

impl<T, U> Copy for Listener<T, U> {}

impl<T> ReaderListener<T> {
    pub fn new(
        before_read: Option<fn(&JobContext)>,
        after_read: Option<fn(&JobContext, &Vec<T>)>,
    ) -> ReaderListener<T> {
        ReaderListener {
            before_read,
            after_read,
        }
    }

    pub fn default() -> ReaderListener<T> {
        ReaderListener::new(None, None)
    }
}

impl<T> Clone for ReaderListener<T> {
    fn clone(&self) -> Self {
        ReaderListener {
            before_read: self.before_read,
            after_read: self.after_read,
        }
    }
}

impl<T> Copy for ReaderListener<T> {}

impl<T> WriterListener<T> {
    pub fn new(
        before_chunk: Option<fn(&JobContext, &[T])>,
        before_write: Option<fn(&JobContext, &T)>,
        after_write: Option<fn(&JobContext, &T)>,
        after_chunk: Option<fn(&JobContext, &[T])>,
    ) -> WriterListener<T> {
        WriterListener {
            before_chunk,
            before_write,
            after_write,
            after_chunk,
        }
    }

    pub fn default() -> WriterListener<T> {
        WriterListener::new(None, None, None, None)
    }
}

impl<T> Clone for WriterListener<T> {
    fn clone(&self) -> Self {
        WriterListener {
            before_chunk: self.after_chunk,
            before_write: self.before_write,
            after_write: self.after_write,
            after_chunk: self.after_chunk,
        }
    }
}

impl<T> Copy for WriterListener<T> {}

impl JobListener {
    pub fn new(
        before_job: Option<fn(&JobContext)>,
        after_job: Option<fn(&JobContext)>,
    ) -> JobListener {
        JobListener {
            before_job,
            after_job,
        }
    }

    pub fn default() -> JobListener {
        JobListener::new(None, None)
    }
}

impl Clone for JobListener {
    fn clone(&self) -> Self {
        JobListener {
            before_job: self.after_job,
            after_job: self.after_job,
        }
    }
}

impl Copy for JobListener {}

impl<T, U> ProcessorListener<T, U> {
    pub fn new(
        before_chunk: Option<fn(&JobContext, &[T])>,
        before_process: Option<fn(&JobContext, &T)>,
        after_process: Option<fn(&JobContext, &U)>,
        after_chunk: Option<fn(&JobContext, &[U])>,
    ) -> ProcessorListener<T, U> {
        ProcessorListener {
            before_chunk,
            before_process,
            after_process,
            after_chunk,
        }
    }

    pub fn default() -> ProcessorListener<T, U> {
        ProcessorListener::new(None, None, None, None)
    }
}

impl<T, U> Clone for ProcessorListener<T, U> {
    fn clone(&self) -> Self {
        ProcessorListener {
            before_chunk: self.before_chunk,
            before_process: self.before_process,
            after_process: self.after_process,
            after_chunk: self.after_chunk,
        }
    }
}

impl<T, U> Copy for ProcessorListener<T, U> {}
