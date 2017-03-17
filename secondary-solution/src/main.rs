use std::io;

fn main() {

    let mut m = String::new();
    let mut w = String::new();
    let mut k = String::new();

    let m2: i8;

    loop {
        println!("Choose one of these methods\n0 > Encipher\n1 > Decipher");
        io::stdin().read_line(&mut m).expect("Error");

        m2 = match m.trim().parse() {
            Ok(x) => x,
            Err(_) => continue
        };

        break;
    }

    println!("Type in a message to write");
    io::stdin().read_line(&mut w).expect("Error!");

    println!("Type in a key");
    io::stdin().read_line(&mut k).expect("Error!");

    let k: i32 = match k.trim().parse() {
        Ok(x) => x,
        Err(_) => 0
    };

    println!("Your translated string is\n{:?}", trs(w, k, m2));
}

fn trs(w: String, k: i32, m: i8) -> String {
    if m == 0 {
        return encsr(w.trim(), k);
    }
    return decsr(w.trim(), k);
}

fn key_v(starting: &str, key: i32) -> Vec<char> {
    let k: i32 = key;
    let s = starting;

    let s_v: Vec<char> = s.chars().collect();
    let mut new_v: Vec<char> = Vec::new();

    let mut i: i32 = k-1;

    while i < (s_v.len() as i32) {
        new_v.push(s_v[(i as usize)]);
        i += 1;
    }

    let mut new_i = 0;
    while new_i <= k-2 {
        new_v.push(s_v[(new_i as usize)]);
        new_i += 1;
    }

    return new_v;
}

fn l_to_n(l: char) -> i16 {
    match l {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        'J' => 9,
        'K' => 10,
        'L' => 11,
        'M' => 12,
        'N' => 13,
        'O' => 14,
        'P' => 15,
        'Q' => 16,
        'R' => 17,
        'S' => 18,
        'T' => 19,
        'U' => 20,
        'V' => 21,
        'W' => 22,
        'X' => 23,
        'Y' => 24,
        'Z' => 25,
        _ => 30
    }
}

fn encsr_w(word: &str, key: i32) -> String {
    let starting = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let key_v = key_v(starting, key);

    let mut index_v: Vec<i16> = Vec::new();

    for l in word.to_uppercase().chars() {
        index_v.push(l_to_n(l));
    }

    let mut new_w_v: Vec<char> = Vec::new();

    for i in index_v {
        new_w_v.push(key_v[(i as usize)]);
    }

    let s: String = new_w_v.into_iter().collect();

    return s;
}

fn decsr_w(word: &str, key: i32) -> String {

    let starting = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let key_v = key_v(starting, key);

    let mut pos_v: Vec<i16> = Vec::new();
    let mut cur_i: i16 = 0;

    for c in word.to_uppercase().chars() {
        while cur_i < (key_v.len() as i16) - 1 {
            if c == key_v[(cur_i as usize)] {
                pos_v.push(cur_i);
                cur_i = 0;
                break;
            }
            cur_i += 1
        }
    }

    let c_vec: Vec<char> = starting.chars().collect();
    let mut fin_v: Vec<char> = Vec::new();

    for p in pos_v {
        fin_v.push(c_vec[(p as usize)]);
    }

    let s: String = fin_v.into_iter().collect();
    return s;
}

fn encsr(word: &str, key: i32) -> String {
    let sp_w: Vec<&str> = word.split(" ").collect();
    let mut w_v: Vec<String> = Vec::new();

    for cur_w in sp_w {
        w_v.push(encsr_w(cur_w, key));
    }

    return w_v.join(" ");
}

fn decsr(enc: &str, key: i32) -> String {
    let mut fin_v: Vec<String> = Vec::new();

    for pt in enc.split(" ") {
        fin_v.push(decsr_w(pt.trim(), key));
    }

    return fin_v.join(" ");
}
