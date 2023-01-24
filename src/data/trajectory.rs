use crate::data::event::Event;
use crate::data::observable::Observable;
use anymap::any::UncheckedAnyExt;
use uuid::Uuid;

struct Trajectory<T, Obs: Observable + UncheckedAnyExt> {
    id: Uuid,
    events: Vec<Event<T, Obs>>,
}
