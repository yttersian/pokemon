use crate::Pokemon;

pub fn print_party_members(members: &[Pokemon]) {
    let names: Vec<String> = members
        .iter()
        .map(|pokemon| pokemon.name().to_string())
        .collect();
    let s = format!("[ {} ]", names.join(", "));
    println!("{}", s);
}
