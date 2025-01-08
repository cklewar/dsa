use std::cell::RefCell;
use std::rc::Rc;

type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug, Clone)]
struct TreeNode {
    _val: u32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

fn main() {
    let mut node_a = TreeNode {
        _val: 1,
        left: None,
        right: None,
    };
    let mut node_b = TreeNode {
        _val: 2,
        left: None,
        right: None,
    };
    let node_c = TreeNode {
        _val: 3,
        left: None,
        right: None,
    };
    let node_d = TreeNode {
        _val: 4,
        left: None,
        right: None,
    };
    let node_e = TreeNode {
        _val: 5,
        left: None,
        right: None,
    };

    //      a
    //    /   \
    //   b     c
    //  / \
    // d   e

    node_b.left = Some(Rc::new(RefCell::new(node_d)));
    node_b.right = Some(Rc::new(RefCell::new(node_e)));
    node_a.left = Some(Rc::new(RefCell::new(node_b)));
    node_a.right = Some(Rc::new(RefCell::new(node_c)));

    // Should become
    // a
    //  \
    //   b
    //    \
    //     c
    //      \
    //       d
    //        \
    //         e

    fn flatten(node: &mut Option<TreeNodeRef>) {
        if node.is_none() {
            return;
        }

        let mut v: Vec<Option<TreeNodeRef>> = Vec::new();
        pre_order_traversal(&mut v, node);

        for i in 0..v.len() - 1 {
            v[i].as_ref().unwrap().borrow_mut().left = None;
            v[i].as_ref().unwrap().borrow_mut().right = v[i + 1].clone();
        }

        for item in v.iter() {
            println!("FLATTENED: {:#?}", &item.as_ref().unwrap().borrow()._val);
        }
    }

    fn pre_order_traversal(v: &mut Vec<Option<TreeNodeRef>>, node: &Option<TreeNodeRef>) {
        if node.is_none() {
            return;
        }

        v.push(node.clone());
        let left = node.as_ref().unwrap().borrow().left.clone();
        let right = node.as_ref().unwrap().borrow().right.clone();

        pre_order_traversal(v, &left);
        pre_order_traversal(v, &right);
    }

    flatten(&mut Some(TreeNodeRef::new(RefCell::from(node_a))));
}
