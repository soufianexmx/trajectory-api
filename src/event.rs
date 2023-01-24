use crate::observable::Observable;
use crate::time;
use anymap::any::UncheckedAnyExt;
use anymap::Map;

struct Event<T, Obs: Observable + UncheckedAnyExt> {
    time: time::Time<T>,
    observables: Map<Obs>,
}
