mod examples;

fn main() {
    // generic lifetime annotation - 'a
    // lifetime is how long a variable lives

    let s1 = String::from("first");
    let s2 = String::from("second");

    // because we don't know who (s1, s2) will be returned, we get an error
    // we must specify a lifetime as either x or y can be returned
    let result = longest(s1.as_str(), s2.as_str());

    imp_exc();
}

// &i32        // reference
// &'a i32     // reference with an explicit lifetime
// &'a mut i32 // mutable reference with an explicit lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// no need to annotate y as we can't return it
fn shortest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}


#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn imp_exc() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Couldn't find it");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("struct: {:#?}", i);
}