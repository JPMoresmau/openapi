use std::io;

use pretty_assertions::assert_eq;

use openapi::{read_from_file, Version};

#[test]
fn parse_petstore_3_1_1() -> io::Result<()> {
    let spec = read_from_file("./data/petstore-3.1.0.yaml")?;
    assert_eq!(Version::OpenApi3_1, spec.openapi);
    assert_eq!(1, spec.servers.len());
    let server = &spec.servers[0];
    assert_eq!("/v3", &server.url);
    Ok(())
}
