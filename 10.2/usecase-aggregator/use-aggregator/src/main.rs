use aggregator::Summary;

pub struct Paper {
    pub content: String,
}

impl Summary for Paper {
    fn summarize(&self) -> String {
        format!("{}", self.content)
    }
}

fn main() {
    let paper = Paper {
        content: String::from("In this paper, we propose a novel method to .."),
    };
    println!("Summary: {}", paper.summarize());
}
