use crate::{
    protocol::rusty::belt::{self, segment_value},
    state::rehydrator::CacheKey,
};

use super::{CacheSnapshot, Context, Model};

#[derive(Debug, PartialEq, Eq)]
pub struct Session {}

impl Session {
    pub fn new() -> Self {
        Session {}
    }
}

impl Model for Session {
    fn get_cache_key(&self) -> Option<CacheKey> {
        None
    }

    fn get_state(&self, context: &Context) -> segment_value::Segment {
        let mut result = belt::TmuxSessionName::default();
        result.session_name = context.tmux_session_name.clone().unwrap_or("".to_string());
        segment_value::Segment::TmuxSessionName(result)
    }

    fn rehydrate(&mut self, _cache_snapshot: &CacheSnapshot) -> Result<(), ()> {
        Ok(())
    }
}

#[cfg(test)]
mod session_tests {
    use crate::model::tests::{create_cache_snapshot_with_resources, generate_sample_context};

    use super::*;

    #[test]
    fn test_session_get_cache_key() {
        let session_model = Session::new();
        assert_eq!(session_model.get_cache_key(), None);
    }

    #[test]
    fn test_session_get_state() {
        // Create a sample context for testing
        let sample_context = generate_sample_context();

        // Create a Session model
        let session_model = Session::new();

        // Test get_state method
        let expected_segment = segment_value::Segment::TmuxSessionName(belt::TmuxSessionName {
            session_name: String::from("my_tmux_session"),
            ..Default::default()
        });
        assert_eq!(session_model.get_state(&sample_context), expected_segment);
    }

    #[test]
    fn test_session_rehydrate() {
        // Create a cache snapshot with resources
        let cache_snapshot = create_cache_snapshot_with_resources();

        // Create a Session model
        let mut session_model = Session::new();

        // Test rehydrate method
        let result = session_model.rehydrate(&cache_snapshot);
        assert_eq!(result, Ok(()));

        // Rehydrate should not modify the Session model, so there's nothing to assert here
    }
}
