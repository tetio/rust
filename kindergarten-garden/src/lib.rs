static STUDENTS: &[&str] = &[
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let rows = diagram.replace("\n", "").chars().collect::<Vec<char>>();
    let p1 = student_index(student) * 2;
    let p2 = p1 + rows.len() / 2;
    let res = &[rows[p1], rows[p1 + 1], rows[p2], rows[p2 + 1]];
    res.iter().map(|p| get_plant_name(*p)).collect()
}

fn student_index(student: &str) -> usize {
    STUDENTS.iter().position(|&s| s == student).unwrap()
}

fn get_plant_name(plant: char) -> &'static str {
    match plant {
        'G' => "grass",
        'V' => "violets",
        'C' => "clover",
        _ => "radishes"
    }
}
