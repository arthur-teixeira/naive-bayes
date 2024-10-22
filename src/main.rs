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

impl News {
    fn words(&self) -> Vec<String> {
        let news_data = format!("{} {}", self.title, self.description);
        let news_chars: Vec<char> = news_data.chars().collect();
        Lexer::new(&news_chars).collect()
    }
}

#[derive(Debug, Default)]
struct Class {
    word_count: HashMap<String, usize>,
    total_word_count: usize,
    document_count: usize,
}

#[derive(Debug, Default)]
struct Model {
    pub class_names: Vec<String>,
    pub classes: Vec<Class>,
}

impl Model {
    fn train(&mut self, news: Vec<News>) {
        for news in news {
            let class_data = &mut self.classes[news.class_index as usize - 1];
            class_data.document_count += 1;
            for word in news.words() {
                class_data.total_word_count += 1;
                class_data
                    .word_count
                    .entry(word)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
    }

    fn classify(&self, document: &News) -> (usize, f64) {
        // P(class | document) = P(document | class) * P(class) / P(document)
        // P(document) is constant for all classes in a Naive model
        // So P(class | document) can be simplified as P(document | class) * P(class)
        // P(class | document) ∝ P(document | class) * P(class)
        // P(class) = number of documents in class / number of documents
        // P(document | class) = product of P(word | class) for each word in document
        // P(word | class) = word count in class / total count of words in class
        // Deriving:
        // P(class | document) = P(document | class) * P(class)
        // P(class | document) = ∏P(word | class) * P(class)
        // P(class | document) = ∏P(word count in class / total words in class) * P(class)
        // log(∏P(word / class)) = ∑log(P(word / class))
        // using log avoids underflows

        let mut total_documents = 0;
        for class in &self.classes {
            total_documents += class.document_count;
        }

        let mut max_probability = 0f64;
        let mut max_class = 0;

        for (i, class) in self.classes.iter().enumerate() {
            let p_class = (class.document_count as f64) / (total_documents as f64);
            let mut p_document_given_class = 0.0f64;
            for word in document.words() {
                let p_word = (*class.word_count.get(&word).unwrap_or(&0) as f64)
                    / (class.total_word_count as f64);
                p_document_given_class += p_word.log2();
            }

            let p_class_given_document = p_class * p_document_given_class.exp2();
            if p_class_given_document > max_probability {
                max_probability = p_class_given_document;
                max_class = i;
            }
        }

        (max_class, max_probability)
    }
}

fn validate_model(model: &Model) -> Result<(), Box<dyn Error>> {
    let test = File::open("./ag-news/test.csv")?;
    let mut reader = csv::Reader::from_reader(test);
    let news: Result<Vec<News>, csv::Error> = reader.deserialize().collect();
    let news = news?;

    // confusion_matrix[predicted][actual]
    let n = model.class_names.len();
    let mut confusion_matrix: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for news in &news {
        let (predicted, _) = model.classify(news);
        confusion_matrix[predicted][news.class_index as usize - 1] += 1;
    }

    println!("RESULTS:");
    let mut true_positives = 0;
    let mut false_positives = 0;
    let mut precisions = vec![0f64; n];
    for i in 0..n {
        let current_true_positives = confusion_matrix[i][i];
        true_positives += current_true_positives;

        let mut current_false_positives = 0;
        for j in 0..n {
            if i != j {
                current_false_positives += confusion_matrix[i][j];
            }
        }

        precisions[i] = current_true_positives as f64 / (current_true_positives + current_false_positives) as f64;
        true_positives += current_true_positives;
        false_positives += current_false_positives;
        println!("Precision for class {i}: {:?}", precisions[i]);
    };

    println!("Overall model precision: {:?}", true_positives as f64 / (true_positives + false_positives) as f64);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let train = File::open("./ag-news/train.csv")?;
    let mut reader = csv::Reader::from_reader(train);

    let classes = File::open("./ag-news/classes.csv")?;
    let mut classes_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(classes);
    let classes: Result<Vec<String>, csv::Error> = classes_reader.deserialize().collect();
    let classes = classes?;

    let news: Result<Vec<News>, csv::Error> = reader.deserialize().collect();
    let news = news?;
    let unique_classes: HashSet<u32> = news.iter().map(|n| n.class_index).collect();

    let mut model: Model = Model::default();
    model.class_names = classes;
    for _ in 0..unique_classes.len() {
        model.classes.push(Default::default());
    }
    model.train(news);
    validate_model(&model)?;

    Ok(())
}
