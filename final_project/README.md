## Final Project for CSCI-3334-01

## Project Objective
- This project is to implement a concurrent task dispatcher in Rust that can simulate a stream of two bound tasks with CPU and IO. These task will be created over time, and placed in a shared queue, as well as dispatching to a bounded worker pool according to a scheduling policy. Our main goal of this project is to model a simple system-style scheduler as well as showing the concurrency, synchronization, and performance comparison.

## This project will have two results we are looking for in this test.
- FIFO Scheduling
- Optimized IO-priority scheduling

## This Project will feature the following
- Will be creating 1000 task using a fixed random seed to see for reproducibilty.
- 20 Millisecond simulations for every staggered task arrival.
- Using a bounded pool of 8 worker threads within the program.
- Will start Dispatching tasks only when a sufficient CPU capacity is available within the simulation.
- Will enforce a Global CPU cap at 100%
- Will include a monitor thread that can samples queue length and CPU usage
- Collecting and printing summaries of performace metrics.
- Supporting the comparison between the FIFO and optimized scheduling behavior

## System Design
- Generator thread: This will create tasks over time and send them to the dispatcher.
- Dispatcher thread: This will be receiving the tasks, storing them in a queue, checking the CPU availability, and as well assigning tasking to different workers.
- Worker pool: This is a fixed set of 8 worker threads that are repeatedly receiving and executing tasks until the program is finished.
- Monitor thread: This will monitor the samples queue length and CPU usage every 10 milliseconds and records the CPU statistics for average usage.

## Task Types
The task types that are included in this project are 
- 'id'
- 'arrival_time'
- 'kind' ('CPU'/'IO')
- 'duration'
- 'cpu_cost'

## Task Behavior within each task
- CPU Task: This task willo simulate the CPU work with busy loop.
- IO Task: This task will simulate the I0 work using a sleep function lower than CPU Cost.

## Synchronization Strategy
This project will be using a Rust Concurrency from a standard Library.
- 'thread' for the concurrency
- 'mspc' this for the channels for task communication
- 'Arc<Mutex<...>>' to have the shared state.

## The Shared state will includes:
- ready queue
- cureent CPU usage
- performance metrics
- CPU usage statistics

## Channels that are used in the project.
These channels are used to help separate the task flow between the two.
1. generator -> dispatcher
2. dispatcher -> workers

With this design will help separate every responsibilities while keeping the system concurrent and easier.

## Scheduling Policies 

1. FIFO Scheduler: This scheduler will dispatches the tasks in the arrival order, as long as it is sending the next task that would not cause the global CPU usage to exceed over 100%
2. Optimized IO_Priority Scheduler: This scheduler will give the preferences to the IO-bound tasks before the CPU-bound task and when selecting the next task from the queue. This approach is intended to help improve the responsiveness for lighter tasks and will also reduce the queue buildup under the mixed workloads within the program.

## Metrics that will be Collected 
With the program results, we will be looking at the following metrics:
- total tasks completed
- number of CPU tasks completed
- number of IO tasks completed
- total runtime
- average CPU usage
- average wait time  
- average turnaround time
These are the metrics that will be getting results for this program

## Build instructions 
To build the project:
1. Open the terminal 'bash' and then run 'cargo build'
2. Once the cargo is build then run 'cargo run
3. Remember to get the results that are need make sure to run the cargo like this, 'cargo run > fifo_results.txt' and 'cargo run > optimized_results.txt

## Tool Use Disclosure
**Tools used:** ChatGPT and Gemini were used to help with any debuggin issues I had. Both for a second options to see if I was on the right track on debugging certain areas of my code.