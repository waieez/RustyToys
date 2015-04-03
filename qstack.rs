struct Stack<T> {
    storage: Vec<T>
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack{storage: vec![]}
    }

    fn push(self: &mut Self, val:T) {
        self.storage.push(val);
    }

    fn pop(self: &mut Self) -> Option<T> {
        self.storage.pop()
    }

    fn size(&self) -> usize {
        self.storage.len()
    }
}

struct Queue<T> {
    inbox: Stack<T>,
    outbox: Stack<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        let inbox:Stack<T> = Stack::new();
        let outbox:Stack<T> = Stack::new();
        Queue{inbox:inbox, outbox:outbox}
    }

    fn enqueue(self: &mut Self, val:T) {
        self.inbox.push(val);
    }

    fn dequeue(self: &mut Self) -> Option<T> {
        if self.outbox.size() == 0 {
            let mut done:bool = false;
            while !done {
                match self.inbox.pop() {
                    Some(x) => self.outbox.push(x),
                    None => done = true,
                }
            }
        }

        self.outbox.pop()
    }
}

fn main() {
    let mut q:Queue<i32> = Queue::new();

    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    q.dequeue();
    q.enqueue(4);
    q.dequeue();
    q.dequeue();
    println!("{:?}", q.inbox.storage);
    println!("{:?}", q.outbox.storage);
}