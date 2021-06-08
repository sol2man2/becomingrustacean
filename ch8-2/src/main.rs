fn main() {
    println!("ch8-2!");

    // Storing UTF-8 Encoded Text with Strings

    // What Is a String?
    // OsString, OsStr, CString, and CStr

    // Creating a New String
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Updating a String
    // Appending to a String with push_str and push
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // fn add(self, s: &str) -> String {}

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    // Indexing into Strings
    // let s1 = String::from("hello");
    // let h = s1[0];

    // Internal Representation
    let hello = String::from("Hola");
    println!("size of {}: {}", hello, hello.len());
    let hello = String::from("Здравствуйте");
    println!("size of {}: {}", hello, hello.len());

    // let hello = "Здравствуйте";
    // let answer = &hello[0];
    // 3 -> 208, д -> 151

    // Bytes and Scalar Values and Grapheme Clusters! Oh My!
    // “नमस्ते”

    // bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // chars, diacritics
    // ['न', 'म', 'स', '्', 'त', 'े']
    // grapheme
    // ["न", "म", "स्", "ते"]

    // Slicing Strings
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!(":::{}", s);

    // Methods for Iterating Over Strings
    // for c in "नमस्ते".chars() {
    //     println!("{}", c);
    // }

    // for b in "नमस्ते".bytes() {
    //     println!("{}", b);
    // }

    // Strings Are Not So Simple
}
