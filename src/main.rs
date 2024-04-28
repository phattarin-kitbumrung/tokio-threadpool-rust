use tokio::runtime::Builder;
use tokio::time::{self, Duration};

async fn generate_task(task_id: u32, time: u64) {
    println!("Task {} started", task_id);
    // Simulate some asynchronous work, like fetching data from a remote server
    time::sleep(Duration::from_secs(time)).await;
    println!("Task {} completed", task_id);
}

async fn cpu_intensive_task() -> u64 {
    // Simulate a CPU-intensive computation
    let mut result: u64 = 0;
    for i in 0..1000000 {
        result = result.wrapping_add(i as u64); // using wrapping_add to handle overflow
    }

    result
}

fn main() {
    // Create a runtime with a thread pool.
    let runtime = Builder::new_multi_thread()
        .worker_threads(4) // Number of threads in the thread pool
        .enable_all()
        .build()
        .unwrap();

    // Spawn a task onto the thread pool.
    let handle = runtime.spawn(async {
        println!("Running tasks on the tokio thread pool!");
        let task1 = generate_task(1, 3);
        let task2 = generate_task(2, 2);
        let task3 = generate_task(3, 1);
        // Wait for the spawned tasks to complete
        tokio::join!(task1, task2, task3);

        let task4 = cpu_intensive_task();
        let task5 = cpu_intensive_task();
        let task6 = cpu_intensive_task();
        let result = tokio::join!(task4, task5, task6);
        println!("Final computation result = {}", result.0 + result.1 + result.2);
    });

    runtime.block_on(async {
        handle.await.unwrap(); // Wait for the spawned task to finish
    });
}
