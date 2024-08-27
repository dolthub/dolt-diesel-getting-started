// @generated automatically by Diesel CLI.

diesel::table! {
    movies (title) {
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        genre -> Varchar,
        year -> Integer,
        rating -> Nullable<Integer>,
    }
}

diesel::table! {
    dolt_diff_movies (to_commit) {
        #[max_length = 255]
        to_title  -> Nullable<Varchar>,
        #[max_length = 255]
        to_genre  -> Nullable<Varchar>,
        to_year   -> Nullable<Integer>,
        to_rating -> Nullable<Integer>,
        to_commit -> Nullable<Varchar>,

        #[max_length = 255]
        from_title  -> Nullable<Varchar>,
        #[max_length = 255]
        from_genre  -> Nullable<Varchar>,
        from_year   -> Nullable<Integer>,
        from_rating -> Nullable<Integer>,
        from_commit -> Nullable<Varchar>,

        diff_type -> Varchar,
    }
}

diesel::table! {
    dolt_log (commit_hash) {
        commit_hash -> Varchar,
        committer   -> Varchar,
        email       -> Varchar,
        date        -> Varchar,
        message     -> Varchar,
    }
}

diesel::table! {
    dolt_branches (name) {
        name -> Varchar,
    }
}