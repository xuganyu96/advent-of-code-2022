use std::fs;

/// Return True iff the tree at the input is visible
fn is_visible(grid: &[Vec<u32>], tree: (usize, usize)) -> bool {
    let (mut from_top, mut from_bot, mut from_left, mut from_right) = (true, true, true, true);

    let nrows = grid.len();
    let ncols = grid.get(0).unwrap().len();
    let (tree_r, tree_c) = tree;
    let tree_h = grid.get(tree_r).unwrap().get(tree_c).unwrap();

    for r in 0..tree_r {
        let other_tree = grid.get(r).unwrap().get(tree_c).unwrap();
        if *other_tree >= *tree_h {
            from_top = false;
        }
    }

    for r in (tree_r + 1)..nrows {
        let other_tree = grid.get(r).unwrap().get(tree_c).unwrap();
        if *other_tree >= *tree_h {
            from_bot = false;
        }
    }

    for c in 0..tree_c {
        let other_tree = grid.get(tree_r).unwrap().get(c).unwrap();
        if *other_tree >= *tree_h {
            from_left = false;
        }
    }
    for c in (tree_c + 1)..ncols {
        let other_tree = grid.get(tree_r).unwrap().get(c).unwrap();
        if *other_tree >= *tree_h {
            from_right = false;
        }
    }
    return from_top || from_bot || from_left || from_right;
}

fn view_score(grid: &[Vec<u32>], tree: (usize, usize)) -> u32 {
    let (mut from_top, mut from_bot, mut from_left, mut from_right) = (0, 0, 0, 0);
    let nrows = grid.len();
    let ncols = grid.get(0).unwrap().len();
    let (tree_r, tree_c) = tree;
    let tree_h = grid.get(tree_r).unwrap().get(tree_c).unwrap();

    // look up
    let mut r = (tree_r as i32) - 1;
    while r >= 0 {
        let other_tree = grid.get(r as usize).unwrap().get(tree_c).unwrap();
        from_top += 1;
        if other_tree >= tree_h {
            break;
        }
        r -= 1;
    }
    // look down
    let mut r = tree_r + 1;
    while r < nrows {
        let other_tree = grid.get(r as usize).unwrap().get(tree_c).unwrap();
        from_bot += 1;
        if other_tree >= tree_h {
            break;
        }
        r += 1;
    }
    // look left
    let mut c = (tree_c as i32) - 1;
    while c >= 0 {
        let other_tree = grid.get(tree_r).unwrap().get(c as usize).unwrap();
        from_left += 1;
        if other_tree >= tree_h {
            break;
        }
        c -= 1;
    }
    // look right
    let mut c = tree_c + 1;
    while c < ncols {
        let other_tree = grid.get(tree_r).unwrap().get(c as usize).unwrap();
        from_right += 1;
        if other_tree >= tree_h {
            break;
        }
        c += 1;
    }

    return from_top * from_bot * from_left * from_right;
}

fn main() {
    let input = fs::read_to_string("inputs/8.txt").unwrap();
    let mut grid: Vec<Vec<u32>> = vec![];

    input.lines().for_each(|line| {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        grid.push(row);
    });

    let nrows = grid.len();
    let ncols = grid.get(0).unwrap().len();

    let mut sum = 0;
    let mut max = 0;
    for r in 0..nrows {
        for c in 0..ncols {
            if is_visible(&grid, (r, c)) {
                sum += 1;
            }
            let score = view_score(&grid, (r, c));
            if score > max {
                max = score;
            }
        }
    }
    println!("{sum}, {max}");
}
