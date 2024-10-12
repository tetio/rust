static NEIGBOURHOOD_OFFSETS: &'static [(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len() as i32;
    (0..height).map(|y| {
        let width = minefield[y as usize].len() as i32;
        (0..width).map(|x| {
            if minefield[y as usize].as_bytes()[x as usize] == b'*' {
                '*'
            } else {
                match NEIGBOURHOOD_OFFSETS.iter()
                .map(|&(ox, oy)| {(x+ox, y+oy)})
                .filter(|&(x, y)| {x >= 0 && y >= 0 && x < width && y < height})
                .filter(|&(x,y)| {minefield[y as usize].as_bytes()[x as usize] == b'*'})
                .count() {
                    0 => ' ',
                    n => (n as u8 + '0' as u8) as char 
                }

            }
        }).collect()
    }).collect()

    // if minefield.is_empty() {
    //     return vec![];
    // }
    // if minefield == [""] {
    //     return ["".to_string()].to_vec();
    // }
    // let field = minefield
    //     .iter()
    //     .flat_map(|s| s.as_bytes())
    //     .collect::<Vec<&u8>>();
    // let width = minefield[0].len();
    // let height = minefield.len();
    // let mut aux_result = vec![String::new(); width * height];

    // for x in 0_usize..width {
    //     for y in 0_usize..height {
    //         if *field[y * width + x] == b'*' {
    //             aux_result[y * width + x] = "*".to_string();
    //             continue;
    //         }
    //         let mut count = 0;
    //         for dx in [-1_i32, 0, 1] {
    //             for dy in [-1_i32, 0, 1] {
    //                 if dx == 0 && dy == 0 {
    //                     continue;
    //                 }
    //                 let nx: i32 = x as i32 + dx;
    //                 let ny = y as i32 + dy;
    //                 if nx >= 0
    //                     && nx < width as i32
    //                     && ny >= 0
    //                     && ny < height as i32
    //                     && *field[ny as usize * width + nx as usize] == b'*'
    //                 {
    //                     count += 1;
    //                 }
    //             }
    //             if count == 0 {
    //                 aux_result[y * width + x] = " ".to_string();
    //             } else {
    //                 aux_result[y * width + x] = count.to_string();
    //             }
    //         }
    //     }
    // }
    // let mut result: Vec<String> = Vec::new();
    // for i in (0..aux_result.len()).step_by(width) {
    //     let mut s = String::new();
    //     for n in 0..width {
    //         s = format!("{}{}", s, aux_result[i + n]);
    //     }
    //     result.push(s);
    // }
    // result
}
