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

    assert_eq!("1.0.14", spec.info.version);

    assert_eq!(3, spec.tags.len());
    let t1 = spec.tags.get(0).unwrap();
    assert_eq!("pet", t1.name);
    let t2 = spec.tags.get(1).unwrap();
    assert_eq!("store", t2.name);
    let t3 = spec.tags.get(2).unwrap();
    assert_eq!("user", t3.name);

    assert_eq!(13, spec.paths.len());
    for path in [
        "/pet",
        "/pet/findByStatus",
        "/pet/findByTags",
        "/pet/{petId}",
        "/pet/{petId}/uploadImage",
        "/store/inventory",
        "/store/order",
        "/store/order/{orderId}",
        "/user",
        "/user/createWithList",
        "/user/login",
        "/user/logout",
        "/user/{username}",
    ] {
        assert!(spec.paths.contains_key(path));
    }

    assert_eq!(8, spec.components.schemas.len());
    for schema in [
        "Order",
        "Customer",
        "Address",
        "Category",
        "User",
        "Tag",
        "Pet",
        "ApiResponse",
    ] {
        assert!(spec.components.schemas.contains_key(schema));
    }

    assert_eq!(2, spec.components.request_bodies.len());
    for body in ["Pet", "UserArray"] {
        assert!(spec.components.request_bodies.contains_key(body));
    }

    assert_eq!(2, spec.components.security_schemes.len());
    for security in ["petstore_auth", "api_key"] {
        assert!(spec.components.security_schemes.contains_key(security));
    }

    Ok(())
}
