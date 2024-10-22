mod lexer;

use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;

use crate::lexer::Lexer;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct News {
    class_index: u32,
    title: String,
    description: String,
}

#[derive(Debug, Default)]
struct Class {
    word_count: HashMap<String, usize>,
    total_count: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./ag-news/train.csv")?;
    let mut reader = csv::Reader::from_reader(file);

    let news: Result<Vec<News>, csv::Error> = reader.deserialize().collect();
    let news = news?;
    let unique_classes: HashSet<u32> = news.iter().map(|n| n.class_index).collect();

    let mut classes_data: Vec<Class> = Vec::new();
    for _ in 0..unique_classes.len() {
        classes_data.push(Default::default());
    }

    for news in news {
        let class_data = &mut classes_data[news.class_index as usize - 1];
        let news_data = format!("{} {}", news.title, news.description);
        let news_chars: Vec<char> = news_data.chars().collect();
        let lexer = Lexer::new(&news_chars);
        for word in lexer {
            class_data.total_count += 1;
            class_data
                .word_count
                .entry(word)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }

    for (i, class) in classes_data.iter().enumerate() {
        println!("Class {i} data: {:?}", class.total_count)
    }

    println!("Classes: {:?}", unique_classes);

    Ok(())
}
