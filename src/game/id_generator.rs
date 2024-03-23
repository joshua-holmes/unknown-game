pub type Id = u32;

#[derive(Debug)]
pub enum IdGenerationError {
    RanOutOfIds,
    DeletedNonExistentId,
}

pub struct IdGenerator {
    largest_id: Option<Id>,
    deleted_ids: Vec<Id>,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self {
            largest_id: None,
            deleted_ids: Vec::with_capacity(1000),
        }
    }

    pub fn new_id(&mut self) -> Result<Id, IdGenerationError> {
        if let Some(id) = self.deleted_ids.pop() {
            Ok(id)
        } else {
            if let Some(id) = self.largest_id.as_mut() {
                if *id == Id::MAX {
                    return Err(IdGenerationError::RanOutOfIds);
                }
                *id += 1;
                Ok(*id)
            } else {
                self.largest_id = Some(0);
                Ok(0)
            }
        }
    }

    pub fn mark_id_deleted(&mut self, id: Id) -> Result<(), IdGenerationError> {
        if let Some(largest_id) = self.largest_id {
            if id <= largest_id {
                self.deleted_ids.push(id);
                Ok(())
            } else {
                Err(IdGenerationError::DeletedNonExistentId)
            }
        } else {
            Err(IdGenerationError::DeletedNonExistentId)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Id, IdGenerator, IdGenerationError};

    #[test]
    fn test_first_id_is_0() {
        let mut id_gen = IdGenerator::new();
        assert_eq!(id_gen.new_id().unwrap(), 0);
    }

    #[test]
    fn test_new_id_is_larger_than_last_id() {
        let mut id_gen = IdGenerator {
            largest_id: Some(0),
            deleted_ids: Vec::new(),
        };
        assert_eq!(id_gen.new_id().unwrap(), 1);
    }

    #[test]
    fn test_maxing_out_ids_panics() {
        let mut id_gen = IdGenerator {
            largest_id: Some(Id::MAX),
            deleted_ids: Vec::new(),
        };
        if let Err(err) = id_gen.new_id() {
            match err {
                IdGenerationError::RanOutOfIds => assert!(true),
                _ => assert!(false, "Ran into an unexpected error -> {:?}", err),
            }
        } else {
            assert!(false, "IdGenerator did not return error when it ran out of ids");
        }
    }

    #[test]
    fn test_id_gets_deleted() {
        let mut id_gen = IdGenerator {
            largest_id: Some(80),
            deleted_ids: Vec::new(),
        };
        id_gen.mark_id_deleted(30).unwrap();
        if let Some(id) = id_gen.deleted_ids.pop() {
            assert_eq!(id, 30);
        } else {
            assert!(false, "Id did not get deleted or was not added to the deletion vector");
        }
    }

    #[test]
    fn test_deleted_ids_get_used() {
        let mut id_gen = IdGenerator {
            largest_id: Some(80),
            deleted_ids: vec![30],
        };
        assert_eq!(id_gen.new_id().unwrap(), 30);
    }

    #[test]
    fn test_cant_delete_id_before_any_are_made() {
        let mut id_gen = IdGenerator {
            largest_id: None,
            deleted_ids: Vec::new(),
        };
        if let Err(err) = id_gen.mark_id_deleted(0) {
            match err {
                IdGenerationError::DeletedNonExistentId => assert!(true),
                _ => assert!(false, "Reached unexpected error -> {:?}", err),
            }
        } else {
            assert!(false, "Non existent id was deleted, when it should have returned an error");
        }
    }

    #[test]
    fn test_cant_delete_non_existent_id_after_some_are_made() {
        let mut id_gen = IdGenerator {
            largest_id: Some(4),
            deleted_ids: Vec::new(),
        };
        if let Err(err) = id_gen.mark_id_deleted(5) {
            match err {
                IdGenerationError::DeletedNonExistentId => assert!(true),
                _ => assert!(false, "Reached unexpected error -> {:?}", err),
            }
        } else {
            assert!(false, "Non existent id was deleted, when it should have returned an error");
        }
    }
}
