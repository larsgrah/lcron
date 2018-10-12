extern crate job_scheduler;
use std::process::Command;
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;

fn main() {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("5/10 * * * * *".parse().unwrap(), || {
        Command::new("bash")
            .arg("/home/lars/.scripts/bin/newsboat-reload")
            .spawn()
            .expect("sh command failed to start");
    }));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}
