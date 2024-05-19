use std::cmp::PartialEq;
use std::{fmt};
use union_find::UnionFind;

#[derive(Clone)]
pub struct Board(Vec<Vec<Cell>>);

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.0.iter() {
            for cell in row.iter() {
                match cell {
                    Cell::Wall(wall) => {
                        match wall {
                            WallEnum::Wall => write!(f, " W")?,
                            WallEnum::Hint(dir, num) => write!(f, "{}{}", dir_to_char(dir.clone()), num)?,
                        }
                    }
                    Cell::Space(Some(one), Some(twh)) => {
                        write!(f, "{}{}", dir_to_char(one.clone()), dir_to_char(twh.clone()))?
                    }
                    Cell::Space(Some(one), _) => {
                        write!(f, "{}?", dir_to_char(one.clone()))?
                    }
                    Cell::Space(_, _) => write!(f, "..")?,
                    Cell::Unknown => write!(f, "??")?,
                }
            }
            write!(f, "\n")?;
        }
        write!(f, " ")
    }
}

#[derive(Debug, Clone)]
enum DirectionEnum {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl PartialEq for DirectionEnum {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DirectionEnum::None, DirectionEnum::None) => true,
            (DirectionEnum::Up, DirectionEnum::Up) => true,
            (DirectionEnum::Down, DirectionEnum::Down) => true,
            (DirectionEnum::Left, DirectionEnum::Left) => true,
            (DirectionEnum::Right, DirectionEnum::Right) => true,
            _ => false,
        }
    }
}

impl DirectionEnum {
    fn to_vector(&self) -> (i32, i32) {
        match self {
            DirectionEnum::None => (0, 0),
            DirectionEnum::Up => (-1, 0),
            DirectionEnum::Down => (1, 0),
            DirectionEnum::Left => (0, -1),
            DirectionEnum::Right => (0, 1),
        }
    }
    fn reverse(&self) -> DirectionEnum {
        match self {
            DirectionEnum::None => DirectionEnum::None,
            DirectionEnum::Up => DirectionEnum::Down,
            DirectionEnum::Down => DirectionEnum::Up,
            DirectionEnum::Left => DirectionEnum::Right,
            DirectionEnum::Right => DirectionEnum::Left,
        }
    }
}

#[derive(Debug, Clone)]
enum WallEnum {
    Wall,
    Hint(DirectionEnum, i64),
}

#[derive(Debug, Clone)]
enum CellEnum {
    Wall(WallEnum),
    Space(Option<DirectionEnum>, Option<DirectionEnum>),
    Unknown,
}

type Cell = CellEnum;

fn main() {
    // let problem = "2/2/d";
    // let problem = "3/3/40";
    // let problem = "2/3/41";
    // let problem = "2/5/j";
    // let problem = "5/5/g22q";
    let problem = "10/10/202022l40i4141h40f122242l31i2131h30b42c101210c41i";
    // let problem = "10/10/23l24zg21c42n13b11l42m14c";

    let board = create_board(problem);
    println!("{}", board);
    println!("{:?}", check(&board));

    let (result, result_board) = solve(&board);
    println!("{:?}", result);
    match result_board {
        Some(b) => println!("{}", b),
        None => println!("None")
    }
}

fn create_board(problem: &str) -> Board {
    // まずは/で分割
    let iter: Vec<&str> = problem.split("/").collect();

    // 幅と高さを取得
    let width: usize = iter[0].parse().unwrap();
    let height: usize = iter[1].parse().unwrap();

    // iter[2]を一文字ずつ取り出して処理
    let mut b = create_board_sub(iter[2].chars());
    b.extend(vec![CellEnum::Unknown; (height * width - b.len()) as usize]);

    return Board(b.chunks(width).map(|x| x.to_vec()).collect());
}


/*
var ca = bstr.charAt(i), cell=bd.cell[c];

if(this.include(ca,"0","4")){
	var ca1 = bstr.charAt(i+1);
	cell.qdir = parseInt(ca,16);
	cell.qnum = (ca1!=="." ? parseInt(ca1,16) : -2);
	i++;
}
else if(this.include(ca,"5","9")){
	cell.qdir = parseInt(ca,16)-5;
	cell.qnum = parseInt(bstr.substr(i+1,2),16);
	i+=2;
}
else if(ca==="-"){
	cell.qdir = parseInt(bstr.substr(i+1,1),16);
	cell.qnum = parseInt(bstr.substr(i+2,3),16);
	i+=4;
}
else if(ca>='a' && ca<='z'){ c+=(parseInt(ca,36)-10);}

c++;
if(!bd.cell[c]){ break;}
 */
fn create_board_sub(mut chars: std::str::Chars) -> Vec<CellEnum> {
    let now = chars.next();
    return match now {
        None => {
            Vec::new()
        }
        Some(c) => {
            match c {
                '0'..='4' => {
                    let next = chars.next();
                    let qdir = c.to_digit(16).unwrap();
                    let qnum = match next {
                        Some('.') => -2,
                        Some(ca1) => ca1.to_digit(16).unwrap() as i64,
                        None => -2,
                    };
                    let mut result = vec![Cell::Wall(WallEnum::Hint(dir_to_direction_enum(qdir), qnum))];
                    result.extend(create_board_sub(chars));
                    result
                }
                '5'..='9' => {
                    let qdir = c.to_digit(16).unwrap();
                    let qnum = chars.next().unwrap().to_digit(16).unwrap() as i64 * 16 + chars.next().unwrap().to_digit(16).unwrap() as i64;
                    let mut result = vec![Cell::Wall(WallEnum::Hint(dir_to_direction_enum(qdir), qnum))];
                    result.extend(create_board_sub(chars));
                    result
                }
                '-' => {
                    let qdir = chars.next().unwrap().to_digit(16).unwrap();
                    let qnum = chars.next().unwrap().to_digit(16).unwrap() as i64 * 16 * 16 + chars.next().unwrap().to_digit(16).unwrap() as i64 * 16 + chars.next().unwrap().to_digit(16).unwrap() as i64;
                    let mut result = vec![Cell::Wall(WallEnum::Hint(dir_to_direction_enum(qdir), qnum))];
                    result.extend(create_board_sub(chars));
                    result
                }
                'a'..='z' => {
                    let c = c.to_digit(36).unwrap() - 'a'.to_digit(36).unwrap() + 1;
                    let mut result = vec![Cell::Unknown; c as usize];
                    result.extend(create_board_sub(chars));
                    result
                }
                _ => {
                    panic!("error")
                }
            }
        }
    };
}

fn dir_to_direction_enum(dir: u32) -> DirectionEnum {
    return match dir {
        1 => DirectionEnum::Up,
        2 => DirectionEnum::Down,
        3 => DirectionEnum::Left,
        4 => DirectionEnum::Right,
        _ => DirectionEnum::None,
    };
}

fn dir_to_char(dir: DirectionEnum) -> char {
    return match dir {
        DirectionEnum::Up => '↑',
        DirectionEnum::Down => '↓',
        DirectionEnum::Left => '←',
        DirectionEnum::Right => '→',
        _ => ' ',
    };
}

#[derive(Debug)]
enum CheckResultInvalidEnum {
    AdjacentWall(i32, i32),
    Hint(i32, i32),
    NoAnswer,
}

#[derive(Debug)]
enum CheckResultEnum {
    Valid, // 矛盾点はない
    Invalid(CheckResultInvalidEnum), // 矛盾点がある
    Complete, // 解けている
}

impl PartialEq for CellEnum {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CellEnum::Wall(WallEnum::Wall), CellEnum::Wall(WallEnum::Wall)) => true,
            (CellEnum::Wall(WallEnum::Hint(_, _)), CellEnum::Wall(WallEnum::Hint(_, _))) => true,
            (CellEnum::Space(_, _), CellEnum::Space(_, _)) => true,
            (CellEnum::Unknown, CellEnum::Unknown) => true,
            _ => false,
        }
    }
}


fn check(board: &Board) -> CheckResultEnum {
    let mut complete_flag = true;
    for (i_x, row) in board.0.iter().enumerate() {
        let i = i_x as i32;
        for (j_x, cell) in row.iter().enumerate() {
            let j = j_x as i32;
            match cell {
                Cell::Wall(wall) => {
                    match wall {
                        WallEnum::Wall => {
                            // 4方向を確認し、壁があれば矛盾
                            if i - 1 >= 0 {
                                if board.0[(i - 1) as usize][j as usize] == Cell::Wall(WallEnum::Wall) {
                                    return CheckResultEnum::Invalid(CheckResultInvalidEnum::AdjacentWall(i, j));
                                }
                            }
                            if i + 1 < board.0.len() as i32 {
                                if board.0[(i + 1) as usize][j as usize] == Cell::Wall(WallEnum::Wall) {
                                    return CheckResultEnum::Invalid(CheckResultInvalidEnum::AdjacentWall(i, j));
                                }
                            }
                            if j - 1 >= 0 {
                                if board.0[i as usize][(j - 1) as usize] == Cell::Wall(WallEnum::Wall) {
                                    return CheckResultEnum::Invalid(CheckResultInvalidEnum::AdjacentWall(i, j));
                                }
                            }
                            if j + 1 < row.len() as i32 {
                                if board.0[i as usize][(j + 1) as usize] == Cell::Wall(WallEnum::Wall) {
                                    return CheckResultEnum::Invalid(CheckResultInvalidEnum::AdjacentWall(i, j));
                                }
                            }
                        }
                        WallEnum::Hint(dir, num) => {
                            if dir.clone() == DirectionEnum::None {
                                continue;
                            }
                            let vec = dir.to_vector();
                            let mut now = (i as i32, j as i32);
                            let mut wall_count: i64 = 0;
                            let mut unknown_count: i64 = 0;
                            let mut prev_wall_flag = false;
                            // breakされるまで
                            loop {
                                now.0 += vec.0;
                                now.1 += vec.1;
                                if now.0 < 0 || now.0 >= board.0.len() as i32 || now.1 < 0 || now.1 >= row.len() as i32 {
                                    break;
                                }
                                if board.0[now.0 as usize][now.1 as usize] == Cell::Wall(WallEnum::Wall) {
                                    wall_count += 1;
                                    prev_wall_flag = true;
                                    continue;
                                } else if board.0[now.0 as usize][now.1 as usize] == Cell::Unknown {
                                    if !prev_wall_flag {
                                        unknown_count += 1;
                                        prev_wall_flag = true
                                    } else {
                                        prev_wall_flag = false;
                                    }
                                    continue;
                                }
                                prev_wall_flag = false;
                            }
                            // 壁の数がすでにnumを超えていたら矛盾
                            if wall_count.clone() > *num {
                                return CheckResultEnum::Invalid(CheckResultInvalidEnum::Hint(i, j));
                            }
                            // 壁の数壁を置けるマスの数の合計がnumを超えていたら矛盾
                            if wall_count.clone() + unknown_count.clone() < *num {
                                return CheckResultEnum::Invalid(CheckResultInvalidEnum::Hint(i, j));
                            }
                        }
                    }
                }
                Cell::Space(one, two) => {
                    if let CheckResultEnum::Invalid(_) = check_direction_and_return_result(one, i, j, &board) {
                        return CheckResultEnum::Invalid(CheckResultInvalidEnum::Hint(i, j));
                    }
                    if let CheckResultEnum::Invalid(_) = check_direction_and_return_result(two, i, j, &board) {
                        return CheckResultEnum::Invalid(CheckResultInvalidEnum::Hint(i, j));
                    }
                    if *one == None || *two == None {
                        complete_flag = false;
                    }
                }
                Cell::Unknown => {
                    complete_flag = false;
                }
            }
        }
    }

    // TODO 一つの線で繋がっていることのチェック
    if complete_flag {
        return CheckResultEnum::Complete;
    }
    return CheckResultEnum::Valid;
}

fn check_direction_and_continue(direction: &Option<DirectionEnum>, next: (i32, i32), target: (i32, i32)) -> bool {
    match direction {
        Some(dir) => {
            let vec = dir.to_vector();
            let next = (next.0 + vec.0, next.1 + vec.1);
            next == target
        }
        None => false,
    }
}

fn check_direction_and_return_result(direction: &Option<DirectionEnum>, i: i32, j: i32, board: &Board) -> CheckResultEnum {
    match direction {
        Some(now) => {
            let vec = now.to_vector();
            let next = (i + vec.0, j + vec.1);
            if next.0 < 0 || next.0 >= board.0.len() as i32 || next.1 < 0 || next.1 >= board.0[0].len() as i32 {
                return CheckResultEnum::Invalid(CheckResultInvalidEnum::Hint(i, j));
            }
            match &board.0[next.0 as usize][next.1 as usize] {
                Cell::Space(another_one, another_two) => {
                    if check_direction_and_continue(another_one, next, (i, j)) {
                        return CheckResultEnum::Valid;
                    }
                    if check_direction_and_continue(another_two, next, (i, j)) {
                        return CheckResultEnum::Valid;
                    }
                    CheckResultEnum::Invalid(CheckResultInvalidEnum::Hint(i, j))
                }
                _ => CheckResultEnum::Invalid(CheckResultInvalidEnum::Hint(i, j)),
            }
        }
        None => CheckResultEnum::Valid,
    }
}

fn solve(board: &Board) -> (CheckResultEnum, Option<Board>) {
    let result = check(&board);
    match result {
        CheckResultEnum::Invalid(_) => return (result, None),
        CheckResultEnum::Complete => return (result, Some(board.clone())),
        _ => {}
    }
    // println!("{}", board);
    // sleep(time::Duration::from_millis(100));
    for (i_x, row) in board.0.iter().enumerate() {
        let i = i_x as i32;
        for (j_x, cell) in row.iter().enumerate() {
            let j = j_x as i32;
            match cell {
                CellEnum::Unknown => {
                    let candidate = vec![
                        Cell::Wall(WallEnum::Wall),
                        Cell::Space(Some(DirectionEnum::Up), None),
                        Cell::Space(Some(DirectionEnum::Down), None),
                        Cell::Space(Some(DirectionEnum::Left), None),
                        Cell::Space(Some(DirectionEnum::Right), None),
                    ];
                    for c in candidate {
                        let mut new_board = board.clone();
                        match c.clone() {
                            Cell::Space(Some(one), _) => {
                                let vec = one.to_vector();
                                let next = (i + vec.0, j + vec.1);
                                if next.0 < 0 || next.0 >= board.0.len() as i32 || next.1 < 0 || next.1 >= row.len() as i32 {
                                    continue;
                                }
                                match new_board.0[next.0 as usize][next.1 as usize].clone() {
                                    Cell::Unknown | Cell::Space(None, _) => {
                                        new_board.0[next.0 as usize][next.1 as usize] = Cell::Space(Some(one.reverse()), None);
                                    }
                                    Cell::Space(Some(another), None) => {
                                        new_board.0[next.0 as usize][next.1 as usize] = Cell::Space(Some(another), Some(one.reverse()));
                                    }
                                    _ => continue
                                }
                            }
                            CellEnum::Wall(_) => {
                                // 上下左右は必ずSpace
                                if i + 1 < board.0.len() as i32 && board.0[(i + 1) as usize][j as usize] == Cell::Unknown {
                                    new_board.0[(i + 1) as usize][j as usize] = Cell::Space(None, None);
                                }
                                if i - 1 >= 0 && board.0[(i - 1) as usize][j as usize] == Cell::Unknown {
                                    new_board.0[(i - 1) as usize][j as usize] = Cell::Space(None, None);
                                }
                                if j + 1 < row.len() as i32 && board.0[i as usize][(j + 1) as usize] == Cell::Unknown {
                                    new_board.0[i as usize][(j + 1) as usize] = Cell::Space(None, None);
                                }
                                if j - 1 >= 0 && board.0[i as usize][(j - 1) as usize] == Cell::Unknown {
                                    new_board.0[i as usize][(j - 1) as usize] = Cell::Space(None, None);
                                }

                                // 壁と壁に挟まれているのであれば、斜めはSpace
                                // TODO 線を引いても大丈夫
                                if i + 2 < board.0.len() as i32 {
                                    match board.0[(i + 2) as usize][j as usize].clone() {
                                        Cell::Wall(_) => {
                                            if j + 1 < row.len() as i32 && board.0[(i + 1) as usize][(j + 1) as usize] == Cell::Unknown {
                                                new_board.0[(i + 1) as usize][(j + 1) as usize] = Cell::Space(None, None);
                                            }
                                            if j - 1 >= 0 && board.0[(i + 1) as usize][(j - 1) as usize] == Cell::Unknown {
                                                new_board.0[(i + 1) as usize][(j - 1) as usize] = Cell::Space(None, None);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                if i + 2 == board.0.len() as i32 {
                                    if j + 1 < row.len() as i32 && board.0[(i + 1) as usize][(j + 1) as usize] == Cell::Unknown {
                                        new_board.0[(i + 1) as usize][(j + 1) as usize] = Cell::Space(None, None);
                                    }
                                    if j - 1 >= 0 && board.0[(i + 1) as usize][(j - 1) as usize] == Cell::Unknown {
                                        new_board.0[(i + 1) as usize][(j - 1) as usize] = Cell::Space(None, None);
                                    }
                                }

                                if i - 2 >= 0 {
                                    match board.0[(i - 2) as usize][j as usize].clone() {
                                        Cell::Wall(_) => {
                                            if j + 1 < row.len() as i32 && board.0[(i - 1) as usize][(j + 1) as usize] == Cell::Unknown {
                                                new_board.0[(i - 1) as usize][(j + 1) as usize] = Cell::Space(None, None);
                                            }
                                            if j - 1 >= 0 && board.0[(i - 1) as usize][(j - 1) as usize] == Cell::Unknown {
                                                new_board.0[(i - 1) as usize][(j - 1) as usize] = Cell::Space(None, None);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                if i - 2 == 0 {
                                    if j + 1 < row.len() as i32 && board.0[(i - 1) as usize][(j + 1) as usize] == Cell::Unknown {
                                        new_board.0[(i - 1) as usize][(j + 1) as usize] = Cell::Space(None, None);
                                    }
                                    if j - 1 >= 0 && board.0[(i - 1) as usize][(j - 1) as usize] == Cell::Unknown {
                                        new_board.0[(i - 1) as usize][(j - 1) as usize] = Cell::Space(None, None);
                                    }
                                }

                                if j + 2 < row.len() as i32 {
                                    match board.0[i as usize][(j + 2) as usize].clone() {
                                        Cell::Wall(_) => {
                                            if i + 1 < board.0.len() as i32 && board.0[(i + 1) as usize][(j + 1) as usize] == Cell::Unknown {
                                                new_board.0[(i + 1) as usize][(j + 1) as usize] = Cell::Space(None, None);
                                            }
                                            if i - 1 >= 0 && board.0[(i - 1) as usize][(j + 1) as usize] == Cell::Unknown {
                                                new_board.0[(i - 1) as usize][(j + 1) as usize] = Cell::Space(None, None);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                if j + 2 == row.len() as i32 {
                                    if i + 1 < board.0.len() as i32 && board.0[(i + 1) as usize][(j + 1) as usize] == Cell::Unknown {
                                        new_board.0[(i + 1) as usize][(j + 1) as usize] = Cell::Space(None, None);
                                    }
                                    if i - 1 >= 0 && board.0[(i - 1) as usize][(j + 1) as usize] == Cell::Unknown {
                                        new_board.0[(i - 1) as usize][(j + 1) as usize] = Cell::Space(None, None);
                                    }
                                }

                                if j - 2 >= 0 {
                                    match board.0[i as usize][(j - 2) as usize].clone() {
                                        Cell::Wall(_) => {
                                            if i + 1 < board.0.len() as i32 && board.0[(i + 1) as usize][(j - 1) as usize] == Cell::Unknown {
                                                new_board.0[(i + 1) as usize][(j - 1) as usize] = Cell::Space(None, None);
                                            }
                                            if i - 1 >= 0 && board.0[(i - 1) as usize][(j - 1) as usize] == Cell::Unknown {
                                                new_board.0[(i - 1) as usize][(j - 1) as usize] = Cell::Space(None, None);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                if j - 2 == 0 {
                                    if i + 1 < board.0.len() as i32 && board.0[(i + 1) as usize][(j - 1) as usize] == Cell::Unknown {
                                        new_board.0[(i + 1) as usize][(j - 1) as usize] = Cell::Space(None, None);
                                    }
                                    if i - 1 >= 0 && board.0[(i - 1) as usize][(j - 1) as usize] == Cell::Unknown {
                                        new_board.0[(i - 1) as usize][(j - 1) as usize] = Cell::Space(None, None);
                                    }
                                }
                            }
                            _ => {}
                        }
                        new_board.0[i as usize][j as usize] = c;
                        let (result, board) = solve(&new_board);
                        if let CheckResultEnum::Complete = result {
                            return (result, board);
                        }
                    }
                    return (CheckResultEnum::Invalid(CheckResultInvalidEnum::NoAnswer), None);
                }
                CellEnum::Space(None, _) => {
                    let candidate = vec![
                        Cell::Space(Some(DirectionEnum::Up), None),
                        Cell::Space(Some(DirectionEnum::Down), None),
                        Cell::Space(Some(DirectionEnum::Left), None),
                        Cell::Space(Some(DirectionEnum::Right), None),
                    ];
                    for c in candidate {
                        let mut new_board = board.clone();
                        match c.clone() {
                            Cell::Space(Some(one), _) => {
                                let vec = one.to_vector();
                                let next = (i + vec.0, j + vec.1);
                                if next.0 < 0 || next.0 >= board.0.len() as i32 || next.1 < 0 || next.1 >= row.len() as i32 {
                                    continue;
                                }
                                match new_board.0[next.0 as usize][next.1 as usize].clone() {
                                    Cell::Unknown | Cell::Space(None, _) => {
                                        new_board.0[next.0 as usize][next.1 as usize] = Cell::Space(Some(one.reverse()), None);
                                    }
                                    Cell::Space(Some(another), None) => {
                                        new_board.0[next.0 as usize][next.1 as usize] = Cell::Space(Some(another), Some(one.reverse()));
                                    }
                                    _ => continue
                                }
                            }
                            _ => {}
                        }
                        new_board.0[i as usize][j as usize] = c;
                        let (result, board) = solve(&new_board);
                        if let CheckResultEnum::Complete = result {
                            return (result, board);
                        }
                    }
                    return (CheckResultEnum::Invalid(CheckResultInvalidEnum::NoAnswer), None);
                }
                CellEnum::Space(Some(r), None) => {
                    let candidate = vec![
                        Cell::Space(Some(r.clone()), Some(DirectionEnum::Up)),
                        Cell::Space(Some(r.clone()), Some(DirectionEnum::Down)),
                        Cell::Space(Some(r.clone()), Some(DirectionEnum::Left)),
                        Cell::Space(Some(r.clone()), Some(DirectionEnum::Right)),
                    ];
                    for c in candidate {
                        let mut new_board = board.clone();
                        match c.clone() {
                            Cell::Space(Some(another), Some(one)) => {
                                if another == one {
                                    continue; // 同じ方向はダメ
                                }
                                let vec = one.to_vector();
                                let next = (i + vec.0, j + vec.1);
                                if next.0 < 0 || next.0 >= board.0.len() as i32 || next.1 < 0 || next.1 >= row.len() as i32 {
                                    continue;
                                }
                                match new_board.0[next.0 as usize][next.1 as usize].clone() {
                                    Cell::Unknown | Cell::Space(None, _) => {
                                        new_board.0[next.0 as usize][next.1 as usize] = Cell::Space(Some(one.reverse()), None);
                                    }
                                    Cell::Space(Some(another), None) => {
                                        new_board.0[next.0 as usize][next.1 as usize] = Cell::Space(Some(another), Some(one.reverse()));
                                    }
                                    _ => continue
                                }
                            }
                            _ => {}
                        }
                        new_board.0[i as usize][j as usize] = c;
                        let (result, board) = solve(&new_board);
                        if let CheckResultEnum::Complete = result {
                            return (result, board);
                        }
                    }
                    return (CheckResultEnum::Invalid(CheckResultInvalidEnum::NoAnswer), None);
                }
                _ => continue
            }
        }
    }
    return (CheckResultEnum::Invalid(CheckResultInvalidEnum::NoAnswer), None);
}

fn candidates(board: &Board, i: usize, j: usize) -> Vec<Cell> {
    return match &board.0[i][j] {
        Cell::Wall(_) => vec![],
        Cell::Space(Some(_), Some(_)) => vec![],
        Cell::Space(Some(one), None) => {
            vec![DirectionEnum::Up, DirectionEnum::Down, DirectionEnum::Left, DirectionEnum::Right]
                .iter()
                .filter(|dir| {
                    dir != &one
                })
                .filter(|dir| {
                    let vec = dir.to_vector();
                    let next = (i as i32 + vec.0, j as i32 + vec.1);
                    next.0 >= 0 && next.0 < board.0.len() as i32 && next.1 >= 0 && next.1 < board.0[0].len() as i32 &&
                        (match &board.0[next.0 as usize][next.1 as usize] {
                            Cell::Unknown => true,
                            Cell::Space(_, None) => true,
                            _ => false
                        })
                })
                .map(|dir| Cell::Space(Some(one.clone()), Some(dir.clone())))
                .collect()
        }
        Cell::Space(None, _) => {
            let dirs: Vec<DirectionEnum> = vec![DirectionEnum::Up, DirectionEnum::Down, DirectionEnum::Left, DirectionEnum::Right]
                .into_iter()
                .filter(|dir| {
                    let vec = dir.to_vector();
                    let next = (i as i32 + vec.0, j as i32 + vec.1);
                    next.0 >= 0 && next.0 < board.0.len() as i32 && next.1 >= 0 && next.1 < board.0[0].len() as i32 &&
                        (match &board.0[next.0 as usize][next.1 as usize] {
                            Cell::Unknown => true,
                            Cell::Space(_, None) => true,
                            _ => false
                        })
                }).collect();
            dirs
                .iter()
                .flat_map(|item1| {
                    dirs.iter()
                        .filter(move |&item2| *item1 != *item2)
                        .map(move |item2| Cell::Space(Some(item1.clone()), Some(item2.clone())))
                })
                .collect()
        }
        Cell::Unknown => {
            let mut next_board = board.clone();
            next_board.0[i][j] = Cell::Space(None, None);
            let mut ret = candidates(&next_board, i, j);
            ret.push(Cell::Wall(WallEnum::Wall));
            ret
        }
    };
}
