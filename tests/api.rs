use maptiler_cloud::errors::ArgumentError;
use maptiler_cloud::{Maptiler, TileRequest, TileSet};
use std::env;

#[tokio::test]
async fn get_tile() {
    let api_key = env::var("MAPTILER_KEY").expect("Environment variable MAPTILER_KEY not set");

    let maptiler = Maptiler::new(api_key).unwrap();

    // Gets a satellite view, with zoom 0 (the whole world)
    // The X and Y coordinates of this type must be 0, 0, because there is only one tile in the set
    // that contains the world
    let tile_request = TileRequest::new(TileSet::Satellite, 0, 0, 0).unwrap();

    // Create the request using the Maptiler session
    let constructed = maptiler.create_request(tile_request);

    // Actually perform the request to get the data
    let tile = constructed.execute().await.unwrap();

    // Check for JPEG file magic
    assert_eq!(&tile[0..3], &[0xFF, 0xD8, 0xFF]);
}

#[test]
fn zoom_low() {
    // TileSet::Outdoor's minimum zoom level is 5
    let request_err =
        TileRequest::new(TileSet::Outdoor, 0, 0, 2).expect_err("Invalid request succeeded");

    assert_eq!(
        request_err,
        ArgumentError::ZoomTooSmall(2, TileSet::Outdoor, 5)
    );
}

#[test]
fn zoom_high() {
    // TileSet::Satellite's maximum zoom level is 20
    let request_err =
        TileRequest::new(TileSet::Satellite, 0, 0, 21).expect_err("Invalid request succeeded");

    assert_eq!(
        request_err,
        ArgumentError::ZoomTooLarge(21, TileSet::Satellite, 20)
    );
}

#[test]
fn x_high() {
    // At zoom level 2, the maximum x-coordinate is 4
    let request_err =
        TileRequest::new(TileSet::Satellite, 5, 0, 2).expect_err("Invalid request succeeded");

    assert_eq!(request_err, ArgumentError::XTooLarge(5, 2, 4));
}

#[test]
fn y_high() {
    // At zoom level 3, the maximum y-coordinate is 8
    let request_err =
        TileRequest::new(TileSet::Satellite, 5, 10, 3).expect_err("Invalid request succeeded");

    assert_eq!(request_err, ArgumentError::YTooLarge(10, 3, 8));
}
