use std::cell::RefCell;
use std::rc::Rc;
use colored::Colorize;
use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ShapeType {
    //   *   *
    //     *
    //   *   *
    ShapeTypeStar,

    //   *    *
    // *   *    *
    ShapeTypeM,

    //  *
    //  *
    //  *  *  *
    ShapeTypeBigCorner,

    // *  *
    //    *  *  *
    ShapeTypeZigzag,

    //  *  *
    //  *  *  *
    ShapeTypeSquareHorn,

    // *  *  *
    // *     *
    ShapeTypeOpenMouth,

    // *  *  *  *
    // *
    ShapeTypeBigL,

    // *  *  *  *
    //    *
    ShapeTypeLineHorn,

    // *  *  *
    // *
    ShapeTypeSmallL,

    // *
    // *  *
    ShapeTypeSmallCorner,

    // *  *
    // *  *
    ShapeTypeSquare,

    // *  *  *  *
    ShapeTypeLine,

    ShapeTypeMax,
}

pub type DescArray = [[usize;4];4];
pub struct SubShape {
    pub width: usize,
    pub height: usize,
    pub desc: DescArray,
}

struct Shape {
    shape_type: ShapeType,
    sub_shapes: Vec<SubShape>,
}

fn show_sub_shape(sub: &SubShape) {
    for i in 0..sub.height {
        for j in 0..sub.width {
            if sub.desc[i][j] == 0 {
                print!("  ");
            }
            else {
                print!("* ");
            }
        }
        println!(" ");
    }
}

pub fn show_shape_description(shape_type: ShapeType) {
    println!("Show shape descriptor {:?}", shape_type);

    let descriptors = get_shape_descriptors();

    for desc in descriptors.borrow().iter() {
        //println!("type is {:?}", desc.shape_type);
        if desc.shape_type == shape_type {

            /*
            let mut cnt: u32 = 0;

            for s in &desc.sub_shapes {
                println!("sub[{}]",cnt);
                show_sub_shape(s);
                cnt += 1;
            }

             */

            for cnt in 0..desc.sub_shapes.len() {
                println!("sub[{}]",cnt);
                show_sub_shape(&desc.sub_shapes[cnt]);
            }
        }
    }
}

pub fn show_shape_index() {
    let descriptors = get_shape_descriptors();

    for desc in descriptors.borrow().iter() {
        match desc.shape_type {
            ShapeType::ShapeTypeStar => {
                println!("[{}]", map_from_shape_to_index(ShapeType::ShapeTypeStar));
                println!("{}",format!("O   O").red());
                println!("{}",format!("  O  ").red());
                println!("{}",format!("O   O").red());
            }
            ShapeType::ShapeTypeM => {
                println!("[{}]", map_from_shape_to_index(ShapeType::ShapeTypeM));
                println!("{}",format!("  O   O  ").red());
                println!("{}",format!("O   O   O").red());

            }
            ShapeType::ShapeTypeBigCorner => {
                println!("[{}]", map_from_shape_to_index(ShapeType::ShapeTypeBigCorner));
                println!("{}",format!("O    ").blue());
                println!("{}",format!("O    ").blue());
                println!("{}",format!("O O O").blue());

            }
            ShapeType::ShapeTypeZigzag => {
                println!("[{}]", map_from_shape_to_index(ShapeType::ShapeTypeZigzag));
                println!("{}",format!("O O    ").purple());
                println!("{}",format!("  O O O").purple());

            }
            ShapeType::ShapeTypeSquareHorn => {
                println!("[{}]", map_from_shape_to_index(ShapeType::ShapeTypeSquareHorn));
                println!("{}",format!("O O    ").cyan());
                println!("{}",format!("O O O  ").cyan());

            }
            ShapeType::ShapeTypeOpenMouth => {
                println!("[{}]", map_from_shape_to_index(ShapeType::ShapeTypeOpenMouth));
                println!("{}",format!("O O O").white());
                println!("{}",format!("O   O").white());

            }
            ShapeType::ShapeTypeBigL => {
                println!("[{}]", map_from_shape_to_index(ShapeType::ShapeTypeBigL));
                println!("{}",format!("O O O O").yellow());
                println!("{}",format!("O      ").yellow());

            }
            ShapeType::ShapeTypeLineHorn => {
                println!("[{}]", map_from_shape_to_index(ShapeType::ShapeTypeLineHorn));
                println!("{}",format!("O O O O").green());
                println!("{}",format!("  O    ").green());

            }
            ShapeType::ShapeTypeSmallL => {
                println!("[{}]", map_from_shape_to_index(ShapeType::ShapeTypeSmallL));
                println!("{}",format!("O O O").white());
                println!("{}",format!("O    ").white());

            }
            ShapeType::ShapeTypeSmallCorner => {
                println!("[{}]", map_from_shape_to_index(ShapeType::ShapeTypeSmallCorner));
                println!("{}",format!("O   ").red());
                println!("{}",format!("O O ").red());

            }
            ShapeType::ShapeTypeSquare => {
                println!("[{}]", map_from_shape_to_index(ShapeType::ShapeTypeSquare));
                println!("{}",format!("O O").yellow());
                println!("{}",format!("O O").yellow());

            }
            ShapeType::ShapeTypeLine => {
                println!("[{}]", map_from_shape_to_index(ShapeType::ShapeTypeLine));
                println!("{}",format!("O O O O").green());
            }
            ShapeType::ShapeTypeMax => {
                println!(" ");
            }
        }
    }
}

pub fn map_from_index_to_shape(idx: usize) -> ShapeType {
    match idx {
        0 => ShapeType::ShapeTypeStar,
        1 => ShapeType::ShapeTypeM,
        2 => ShapeType::ShapeTypeBigCorner,
        3 => ShapeType::ShapeTypeZigzag,
        4 => ShapeType::ShapeTypeSquareHorn,
        5 => ShapeType::ShapeTypeOpenMouth,
        6 => ShapeType::ShapeTypeBigL,
        7 => ShapeType::ShapeTypeLineHorn,
        8 => ShapeType::ShapeTypeSmallL,
        9 => ShapeType::ShapeTypeSmallCorner,
        10 => ShapeType::ShapeTypeSquare,
        11 => ShapeType::ShapeTypeLine,
        _ => ShapeType::ShapeTypeMax,
    }
}

pub fn map_from_shape_to_index(s_type: ShapeType) -> usize {
    match s_type {
        ShapeType::ShapeTypeStar => 0,
        ShapeType::ShapeTypeM => 1,
        ShapeType::ShapeTypeBigCorner => 2,
        ShapeType::ShapeTypeZigzag => 3,
        ShapeType::ShapeTypeSquareHorn => 4,
        ShapeType::ShapeTypeOpenMouth => 5,
        ShapeType::ShapeTypeBigL => 6,
        ShapeType::ShapeTypeLineHorn => 7,
        ShapeType::ShapeTypeSmallL => 8,
        ShapeType::ShapeTypeSmallCorner => 9,
        ShapeType::ShapeTypeSquare => 10,
        ShapeType::ShapeTypeLine => 11,
        ShapeType::ShapeTypeMax => 12,
    }
}

pub fn get_sub_shape(s_index: usize, sub_index: usize) -> Option<SubShape> {
    let descriptor = get_shape_descriptors();
    for i in descriptor.borrow().iter() {
        if i.shape_type == map_from_index_to_shape(s_index) {
            if sub_index < i.sub_shapes.len() {
                let s = SubShape{
                    width: i.sub_shapes[sub_index].width,
                    height: i.sub_shapes[sub_index].height,
                    desc: i.sub_shapes[sub_index].desc,
                };
                return Some(s);
            }
        }
    }
    return None;
}

pub fn get_sub_shape_number(s_index: usize) -> usize {
    let descriptor = get_shape_descriptors();
    for i in descriptor.borrow().iter() {
        if i.shape_type == map_from_index_to_shape(s_index) {
            return i.sub_shapes.len();
        }
    }
    return 0;
}

static mut G_DESCRIPTOR: RefCell<Vec<Rc<Shape>>> = RefCell::new(vec![]);

fn get_shape_descriptors() ->  RefCell<Vec<Rc<Shape>>> {
    //let mut descriptors: Vec<Rc<Shape>> = vec![];

    unsafe
    {
        if G_DESCRIPTOR.borrow().len() > 0 {
            //println!("G_DESCRIPTOR already valid");
            return RefCell::clone(&G_DESCRIPTOR);
        }
    }

    /*
     *********************************************
     * ShapeTypeStar
     *********************************************
     */
    let mut shp = Shape {
        shape_type: ShapeType::ShapeTypeStar,
        sub_shapes: vec![],
    };

    let sub = SubShape {
        width: 3,
        height: 3,
        desc: [[0,1,0,0],[1,1,1,0,],[0,1,0,0,],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    unsafe {
        G_DESCRIPTOR.borrow_mut().push(Rc::new(shp));
    }


    /*
     *********************************************
     * ShapeTypeM
     *********************************************
     */
    let mut shp = Shape {
        shape_type: ShapeType::ShapeTypeM,
        sub_shapes: vec![],
    };

    let sub = SubShape {
        width: 3,
        height: 3,
        desc: [[1,0,0,0],[1,1,0,0],[0,1,1,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 3,
        height: 3,
        desc: [[0,0,1,0],[0,1,1,0],[1,1,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 3,
        height: 3,
        desc: [[1,1,0,0],[0,1,1,0],[0,0,1,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 3,
        height: 3,
        desc: [[0,1,1,0],[1,1,0,0],[1,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    unsafe {
        G_DESCRIPTOR.borrow_mut().push(Rc::new(shp));
    }


    /*
     *********************************************
     * ShapeTypeBigCorner
     *********************************************
     */
    let mut shp = Shape {
        shape_type: ShapeType::ShapeTypeBigCorner,
        sub_shapes: vec![],
    };

    let sub = SubShape {
        width: 3,
        height: 3,
        desc: [[1,0,0,0],[1,0,0,0],[1,1,1,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 3,
        height: 3,
        desc: [[0,0,1,0],[0,0,1,0],[1,1,1,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 3,
        height: 3,
        desc: [[ 1,1,1,0],[0,0,1,0],[0,0,1,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 3,
        height: 3,
        desc: [[1,1,1,0],[1,0,0,0],[1,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    unsafe {
        G_DESCRIPTOR.borrow_mut().push(Rc::new(shp));
    }

    /*
     *********************************************
     * ShapeTypeZigzag
     *********************************************
     */
    let mut shp = Shape {
        shape_type: ShapeType::ShapeTypeZigzag,
        sub_shapes: vec![],
    };

    let sub = SubShape {
        width: 4,
        height: 2,
        desc: [[1,1,0,0],[0,1,1,1],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 4,
        height: 2,
        desc: [[0,0,1,1],[1,1,1,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 4,
        height: 2,
        desc: [[ 0,1,1,1],[1,1,0,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 4,
        height: 2,
        desc: [[1,1,1,0],[0,0,1,1],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 4,
        desc: [[1,0,0,0],[1,1,0,0],[0,1,0,0],[0,1,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 4,
        desc: [[0,1,0,0],[1,1,0,0],[1,0,0,0],[1,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 4,
        desc: [[0,1,0,0],[0,1,0,0],[1,1,0,0],[1,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 4,
        desc: [[1,0,0,0],[1,0,0,0],[1,1,0,0],[0,1,0,0]],
    };
    shp.sub_shapes.push(sub);

    unsafe {
        G_DESCRIPTOR.borrow_mut().push(Rc::new(shp));
    }


    /*
     *********************************************
     * ShapeTypeSquareHorn
     *********************************************
     */
    let mut shp = Shape {
        shape_type: ShapeType::ShapeTypeSquareHorn,
        sub_shapes: vec![],
    };

    let sub = SubShape {
        width: 3,
        height: 2,
        desc: [[1,1,0,0],[1,1,1,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 3,
        height: 2,
        desc: [[0,1,1,0],[1,1,1,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 3,
        height: 2,
        desc: [[ 1,1,1,0],[1,1,0,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 3,
        height: 2,
        desc: [[1,1,1,0],[0,1,1,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 3,
        desc: [[1,1,0,0],[1,1,0,0],[0,1,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 3,
        desc: [[1,1,0,0],[1,1,0,0],[1,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 3,
        desc: [[0,1,0,0],[1,1,0,0],[1,1,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 3,
        desc: [[1,0,0,0],[1,1,0,0],[1,1,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    unsafe {
        G_DESCRIPTOR.borrow_mut().push(Rc::new(shp));
    }

    /*
     *********************************************
     * ShapeTypeOpenMouth
     *********************************************
     */
    let mut shp = Shape {
        shape_type: ShapeType::ShapeTypeOpenMouth,
        sub_shapes: vec![],
    };

    let sub = SubShape {
        width: 3,
        height: 2,
        desc: [[1,1,1,0],[1,0,1,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 3,
        height: 2,
        desc: [[1,0,1,0],[1,1,1,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 3,
        desc: [[ 1,1,0,0],[1,0,0,0],[1,1,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 3,
        desc: [[1,1,0,0],[0,1,0,0],[1,1,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    unsafe {
        G_DESCRIPTOR.borrow_mut().push(Rc::new(shp));
    }

    /*
     *********************************************
     * ShapeTypeBigL
     *********************************************
     */
    let mut shp = Shape {
        shape_type: ShapeType::ShapeTypeBigL,
        sub_shapes: vec![],
    };

    let sub = SubShape {
        width: 4,
        height: 2,
        desc: [[1,1,1,1],[1,0,0,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 4,
        height: 2,
        desc: [[1,1,1,1],[0,0,0,1],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 4,
        height: 2,
        desc: [[ 1,0,0,0],[1,1,1,1],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 4,
        height: 2,
        desc: [[0,0,0,1],[1,1,1,1],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 4,
        desc: [[1,1,0,0],[1,0,0,0],[1,0,0,0],[1,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 4,
        desc: [[1,1,0,0],[0,1,0,0],[0,1,0,0],[0,1,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 4,
        desc: [[1,0,0,0],[1,0,0,0],[1,0,0,0],[1,1,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 4,
        desc: [[0,1,0,0],[0,1,0,0],[0,1,0,0],[1,1,0,0]],
    };
    shp.sub_shapes.push(sub);

    unsafe {
        G_DESCRIPTOR.borrow_mut().push(Rc::new(shp));
    }

    /*
     *********************************************
     * ShapeTypeLineHorn
     *********************************************
     */
    let mut shp = Shape {
        shape_type: ShapeType::ShapeTypeLineHorn,
        sub_shapes: vec![],
    };

    let sub = SubShape {
        width: 4,
        height: 2,
        desc: [[1,1,1,1],[0,1,0,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 4,
        height: 2,
        desc: [[1,1,1,1],[0,0,1,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 4,
        height: 2,
        desc: [[ 0,1,0,0],[1,1,1,1],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 4,
        height: 2,
        desc: [[0,0,1,0],[1,1,1,1],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 4,
        desc: [[1,0,0,0],[1,1,0,0],[1,0,0,0],[1,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 4,
        desc: [[0,1,0,0],[1,1,0,0],[0,1,0,0],[0,1,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 4,
        desc: [[1,0,0,0],[1,0,0,0],[1,1,0,0],[1,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 4,
        desc: [[0,1,0,0],[0,1,0,0],[1,1,0,0],[0,1,0,0]],
    };
    shp.sub_shapes.push(sub);

    unsafe {
        G_DESCRIPTOR.borrow_mut().push(Rc::new(shp));
    }

    /*
     *********************************************
     * ShapeTypeSmallL
     *********************************************
     */
    let mut shp = Shape {
        shape_type: ShapeType::ShapeTypeSmallL,
        sub_shapes: vec![],
    };

    let sub = SubShape {
        width: 3,
        height: 2,
        desc: [[1,1,1,0],[1,0,0,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 3,
        height: 2,
        desc: [[1,1,1,0],[0,0,1,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 3,
        height: 2,
        desc: [[ 1,0,0,0],[1,1,1,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 3,
        height: 2,
        desc: [[0,0,1,0],[1,1,1,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 3,
        desc: [[1,1,0,0],[1,0,0,0],[1,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 3,
        desc: [[1,1,0,0],[0,1,0,0],[0,1,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 3,
        desc: [[1,0,0,0],[1,0,0,0],[1,1,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 3,
        desc: [[0,1,0,0],[0,1,0,0],[1,1,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    unsafe {
        G_DESCRIPTOR.borrow_mut().push(Rc::new(shp));
    }

    /*
     *********************************************
     * ShapeTypeSmallCorner
     *********************************************
     */
    let mut shp = Shape {
        shape_type: ShapeType::ShapeTypeSmallCorner,
        sub_shapes: vec![],
    };

    let sub = SubShape {
        width: 2,
        height: 2,
        desc: [[1,0,0,0],[1,1,0,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 2,
        desc: [[0,1,0,0],[1,1,0,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 2,
        desc: [[ 1,1,0,0],[1,0,0,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 2,
        height: 2,
        desc: [[1,1,0,0],[0,1,0,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    unsafe {
        G_DESCRIPTOR.borrow_mut().push(Rc::new(shp));
    }

    /*
     *********************************************
     * ShapeTypeSquare
     *********************************************
     */
    let mut shp = Shape {
        shape_type: ShapeType::ShapeTypeSquare,
        sub_shapes: vec![],
    };

    let sub = SubShape {
        width: 2,
        height: 2,
        desc: [[1,1,0,0],[1,1,0,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    unsafe {
        G_DESCRIPTOR.borrow_mut().push(Rc::new(shp));
    }

    /*
     *********************************************
     * ShapeTypeLine
     *********************************************
     */
    let mut shp = Shape {
        shape_type: ShapeType::ShapeTypeLine,
        sub_shapes: vec![],
    };

    let sub = SubShape {
        width: 4,
        height: 1,
        desc: [[1,1,1,1],[0,0,0,0],[0,0,0,0],[0,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    let sub = SubShape {
        width: 1,
        height: 4,
        desc: [[1,0,0,0],[1,0,0,0],[1,0,0,0],[1,0,0,0]],
    };
    shp.sub_shapes.push(sub);

    unsafe {
        G_DESCRIPTOR.borrow_mut().push(Rc::new(shp));
    }

    // Finally return the descriptors.
    unsafe {
        RefCell::clone(&G_DESCRIPTOR)
    }

}

pub struct ShapeTreeNode {
    pub s_index: usize,
    pub sub_index: usize,
    pub board_x: usize,
    pub board_y: usize,
    pub parent: Option<Rc<ShapeTreeNode>>,
}


impl ShapeTreeNode {
    pub fn new(
        s_index: usize,
        sub_index: usize,
        board_x: usize,
        board_y: usize,
        parent: Option<Rc<ShapeTreeNode>>,
    ) -> ShapeTreeNode {

        ShapeTreeNode {
            s_index,
            sub_index,
            board_x,
            board_y,
            parent,
        }
    }

    pub fn show_node_info(&self) {
        println!("[{},{},{},{}]",self.s_index,self.sub_index,self.board_x,self.board_y);
    }

    // check if the shape can be put on the board.
    pub fn check_shape_on_board(&self, board: &Box<[[usize;BOARD_SIZE];BOARD_SIZE]>) -> bool {
        let sub = match get_sub_shape(self.s_index,self.sub_index) {
            Some(t) => t,
            None => {
                return false;
            }
        };

        // width range
        if self.board_x + sub.width > BOARD_SIZE  || self.board_x > self.board_y + sub.height - 1 {
            return false;
        }

        // height range
        if self.board_y + sub.height > BOARD_SIZE {
            return false;
        }

        for h in 0..sub.height {
            for w in 0..sub.width {
                if sub.desc[h][w] != 0 && board[self.board_y + h][self.board_x + w] != 0 {
                    return false;
                }
            }
        }

        return true;
    }

    // put shape on board.
    pub fn put_shape_on_board(&self, board: &mut Box<[[usize;BOARD_SIZE];BOARD_SIZE]>) -> bool {

        let sub = match get_sub_shape(self.s_index,self.sub_index) {
            Some(t) => t,
            None => {
                return false;
            }
        };

        for h in 0..sub.height {
            for w in 0..sub.width {
                if sub.desc[h][w] != 0 {
                    board[self.board_y + h][self.board_x + w] = self.s_index + 1;
                }
            }
        }

        return true;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_tree () {
        // construct a tree like this.
        //          | b
        // r <- a <-| c
        //          | d
        let r = Rc::new(ShapeTreeNode::new(0,0,0,0,None));
        let a = Rc::new(ShapeTreeNode::new(1,1,1,1,Some(Rc::clone(&r))));
        let b = Rc::new(ShapeTreeNode::new(2,2,2,2,Some(Rc::clone(&a))));
        let c = Rc::new(ShapeTreeNode::new(3,3,3,3,Some(Rc::clone(&a))));
        let d = Rc::new(ShapeTreeNode::new(4,4,4,4,Some(Rc::clone(&a))));

        let mut level_prev = vec![];
        level_prev.push(&a);

        for node in level_prev {
            let mut p = node;
            p.show_node_info();
            while let Some(t) = &p.parent {
                println!("    |    ");
                println!("    v    ");
                t.show_node_info();

                p = t;
            }
            println!("=================");
        }

        println!(" ");
        println!(" ");

        let mut level_prev = vec![];
        level_prev.push(&b);
        level_prev.push(&c);
        level_prev.push(&d);

        for node in level_prev {
            let mut p = node;
            p.show_node_info();
            while let Some(t) = &p.parent {
                println!("    |    ");
                println!("    v    ");
                t.show_node_info();
                p = t;
            }
            println!("=================");
        }
    }

    #[test]
    fn test_get_sub_shape_numbers () {
        let shape_index_max = map_from_shape_to_index(ShapeType::ShapeTypeMax);

        for i in 0..shape_index_max {
            println!("{}:{}", i, get_sub_shape_number(i));
        }
    }
}

