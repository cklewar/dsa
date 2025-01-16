use std::cell::RefCell;
use std::rc::Rc;

type Link = RefCell<Option<Rc<Node>>>;

#[derive(Debug)]
struct Node {
    element: i32,
    next: Link,
}

fn main() {
    let node4 = Some(Rc::new(Node {
        element: 4,
        next: Link::new(None),
    }));

    let node3 = Some(Rc::new(Node {
        element: 3,
        next: Link::new(None),
    }));

    let node2 = Some(Rc::new(Node {
        element: 2,
        next: Link::new(None),
    }));

    let node1 = Some(Rc::new(Node {
        element: 1,
        next: Link::new(None),
    }));

    node1.clone().unwrap().next.replace(node4.clone());
    node2.unwrap().next.replace(node3.clone());
    node3.unwrap().next.replace(node4.clone());

    if node4.is_some() {
        node4.unwrap().next.replace(node1.clone());
    }

    fn has_cycle(list: Option<Rc<Node>>) -> bool {
        let mut slow = list.clone();
        let mut fast = match list {
            None => {
                println!("FAST is None");
                return false;
            }
            Some(node) => node.next.borrow().clone(),
        };

        while let (Some(fast_next), Some(slow_next)) = (slow, fast) {
            if Rc::ptr_eq(&fast_next, &slow_next) {
                println!("Found cycle!");
                return true;
            }
            slow = slow_next.next.borrow().clone();
            fast = match fast_next.next.borrow().clone() {
                None => return false,
                Some(node) => node.next.borrow().clone(),
            };
        }
        println!("No cycle found!");
        false
    }

    println!("{}", has_cycle(node1));
}
