use rust_bert::pipelines::sequence_classification::Label;
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


fn create_db()-> sqlite::Connection{
    let db = sqlite::open(":memory").unwrap();
    db.execute("CREATE TABLE zeroshotclassification (id INTEGER PRIMARY KEY, label TEXT)").unwrap();
    db.execute("INSERT INTO zeroshotclassification (label) VALUES ('rock')").unwrap();
    db.execute("INSERT INTO zeroshotclassification (label) VALUES ('hip hop')").unwrap();
    db.execute("INSERT INTO zeroshotclassification (label) VALUES ('pop')").unwrap();
    db.execute("INSERT INTO zeroshotclassification (label) VALUES ('country')").unwrap();
    db.execute("INSERT INTO zeroshotclassification (label) VALUES ('latin')").unwrap();

    db
}


pub fn get_all_zeroshotCandidate()-> Vec<String>{
    let db = create_db();
    let query ="SELECT label FROM zeroshotclassification";
    let mut candidates: Vec<String> = Vec::new();
    db.iterate(query,|pairs|{
        for &(_column,value) in pairs.iter(){
            let value = value.unwrap();
            candidates.push(value.to_string());
        }
        true
    }).unwrap();
    candidates
}

pub fn read_lyrics(file:&str)->Vec<String>{
    let mut lyrics:Vec<String>=Vec::new();
    let file=File::open(file).expect("unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let line = line.unwrap();
        lyrics.push(line);
    }
    lyrics
}

pub fn classify_lyrics(lyrics: Vec<String>)-> Vec<Vec<Label>>{
    let candidates = get_all_zeroshotCandidate();
    let model=ZeroShotClassificationModel::new(Default::default()).unwrap();
    let classfication=model.predict(&lyrics, candidates, None, None);
    classification
}

