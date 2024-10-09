#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let superlist = second_list.is_empty()
        || first_list
            .windows(second_list.len())
            .any(|l| l == second_list);
    let sublist = first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|l| l == first_list);

    match (superlist, sublist) {
        (true, true) => Comparison::Equal,
        (false, true) => Comparison::Sublist,
        (true, false) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }

    /*
        let common_elements = match first_list.len() >= second_list.len() {
            true => calculate(first_list, second_list),
            _ => calculate(second_list, first_list)
        };
        match (first_list.len(), second_list.len(), common_elements) {
            (0, 0, _) => Comparison::Equal,
            (0, _, 0) => Comparison::Sublist,
            (_, 0, 0) => Comparison::Superlist,
            (len1, len2, _) if len1 > len2 => {
                if common_elements == len2 {
                    Comparison::Superlist
                } else {
                    Comparison::Unequal
                }
            }
            (len1, len2, _) if len2 > len1 => {
                if common_elements == len1 {
                    Comparison::Sublist
                } else {
                    Comparison::Unequal
                }
            }
            _ => {
                if common_elements == first_list.len() && common_elements == second_list.len() {
                    Comparison::Equal
                } else {
                    Comparison::Unequal
                }
            }
        }
    */
}

// fn calculate<T: PartialEq>(longer: &[T], shorter: &[T]) -> usize {
//     match shorter {
//         [] => 0,
//         _ => {
//             let count = common_elements(longer, shorter);
//             if count == shorter.len() {
//                 count
//             } else if longer.len() > shorter.len() {
//                 calculate(&longer[1..], shorter)
//             } else {
//                 0
//             }
//         }
//     }
// }

// fn common_elements<T: PartialEq>(longer: &[T], shorter: &[T]) -> usize {
//     shorter
//         .iter()
//         .enumerate()
//         .filter(|(i, x)| longer.get(*i) == Some(x))
//         .count()
// }
