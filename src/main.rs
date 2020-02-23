use std::fs;
use std::env;
use std::io::BufReader;
use std::collections::HashMap;
use std::time::SystemTime;
use std::path::PathBuf;

use review::Question;

fn main() {
    let filename = env::args().nth(1).expect("Need to provide a JSON file as first argument");
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let questions: Vec<Question<_>> = serde_json::from_reader(reader)
        .expect("Failed to parse JSON");
    let mut map = HashMap::new();

    // FIXME: This only works when all answers of type `Select` have the concrete
    // type `f32`. Need to abstract over that as well.
    questions.iter().for_each(|q: &Question<f32>| { map.insert(&q.name, q.ask()); } );

    let serialized: String = serde_json::to_string(&map)
        .expect("Failed to write JSON");

    fs::create_dir_all(data_dir()).expect("Could not create data dir");
    fs::write(result_filename(), serialized).expect("Unable to write file");
}

/// The file where the results are written to
fn result_filename() -> PathBuf {
    let now = SystemTime::now();

    let since_the_epoch = now.duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp: String = since_the_epoch.as_millis().to_string();
    data_dir().join(format!("data-{}.json", timestamp))
}

fn data_dir() -> PathBuf {
    dirs::home_dir().unwrap().join("Documents").join("review-data")
}

