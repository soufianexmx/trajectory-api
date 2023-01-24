mod chronon;
pub mod order;
mod period;

pub enum Time<T> {
    Chronon(chronon::Chronon<T>),
    Period(period::Period<T>),
}
