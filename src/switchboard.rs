use std::collections::HashMap;
use std::sync::Arc;
use std::path::Path;

use bidirectional_multimap::BidirectionalMultimap;
use messages::RoomId;
use recorder::Recorder;
use session::Session;

#[derive(Debug)]
pub struct Switchboard {
    sessions: Vec<Box<Arc<Session>>>,
    publishers: HashMap<RoomId, Arc<Session>>,
    publishers_subscribers: BidirectionalMultimap<Arc<Session>, Arc<Session>>,
    recorders: HashMap<Arc<Session>, Recorder>,
}

impl Switchboard {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            publishers: HashMap::new(),
            publishers_subscribers: BidirectionalMultimap::new(),
            recorders: HashMap::new(),
        }
    }

    pub fn connect(&mut self, session: Box<Arc<Session>>) {
        self.sessions.push(session);
    }

    pub fn disconnect(&mut self, sess: &Session) {
        self.sessions.retain(|s| s.handle != sess.handle);
        self.publishers_subscribers.remove_key(sess);
        self.publishers_subscribers.remove_value(sess);
        self.recorders.remove(sess);
    }

    pub fn subscribers_to(&self, publisher: &Session) -> &[Arc<Session>] {
        self.publishers_subscribers.get_values(publisher)
    }

    pub fn publisher_to(&self, subscriber: &Session) -> Option<&Arc<Session>> {
        self.publishers_subscribers.get_key(subscriber)
    }

    pub fn create_room(&mut self, room_id: RoomId, publisher: Arc<Session>) {
        {
            let save_dir = Path::new(&room_id);
            self.recorders.insert(publisher.clone(), Recorder::new(&save_dir));
        }

        self.publishers.insert(room_id, publisher);
    }

    pub fn join_room(&mut self, room_id: RoomId, subscriber: Arc<Session>) {
        if let Some(publisher) = self.publishers.get(&room_id) {
            self.publishers_subscribers
                .associate(publisher.clone(), subscriber);
        }
    }

    pub fn recorder_for(&self, publisher: Arc<Session>) -> Option<&Recorder> {
        self.recorders.get(&publisher)
    }
}
