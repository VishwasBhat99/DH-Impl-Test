use std::thread;
use std::time::{SystemTime, Duration};
use super::*;

#[test]
fn map_applies_on_new_thread_successfully() {
    let main_thread_id = thread::current().id();

    let pool_name = "Pool".to_string();
    let name_clone = pool_name.clone();

    let f = move |num: i32| {
        let current_thread = thread::current();

        // Check that the thread is not the main thread.
        assert_ne!(current_thread.id(), main_thread_id);
        assert_eq!(current_thread.name().unwrap(), format!("{}-0", pool_name));

        // Check the map is being applied.
        assert_eq!(num, 1);

        num + 1
    };

    let pool = WorkerPool::new_named(name_clone,1, f);

    let items = vec!(1,1,1);
    pool.submit(items);
}

#[test]
fn map_outputs_are_correct() {
    let numbers = vec!(1,1,1);

    let pool = WorkerPool::new(8, |num| num+1);
    let results = pool.submit(numbers);

    results.for_each(|num| {
        assert_eq!(num, 2);
    })
}

#[test]
fn outputs_count_match_inputs_count() {
    let count = 300;

    let numbers = new_vec(1, count);

    let f = |num| num+1;
    let pool = WorkerPool::new(8, f);
    let results = pool.submit(numbers);

    assert_eq!(results.count(), count as usize);
}

#[test]
fn map_from_one_type_to_another_works() {
    let numbers = vec!(1,1,1);

    let pool = WorkerPool::new(8, |num| format!("{}", num));
    let results = pool.submit(numbers);

    results.for_each(|num_string| {
        assert_eq!(num_string, format!("{}", 1));
    })
}

#[test]
fn multiple_jobs_stay_distinct() {
    let count1 = 500;
    let numbers1 = new_vec(1, count1);
    let f1 = |_num| {
        let duration = Duration::new(0, 100000);
        thread::sleep(duration);
    };

    let count2 = 100;
    let numbers2 = new_vec(1, count2);

    let pool = WorkerPool::new(8, f1);
    let result1 = pool.submit(numbers1);
    let result2 = pool.submit(numbers2);

    assert_eq!(result1.count(), count1 as usize);
    assert_eq!(result2.count(), count2 as usize);
}

#[test]
fn work_is_distributed_across_threads() {
    // This test is the 'multi-threading' test.
    // We submit `num` jobs to `num` threads. The job puts the thread to sleep for a duration.
    //
    // The time taken to complete the jobs should be the duration + (a small duration as an overhead).

    let num = 8;
    let half_second_nanos = 500_000_000;

    let items = new_vec(1, num);
    let f = move |_num| {
        let duration = Duration::new(0, half_second_nanos);
        thread::sleep(duration);
    };

    let pool = WorkerPool::new(8, f);

    let start_time = SystemTime::now();
    let results = pool.submit(items);
    for _ in results { /* Blocks until all results are achieved */ }
    let end_time = SystemTime::now();
    let duration_as_nanos = duration_as_nanos(end_time.duration_since(start_time).unwrap());
    let expected_duration = 510_000_000;

    assert!(duration_as_nanos < expected_duration);
}

// HELPERS:
fn new_vec<T>(value: T, count: i32) -> Vec<T>
    where T: std::clone::Clone {
    if count < 0 {
        panic!("Count should be positive, not {}", count)
    }

    let mut vec = Vec::new();
    for _ in 0..(count as u16) {
        vec.push(value.clone());
    }

    return vec;
}

fn duration_as_nanos(duration: Duration) -> u64 {
    let mut total_nanos = duration.as_secs() * 1_000_000_000;
    total_nanos += duration.subsec_nanos() as u64;
    return total_nanos
}