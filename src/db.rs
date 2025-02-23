use sqlx::{Column, Row};
use std::collections::HashMap;

pub async fn query_database(
    sql: &str,
    pool: &sqlx::PgPool,
) -> Result<Vec<HashMap<String, serde_json::Value>>, sqlx::Error> {
    let rows = sqlx::query(sql).fetch_all(pool).await?;

    let mut results = Vec::new();
    for row in rows {
        let mut map = HashMap::new();
        for (i, column) in row.columns().iter().enumerate() {
            let value: serde_json::Value = row.try_get(i)?;
            map.insert(column.name().to_string(), value);
        }
        results.push(map);
    }

    Ok(results)
}
