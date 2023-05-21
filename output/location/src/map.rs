

/// The AWS::Location::Map resource specifies a map resource in your       AWS account, which provides map tiles of different styles sourced       from global location data providers.
#[derive(Default, serde::Serialize)]
pub struct CfnMap {


    /// 
    /// No longer used. If included, the only allowed value is       RequestBasedUsage.
    /// 
    /// Allowed Values: RequestBasedUsage
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PricingPlan")]
    pub pricing_plan: Option<String>,


    /// 
    /// An optional description for the map resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name for the map resource.
    /// 
    /// Requirements:
    /// 
    /// Must contain only alphanumeric characters (A–Z, a–z, 0–9), hyphens (-), periods (.), and underscores (_). Must be a unique map resource name.         No spaces allowed. For example, ExampleMap.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[-._\w]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "MapName")]
    pub map_name: String,


    /// 
    /// Specifies the MapConfiguration, including the map style, for the       map resource that you create. The map style defines the look of maps and the data       provider for your map resource.
    /// 
    /// Required: Yes
    ///
    /// Type: MapConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "Configuration")]
    pub configuration: MapConfiguration,

}


/// Specifies the map tile style selected from an available provider.
#[derive(Default, serde::Serialize)]
pub struct MapConfiguration {


    /// 
    /// Specifies the map style selected from an available data provider.
    /// 
    /// Valid Esri map styles:
    /// 
    /// VectorEsriDarkGrayCanvas – The Esri Dark Gray Canvas map style. A           vector basemap with a dark gray, neutral background with minimal colors, labels,           and features that's designed to draw attention to your thematic content.                RasterEsriImagery – The Esri Imagery map style. A raster basemap           that provides one meter or better satellite and aerial imagery in many parts of           the world and lower resolution satellite imagery worldwide.                VectorEsriLightGrayCanvas – The Esri Light Gray Canvas map style,           which provides a detailed vector basemap with a light gray, neutral background           style with minimal colors, labels, and features that's designed to draw           attention to your thematic content.                VectorEsriTopographic – The Esri Light map style, which provides           a detailed vector basemap with a classic Esri map style.               VectorEsriStreets – The Esri World Streets map style, which           provides a detailed vector basemap for the world symbolized with a classic Esri           street map style. The vector tile layer is similar in content and style to the           World Street Map raster map.               VectorEsriNavigation – The Esri World Navigation map style, which           provides a detailed basemap for the world symbolized with a custom navigation           map style that's designed for use during the day in mobile devices.
    /// 
    /// Valid HERE         Technologies map styles:
    /// 
    /// VectorHereContrast – The HERE Contrast (Berlin) map style is a           high contrast           detailed base map of the world that blends 3D and 2D rendering.         NoteThe VectorHereContrast style has been renamed from           VectorHereBerlin.           VectorHereBerlin has been deprecated, but will continue to work in           applications that use it.               VectorHereExplore – A default HERE map style containing a           neutral, global map and its features including roads, buildings, landmarks,           and water features. It also now includes a fully designed map of Japan.               VectorHereExploreTruck – A global map containing truck           restrictions and attributes (e.g. width / height / HAZMAT) symbolized with           highlighted segments and icons on top of HERE Explore to support use cases           within transport and logistics.               RasterHereExploreSatellite – A global map containing high           resolution satellite imagery.               HybridHereExploreSatellite – A global map displaying the road           network, street names, and city labels over satellite imagery. This style           will automatically retrieve both raster and vector tiles, and your charges           will be based on total tiles retrieved.         NoteHybrid styles use both vector and raster tiles when rendering the           map that you see. This means that more tiles are retrieved than when using           either vector or raster tiles alone. Your charges will include all tiles           retrieved.
    /// 
    /// Valid GrabMaps map styles:
    /// 
    /// VectorGrabStandardLight – The Grab Standard Light           map style provides a basemap with detailed land use coloring,           area names, roads, landmarks, and points of interest covering           Southeast Asia.               VectorGrabStandardDark – The Grab Standard Dark           map style provides a dark variation of the standard basemap           covering Southeast Asia.
    /// 
    /// NoteGrab provides maps only for countries in Southeast Asia, and is only available         in the Asia Pacific (Singapore) Region (ap-southeast-1).         For more information, see GrabMaps countries and area covered.
    /// 
    /// Valid Open Data map styles:
    /// 
    /// VectorOpenDataStandardLight – The Open Data Standard Light           map style provides a detailed basemap for the world suitable for           website and mobile application use. The map includes highways major roads,           minor roads, railways, water features, cities, parks, landmarks, building           footprints, and administrative boundaries.               VectorOpenDataStandardDark – Open Data Standard Dark is a           dark-themed map style that provides a detailed basemap for the world           suitable for website and mobile application use. The map includes highways           major roads, minor roads, railways, water features, cities, parks,           landmarks, building footprints, and administrative boundaries.               VectorOpenDataVisualizationLight – The Open Data           Visualization Light map style is a light-themed style with muted colors and           fewer features that aids in understanding overlaid data.               VectorOpenDataVisualizationDark – The Open Data           Visualization Dark map style is a dark-themed style with muted colors and           fewer features that aids in understanding overlaid data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[-._\w]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Style")]
    pub style: String,

}
