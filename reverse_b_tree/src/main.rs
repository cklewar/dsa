use std::cell::{RefCell};
use std::rc::Rc;

type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug, Clone)]
struct TreeNode {
    _value: i32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}
fn main() {
    let mut node_a = TreeNode {
        _value: 4,
        left: None,
        right: None,
    };

    let mut node_b = TreeNode {
        _value: 2,
        left: None,
        right: None,
    };

    let mut node_c = TreeNode {
        _value: 7,
        left: None,
        right: None,
    };

    let node_d = TreeNode {
        _value: 1,
        left: None,
        right: None,
    };

    let node_e = TreeNode {
        _value: 3,
        left: None,
        right: None,
    };

    let node_f = TreeNode {
        _value: 6,
        left: None,
        right: None,
    };

    let node_g = TreeNode {
        _value: 9,
        left: None,
        right: None,
    };

    node_b.left = Some(TreeNodeRef::new(RefCell::from(node_d)));
    node_b.left = Some(TreeNodeRef::new(RefCell::from(node_e)));
    node_c.left = Some(TreeNodeRef::new(RefCell::from(node_f)));
    node_c.left = Some(TreeNodeRef::new(RefCell::from(node_g)));
    node_a.left = Some(TreeNodeRef::new(RefCell::from(node_b)));
    node_a.right = Some(TreeNodeRef::new(RefCell::from(node_c)));

    fn reverse(node: &mut Option<TreeNodeRef>) {
        if node.is_none() {
            return;
        }

        let mut left = node.as_ref().unwrap().borrow().left.clone();
        let mut right = node.as_ref().unwrap().borrow().right.clone();

        reverse(&mut left);
        reverse(&mut right);

        let left = node.as_ref().unwrap().borrow().left.clone();
        let right = node.as_ref().unwrap().borrow().right.clone();

        let mut _node = node.as_ref().unwrap().clone();

        _node.try_borrow_mut().unwrap().left = right;
        _node.try_borrow_mut().unwrap().right = left;
        println!("{:?}", &_node);

    }

    reverse(&mut Some(TreeNodeRef::new(RefCell::new(node_a))));


}
