use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct MovieResponse {
    pub results: Vec<Movie>,
}

#[derive(Debug, Deserialize)]
pub struct Movie {
    pub title: String,
}

pub async fn fetch_movies() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("TMDB_API_KEY")?;

    let url = format!(
        "https://api.themoviedb.org/3/movie/now_playing?api_key={}",
        api_key
    );

    let response: MovieResponse = reqwest::get(&url)
        .await?
        .json()
        .await?;

    for movie in response.results {
        println!("{}", movie.title);
    }

    Ok(())
}
