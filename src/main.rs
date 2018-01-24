use std::fmt::Display;

#[derive(Clone, Debug)]
struct MuString {
    chars: Vec<String>, // We use these as individual characters
    history: Vec<String>,
}

impl Display for MuString {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        fmt.write_str(&format!(
            "[{}, history: {:?}]",
            mu_to_string(self.clone()),
            self.history
        ))
    }
}

fn mu_to_string(input: MuString) -> String {
    input.chars.iter().fold("M".to_string(), |mut acc, c| {
        acc.push_str(c);
        acc
    })
}

fn double(input: MuString) -> Vec<MuString> {
    let mut new_string = input.clone();
    new_string.chars.append(&mut input.chars.clone());
    new_string
        .history
        .push(format!("Double: {}", mu_to_string(input)));
    vec![new_string]
}

fn iii_to_u(input: MuString) -> Vec<MuString> {
    let mut vec = vec![];

    let length = input.chars.len();

    for (i, c) in input.chars.iter().enumerate() {
        if length >= 3 && i < length - 2 && c == "I" && input.chars[i + 1] == "I"
            && input.chars[i + 2] == "I"
        {
            let mut new_string = input.clone();
            new_string.chars.remove(i);
            new_string.chars.remove(i);
            new_string.chars.remove(i);
            new_string.chars.insert(i, "U".to_string());
            new_string
                .history
                .push(format!("iii_to_u: {}", mu_to_string(input.clone())));

            vec.push(new_string);
        }
    }

    vec
}

fn uu_to_blank(input: MuString) -> Vec<MuString> {
    let mut vec = vec![];

    let length = input.chars.len();

    for (i, c) in input.chars.iter().enumerate() {
        if length >= 2 && i < length - 1 && c == "U" && input.chars[i + 1] == "U" {
            let mut new_string = input.clone();
            new_string.chars.remove(i);
            new_string.chars.remove(i);
            new_string
                .history
                .push(format!("uu_to_blank: {}", mu_to_string(input.clone())));

            vec.push(new_string);
        }
    }

    vec
}

fn append_u(input: MuString) -> Vec<MuString> {
    let mut new_string = input.clone();
    if *input.chars.last().unwrap() == "I".to_string() {
        new_string.chars.push("U".to_string());
        new_string
            .history
            .push(format!("append_u: {}", mu_to_string(input.clone())));
        vec![new_string]
    } else {
        vec![]
    }
}

fn make_possible_new_strings(input: MuString) -> Vec<MuString> {
    let mut vec: Vec<MuString> = vec![];

    // Doubling
    vec.append(&mut double(input.clone()));

    // III -> U
    vec.append(&mut iii_to_u(input.clone()));

    // UU -> _
    vec.append(&mut uu_to_blank(input.clone()));

    // MxI -> MxIU
    vec.append(&mut append_u(input.clone()));

    vec
}

fn main() {
    let mi = MuString {
        chars: vec!["I".to_string()],
        history: vec![],
    };

    let mut vec: Vec<MuString> = make_possible_new_strings(mi);

    while !vec.iter()
        .map(|x| mu_to_string(x.clone()))
        .collect::<Vec<String>>()
        .contains(&"MU".to_string())
    {
        for mu_string in vec.clone() {
            vec.append(&mut make_possible_new_strings(mu_string));
        }
    }

    for mu_string in vec.clone() {
        if mu_to_string(mu_string.clone()) == "MU" {
            println!("{}", mu_string);
        }
    }
}
