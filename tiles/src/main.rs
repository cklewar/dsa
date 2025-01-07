use std::{cell::RefCell, rc::Rc};

type TileNodeRef = Rc<RefCell<TileNode>>;

#[derive(Debug, Clone)]
enum Color {
    Black,
    White,
}

#[derive(Debug, Clone)]
pub struct Image {
    tiles: Vec<TileNode>,
}

#[derive(Debug, Clone)]
pub struct TileNode {
    _val: Color,
    refs: Vec<TileNodeRef>,
}

fn main() {
    let tile_1 = TileNode { _val: Color::White, refs: vec![]};
    let tile_2 = TileNode { _val: Color::Black, refs: vec![]};
    let tile_3 = TileNode { _val: Color::Black, refs: vec![]};

    let tile_5 = TileNode { _val: Color::Black, refs: vec![]};
    let tile_4 = TileNode { _val: Color::White, refs: vec![
        Some(Rc::new(RefCell::new(tile_5))).unwrap()
    ]};

    let image = Image {
        tiles: vec![
            tile_1,
            tile_2,
            tile_3,
            tile_4
        ],
    };

    fn draw(image: Image) {
        for tile in image.tiles {
            println!("image: {:?}", tile);
        }
    }

    draw(image);

}
