use crate::entity::batch::Batch;
use crate::entity::job::{Job, Tasklet};
use crate::entity::step::listener::{JobListener, Listener, ProcessorListener, ReaderListener, WriterListener};
use crate::entity::step::processor::Processor;
use crate::entity::step::reader::Reader;
use crate::entity::step::writer::{ManyWriter, Writer};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        main()
    }
}

mod entity;

fn main() {
    get_batch().run();
}

fn get_batch() -> Batch {
    Batch::new(Some(String::from("first_batch")), vec![Box::new(get_job()), Box::new(get_tasklet())])
}

fn get_tasklet() -> Tasklet {
    Tasklet::new(None, Some(get_job_listener()), |context| {
        println!("tasklet {}", context.context.name)
    })
}

fn get_job() -> Job<i32, String> {
    Job::new(
        Box::new(get_reader()),
        Box::new(get_processor()),
        Box::new(get_multi_writer()),
        Some(2),
        Some(Listener {
            writer_listener: Some(get_writer_listener()),
            processor_listener: Some(get_processor_listener()),
            reader_listener: Some(get_reader_listener()),
            job_listener: Some(get_job_listener()),
        }),
        Some(String::from("first_job")),
    )
}

fn get_reader() -> Reader<i32> {
    Reader::new(|_| vec![1, 2, 3, 4, 5])
}

fn get_processor() -> Processor<i32, String> {
    Processor::new(|_, item| {
        format!("string({item})")
    })
}

fn get_writer_1() -> Writer<String> {
    Writer::new(|context, item| {
        println!("w1 {} : {}", context.context.name, item);
    })
}

fn get_writer_2() -> Writer<String> {
    Writer::new(|context, item| {
        println!("w2 {} : {}", context.context.name, item);
    })
}

fn get_multi_writer() -> ManyWriter<String> {
    ManyWriter::new(vec![Box::new(get_writer_1()), Box::new(get_writer_2())])
}

fn get_reader_listener() -> ReaderListener<i32> {
    ReaderListener {
        before_read: Some(|_| println!("ReaderListener before_read")),
        after_read: Some(|_, items: &Vec<i32>| println!("ReaderListener after_read {:?}", items)),
    }
}

fn get_job_listener() -> JobListener {
    JobListener {
        before_job: Some(|context| println!("JobListener before_job : {}", context.context.time_keeper.to_string())),
        after_job: Some(|context| println!("JobListener after_job : {}", context.context.time_keeper.to_string())),
    }
}

fn get_writer_listener() -> WriterListener<String> {
    WriterListener {
        before_chunk: Some(|_, item: &[String]| println!("WriterListener before_chunk {:?}", item)),
        after_chunk: Some(|_, item: &[String]| println!("WriterListener after_chunk {:?}", item)),
        after_write: Some(|_, item: &String| println!("WriterListener after_write : {}", item)),
        before_write: Some(|_, item: &String| println!("WriterListener before_write : {}", item)),
    }
}

fn get_processor_listener() -> ProcessorListener<i32, String> {
    ProcessorListener {
        before_chunk: Some(|_, item: &[i32]| println!("ProcessorListener before_chunk {:?}", item)),
        after_chunk: Some(|_, item: &[String]| println!("ProcessorListener after_chunk {:?}", item)),
        after_process: Some(|_, item: &String| println!("ProcessorListener after_process : {}", item)),
        before_process: Some(|_, item: &i32| println!("ProcessorListener before_process : {}", item)),
    }
}

