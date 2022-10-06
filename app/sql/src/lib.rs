mod buildings;
mod lessors;
mod test_table;
mod users;
pub use buildings::*;
pub use lessors::*;
pub use sqlx::{types::Decimal, MySqlPool};
pub use test_table::*;
pub use users::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
