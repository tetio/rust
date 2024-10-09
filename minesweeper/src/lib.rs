pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // todo!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
    let field = minefield.iter().map(|s| {s.as_bytes()}).flatten().collect::<Vec<u8>>();
    let width = minefield[0].len();

}
