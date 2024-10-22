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

#[derive(Debug)]
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

    let mut news_by_class: Vec<Vec<News>> = Vec::new();
    for _ in 0..unique_classes.len() {
        news_by_class.push(vec![]);
    }

    for news in news {
        let news_group = &mut news_by_class[news.class_index as usize - 1];
        news_group.push(news)
    }

    for (i, class) in news_by_class.iter().enumerate() {
        let mut class_data = Class {
            word_count: Default::default(),
            total_count: 0,
        };

        for news in class {
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

    }

    println!("Classes: {:?}", unique_classes);

    Ok(())
}
