use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(first_word) => list
            .windows(2)
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            .chain(once(format!("And all for the want of a {first_word}.")))
            .collect()
    }
}

// pub fn build_proverb(list: &[&str]) -> String {
//     let l = list.to_vec();
//     do_build_proverb(l, Vec::new(), 0).join("\n")
// }

// fn do_build_proverb(list: Vec<&str>, mut accu: Vec<String>, mut i: usize) -> Vec<String> {
//     match (
//         list.is_empty() || i == list.len(),
//         list.len() == 1 || (list.len() > 1 && i == list.len()-1),
//         list.len() > 1 && i < list.len() - 1,
//     ) {
//         (true, _, _) => accu,
//         (_, true, _) => {
//             let s = list[0];
//             accu.push(format!("And all for the want of a {}.", s));
//             i += 1;
//             do_build_proverb(list, accu, i)
//         }
//         (_, _, true) => {
//             let s1 = list[i];
//             let s2 = list[i + 1];
//             accu.push(format!("For want of a {} the {} was lost.", s1, s2));
//             i += 1;
//             do_build_proverb(list, accu, i)
//         },
//         (_,_,_) => vec![]
//     }
// }
