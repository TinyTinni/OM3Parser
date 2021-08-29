use om3_parser_rs;
use std::fs;

#[test]
fn load_cube_point_cloud() {
    let filename = "tests/data/cube_point.om3";

    let content = fs::read(filename).unwrap();

    let output = om3_parser_rs::parse_om3(&content[..]);

    assert_eq!(output.is_ok(), true);
    let (rest, mesh) = output.unwrap();
    assert_eq!(rest, b"");

    assert_eq!(mesh.faces.is_none(), true);
    assert_eq!(mesh.vertices.x.len(), 8);
    assert_eq!(mesh.vertices.y.len(), 8);
    assert_eq!(mesh.vertices.z.len(), 8);
}

#[test]
fn load_cube() {
    let filename = "tests/data/cube.om3";

    let content = fs::read(filename).unwrap();

    let output = om3_parser_rs::parse_om3(&content[..]);

    assert_eq!(output.is_ok(), true);
    let (rest, mesh) = output.unwrap();
    assert_eq!(rest, b"");

    assert_eq!(mesh.faces.is_none(), false);
    assert_eq!(mesh.vertices.x.len(), 8);
    assert_eq!(mesh.vertices.y.len(), 8);
    assert_eq!(mesh.vertices.z.len(), 8);
}
