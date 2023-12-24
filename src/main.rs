// main.rs

// 导入来自linked_list.rs的LinkedList和Node类型
mod linked_list;

use linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);

    println!("Linked List: {:?}", list);

    if let Some(value) = list.pop() {
        println!("Popped element: {}", value);
    }

    if let Some(value) = list.peek() {
        println!("Peeked element: {}", value);
    }

    println!("Is the linked list empty? {}", list.is_empty());
}
