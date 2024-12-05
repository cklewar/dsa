use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct TreeNode {
    _val: u32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

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

    let mut max_diameter: [u32; 1] = [0];

    fn diameter(node: Option<&TreeNodeRef>, max_diameter: &mut [u32; 1]) -> u32 {
        if node.is_some() {
            let left_height = diameter(node.unwrap().borrow().left.as_ref(), max_diameter);
            let right_height = diameter(node.unwrap().borrow().right.as_ref(), max_diameter);
            max_diameter[0] = cmp::max(max_diameter[0], left_height + right_height);

            return 1 + cmp::max(left_height, right_height);
        }
        0
    }

    println!("Diameter: {:?}", diameter(Some(&Rc::new(RefCell::new(node_a))), &mut max_diameter));
}
