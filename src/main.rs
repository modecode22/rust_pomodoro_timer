use crossterm::{cursor, execute, terminal, ExecutableCommand};
use std::thread;
use std::time::{Duration, Instant};
use std::io::{self, Write}; // Import io::Write trait



fn format_duration(duration: Duration) -> String {
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

struct PomodoroTimer {
    work_duration: Duration,
    short_break_duration: Duration,
    long_break_duration: Duration,
    sections_before_long_break: u32,
    running: bool,
    start_time: Instant,
    remaining_time: Duration,
    section_count: u32,
}

impl PomodoroTimer {
    fn new(
        work_minutes: u64,
        short_break_minutes: u64,
        long_break_minutes: u64,
        sections_before_long_break: u32,
    ) -> Self {
        PomodoroTimer {
            work_duration: Duration::from_secs(work_minutes * 60),
            short_break_duration: Duration::from_secs(short_break_minutes * 60),
            long_break_duration: Duration::from_secs(long_break_minutes * 60),
            sections_before_long_break,
            running: false,
            start_time: Instant::now(),
            remaining_time: Duration::from_secs(work_minutes * 60),
            section_count: 0,
        }
    }



    fn start(&mut self) {
        if !self.running {
            self.running = true;
            self.start_time = Instant::now();
            self.remaining_time = self.work_duration;
            println!("Start working ({} minutes)...", format_duration(self.work_duration));
        }
    }

    fn pause(&mut self) {
        if self.running {
            self.running = false;
            self.remaining_time -= self.start_time.elapsed();
            println!("Timer paused.");
        }
    }

    fn resume(&mut self) {
        if !self.running {
            self.running = true;
            self.start_time = Instant::now();
            println!("Timer resumed.");
        }
    }

    fn run(&mut self) {
        loop {
            if self.running && self.remaining_time <= Duration::new(0, 0) {
                self.running = false;
                self.section_count += 1;

                if self.section_count % self.sections_before_long_break == 0 {
                    println!("Long break ({} minutes)...", format_duration(self.long_break_duration));
                    thread::sleep(self.long_break_duration);
                } else {
                    println!("Short break ({} minutes)...", format_duration(self.short_break_duration));
                    thread::sleep(self.short_break_duration);
                }

                self.start();
            }

            if self.running {
                let elapsed_time = self.start_time.elapsed();
                if self.remaining_time > elapsed_time {
                    thread::sleep(self.remaining_time - elapsed_time);
                }
                println!("Time remaining: {}", format_duration(self.remaining_time - elapsed_time));
            }
        }
    }
}


fn main() {
    let work_minutes = 25;
    let short_break_minutes = 5;
    let long_break_minutes = 15;
    let sections_before_long_break = 4;

    let mut timer = PomodoroTimer::new(
        work_minutes,
        short_break_minutes,
        long_break_minutes,
        sections_before_long_break,
    );

    timer.start();
    
    timer.run();
    loop {

    println!("some thing {}",format_duration(timer.remaining_time));
        // timer.run();
        // print!("Time remaining: {}", format_duration(timer.remaining_time));

        thread::sleep(Duration::from_secs(1));
        print!("\r"); // Move cursor back to the beginning of the line
    }
}