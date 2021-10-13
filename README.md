# maptiler-cloud
Rust wrapper around  the [Maptiler Cloud](https://cloud.maptiler.com/maps/) API

Supports requesting all currently available tilesets

Tiles are requested using the [Tiled Web Map](https://en.wikipedia.org/wiki/Tiled_web_map) format.
X and Y coordinates are specified, and a zoom level is specified.

## Example Usage

```rust
#[tokio::main]
async fn main() {
    // Create a new Maptiler Cloud session
    // Use your own API key from Maptiler Cloud
    let maptiler = maptiler_cloud::Maptiler::new("placeholder api key");

    // Create a new tile request
    let x = 2;
    let y = 1;
    let zoom = 2;

    let tile_request = maptiler_cloud::TileRequest::new(
        maptiler_cloud::TileSet::Satellite,
        x,
        y,
        zoom
    ).unwrap();

    // Perform the request (make the API call)
    let satellite_jpg = maptiler.request(tile_request).await.unwrap();

    // Check for JPEG file magic to make sure we got an image
    assert_eq!(&satellite_jpg[0..3], &[0xFF, 0xD8, 0xFF]);
}
```

From there, most users will write those bytes to a file, or load them into another function
that will be able to display the image from the raw JPEG bytes.
