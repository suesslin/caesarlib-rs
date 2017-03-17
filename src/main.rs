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
    fn last_pos(self) -> usize;
}

impl VecExtensions<String> for Vec<String> {
    fn last_pos(self) -> usize {
        self.len() - 1
    }
}

fn vec_shift(input: &Vec<String>, start: usize) -> String {
    input
    .iter()
    .enumerate()
    .filter(|&(i, _)| { i >= start })
    .map(|(i, x)| { x.to_string() })
    .collect::<Vec<String>>()
    .join("")
}

fn shift_seq(offset: i32, base_seq: String) -> String {
    let mut new_seq = String::new();
    let seq_vec: Vec<String> = base_seq.all_chars();

    if offset >= seq_vec.len() as i32 {
        new_seq = vec_shift(&seq_vec, (offset as usize) % seq_vec.len());
        if offset % seq_vec.len() as i32 != 0 {
            new_seq = format!("{}{}", new_seq, vec_shift(&seq_vec, 0))
        }
    } else {
        new_seq = vec_shift(&seq_vec, offset as usize)
        // if offset != 0 {
        //     new_seq = format!("{}{}", new_seq, )
        // }
    }
    new_seq
}

fn main() {
    // PROBLEMS:
    // - Doesn't add Everything before starting point to the end
    // - Doesn't start shifting at 1, but at 2
    // - In Else part, fix the offset != 0 problem

    let shift = shift_seq(3, "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string());
    println!("{}", shift);
}
