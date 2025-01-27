extern crate nom;

use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::bytes::streaming::take_till;
use nom::combinator::opt;
use nom::multi::count;
use nom::number::streaming::be_f32;
use nom::number::streaming::be_u32;
use nom::IResult;
use nom::Parser;

struct Header {}

pub struct PointCloud {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub z: Vec<f32>,
}

pub struct Face {
    pub indices: Vec<u32>,
}
pub struct FacePolygon {
    pub faces: Vec<Face>,
}

pub struct Om3Out {
    pub vertices: PointCloud,
    pub faces: Option<FacePolygon>,
}

fn parse_header(input: &[u8]) -> IResult<&[u8], Header> {
    //let (input, _) = tag(b"HOME3DF\0xA\x00\x01\x00")(input)?;
    let (input, _) = tag("\x48\x4F\x4D\x33\x44\x46\x0A\x00\x01\x00")(input)?;
    let (input, _) = alt((tag("\x0B"), tag("\x0C"))).parse(input)?;

    Ok((input, Header {}))
}

fn parse_end(input: &[u8]) -> IResult<&[u8], ()> {
    //let (input, _) = tag(b"FD3EMOH\x2E")(input)?;
    let (input, _) = tag("\x46\x44\x33\x4D\x4F\x48\x2E")(input)?;
    Ok((input, ()))
}

fn parse_point_cloud(input: &[u8]) -> IResult<&[u8], PointCloud> {
    let (input, _) = tag("point_coord")(input)?;
    let (input, n_vertices) = be_u32(input)?;

    let (input, x) = count(be_f32, n_vertices as usize).parse(input)?;
    let (input, y) = count(be_f32, n_vertices as usize).parse(input)?;
    let (input, z) = count(be_f32, n_vertices as usize).parse(input)?;

    Ok((input, PointCloud { x, y, z }))
}

fn parse_face(input: &[u8], n_indices: u32) -> IResult<&[u8], Face> {
    let (input, indices) = count(be_u32, n_indices as usize).parse(input)?;
    Ok((input, Face { indices }))
}

fn parse_face_polygon(input: &[u8]) -> IResult<&[u8], FacePolygon> {
    let (input, _) = tag("face_polygon")(input)?;
    let (input, n_faces) = be_u32(input)?;
    let (input, n_indices) = be_u32(input)?;

    let indices_per_face = n_indices / n_faces;

    let (input, faces) =
        count(|i| parse_face(i, indices_per_face), n_faces as usize).parse(input)?;

    Ok((input, FacePolygon { faces }))
}

pub fn parse_om3(input: &[u8]) -> IResult<&[u8], Om3Out> {
    let (input, _) = parse_header(input)?;
    let (input, face_polygon) = opt(parse_face_polygon).parse(input)?;

    // skip some unknown stuff. Maybe attributes per face?
    // 'p' for the start of 'point_cloud'
    let (input, _) = take_till(|s| s == b'p')(input)?;

    let (input, point_cloud) = parse_point_cloud(input)?;

    // skip empty field until the end
    let (input, _) = take_till(|s| s != b'\x00')(input)?;

    let (input, _) = parse_end(input)?;
    Ok((
        input,
        Om3Out {
            vertices: point_cloud,
            faces: face_polygon,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_header() {
        let input = b"\x48\x4F\x4D\x33\x44\x46\x0A\x00\x01\x00\x0B";
        //let input2 = b"HOME3DF\x0A\x00\x01\x00\x0B";
        let result = parse_header(input);

        assert!(result.is_ok());
        let (left_input, _) = result.unwrap();

        assert_eq!(left_input, b"");

        let input = b"\x48\x4F\x4D\x33\x44\x46\x0A\x00\x01\x00\x0C";
        let result = parse_header(input);

        assert!(result.is_ok());
        let (left_input, _) = result.unwrap();

        assert_eq!(left_input, b"");
    }

    #[test]
    fn test_end() {
        let input = b"\x46\x44\x33\x4D\x4F\x48\x2E";
        let result = parse_end(input);

        assert!(result.is_ok());
        let (left_input, _) = result.unwrap();

        assert_eq!(left_input, b"");
    }

    #[test]
    fn test_point_cloud() {
        let input = b"point_coord\x00\x00\x00\x01\x3f\x80\x00\x00\x40\x00\x00\x00\x40\x40\x00\x00";

        let result = parse_point_cloud(input);
        assert!(result.is_ok());

        let (left, pc) = result.unwrap();
        assert_eq!(left, []);
        assert_eq!(pc.x.len(), 1);
        assert_eq!(pc.y.len(), 1);
        assert_eq!(pc.z.len(), 1);

        assert_eq!(pc.x[0], 1.0_f32);
        assert_eq!(pc.y[0], 2.0_f32);
        assert_eq!(pc.z[0], 3.0_f32);
    }

    #[test]
    fn test_face_polygon() {
        let input = b"face_polygon\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x03";

        let result = parse_face_polygon(input);
        assert!(result.is_ok());

        let (left, fp) = result.unwrap();
        assert_eq!(left, []);
        assert_eq!(fp.faces.len(), 1);
        let indices = &fp.faces[0].indices;
        assert_eq!(indices.len(), 4);
        assert_eq!(indices[0], 0);
        assert_eq!(indices[1], 1);
        assert_eq!(indices[2], 2);
        assert_eq!(indices[3], 3);
    }
}
