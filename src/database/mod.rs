pub mod models;

pub mod connect;
pub mod select;
pub mod insert;
pub mod update;
pub mod erase;

// For batch queries, how many records per batch
const BATCH_MAX_SIZE : usize = 50;

// Converts a set of parameters to an Vector of references that implement ToSql
// batch_params![a,b,c]
// => vec![&a as &dyn ToSql, &b as &dyn ToSql, &c as &dyn ToSql]
// For use in creating an element of the values input to 'batch_insert_query', or 'batch_update_query', etc. functions
#[macro_export]
macro_rules! batch_params {
    ( $($a:expr),+) => {{
        vec![$( &($a) as &dyn ToSql),+]
    }};
}
