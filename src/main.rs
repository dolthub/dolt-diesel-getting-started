pub mod models;
pub mod schema;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::sql_query;
use dotenvy::dotenv;
use std::env;

use self::models::*;

static DELIMITER: &str = "-----------";

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn add_movie(conn: &mut MysqlConnection, new_title: &str, new_genre: &str, new_year: i32, new_rating: Option<i32>) {
    println!("Inserting '{new_title}'...");
    use self::schema::movies::dsl::*;
    let new_movie = NewMovie {
        title:  new_title,
        genre:  new_genre,
        year:   new_year,
        rating: new_rating,
    };

    let _ = diesel::insert_into(movies)
        .values(&new_movie)
        .execute(conn)
        .expect("Error inserting new movie");
}

fn remove_movie(conn: &mut MysqlConnection, delete_title: &str) {
    println!("Deleting '{delete_title}'...");
    use self::schema::movies::dsl::*;
    let _ = diesel::delete(movies.filter(title.eq(&delete_title)))
        .execute(conn)
        .expect("Error deleting movie");
}

fn update_rating(conn: &mut MysqlConnection, update_title: &str, new_rating: Option<i32>) {
    use self::schema::movies::dsl::*;
    diesel::update(movies.filter(title.eq(&update_title)))
        .set(rating.eq(new_rating))
        .execute(conn)
        .expect("Error updating movie");
}

fn dolt_add(conn: &mut MysqlConnection, tbl: &str) {
    println!("Staging changes to Dolt...");
    let query = format!("CALL dolt_add('{tbl}')");
    let _ = diesel::sql_query(query)
        .execute(conn)
        .expect("Error calling dolt_add");
}

fn dolt_commit(conn: &mut MysqlConnection, msg: &str) {
    println!("Committing changes to Dolt...");
    let query = format!("CALL dolt_commit('-m', '{msg}')");
    let _ = diesel::sql_query(query)
        .execute(conn)
        .expect("Error calling dolt_commit");
}

fn print_movie(title: Option<String>, genre: Option<String>, year: Option<i32>, rating: Option<i32>) {
    println!("Title:  {}", title.unwrap_or("NULL".to_string()));
    println!("Genre:  {}", genre.unwrap_or("NULL".to_string()));
    match year {
        Some(year) => println!("Year:   {}", year),
        None =>       println!("Year:   NULL"),
    }
    match rating { 
        Some(rating) => println!("Rating: {}", rating),
        None =>         println!("Rating: NULL"),
    }
}

fn print_movies(conn: &mut MysqlConnection) {
    use self::schema::movies::dsl::*;
    println!("Retrieving movies...");

    let results = movies
        .select(Movie::as_select())
        .load(conn)
        .expect("Error loading movies");

    for movie in results {
        println!("{}", DELIMITER);
        print_movie(
            Some(movie.title),
            Some(movie.genre),
            Some(movie.year),
            movie.rating,
        );
    }
    println!("{}\n", DELIMITER);
}

fn print_dolt_log(conn: &mut MysqlConnection) {
    println!("Retrieving Dolt log...");
    let query = "
        SELECT 
            commit_hash,
            committer,
            CAST(date as CHAR) as date,
            email,
            message
        FROM 
            dolt_log
        ";

    let results: Vec<DoltLog> = sql_query(query)
        .load(conn)
        .expect("Error loading log");

    for log in results {
        println!("{}", DELIMITER);
        println!("commit_hash: {}", log.commit_hash);
        println!("author:      {} <{}>", log.committer, log.email);
        println!("date:        {}", log.date);
        println!("message:     {}", log.message);
    }
    println!("{}\n", DELIMITER);
}

fn print_diff(diff: DoltDiffMovies) {
    match diff.diff_type.as_str() {
        "added" => {
            println!("Added movie: ");
            print_movie(
                diff.to_title,
                diff.to_genre,
                diff.to_year,
                diff.to_rating,
            );
        }
        "removed" => {
            println!("Removed movie: ");
            print_movie(
                diff.from_title,
                diff.from_genre,
                diff.from_year,
                diff.from_rating,
            );
        }
        "modified" => {
            println!("Updated movie rating:");
            println!("Title:  {}", diff.to_title.unwrap_or("NULL".to_string()));
            println!("Genre:  {}", diff.to_genre.unwrap_or("NULL".to_string()));
            match diff.to_year {
                Some(year) => println!("Year:   {}", year),
                None =>       println!("Year:   NULL"),
            }
            let from_rating : String = match diff.from_rating {
                Some(rating) => format!("{rating}"),
                None =>         "NULL".to_string(),
            };
            let to_rating : String = match diff.to_rating {
                Some(rating) => format!("{rating}"),
                None =>         "NULL".to_string(),
            };
            println!("Rating: {} -> {}", from_rating, to_rating);
        }
        _ => println!("Unknown diff type: {}", diff.diff_type),
    }
}

fn print_dolt_diff(conn: &mut MysqlConnection) {
    println!("Retrieving Dolt diff...");
    let query = "
        SELECT 
            to_title, 
            to_genre, 
            to_year, 
            to_rating,
            to_commit, 
            from_title, 
            from_genre, 
            from_year, 
            from_rating, 
            from_commit,
            diff_type 
        FROM 
            dolt_diff_movies
        WHERE
            to_commit = 'WORKING'
        ";

    let results: Vec<DoltDiffMovies> = sql_query(query)
        .load(conn)
        .expect("Error loading diff");

    for diff in results {
        println!("{}", DELIMITER);
        print_diff(diff);
    }
    println!("{}\n", DELIMITER);
}

fn print_dolt_branch_diff(conn: &mut MysqlConnection, from_branch: &str, to_branch: &str) {
    println!("Comparing diff from {from_branch} to {to_branch}...");
    let query = format!("
        SELECT 
            to_title,
            to_genre,
            to_year,
            to_rating,
            to_commit, 
            from_title,
            from_genre,
            from_year,
            from_rating,
            from_commit,
            diff_type
        FROM 
            dolt_diff('{from_branch}', '{to_branch}', 'movies')
        ");

    let results: Vec<DoltDiffMovies> = sql_query(query)
        .load(conn)
        .expect("Error loading diff");

    for diff in results {
        println!("{}", DELIMITER);
        print_diff(diff);
    }
    println!("{}\n", DELIMITER);
}

fn print_dolt_branches(conn: &mut MysqlConnection) {
    println!("Retrieving Dolt branches...");
    let query = "select name from dolt_branches";

    let results: Vec<DoltBranches> = sql_query(query)
        .load(conn)
        .expect("Error loading branches");

    for branch in results {
        println!("{}", branch.name);
    }
    println!();
}

fn create_branch(conn: &mut MysqlConnection, branch_name: &str) {
    println!("Creating branch '{branch_name}'...");
    let query = format!("CALL dolt_branch('{branch_name}')");
    let _ = diesel::sql_query(query)
        .execute(conn)
        .expect("Error creating branch");
}

fn checkout_branch(conn: &mut MysqlConnection, branch_name: &str) {
    println!("Switching to branch '{branch_name}'...");
    let query = format!("CALL dolt_checkout('{branch_name}')");
    let _ = diesel::sql_query(query)
        .execute(conn)
        .expect("Error switching branch");
}

fn merge_branch(conn: &mut MysqlConnection, branch_name: &str) {
    println!("Merging branch '{branch_name}'...");
    let query = format!("CALL dolt_merge('{branch_name}')");
    let _ = diesel::sql_query(query)
        .execute(conn)
        .expect("Error merging branch");
}

fn main() {
    let conn = &mut establish_connection();

    // Initialize repo
    dolt_add(conn, ".");
    dolt_commit(conn, "Diesel migrate and initialize movies table");
    print_movies(conn);
    print_dolt_diff(conn);
    print_dolt_log(conn);

    // Insert some movies
    add_movie(conn, "The Shawshank Redemption", "Prison Drama", 1994, Some(93));
    add_movie(conn, "The Godfather", "Mafia", 1972, Some(92));
    add_movie(conn, "The Dark Knight", "Action", 2008, None);
    print_movies(conn);
    print_dolt_diff(conn);
    print_dolt_log(conn);

    // Add, Commit, and Log
    dolt_add(conn, "movies");
    dolt_commit(conn, "Added 3 movies");
    print_dolt_log(conn);

    // Show branches
    print_dolt_branches(conn);

    // Make changes to other branch
    create_branch(conn, "other");
    checkout_branch(conn, "other");
    remove_movie(conn, "The Godfather");
    add_movie(conn, "The Godfather Part II", "Mafia", 1974, Some(90));
    print_movies(conn);
    print_dolt_diff(conn);

    // Commit and display log on other branch
    dolt_add(conn, "movies");
    dolt_commit(conn, "Replaced Godfather with Godfather Part II");
    print_dolt_log(conn);

    // Make changes to main branch
    checkout_branch(conn, "main");
    update_rating(conn, "The Dark Knight", Some(90));
    print_movies(conn);
    print_dolt_diff(conn);

    // Commit and display log on main branch
    dolt_add(conn, "movies");
    dolt_commit(conn, "Updated The Dark Knight rating");
    print_dolt_log(conn);

    // View diff from main to other
    print_dolt_branch_diff(conn, "main", "other");

    // Merge changes
    merge_branch(conn, "other");
    print_movies(conn);
    print_dolt_log(conn);
}