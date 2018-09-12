use std::collections::HashMap;

const NAME: &str = "Michael Li";
const EMAIL: &str = "alimy@gility.net";

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


pub fn run() {
    let mut v = vec![100, 32, 57];
    v.push(6);
    for i in &v {
        println!("{}", i);
    }
    println!("{}", v[0]);
    v.push(6);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
    if let SpreadsheetCell::Int(x) = row[0] {
        println!("{}", x)
    }

    if let SpreadsheetCell::Float(f) = row[2] {
        println!("{}", f);
    }

    println!("Name: {}, Email: {}, Name.0: {}", NAME, EMAIL, NAME.as_bytes()[0]);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    let c = 'l';
    s.push(c);
    println!("{}", c);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let hello = String::from("Здравствуйте");
    let s = &hello[0..4];
    println!("{}", s);

    for c in hello.chars() {
        println!("{}", c);
    }

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
