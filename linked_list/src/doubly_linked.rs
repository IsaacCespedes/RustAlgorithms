pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        new_node.next = self.head.take();
        if let Some(mut old_head) = new_node.next.take() {
            old_head.prev = Some(new_node.as_mut());
        } else {
            self.tail = Some(new_node.as_mut());
        }
        self.head = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut old_head| {
            self.head = old_head.next.take();
            if let Some(new_head) = self.head.as_mut() {
                new_head.prev = None;
            } else {
                self.tail = None;
            }
            old_head.value
        })
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        new_node.prev = self.tail.take();
        if let Some(mut old_tail) = new_node.prev.take() {
            old_tail.next = Some(new_node.as_mut());
        } else {
            self.head = Some(new_node.as_mut());
        }
        self.tail = Some(new_node);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|mut old_tail| {
            self.tail = old_tail.prev.take();
            if let Some(new_tail) = self.tail.as_mut() {
                new_tail.next = None;
            } else {
                self.head = None;
            }
            old_tail.value
        })
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.tail.as_ref().map(|node| &node.value)
    }
}
