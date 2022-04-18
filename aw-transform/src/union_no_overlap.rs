use super::sort::sort_by_timestamp;
use aw_models::Event;
use aw_models::TimeInterval;

/// unions two events list without overlap
///
/// # Example
/// ```ignore
/// events1:       [a           ][b     ]
/// events2:       [c    ]   [d     ]
/// output:        [c    ][a][d     ][b ]
/// ```
pub fn union_no_overlap(events1: &[Event], events2: &[Event]) -> Vec<Event> {
    let mut events_union: Vec<Event> = Vec::new();
    let mut sorted_events1: Vec<Event> = Vec::new();
    let mut sorted_events2: Vec<Event> = Vec::new();
    sorted_events1.extend(sort_by_timestamp(events1.to_vec()));
    sorted_events2.extend(sort_by_timestamp(events2.to_vec()));

    let mut idx1 = 0;
    let mut idx2 = 0;
    
    while idx1 < sorted_events1.len() && idx2 < sorted_events2.len() {
        let e1 = sorted_events1[idx1].clone();
        let e2 = sorted_events2[idx2].clone();
        let i1 = e1.interval();
        let i2 = e2.interval();

        match i1.gap(&i2) {
          Some(_) => {
            if *i1.start() <= *i2.start() {
                events_union.push(e1);
                idx1 = idx1 + 1;
            } else {
                events_union.push(e2);
                idx2 = idx2 + 1;
            }
          },
          None => {
            if *i1.start() <= *i2.start() {
                events_union.push(e1);
                idx1 = idx1 + 1;

                // If e2 continues after e1, we need to split up the event so we only get the part that comes after
                if *i2.end() > *i1.end() {
                    let i2new = TimeInterval::new(*i1.end(), *i2.end());
                    let mut e2new = e2.clone();
                    e2new.timestamp = *i2new.start();
                    e2new.duration = i2new.duration();
                    sorted_events2[idx2] = e2new;
                } else {
                    idx2 = idx2 + 1;
                }
            } else {
                if *i1.start() >= *i2.end() {
                    events_union.push(e2);
                    idx2 = idx2 + 1;
                } else {
                    // first split
                    let i2new1 = TimeInterval::new(*i2.start(), *i1.start());
                    let mut e2new1 = e2.clone();
                    e2new1.timestamp = *i2new1.start();
                    e2new1.duration = i2new1.duration();
                    events_union.push(e2new1);
                    
                    
                    // second split
                    let i2new2 = TimeInterval::new(*i1.start(), *i2.end());
                    let mut e2new2 = e2.clone();
                    e2new2.timestamp = *i2new2.start();
                    e2new2.duration = i2new2.duration();
                    sorted_events2[idx2] = e2new2;
                }
            }
          },
        }
    }
    
    while idx1 < sorted_events1.len() {
      events_union.push(sorted_events1[idx1].clone());
      idx1 = idx1 + 1;
    }
    
    while idx2 < sorted_events2.len() {
      events_union.push(sorted_events2[idx2].clone());
      idx2 = idx2 + 1;
    }
    
    
    //events_union.extend(sorted_events1[idx1..].iter());
    //events_union.extend(sorted_events2[idx2..].iter());
    
    events_union
}
