struct Solution;

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        title.split(' ')
            .map(|seg| {
                if seg.len() <= 2 {
                    seg.to_lowercase()
                } else {
                    let f = seg[0..1].to_uppercase();
                    let f = &seg[1..];
                    format!("{}{}", &seg[0..1].to_uppercase(), &seg[1..].to_lowercase())
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn t() {}
}