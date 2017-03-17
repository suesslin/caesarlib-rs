trait StringChars<T> {
    fn all_chars(self) -> Vec<T>;
}

impl StringChars<String> for String {
    fn all_chars(self) -> Vec<String> {
        let char_vec: Vec<String> =
            self
            .split(",")
            .map(|cur_char| { cur_char.to_string() })
            .collect();

        char_vec
    }
}

trait VecExtensions<T> {
    fn distance(self, from: usize, to: usize) -> Vec<T>;
    fn last_pos(self) -> usize;
}

impl VecExtensions<String> for Vec<String> {
    fn distance(self, start: usize, end: usize) -> Vec<String> {
        let mut temp_vec: Vec<String> = Vec::new();
        // for i in start..end {
        //     if let Some(cur_char) = self[i] {
        //         temp_vec.push(cur_char.unwrap())
        //     }
        // }
        return temp_vec
    }

    fn last_pos(self) -> usize {
        self.len() - 1
    }
}



fn shift_seq(offset: i32, base_seq: String) -> String {
    let mut new_seq = String::new();
    let seq_vec: Vec<String> = base_seq.all_chars();

    if offset >= seq_vec.len() as i32 {
        // new_seq =
        //     seq_vec
        //     .distance_vec(offset % seq_vec.len(), )
    }

    new_seq
}

fn main() {
}
