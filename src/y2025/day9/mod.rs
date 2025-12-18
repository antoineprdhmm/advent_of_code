use std::collections::{HashMap, HashSet};

use crate::read_input;

type Coordinate = (isize, isize);

fn read_red_tiles_coordinates() -> Vec<Coordinate> {
    read_input(2025, 9)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            line.split_once(',')
                .map(|(a, b)| (a.parse::<isize>().unwrap(), (b.parse::<isize>().unwrap())))
                .unwrap()
        })
        .collect()
}

pub fn run_part_1() {
    let coordinates = read_red_tiles_coordinates();

    let mut max_area: isize = 0;

    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            let width = (coordinates[i].0 - coordinates[j].0).abs() + 1;
            let height = (coordinates[i].1 - coordinates[j].1).abs() + 1;

            max_area = max_area.max(width * height);
        }
    }

    assert_eq!(max_area, 4755064176);
}

fn find_horizontal_green_borders_coordinates(coordinates: &[Coordinate]) -> Vec<Coordinate> {
    let mut horizontal_coordinates: HashSet<Coordinate> = HashSet::new();

    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            let (a, b) = (coordinates[i], coordinates[j]);

            if a.1 != b.1 {
                continue;
            }

            if a.0 > b.0 {
                for x in (b.0 + 1)..a.0 {
                    horizontal_coordinates.insert((x, coordinates[j].1));
                }
            } else {
                for x in (a.0 + 1)..b.0 {
                    horizontal_coordinates.insert((x, b.1));
                }
            }
        }
    }

    horizontal_coordinates.into_iter().collect()
}

pub fn run_part_2() {
    let coordinates = read_red_tiles_coordinates();
    let horizontal_green_borders_coordinates =
        find_horizontal_green_borders_coordinates(&coordinates);

    /////
    // Bottom half of the Circle

    // The only possible candidate
    let bottom_half_top_right_corner = (94671, 48487);
    // All valid candidates for bottom left corner
    let bottom_candidates: Vec<&Coordinate> = coordinates
        .iter()
        // should be lower (y axis), and on the left (x axis) because valid rectangles at the right will be too small
        .filter(|(x, y)| *x < bottom_half_top_right_corner.0 && *y < bottom_half_top_right_corner.1)
        .collect();
    // find horizontal green tiles (between red tiles -> only the borders) that could collide with a vertical side of the rectangle
    let horizontal_green_border_coordinates: Vec<&Coordinate> =
        horizontal_green_borders_coordinates
            .iter()
            .filter(|(_, y)| *y < bottom_half_top_right_corner.1)
            .collect();
    // Put these green tiles in a Map by x for easy access
    let mut horizontal_green_border_coordinates_by_x: HashMap<isize, Vec<isize>> = HashMap::new();
    for c in horizontal_green_border_coordinates {
        horizontal_green_border_coordinates_by_x
            .entry(c.0)
            .or_default()
            .push(c.1);
    }

    let mut max_bottom_area: isize = 0;
    let mut max_candidate = bottom_candidates[0];

    for candidate in bottom_candidates {
        let width = (candidate.0 - bottom_half_top_right_corner.0).abs() + 1;
        let height = (candidate.1 - bottom_half_top_right_corner.1).abs() + 1;
        let candidate_area = width * height;

        if candidate_area > max_bottom_area {
            let horizontal_green_border_coordinates_opt =
                horizontal_green_border_coordinates_by_x.get(&candidate.0);

            // We found a bigger rectangle, let's make sure it's valid (fully contained in the circle, not crossing borders)
            let mut is_valid = true;
            // Check if the left side of the rectangle cross some green borders
            if let Some(horizontal_green_border_coordinates) =
                horizontal_green_border_coordinates_opt
            {
                for c in horizontal_green_border_coordinates {
                    if c > &candidate.1 {
                        is_valid = false;
                    }
                }
            }
            let horizontal_green_border_coordinates_opt =
                horizontal_green_border_coordinates_by_x.get(&bottom_half_top_right_corner.0);
            // Check if the right side of the rectangle cross some green borders
            if let Some(horizontal_green_border_coordinates) =
                horizontal_green_border_coordinates_opt
            {
                for c in horizontal_green_border_coordinates {
                    if c > &candidate.1 {
                        is_valid = false;
                    }
                }
            }

            if is_valid {
                // if it's valid we found a bigger rectangle !
                max_bottom_area = candidate_area;
                max_candidate = candidate;
            }
        }
    }

    println!(
        "Max bottom area = {:?}, with corner {:?}",
        max_bottom_area, max_candidate
    );
    // With corner (4466, 34127)
    assert_eq!(max_bottom_area, 1295448366);

    /////
    // Top half of the Circle

    // The only possible candidate
    let top_half_bottom_right_corner = (94671, 50270);
    // All valid candidates for top left corner
    let top_candidates: Vec<&Coordinate> = coordinates
        .iter()
        // should be upper (y axis), and on the left (x axis) because valid rectangles at the right will be too small
        .filter(|(x, y)| *x < top_half_bottom_right_corner.0 && *y > top_half_bottom_right_corner.1)
        .collect();
    // find horizontal green tiles (between red tiles -> only the borders) that could collide with a vertical side of the rectangle
    let horizontal_green_border_coordinates: Vec<&Coordinate> =
        horizontal_green_borders_coordinates
            .iter()
            .filter(|(_, y)| *y > top_half_bottom_right_corner.1)
            .collect();
    // Put these green tiles in a Map by x for easy access
    let mut horizontal_green_border_coordinates_by_x: HashMap<isize, Vec<isize>> = HashMap::new();
    for c in horizontal_green_border_coordinates {
        horizontal_green_border_coordinates_by_x
            .entry(c.0)
            .or_default()
            .push(c.1);
    }

    let mut max_top_area: isize = 0;
    let mut max_candidate: &(isize, isize) = top_candidates[0];

    for candidate in top_candidates {
        let width = (candidate.0 - top_half_bottom_right_corner.0).abs() + 1;
        let height = (candidate.1 - top_half_bottom_right_corner.1).abs() + 1;
        let candidate_area = width * height;

        if candidate_area > max_top_area {
            // We found a bigger rectangle, let's make sure it's valid (fully contained in the circle, not crossing borders)
            let mut is_valid = true;

            let horizontal_green_border_coordinates_opt =
                horizontal_green_border_coordinates_by_x.get(&candidate.0);
            // Check if the left side of the rectangle cross some green borders
            if let Some(horizontal_green_border_coordinates) =
                horizontal_green_border_coordinates_opt
            {
                for c in horizontal_green_border_coordinates {
                    if c < &candidate.1 {
                        is_valid = false;
                    }
                }
            }
            let horizontal_green_border_coordinates_opt =
                horizontal_green_border_coordinates_by_x.get(&top_half_bottom_right_corner.0);
            // Check if the right side of the rectangle cross some green borders
            if let Some(horizontal_green_border_coordinates) =
                horizontal_green_border_coordinates_opt
            {
                for c in horizontal_green_border_coordinates {
                    if c < &candidate.1 {
                        is_valid = false;
                    }
                }
            }

            if is_valid {
                // if it's valid we found a bigger rectangle !
                max_top_area = candidate_area;
                max_candidate = candidate;
            }
        }
    }

    println!(
        "Max top area = {:?}, with corner {:?}",
        max_top_area, max_candidate
    );
    // With corner (6165, 68497)
    assert_eq!(max_top_area, 1613305596);
}
