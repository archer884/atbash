use std::collections::HashMap;

fn main() {
    let mapping = build_mapping(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "abcdefghijklmnopqrstuvwxyz",
    );

    let mut parts = vec![];
    for arg in std::env::args().skip(1) {
        parts.push(encode(&arg, &mapping))
    }

    println!("{}", parts.join(" "));
}

fn encode(content: &str, mapping: &HashMap<char, char>) -> String {
    content.chars().map(|c| mapping.get(&c).map(|&c| c).unwrap_or(c)).collect()
}

fn build_mapping(upper: &str, lower: &str) -> HashMap<char, char> {
    let upper: Vec<_> = upper.chars().collect();
    let lower: Vec<_> = lower.chars().collect();

    let upper = upper.iter().zip(upper.iter().rev()).map(|(&a, &b)| (a, b));
    let lower = lower.iter().zip(lower.iter().rev()).map(|(&a, &b)| (a, b));

    upper.chain(lower).collect()
}

#[cfg(test)]
mod tests {
    use super::{build_mapping, encode};

    #[test]
    fn it_works() {
        let inputs = [
            "foobar",
            "wizard",
            "/r/dailyprogrammer",
            "gsrh rh zm vcznkov lu gsv zgyzhs xrksvi",
        ];

        let results = [
            "ullyzi",
            "draziw",
            "/i/wzrobkiltiznnvi",
            "this is an example of the atbash cipher",
        ];

        let mapping = build_mapping(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            "abcdefghijklmnopqrstuvwxyz",
        );

        for (&input, &result) in inputs.iter().zip(results.iter()) {
            assert_eq!(result, encode(&input, &mapping));
        }
    }
}
