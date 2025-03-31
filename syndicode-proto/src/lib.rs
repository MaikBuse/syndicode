use tonic_reflection::server::{
    v1::{ServerReflection, ServerReflectionServer},
    Error,
};

pub mod syndicode_economy_v1;
pub mod syndicode_interface_v1;
pub mod syndicode_warfare_v1;

mod reflection {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("reflection_descriptor");
}

pub fn create_reflection_service() -> Result<ServerReflectionServer<impl ServerReflection>, Error> {
    tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(reflection::FILE_DESCRIPTOR_SET)
        .build_v1()
}
