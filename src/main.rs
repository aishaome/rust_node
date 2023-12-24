// main.rs

// 导入来自linked_list.rs的LinkedList和Node类型
mod linked_list;

use linked_list::LinkedList;

fn main() {
    // 创建单链表
    let mut list = LinkedList::new();

    // 从都头部插入数据
    list.push(1);
    list.push(2);
    list.push(3);

    // 打印链表
    println!("Linked List: {:?}", list);

    // 从头部弹出数据
    if let Some(value) = list.pop() {
        println!("Popped element: {}", value);
    }

    // 获取头部数据
    if let Some(value) = list.peek() {
        println!("Peeked element: {}", value);
    }
    
    // 从头部插入数据
    list.push(4);
    println!("Linked List: {:?}", list);

    // 判断链表是否为空
    println!("Is the linked list empty? {}", list.is_empty());
}
