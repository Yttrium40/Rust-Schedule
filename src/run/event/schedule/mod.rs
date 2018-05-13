use std::collections::HashMap;
use super::Event;
use super::Time;

pub struct Schedule {
    pub table: HashMap<u8, Event>,
    next_id: u8,
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule { table: HashMap::new(), next_id: 0 }
    }

    pub fn add(&mut self, event: Event) -> Result<(), String> {
        if self.next_id > 99 {
            return Err(String::from("Maximum number of events reached."));
        }
        if let Some(id) = self.check_time_overlap(&event) {
            return Err(format!("Time overlap with event ID {}.", id));
        }
        if self.table.insert(self.next_id, event).is_none() {
            self.increment_id();
            return Ok(());
        } else {
            panic!("ID already exists!");
        }
    }

    pub fn remove(&mut self, id: u8) -> Result<(), String> {
        if let Some(event) = self.table.remove(&id) {
            if self.next_id > id {
                self.next_id = id;
            }
            Ok(())
        } else {
            Err(String::from("ID not found."))
        }
    }

    fn increment_id(&mut self) {
        while self.table.contains_key(&self.next_id) {
            self.next_id += 1;
        }
    }

    fn check_time_overlap(&self, event: &Event) -> Option<u8> {
        let conditions = |x: &(&u8, &Event)| {
            let current_start = x.1.time.0;
            let current_end = x.1.time.1;
            let new_start = event.time.0;
            let new_end = event.time.1;
            (current_start >= new_start &&
            current_start <= new_end) ||
            (current_end >= new_start &&
            current_end <= new_end) ||
            (new_start >= current_start &&
            new_start <= current_end) ||
            (new_end >= current_start &&
            new_end <= current_end)
        };

        if let Some((key, _x_event)) = self.table.iter().find(conditions) {
            return Some(*key);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn add_and_overlap() {
        let mut table = Schedule::new();
        table.add(Event::new("N", Time(100, 500), "L", "D")).unwrap();
        match table.add(Event::new("N2", Time(200, 300), "L2", "D2")) {
            Err(_) => panic!("Problem"),
            Ok(()) => (),
        }
    }
    #[test]
    fn remove() {
        let mut table = Schedule::new();
        table.add(Event::new("N", Time(100, 200), "L", "D")).unwrap();
        table.add(Event::new("N2", Time(300, 400), "L2", "D2")).unwrap();
        match table.remove(0) {
            Err(_) => panic!("Problem"),
            Ok(()) => (),
        }
        assert_eq!(table.next_id, 0);
        table.remove(1).unwrap();
        assert!(table.table.is_empty());
    }
}
