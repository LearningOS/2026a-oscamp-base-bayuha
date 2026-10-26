//! # Mutex Shared State
//!
//! In this exercise, you will use `Arc<Mutex<T>>` to safely share and modify data between multiple threads.
//!
//! ## Concepts
//! - `Mutex<T>` mutex protects shared data
//! - `Arc<T>` atomic reference counting enables cross-thread sharing
//! - `lock()` acquires the lock and accesses data

use std::sync::{Arc, Mutex};
use std::thread;

/// Increment a counter concurrently using `n_threads` threads.
/// Each thread increments the counter `count_per_thread` times.
/// Returns the final counter value.
///
/// Hint: Use `Arc<Mutex<usize>>` as the shared counter.
pub fn concurrent_counter(n_threads: usize, count_per_thread: usize) -> usize {
    // TODO: Create Arc<Mutex<usize>> with initial value 0
    // TODO: Spawn n_threads threads
    // TODO: In each thread, lock() and increment count_per_thread times
    // TODO: Join all threads, return final value
    let counter = Arc::new(Mutex::new(0usize)); // 创建共享计数器，初始值为 0，类型是 usize
    let mut handles = Vec::new();              // 创建数组，保存所有线程的句柄

    for _ in 0..n_threads { // 循环 n_threads 次，每次创建一个线程
        let counter_in_thread = Arc::clone(&counter); // 增加一个指向同一计数器的共享拥有者

        let handle = thread::spawn(move || { // 创建线程，把 counter_in_thread 带进去
            for _ in 0..count_per_thread { // 当前线程重复加一 count_per_thread 次
                let mut value = counter_in_thread.lock().unwrap(); // 取得锁，获得操作内部整数的守卫
                *value += 1; // 修改受保护的整数，加一
            } // 每轮结束时 value 离开作用域，自动释放锁
        }); // 保存新线程的句柄

        handles.push(handle); // 把句柄放入数组，稍后统一等待
    } // 所有线程都已创建

    for handle in handles { // 依次取出每个线程的句柄
        handle.join().unwrap(); // 等待对应线程正常结束
    } // 所有线程都已经完成加一

    let result = *counter.lock().unwrap(); // 再次取得锁，读取并复制最终整数
    result // 返回最终值
}

/// Add elements to a shared vector concurrently using multiple threads.
/// Each thread pushes its own id (0..n_threads) to the vector.
/// Returns the sorted vector.
///
/// Hint: Use `Arc<Mutex<Vec<usize>>>`.
pub fn concurrent_collect(n_threads: usize) -> Vec<usize> {
    // TODO: Create Arc<Mutex<Vec<usize>>>
    // TODO: Each thread pushes its own id
    // TODO: After joining all threads, sort the result and return
    let values = Arc::new(Mutex::new(Vec::new())); // 创建一个受锁保护的共享动态数组
    let mut handles = Vec::new();                // 保存所有线程的句柄

    for id in 0..n_threads { // id 依次为 0、1、……、n_threads - 1
        let values_in_thread = Arc::clone(&values); // 给当前线程一个指向同一数组的共享拥有者

        let handle = thread::spawn(move || { // 创建线程，带入共享拥有者和本轮的 id
            values_in_thread.lock().unwrap().push(id); // 取得锁，把自己的编号放入数组；语句结束时释放锁
        }); // 保存线程句柄

        handles.push(handle); // 放入句柄数组
    } // 所有线程都已创建

    for handle in handles { // 依次取出句柄
        handle.join().unwrap(); // 等待线程结束
    } // 至此所有编号都已放入共享数组

    let mut result = values.lock().unwrap().clone(); // 取得锁，把内部数组复制出来；语句结束时释放锁
    result.sort(); // 对复制出的数组排序
    result // 返回排好序的数组
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_single_thread() {
        assert_eq!(concurrent_counter(1, 100), 100);
    }

    #[test]
    fn test_counter_multi_thread() {
        assert_eq!(concurrent_counter(10, 100), 1000);
    }

    #[test]
    fn test_counter_zero() {
        assert_eq!(concurrent_counter(5, 0), 0);
    }

    #[test]
    fn test_collect() {
        let result = concurrent_collect(5);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_collect_single() {
        assert_eq!(concurrent_collect(1), vec![0]);
    }
}
