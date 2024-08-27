use diesel::prelude::*;
use crate::schema::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = movies)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Movie {
    pub title:  String,
    pub genre:  String,
    pub year:   i32,
    pub rating: Option<i32>,
}


#[derive(Insertable)]
#[diesel(table_name = movies)]
pub struct NewMovie<'a> {
    pub title:  &'a str,
    pub genre:  &'a str,
    pub year:   i32,
    pub rating: Option<i32>,
}


#[derive(QueryableByName)]
#[diesel(table_name = dolt_diff_movies)]
pub struct DoltDiffMovies {
    pub to_title:  Option<String>,
    pub to_genre:  Option<String>,
    pub to_year:   Option<i32>,
    pub to_rating: Option<i32>,
    pub to_commit: Option<String>,

    pub from_title:  Option<String>,
    pub from_genre:  Option<String>,
    pub from_year:   Option<i32>,
    pub from_rating: Option<i32>,
    pub from_commit: Option<String>,

    pub diff_type: String,
}


#[derive(QueryableByName)]
#[diesel(table_name = dolt_log)]
pub struct DoltLog {
    pub commit_hash: String,
    pub committer:   String,
    pub email:       String,
    pub date:        String,
    pub message:     String,
}


#[derive(QueryableByName)]
#[diesel(table_name = dolt_branches)]
pub struct DoltBranches {
    pub name: String,
}