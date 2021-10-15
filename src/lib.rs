use std::fmt::Display;

/// Rust bindings for the [Maptiler Cloud API](https://cloud.maptiler.com/maps/)
///
/// The Maptiler Cloud API allows for simple access to images that allow people
/// to make simple maps using satellite imagery, contour maps, or street maps.
///
/// # Example
///
/// ```
/// #[tokio::main]
/// async fn main() {
///     // Create a new Maptiler Cloud session
///     // Use your own API key from Maptiler Cloud
///     let maptiler = maptiler_cloud::Maptiler::new("placeholder api key");
///
///     // Create a new tile request
///     let x = 2;
///     let y = 1;
///     let zoom = 2;
///
///     let tile_request = maptiler_cloud::TileRequest::new(
///         maptiler_cloud::TileSet::Satellite,
///         x,
///         y,
///         zoom
///     ).unwrap();
///
///     // Create the request using the Maptiler session
///     let constructed = maptiler.create_request(tile_request);
///
///     // Actually perform the request to get the data
///     let satellite_jpg = constructed.execute().await.unwrap();
///
///     // Check for JPEG file magic to make sure we got an image
///     assert_eq!(&satellite_jpg[0..3], &[0xFF, 0xD8, 0xFF]);
/// }
/// ```
///
/// From there, most users will write those bytes to a file, or load them into another function
/// that will be able to display the image from the raw JPEG bytes.
///
pub mod errors;

/// The different types of tilesets that Maptiler Cloud supports
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TileSet {
    /// A contour map of the world
    /// Bytes returned will be a .pbf file
    Contours,
    /// A (beta) map of the countries of the world
    /// Bytes returned will be a .pbf file
    Countries,
    /// Shows hills as a transparent shaded relief
    /// Bytes returned will be a .png file
    Hillshading,
    /// A map of land vs. not land
    /// Bytes returned will be a .pbf file
    Land,
    /// Land cover which stores what kinds of plants grow in specific areas
    /// Bytes returned will be a .pbf file
    Landcover,
    /// General purpose map format
    /// Bytes returned will be a .pbf file
    MaptilerPlanet,
    /// Like MaptilerPlanet, but with extra data in only upper-level zooms
    /// Bytes returned will be a .pbf file
    MaptilerPlanetLite,
    /// OpenMapTiles format
    /// Bytes returned will be a .pbf file
    OpenMapTiles,
    /// Same as OpenMapTiles, but in the WGS84 format
    /// Bytes returned will be a .pbf file
    OpenMapTilesWGS84,
    /// Maps for outdoor life like hiking, cycling, etc.
    /// Bytes returned will be a .pbf file
    Outdoor,
    /// Satellite images
    /// Bytes returned will be a .jpg file
    Satellite,
    /// Satellite images but medium resolution from 2016
    /// Bytes returned will be a .jpg file
    SatelliteMediumRes2016,
    /// Satellite images but medium resolution from 2018
    /// Bytes returned will be a .jpg file
    SatelliteMediumRes2018,
    /// Contains terrain elevation data encoded into vector TIN polygons
    /// Bytes returned will be a quantized mesh file
    Terrain3D,
    /// Contains terrain elevation data encoded into RGB color model
    /// height = -10000 + ((R * 256 * 256 + G * 256 + B) * 0.1)
    /// Bytes returned will be a .png file
    TerrainRGB,
    /// Specify your own custom TileSet
    Custom {
        /// The Maptiler Cloud tile endpoint, for satellite imagery: "satellite"
        endpoint: &'static str,
        /// The file extension that this endpoint returns, ex: "png"
        extension: &'static str,
    },
}

impl TileSet {
    /// Returns the endpoint that this tileset requires on the API request
    ///
    /// For the satellite data tileset, the endpoint would be "satellite"
    pub fn endpoint(&self) -> &'static str {
        match self {
            TileSet::Contours => "contours",
            TileSet::Countries => "countries",
            TileSet::Hillshading => "hillshades",
            TileSet::Land => "land",
            TileSet::Landcover => "landcover",
            TileSet::MaptilerPlanet => "v3",
            TileSet::MaptilerPlanetLite => "v3-lite",
            TileSet::OpenMapTiles => "v3-openmaptiles",
            TileSet::OpenMapTilesWGS84 => "v3-4326",
            TileSet::Outdoor => "outdoor",
            TileSet::Satellite => "satellite",
            TileSet::SatelliteMediumRes2016 => "satellite-mediumres",
            TileSet::SatelliteMediumRes2018 => "satellite-mediumres-2018",
            TileSet::Terrain3D => "terrain-quantized-mesh",
            TileSet::TerrainRGB => "terrain-rgb",
            TileSet::Custom {
                endpoint,
                extension: _,
            } => endpoint,
        }
    }

    /// Returns the maximum zoom level that this tileset supports
    ///
    /// The custom tileset variant has a maximum of 20 here, but it may be lower than that. Take
    /// care when using a custom tileset variant.
    ///
    pub fn max_zoom(&self) -> u32 {
        match self {
            TileSet::Contours => 14,
            TileSet::Countries => 11,
            TileSet::Hillshading => 12,
            TileSet::Land => 14,
            TileSet::Landcover => 9,
            TileSet::MaptilerPlanet => 14,
            TileSet::MaptilerPlanetLite => 10,
            TileSet::OpenMapTiles => 14,
            TileSet::OpenMapTilesWGS84 => 13,
            TileSet::Outdoor => 14,
            TileSet::Satellite => 20,
            TileSet::SatelliteMediumRes2016 => 13,
            TileSet::SatelliteMediumRes2018 => 13,
            TileSet::Terrain3D => 13,
            TileSet::TerrainRGB => 12,
            // For the custom
            TileSet::Custom {
                endpoint: _,
                extension: _,
            } => 20,
        }
    }

    /// Returns the minimum zoom level that this tileset supports
    ///
    /// The custom tileset variant has a minimum of 0 here, but it may be higher than that. Take
    /// care when using a custom tileset variant.
    ///
    pub fn min_zoom(&self) -> u32 {
        match self {
            TileSet::Contours => 9,
            TileSet::Countries => 0,
            TileSet::Hillshading => 0,
            TileSet::Land => 0,
            TileSet::Landcover => 0,
            TileSet::MaptilerPlanet => 0,
            TileSet::MaptilerPlanetLite => 0,
            TileSet::OpenMapTiles => 0,
            TileSet::OpenMapTilesWGS84 => 0,
            TileSet::Outdoor => 5,
            TileSet::Satellite => 0,
            TileSet::SatelliteMediumRes2016 => 0,
            TileSet::SatelliteMediumRes2018 => 0,
            TileSet::Terrain3D => 0,
            TileSet::TerrainRGB => 0,
            // For the custom
            TileSet::Custom {
                endpoint: _,
                extension: _,
            } => 0,
        }
    }

    /// Returns the file extension that this tileset returns as a static &str
    ///
    /// Example outputs are: "png", "jpg", "pbf"
    pub fn file_extension(&self) -> &'static str {
        match self {
            TileSet::Contours
            | TileSet::Countries
            | TileSet::Land
            | TileSet::Landcover
            | TileSet::MaptilerPlanet
            | TileSet::MaptilerPlanetLite
            | TileSet::OpenMapTiles
            | TileSet::OpenMapTilesWGS84
            | TileSet::Outdoor => "pbf",
            TileSet::Hillshading | TileSet::TerrainRGB => "png",
            TileSet::Satellite
            | TileSet::SatelliteMediumRes2016
            | TileSet::SatelliteMediumRes2018 => "jpg",
            TileSet::Terrain3D => "quantized-mesh-1.0",
            TileSet::Custom {
                endpoint: _,
                extension,
            } => extension,
        }
    }
}

impl Display for TileSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TileSet::Contours => "Contours",
                TileSet::Countries => "Countries",
                TileSet::Hillshading => "Hillshades",
                TileSet::Land => "Land",
                TileSet::Landcover => "Landcover",
                TileSet::MaptilerPlanet => "MaptilerPlanet",
                TileSet::MaptilerPlanetLite => "MaptilerPlanetLite",
                TileSet::OpenMapTiles => "OpenMapTiles",
                TileSet::OpenMapTilesWGS84 => "OpenMapTilesWGS84",
                TileSet::Outdoor => "Outdoor",
                TileSet::Satellite => "Satellite",
                TileSet::SatelliteMediumRes2016 => "SatelliteMediumRes2016",
                TileSet::SatelliteMediumRes2018 => "SatelliteMediumRes2018",
                TileSet::Terrain3D => "Terrain3D",
                TileSet::TerrainRGB => "TerrainRGB",
                TileSet::Custom {
                    endpoint,
                    extension: _,
                } => endpoint,
            }
        )
    }
}

/// A struct containing the arguments required to make a request for a tile
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TileRequest {
    set: TileSet,
    zoom: u32,
    tile_x: u32,
    tile_y: u32,
}

impl TileRequest {
    /// Creates a new TileRequest with the given parameters
    ///
    /// set: A TileSet representing which tileset to get the tile from. See https://cloud.maptiler.com/tiles/
    ///
    /// x: The x-coordinate of the tile in the [Tiled Web Map format](https://en.wikipedia.org/wiki/Tiled_web_map)
    /// y: The y-coordinate of the tile
    /// zoom: The zoom level of the tile in the Tile Web Map format
    ///
    /// The x and y positions must be in bounds
    ///
    pub fn new(set: TileSet, x: u32, y: u32, zoom: u32) -> Result<Self, errors::ArgumentError> {
        // Check if the zoom is valid
        if zoom > set.max_zoom() {
            return Err(errors::ArgumentError::ZoomTooLarge(
                zoom,
                set,
                set.max_zoom(),
            ));
        } else if zoom < set.min_zoom() {
            return Err(errors::ArgumentError::ZoomTooSmall(
                zoom,
                set,
                set.min_zoom(),
            ));
        }

        // Check if the coordinates are valid
        let max_coordinate = Self::max_coordinate_with_zoom(zoom);

        if x > max_coordinate {
            return Err(errors::ArgumentError::XTooLarge(x, zoom, max_coordinate));
        }

        if y > max_coordinate {
            return Err(errors::ArgumentError::YTooLarge(y, zoom, max_coordinate));
        }

        Ok(Self {
            set,
            zoom,
            tile_x: x,
            tile_y: y,
        })
    }

    // Calculates the maximum x or y coordinate for a given zoom level
    fn max_coordinate_with_zoom(zoom: u32) -> u32 {
        // This special case is if zoom == 0
        //
        // Then there is only one tile, so the max x and y are 0
        if zoom == 0 {
            0
        } else {
            // This does 2^zoom level
            //
            // zoom = 0:
            //      2^0 = 1
            // zoom = 1:
            //      2^1 = 2

            1 << zoom
        }
    }

    /// Returns the x coordinate of this tile request
    pub fn x(&self) -> u32 {
        self.tile_x
    }

    /// Returns the y coordinate of this tile request
    pub fn y(&self) -> u32 {
        self.tile_y
    }

    /// Returns the zoom level of this tile request
    pub fn zoom(&self) -> u32 {
        self.zoom
    }
}

impl From<TileRequest> for RequestType {
    fn from(tile_request: TileRequest) -> Self {
        RequestType::TileRequest(tile_request)
    }
}

/// The type of request to the Maptiler Cloud API
#[derive(Debug, Copy, Clone)]
pub enum RequestType {
    TileRequest(TileRequest),
}

/// Represents a request that has already been constructed using the Maptiler that created it. This
/// can be directly await-ed using execute()
#[derive(Debug, Clone)]
pub struct ConstructedRequest {
    api_key: String,
    inner: RequestType,
}

impl ConstructedRequest {
    /// Actually performs the API call to the Maptiler Cloud API
    pub async fn execute(&self) -> Result<Vec<u8>, errors::Error> {
        match self.inner {
            RequestType::TileRequest(tile_request) => self.execute_tile(tile_request).await,
        }
    }

    async fn execute_tile(&self, tile_request: TileRequest) -> Result<Vec<u8>, errors::Error> {
        let tileset = &tile_request.set;
        let endpoint = tileset.endpoint();
        let extension = tileset.file_extension();
        let zoom = tile_request.zoom;
        let x = tile_request.tile_x;
        let y = tile_request.tile_y;

        // https://api.maptiler.com/tiles/satellite/{z}/{x}/{y}.jpg?key=AAAAAAAAAAAAAAAAAA
        let url = format!(
            "https://api.maptiler.com/tiles/{}/{}/{}/{}.{}?key={}",
            endpoint, zoom, x, y, extension, self.api_key
        );

        // Perform the actual request
        let res = reqwest::get(url).await?;

        match res.status() {
            reqwest::StatusCode::OK => Ok(res.bytes().await?.to_vec()),
            status => Err(errors::Error::Http(status)),
        }
    }
}

/// A struct that serves as a Maptiler "session", which stores the API key and is used to create
/// requests
pub struct Maptiler {
    api_key: String,
}

impl Maptiler {
    /// Initializes this Maptiler Cloud API session
    pub fn new<S>(api_key: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            api_key: api_key.into(),
        }
    }

    /// Performs a generic request to the Maptiler Cloud API
    ///
    /// This may be a little simpler to use so that any type of request can be passed into this
    /// function
    ///
    pub fn create_request(&self, request: impl Into<RequestType>) -> ConstructedRequest {
        ConstructedRequest {
            api_key: self.api_key.to_string(),
            inner: request.into(),
        }
    }

    /// Performs a tile request to the Maptiler Cloud API
    pub fn create_tile_request(&self, tile_request: TileRequest) -> ConstructedRequest {
        ConstructedRequest {
            api_key: self.api_key.to_string(),
            inner: RequestType::TileRequest(tile_request),
        }
    }
}
