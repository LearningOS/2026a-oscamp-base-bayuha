//! # Channel Communication
//!
//! In this exercise, you will use `std::sync::mpsc` channels to pass messages between threads.
//!
//! ## Concepts
//! - `mpsc::channel()` creates a multiple producer, single consumer channel
//! - `Sender::send()` sends a message
//! - `Receiver::recv()` receives a message
//! - Multiple producers can be created via `Sender::clone()`

use std::sync::mpsc;
use std::thread;

/// Create a producer thread that sends each element from items into the channel.
/// The main thread receives all messages and returns them.
pub fn simple_send_recv(items: Vec<String>) -> Vec<String> {
    // TODO: Create channel
    // TODO: Spawn thread to send each element in items
    // TODO: In main thread, receive all messages and collect into Vec
    // Hint: When all Senders are dropped, recv() returns Err
        let (tx, rx) = mpsc::channel::<String>(); // 创建传递 String 的通道，得到发送端和接收端

    let handle = thread::spawn(move || { // 创建线程，把 items 和 tx 交给它
        for item in items { // 消耗输入数组，逐个取出字符串
            tx.send(item).unwrap(); // 发送字符串；发送失败则 panic
        } // 所有字符串发送完毕
    }); // 线程任务结束时，它拥有的 tx 自动释放

    let mut result = Vec::new(); // 在调用线程中创建结果数组

    while let Ok(item) = rx.recv() { // 每成功接收到一条消息，就取出来叫作 item
        result.push(item); // 把收到的字符串放入结果数组
    } // 通道没有后续消息且已经收空时，recv 返回 Err，循环结束

    handle.join().unwrap(); // 等待发送线程正常结束

    result // 返回结果数组
}

/// Create `n_producers` producer threads, each sending a message in format `"msg from {id}"`.
/// Collect all messages, sort them lexicographically, and return.
///
/// Hint: Use `tx.clone()` to create multiple senders. Note that the original tx must also be dropped.
pub fn multi_producer(n_producers: usize) -> Vec<String> {
    // TODO: Create channel
    // TODO: Clone a sender for each producer
    // TODO: Remember to drop the original sender, otherwise receiver won't finish
    // TODO: Collect all messages and sort
        let (tx, rx) = mpsc::channel::<String>(); // 创建一条通道
    let mut handles = Vec::new(); // 保存所有发送线程的句柄

    for id in 0..n_producers { // 每次循环创建一个发送线程，id 是它的编号
        let sender = tx.clone(); // 创建同一条通道的另一个发送端

        let handle = thread::spawn(move || { // 把 sender 和本轮 id 交给新线程
            let message = format!("msg from {id}"); // 构造字符串，例如 "msg from 2"
            sender.send(message).unwrap(); // 发送消息
        }); // 线程任务结束时，它的 sender 自动释放

        handles.push(handle); // 保存线程句柄
    }

    drop(tx); // 主动释放原始发送端，这是本题的关键

    let mut result = Vec::new(); // 保存收到的消息

    while let Ok(message) = rx.recv() { // 一直接收，直到所有发送端释放且通道收空
        result.push(message); // 追加到结果数组
    }

    for handle in handles { // 遍历所有线程句柄
        handle.join().unwrap(); // 等待每个线程正常结束
    }

    result.sort(); // 按字符串的字典顺序排序
    result // 返回结果
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_send_recv() {
        let items = vec!["hello".into(), "world".into(), "rust".into()];
        let result = simple_send_recv(items.clone());
        assert_eq!(result, items);
    }

    #[test]
    fn test_simple_empty() {
        let result = simple_send_recv(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_multi_producer() {
        let result = multi_producer(3);
        assert_eq!(
            result,
            vec![
                "msg from 0".to_string(),
                "msg from 1".to_string(),
                "msg from 2".to_string(),
            ]
        );
    }

    #[test]
    fn test_multi_producer_single() {
        let result = multi_producer(1);
        assert_eq!(result, vec!["msg from 0".to_string()]);
    }
}
