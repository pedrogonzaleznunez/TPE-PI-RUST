#![allow(non_snake_case)]

#[allow(dead_code)]
pub struct RequestCSVFields {
    pub CreatedDate: i8, // yyyy-MM-dd HH:mm:ss
    pub AgencyName: i8,
    pub Acronym: i8,
    pub Status: i8,
    pub Borough: i8,
    pub Latitude: i8,
    pub Longitude: i8,
}

#[allow(dead_code)]
pub struct CityConfig {
    pub typesFilePath: &'static str,
    pub requestsFilePath: &'static str,
    pub requestCSVFields: RequestCSVFields,
}

#[cfg(feature = "nyc")]
pub const CITY_CONFIG: CityConfig = CityConfig {
    typesFilePath: "resources/first_hundred_requests/typesNYC.csv",
    requestsFilePath: "resources/first_hundred_requests/requestsNYC.csv",
    requestCSVFields: RequestCSVFields {
        CreatedDate: 0,
        AgencyName: 1,
        Acronym: 2,
        Status: 3,
        Borough: 4,
        Latitude: 5,
        Longitude: 6,
    },
};

#[cfg(feature = "chi")]
pub const CITY_CONFIG: CityConfig = CityConfig {
    typesFilePath: "resources/first_hundred_requests/typesCHI.csv",
    requestsFilePath: "resources/first_hundred_requests/requestsCHI.csv",
    requestCSVFields: RequestCSVFields {
        Acronym: 0,
        AgencyName: 1,
        Status: 2,
        CreatedDate: 3,
        Borough: 4,
        Latitude: 5,
        Longitude: 6,
    },
};
