fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The rest is {}", result);

    let mut num = 3;

    while num != 0 {
        println!("{}!", num);

        num -= 1;
    }

    println!("LIFTOFF");

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    let mut ss = String::from("hellosss");

    let r1 = &ss; // no problem
    let r2 = &ss; // no problem
    println!("{} and {}", r1, r2);
    println!("{} and {}", r1, r2);

    let r3 = &mut ss; // no problem
    println!("{}", r3);

    let mut sss = String::from("Hello world");

    let word = first_word2(&sss);

    sss.clear();

    println!("the first word is: {}", word);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
