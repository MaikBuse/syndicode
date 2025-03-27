use tonic::{Code, Status};
use uuid::Uuid;

pub(crate) fn parse_uuid(uuid_str: &str) -> Result<Uuid, Status> {
    match Uuid::parse_str(uuid_str) {
        Ok(uuid) => Ok(uuid),
        Err(err) => {
            return Err(Status::new(
                Code::InvalidArgument,
                format!("Failed to parse uuid from string: {}", err),
            ));
        }
    }
}
