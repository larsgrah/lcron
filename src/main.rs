extern crate cron;
extern crate chrono;

use std::env;
use std::process::Command;
use std::time::Duration;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::borrow::Cow;

use cron::Schedule;
use chrono::Utc;
use std::str::FromStr;
use chrono::prelude::*;

struct Cronproc<'a> {
    cronstr: Cow<'a, str>,
    cronprogram: Cow<'a, str>,
    cronargs: Cow<'a, str>,
}

fn main() {
    let path = env::home_dir().unwrap().to_str().unwrap().to_owned() + "/.larscrontab";
    let f = File::open(path).expect("~/.larscrontab file is missing");
    let f = BufReader::new(f);
    let mut cron_vec: Vec<Cronproc> = Vec::new();

    for line in f.lines() {
        let line_str = line.unwrap();
        let mut iter = line_str.split_whitespace();

        let mut cron_str = String::new();
        for i in 0..6 {
            cron_str += iter.next().unwrap();
            if i != 5 {
                cron_str += " ";
            }
        }

        let program = iter.next().unwrap();
        let arg = iter.next().unwrap();


        let cronprc = Cronproc {
            cronstr: Cow::Owned(cron_str.to_owned()),
            cronprogram: Cow::Owned(program.to_owned()),
            cronargs: Cow::Owned(arg.to_owned()),
        };

        cron_vec.push(cronprc);
    }

    for cron_item in &cron_vec {
        print_cron(&cron_item);
    }

    loop {
        let sleep_timer = next_min_sleep(&cron_vec);

        println!("sleeping: {}", sleep_timer);
        std::thread::sleep(Duration::from_millis((sleep_timer * 1000) as u64));

        exec_cron(&cron_vec);
    }
}

fn get_timer_from_cron(cron_item: &Cronproc) -> i64 {
    let utc: DateTime<Utc> = Utc::now();
    let schedule = Schedule::from_str(&cron_item.cronstr).unwrap();
    let timer = schedule.upcoming(Utc).take(1).next().unwrap().timestamp() - utc.timestamp();
    timer
}

fn print_cron(cron_item: &Cronproc) {
    println!("---------------");
    println!("{} {}", cron_item.cronprogram, cron_item.cronargs);
    println!("Next execution due: {}", get_timer_from_cron(&cron_item));
    println!("---------------");
}

fn next_min_sleep(crons: &Vec<Cronproc>) -> i64 {
    let mut timers: Vec<i64> = Vec::new();
    for cron_item in crons {
        timers.push(get_timer_from_cron(cron_item));
    }

    *timers.iter().min().unwrap()
}

fn exec_cron(crons: &Vec<Cronproc>) {
    println!("executing cron jobs {}", crons.len());
    for cron_item in crons {
        print_cron(&cron_item);
        if get_timer_from_cron(cron_item) <= 60 {
            Command::new(cron_item.cronprogram.to_string())
                .arg(cron_item.cronargs.to_string())
                .spawn()
                .expect("sh command failed to start");
        }
    }
}
