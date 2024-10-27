pub fn raindrops(n: u32) -> String {
    let res3 = match n%3 == 0 {
        true => "Pling",
        false => "" 
    };
    let res5 = match n%5 == 0 {
        true => "Plang",
        false => "" 
    };
    let res7 = match n%7 == 0 {
        true => "Plong",
        false => "" 
    };
    match format!("{res3}{res5}{res7}") {
        res if res.is_empty() => n.to_string(),
        res => res
    }
}


