use sqlx::{prelude::FromRow, PgPool};

#[derive(FromRow, Debug)]
pub struct Country {
    pub id: i32,
    pub country_full_name: String,
    pub country_code: String,
    pub country_name_en: String,
    pub population: i64,
    pub area: f64,
}

pub async fn query_database(sql: &str, pool: &PgPool) -> Result<Vec<Country>, sqlx::Error> {
    let countries: Vec<Country> = sqlx::query_as(sql).fetch_all(pool).await?;

    Ok(countries)
}
