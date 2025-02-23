use dotenv::dotenv;
use solagent_ai::{db::query_database, nlp::parse_user_input};
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let user_input = r#"
    根据以下数据库表结构，生成SQL查询语句。

表名: splash
字段:
- id: INT
- country_full_name: VARCHAR(100)
- country_code: VARCHAR(10)
- country_name_en: VARCHAR(100)
- population: BIGINT
- area: FLOAT

请求: 获取人口最多的5个国家的信息。
    "#;
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await.unwrap();

    // Step 1: Parse user input
    let sql_query = parse_user_input(user_input).await.unwrap();
    // let sql_query = "SELECT * FROM splash;".to_string();
    println!("Generated SQL: {}", sql_query);

    // Step 2: Query database
    let results = query_database(&sql_query, &pool).await.unwrap();
    println!("{:#?}", results);

    // Step 3: Visualize data
    // let plot_html = visualize_data(results);
    // println!("Plot HTML: {}", plot_html);
}
