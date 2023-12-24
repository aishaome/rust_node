// linked_list.rs

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    // 创建一个新链表
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // 在链表头部插入元素
    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // 从链表头部弹出元素
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    // 获取链表头部元素的引用
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    // 检查链表是否为空
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
