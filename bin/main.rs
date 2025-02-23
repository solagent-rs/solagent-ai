use dotenv::dotenv;
use solagent_ai::db::query_database;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let user_input = "Show me the top 10 transactions by value";
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await.unwrap();

    // Step 1: Parse user input
    // let sql_query = parse_user_input(user_input).await.unwrap();
    let sql_query = "SELECT * FROM splash ORDER BY population DESC LIMIT 5;";
    println!("Generated SQL: {}", sql_query);

    // Step 2: Query database
    let results = query_database(sql_query, &pool).await.unwrap();
    println!("{:#?}", results);

    // Step 3: Visualize data
    // let plot_html = visualize_data(results);
    // println!("Plot HTML: {}", plot_html);
}
