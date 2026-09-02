use std::vec;

use crate::db::Language;
use crate::graph::Graph;
use crate::models::Vocab;
use crate::{repository, service};

use linfa::traits::Transformer;
use linfa_tsne::TSneParams;
use ndarray::Array2;

pub fn get_user(user_id: i32) -> Result<Vec<String>, &'static str> {
    let words = repository::vocab::get_by_user(user_id)?;
    Ok(words.into_iter().map(|vocab| vocab.word).collect())
}

pub fn get_user_graph(user_id: i32) -> Result<Graph, &'static str> {
    let words = repository::vocab::get_by_user(user_id)?;

    let mut graph = Graph::new();
    for word in words {
        let distances = repository::vocab::get_distances_by_user_and_vec(
            user_id,
            repository::embedding::get_vec_by_word(&word.word)?,
            word.word.as_str(),
            3,
        )?;

        println!("{:?}", word.word);
        println!("{:?}", distances);

        let graph_connections = eval_connections_by_distances(&distances);
        let edges_to_add = distances[..graph_connections as usize]
            .iter()
            .map(|(word, _)| word.id as u32)
            .collect::<Vec<u32>>();

        graph.add_node(word.id.cast_unsigned(), word.word.as_str(), &edges_to_add);
    }

    Ok(graph)
}

fn eval_connections_by_distances(distances: &[(Vocab, f64)]) -> u32 {
    if distances.len() == 0 {
        return 0;
    }

    let mut connections = 0;

    let mut prev_distance = distances[0].1;
    for (vocab, distance) in distances {
        let rel_diff = (distance - prev_distance) / distance;
        if rel_diff > 0.2 {
            println!(
                "not adding connection to {:?} distance: {} relative diff: {}",
                vocab.word, distance, rel_diff
            );
            break;
        }
        println!(
            "adding connection to {:?} distance: {} relative diff: {}",
            vocab.word, distance, rel_diff
        );
        prev_distance = *distance;
        connections += 1;
    }

    connections
}

/*fn get_distances_by_user(user_id: i32) -> Result<Vec<(Vocab, f32)>, diesel::result::Error> {
    //use self::schema::vocab::dsl::*;
    use self::schema::embeddings;
    use self::schema::vocab;

    let connection = &mut db::establish_connection();

    let result = vocab::table
        .inner_join(user_vocab::table.on(vocab::id.eq(user_vocab::vocab)))
        .filter(user_vocab::user.eq(user_id))
        .select((
            vocab::id,
            vocab::word,
            embeddings::vector.l2_distance(Vector::from(sum)),
        ))
        .load(connection);
}*/

#[derive(serde::Serialize)]
pub struct ProjectedWord {
    pub word: String,
    pub x: f32,
    pub y: f32,
}

pub fn get_user_projected(user_id: i32) -> Result<Vec<ProjectedWord>, &'static str> {
    let words = repository::embedding::get_by_user(user_id)?;

    let word_count = words.len();

    if word_count == 0 {
        return Ok(vec![]);
    }

    let dim = 300;
    let values = words.iter().map(|x| x.vector.to_vec()).flatten().collect();

    let values: Array2<f32> = Array2::from_shape_vec((words.len(), dim), values).unwrap();

    let perplexity: f32 = if word_count > 1 {
        12.0 * (word_count as f32) / 250.0
    } else {
        0.0
    };

    let y_2d = TSneParams::embedding_size(2)
        .perplexity(perplexity)
        .approx_threshold(0.3)
        .transform(values)
        .unwrap();

    let mut result: Vec<ProjectedWord> = vec![];
    let mut y_2d_iter = y_2d.outer_iter().into_iter();
    for w in words {
        let y = y_2d_iter.next().unwrap();
        result.push(ProjectedWord {
            word: w.word,
            x: y[0],
            y: y[1],
        });
    }

    Ok(result)
}

pub fn add_user(word: &str, user_id: i32) -> Result<&'static str, &'static str> {
    let word = repository::vocab::get_by_word(word)?;
    repository::vocab::add_to_user(word.id, user_id)?;
    Ok("Word added successfully")
}

pub fn add_user_from_words(words: Vec<String>, user_id: i32) -> Result<String, &'static str> {
    let mut words_to_add = service::embedding::predict_from_words(words, 1, false, user_id)?;
    let word_to_add = words_to_add.pop().unwrap();

    let word = match repository::vocab::get_by_word(&word_to_add) {
        Ok(word) => word,
        Err(_) => repository::vocab::insert(&word_to_add, Language::En)?,
    };

    repository::vocab::add_to_user(word.id, user_id)?;

    Ok(word_to_add)
}

pub fn delete_user_words(words: Vec<String>, user_id: i32) -> Result<&'static str, &'static str> {
    let words = repository::vocab::get_by_words(&words)?;
    let word_ids = words.iter().map(|word| word.id).collect::<Vec<i32>>();

    repository::vocab::delete_from_user(&word_ids, user_id)?;

    Ok("Words deleted successfully")
}

pub fn search(search: &str) -> Result<Vec<String>, &'static str> {
    let words = repository::vocab::search(search, 20)?;
    let words = words.into_iter().map(|vocab| vocab.word).collect();

    Ok(words)
}
