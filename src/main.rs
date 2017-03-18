trait StringChars<T> {
    fn all_chars(self) -> Vec<T>;
}

impl StringChars<String> for String {
    fn all_chars(self) -> Vec<String> {
        let char_vec: Vec<String> = self
                                    .split("")
                                    .map(|cur_char| { cur_char.to_string() })
                                    .collect();
        char_vec
    }
}

trait VecExtensions<T> {
    fn last_pos(&self) -> usize;
}

impl VecExtensions<String> for Vec<String> {
    fn last_pos(&self) -> usize {
        self.len() - 1
    }
}

fn vec_shift(input: &Vec<String>, start: usize, end: usize) -> String {
    input
    .iter()
    .enumerate()
    .filter(|&(i, _)| { i >= start && i <= end }) // end needed for one case otherwise not
    .map(|(i, x)| { x.to_string() })
    .collect::<Vec<String>>()
    .join("")
}

fn shift_seq(offset: i32, base_seq: String) -> String {
    let mut new_seq = String::new();
    let seq_vec: Vec<String> = base_seq.all_chars();

    if offset >= seq_vec.len() as i32 {
        new_seq = vec_shift(&seq_vec, (offset as usize) % seq_vec.len(), seq_vec.last_pos());
        if offset % seq_vec.len() as i32 != 0 {
            new_seq = format!("{}{}", new_seq, vec_shift(&seq_vec, 0, (offset as usize) % seq_vec.len() - 1));
        }
    } else {
        new_seq = vec_shift(&seq_vec, offset as usize, seq_vec.last_pos());
        if offset != 0 {
            new_seq = format!("{}{}", new_seq, vec_shift(&seq_vec, 0, (offset - 1) as usize));
        }
    }
    new_seq
}

fn encipher(offset: i32, message: &str) -> String {
    let base = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let base_seq = base.to_string().all_chars();
    let new_seq = shift_seq(offset, base.to_string()).all_chars();

    let mut new_string = String::new();

    for achar in message.to_string().all_chars() {
        let mut found = false;
        for i in 0..base_seq.last_pos() {
            if achar == base_seq[i] {
                new_string += &new_seq[i];
                found = true;
            }
        }
        if !found {
            new_string += "achar"
        }
    }

    new_string
}

fn main() {
    // PROBLEMS:
    // - Doesn't add Everything before starting point to the end (PROBLEM part of 3)
    //   With offset over 26 it works again
    // - Doesn't start shifting at 1, but at 2
    // - In Else part, fix the offset != 0 problem

    let shift = shift_seq(30, "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string());
    let shiftString = encipher(2, "Konnichiwa");
    println!("{}", shiftString);
}
