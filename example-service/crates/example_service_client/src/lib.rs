pub mod proto {
    tonic::include_proto!("example.service");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("example.service");
}