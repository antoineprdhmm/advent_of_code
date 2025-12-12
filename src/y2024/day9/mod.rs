use crate::read_input;

#[derive(Debug)]
struct MovedFile {
    id: usize,
    number: usize,
}

#[derive(Debug)]
struct FileDiskPart {
    // when file blocks get moved, these are replaced by free space blocks
    // but these are different than input free space blocks
    // Input = file_blocks, free_space_blocks
    // Then = file_free_space_blocks, file_blocks, moved_here, free_space_blocks
    pub file_free_space_blocks: usize,
    pub file_blocks: usize,
    pub moved_here: Vec<MovedFile>,
    pub free_space_blocks: usize,
}

fn compute_checksum(files: Vec<FileDiskPart>) -> usize {
    let mut multiplier = 0;
    let mut checksum = 0;

    for (id, file) in files.iter().enumerate() {
        for _ in 0..file.file_free_space_blocks {
            multiplier += 1;
        }

        for _ in 0..file.file_blocks {
            checksum += multiplier * id;
            multiplier += 1;
        }

        for moved in file.moved_here.iter() {
            for _ in 0..moved.number {
                checksum += multiplier * moved.id;
                multiplier += 1;
            }
        }

        for _ in 0..file.free_space_blocks {
            multiplier += 1;
        }
    }

    checksum
}

fn read_files() -> Vec<FileDiskPart> {
    let lines = read_input(2024, 9).unwrap();
    let mut line: String = lines
        .map_while(Result::ok)
        .collect::<Vec<String>>()
        .join("");
    line.push('0');

    let files: Vec<_> = line
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|pair| FileDiskPart {
            file_blocks: pair[0].to_digit(10).expect("") as usize,
            free_space_blocks: pair[1].to_digit(10).expect("") as usize,
            file_free_space_blocks: 0,
            moved_here: Default::default(),
        })
        .collect();

    files
}

pub fn run_part_2() {
    let mut files = read_files();

    for to_move_idx in (1..files.len()).rev() {
        for candidate_idx in 0..to_move_idx {
            let number_files = files[to_move_idx].file_blocks;

            if number_files > 0 && files[candidate_idx].free_space_blocks >= number_files {
                files[candidate_idx].moved_here.push(MovedFile {
                    id: to_move_idx,
                    number: number_files,
                });
                files[candidate_idx].free_space_blocks -= number_files;
                files[to_move_idx].file_free_space_blocks += number_files;
                files[to_move_idx].file_blocks = 0;
            }
        }
    }

    println!("{}", compute_checksum(files));
}

pub fn run_part_1() {
    let mut files = read_files();

    let mut candidate_idx = 0;
    let mut to_move_idx = files.len() - 1;

    while candidate_idx < to_move_idx {
        let can_move_all_to_candidate =
            files[candidate_idx].free_space_blocks >= files[to_move_idx].file_blocks;

        if can_move_all_to_candidate {
            let number_files = files[to_move_idx].file_blocks;

            files[candidate_idx].free_space_blocks -= number_files;
            files[candidate_idx].moved_here.push(MovedFile {
                id: to_move_idx,
                number: number_files,
            });

            files[to_move_idx].file_blocks = 0;

            to_move_idx -= 1;
            if files[candidate_idx].free_space_blocks == 0 {
                candidate_idx += 1;
            }
        } else {
            let number_files = files[candidate_idx].free_space_blocks;

            files[candidate_idx].free_space_blocks = 0;
            files[candidate_idx].moved_here.push(MovedFile {
                id: to_move_idx,
                number: number_files,
            });

            files[to_move_idx].file_blocks -= number_files;

            candidate_idx += 1;
        }
    }

    println!("{}", compute_checksum(files));
}
