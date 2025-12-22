struct Occupancy {
    occupancy: [[bool; 9]; 9],
}

impl Occupancy {
    pub fn new() -> Self {
        Occupancy {
            occupancy: [[false; 9]; 9],
        }
    }

    pub fn check(&self, index: usize, number: u8) -> bool {
        self.occupancy[index][number as usize]
    }

    pub fn set(&mut self, index: usize, number: u8) {
        self.occupancy[index][number as usize] = true;
    }

    pub fn clear(&mut self, index: usize, number: u8) {
        self.occupancy[index][number as usize] = false;
    }
}

fn row_col_to_box(row: usize, col: usize) -> usize {
    let box_row = row / 3;
    let box_col = col / 3;

    box_row * 3 + box_col
}

#[derive(Debug)]
struct CellDesc {
    row: usize,
    col: usize,
    box_: usize,
}

fn solve_recursive(
    vacancies: &[CellDesc],
    table: &mut [[Option<u8>; 9]; 9],
    row_occ: &mut Occupancy,
    col_occ: &mut Occupancy,
    box_occ: &mut Occupancy,
) -> bool {
    if vacancies.len() == 0 {
        true
    } else {
        let vacancy = &vacancies[0];
        for hyp in 0u8..9 {
            if !row_occ.check(vacancy.row, hyp)
                && !col_occ.check(vacancy.col, hyp)
                && !box_occ.check(vacancy.box_, hyp)
            {
                table[vacancy.row as usize][vacancy.col as usize] = Some(hyp);
                row_occ.set(vacancy.row, hyp);
                col_occ.set(vacancy.col, hyp);
                box_occ.set(vacancy.box_, hyp);

                let hyp_result = solve_recursive(&vacancies[1..], table, row_occ, col_occ, box_occ);

                if hyp_result {
                    return true;
                } else {
                    row_occ.clear(vacancy.row, hyp);
                    col_occ.clear(vacancy.col, hyp);
                    box_occ.clear(vacancy.box_, hyp);
                }
            }
        }
        false
    }
}

struct Solution {}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut sudoku_box: [[Option<u8>; 9]; 9] = board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| match c {
                        '.' => None,
                        c => Some((*c as u8) - ('1' as u8)),
                    })
                    .collect::<Vec<Option<u8>>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[Option<u8>; 9]>>()
            .try_into()
            .unwrap();

        let mut row_occ = Occupancy::new();
        let mut col_occ = Occupancy::new();
        let mut box_occ = Occupancy::new();

        let mut vacancies: Vec<CellDesc> = Vec::new();

        for row in 0..9 {
            for col in 0..9 {
                let box_ = row_col_to_box(row, col);
                match sudoku_box[row][col] {
                    None => {
                        vacancies.push(CellDesc { row, col, box_ });
                    }
                    Some(value) => {
                        row_occ.set(row, value);
                        col_occ.set(col, value);
                        box_occ.set(row_col_to_box(row, col), value);
                    }
                }
            }
        }

        let result = solve_recursive(
            vacancies.as_slice(),
            &mut sudoku_box,
            &mut row_occ,
            &mut col_occ,
            &mut box_occ,
        );

        assert!(result, "Unsolvable sudoku");

        for row in 0..9 {
            for col in 0..9 {
                board[row][col] = match sudoku_box[row][col] {
                    None => '.',
                    Some(c) => (('1' as u8) + c) as char,
                }
            }
        }
    }
}

fn main() {
    let mut board: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    Solution::solve_sudoku(&mut board);

    println!("Board: {:?}", board);
}
