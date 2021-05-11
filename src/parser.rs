// this code in no way is any good. it works and that's what i wanted it to do, pr's welcome which will make the code sane.

pub fn parse(input: &str) {
    // split into new lines
    let lines = input.split("\n").into_iter();
    let mut line_count = 0;
    for line in lines {
        line_count += 1;
        let mut words = line.split_whitespace().into_iter();
        let mut word_count = 0;
        for word in words.clone() {
            word_count += 1;
            // mod
            if word == "mod" {
                let name = words.nth(word_count).unwrap();
                println!("{}", name)
            }
        }
    }
}