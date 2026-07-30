mod dto{
    /// -------------------------------------------------------------------
    /// Defines a Data Transfer Object for a Location, which is a very specific and 
    /// identifiable physical position within the hospital. This is separated
    /// from the Site, which is a higher level and more generalized area.
    /// -------------------------------------------------------------------
    /// 
    #[derive(serde::Deserialize)]
    pub struct Location {
        #[serde(rename = "Id")]
        pub id: u32, // ID BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
        #[serde(rename = "Name")]
        pub name: String, // NAME VARCHAR(200) UNIQUE NOT NULL, 
        #[serde(rename = "Building")]
        pub building: String, // BUILDING VARCHAR(200), 
        #[serde(rename = "Floor")]
        pub floor: String, // FLOOR VARCHAR(200), 
        #[serde(rename = "Wing")]
        pub wing: String, //  WING VARCHAR(200),
        #[serde(rename = "Notes")]
        pub notes: String, //   NOTES VARCHAR(2000),
        #[serde(rename = "SiteId")]
        pub site_id: u32, //  SITE_ID BIGINT NOT NULL REFERENCES SITE (ID),
    } 

    impl Location {
        /// Basic constructor
        /// 
        pub fn new(id: u32,
                name: String,
                building: String,
                floor: String,
                wing: String,
                notes: String,
                site_id: u32
                ) -> Self {
            Self { 
                id,
                name,
                building,
                floor,
                wing,
                notes,
                site_id
            }
        }
    }
}