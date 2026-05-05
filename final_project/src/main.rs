use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};
#[derive(Clone, Debug, PartialEq)]
enum TaskKind {
    CPU,
    IO,
}
#[derive(Clone, Debug)]
struct Task {
    id: usize,
    kind: TaskKind,
    arrival_time: Instant,
    duration: Duration,
    cpu_cost: u32,
}
#[derive(Debug)]
struct Metrics {
    completed: usize,
    cpu_completed: usize,
    io_completed: usize,
    total_wait_ms: u128,
    total_turnaround_ms: u128,
}
#[derive(Debug)]
struct CpuStats {
    total_cpu: u64,
    samples: u64
}
fn main() {
    let start_time = Instant::now();
    let total_tasks = 1000;
    let worker_count = 8;
    let queue = Arc::new(Mutex::new(VecDeque::<Task>::new()));
    let metrics = Arc::new(Mutex::new(Metrics {
        completed: 0,
        cpu_completed: 0,
        io_completed: 0,
        total_wait_ms: 0,
        total_turnaround_ms: 0,
    }));
    let current_cpu = Arc::new(Mutex::new(0u32));
    let cpu_stats = Arc::new(Mutex::new(CpuStats {
        total_cpu: 0,
        samples: 0,
    }));
    let (task_sender, task_receiver) = mpsc::channel::<Task>();
    let (worker_sender, worker_receiver) = mpsc::channel::<Task>();
    let worker_receiver = Arc::new(Mutex::new(worker_receiver));
    let generator = thread::spawn(move || {
        let mut rng = StdRng::seed_from_u64(42);
        for id in 0..total_tasks{
            let random_number = rng.gen_range(0..100);
            let kind = if random_number < 70{
                TaskKind::IO
            } else {
                TaskKind::CPU
            };
            let cpu_cost = match kind {
                TaskKind::IO => 10,
                TaskKind::CPU => 35,
            };
            let task = Task {
                id,
                kind,
                arrival_time: Instant::now(),
                duration: Duration::from_millis(200),
                cpu_cost,
            };
            task_sender.send(task).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    });
    let mut workers = vec![];
    for worker_id in 0..worker_count {
        let receiver_clone = Arc::clone(&worker_receiver);
        let metrics_clone = Arc::clone(&metrics);
        let cpu_clone = Arc::clone(&current_cpu);
        let worker = thread::spawn(move || {
            loop {
                let task_results = {
                    let receiver = receiver_clone.lock().unwrap();
                    receiver.recv()
                };
                match task_results {
                    Ok(task) => {
                        let start_execution = Instant::now();
                        let wait_time = start_execution.duration_since(task.arrival_time);
                        match task.kind {
                            TaskKind::CPU => {
                                busy_work(task.duration);
                            }
                            TaskKind::IO => {
                                thread::sleep(task.duration);
                            }
                        }

                        {
                            {
                                let mut cpu = cpu_clone.lock().unwrap();
                                if *cpu >= task.cpu_cost {
                                     *cpu -= task.cpu_cost;
                                    } else {
                                        *cpu = 0;
                                    }
                                }
                            }
                        let finish_time = Instant::now();
                        let turnaround_time = finish_time.duration_since(task.arrival_time);
                        let mut metrics = metrics_clone.lock().unwrap();
                        metrics.completed += 1;
                        metrics.total_wait_ms += wait_time.as_millis();
                        metrics.total_turnaround_ms += turnaround_time.as_millis();

                        match task.kind {
                            TaskKind::CPU => metrics.cpu_completed += 1,
                            TaskKind::IO => metrics.io_completed +=1,
                        }
                        println!("Worker {} completed task {}", worker_id, task.id);
                    }
                    Err(_) => break,
                }
            }
        });
        workers.push(worker);
    }
    let queue_clone = Arc::clone(&queue);
    let cpu_clone = Arc::clone(&current_cpu);
    let dispatcher = thread::spawn(move || {
    loop {
        match task_receiver.recv() {
            Ok(task) => {
                let mut queue = queue_clone.lock().unwrap();
                queue.push_back(task);
            }
            Err(_) => break,
        }
        loop {
            let maybe_task = {
                let mut queue = queue_clone.lock().unwrap();
                let task_position = queue
                .iter()
                .position(|task| task.kind == TaskKind::IO)
                .or_else(||{
                    queue
                    .iter()
                    .position(|task| {
                        let cpu = cpu_clone.lock().unwrap();
                        *cpu + task.cpu_cost <= 100
                    })
                });
                if let Some(index) = task_position {
                    let task = queue.remove(index);
                    if let Some(ref t) = task {
                        let mut cpu = cpu_clone.lock().unwrap();
                        if *cpu + t.cpu_cost <= 100 {
                            *cpu += t.cpu_cost;
                            task
                        } else {
                            queue.push_front(t.clone());
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            if let Some(task) = maybe_task {
                worker_sender.send(task).unwrap();
            } else {
                break;
            }
        }
    }
    loop {
        let maybe_task = {
            let mut queue = queue_clone.lock().unwrap();
            queue.pop_front()
        };
        if let Some(task) = maybe_task {
            worker_sender.send(task).unwrap();
        } else {
            break;
        }
    }
});
    let queue_monitor = Arc::clone(&queue);
    let cpu_monitor = Arc::clone(&current_cpu);
    let cpu_stats_monitor = Arc::clone(&cpu_stats);
    let monitor = thread::spawn(move || {
        for _ in 0..500 {
            let queue_length = queue_monitor.lock().unwrap().len();
            let cpu_usage = *cpu_monitor.lock().unwrap();
            {
                let mut stats = cpu_stats_monitor.lock().unwrap();
                stats.total_cpu += cpu_usage as u64;
                stats.samples += 1;
            }
            println!(
                "Monitor | Queue length: {} | CPU usage: {}",
                queue_length, cpu_usage
            );
            thread::sleep(Duration::from_millis(10));
        }
    });
    dispatcher.join().unwrap();
    monitor.join().unwrap();
    drop(worker_receiver);
    for worker in workers {
        worker.join().unwrap();
    }
    let final_metrics = metrics.lock().unwrap();
    let total_runtime = start_time.elapsed();
    let final_cpu_stats = cpu_stats.lock().unwrap();
    let average_cpu_usage = if final_cpu_stats.samples > 0 {
        final_cpu_stats.total_cpu as f64 / final_cpu_stats.samples as f64
    } else {
        0.0
    };
    println!("\n===== Final Metrics =====");
    println!("Total tasks completed: {}", final_metrics.completed);
    println!("CPU tasks completed: {}", final_metrics.cpu_completed);
    println!("IO tasks completed: {}", final_metrics.io_completed);
    println!("Total runtime: {:.2?}", total_runtime);
    println!("Average CPU usage: {:2}%", average_cpu_usage);
    if final_metrics.completed > 0 {
        println!("Average wait time: {} ms", final_metrics.total_wait_ms / final_metrics.completed as u128);
        println!("Average turnaround time: {} ms", final_metrics.total_turnaround_ms / final_metrics.completed as u128);
    }
}
fn busy_work(duration: Duration) {
    let start = Instant::now();
    while start.elapsed() < duration {
        let mut x = 0;
        for i in 0..1000 {
            x += i;
        }
        let _ = x;
    }
}