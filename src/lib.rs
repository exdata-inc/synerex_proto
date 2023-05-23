// ChannelTypeVersion is a common version number for Synerex Providers
pub const CHANNEL_TYPE_VERSION: &str = "0.1.12"; // string for pbase version

// if you change this number you should update "ChannelTypeVersion"
pub static CHANNEL_TYPE_MAX: usize = 32; // Default Synerex Server channel size

// Channel Types
pub static RIDE_SHARE:         u32 = 1;  // Rideshare Service Information
pub static AD_SERVICE:         u32 = 2;  // Advertisement Service Information
pub static LIB_SERVICE:        u32 = 3;  // Public Library Service Information
pub static PT_SERVICE:         u32 = 4;  // Public Transit Information
pub static ROUTING_SERVICE:    u32 = 5;  // Routing Service
pub static MARKETING_SERVICE:  u32 = 6;  // Marketing (Ad/Enquate)
pub static FLUENTD_SERVICE:    u32 = 7;  // Fluentd Service (td-agent/fluetnd)
pub static MEETING_SERVICE:    u32 = 8;  // RPA Meetinng Service (rpa provider)
pub static STORAGE_SERVICE:    u32 = 9;  // Storage Service (storage providers)
pub static RETRIEVAL_SERVICE:  u32 = 10; // Retrieval Service (retrieval providers)
pub static PEOPLE_COUNTER_SVC: u32 = 11; // People Counter Service (Pflow providers)
pub static AREA_COUNTER_SVC:   u32 = 12; // Area counter service
pub static PEOPLE_AGENT_SVC:   u32 = 13; // people agent service
pub static GEOGRAPHIC_SVC:     u32 = 14; // Geographical mapping service
pub static JSON_DATA_SVC:      u32 = 15; // Json data service
pub static MQTT_GATEWAY_SVC:   u32 = 16; // MQTT Gateway service
pub static WAREHOUSE_SVC:      u32 = 17; // Warehouse Execution/Management service
pub static PEOPLE_FLOW_SVC:    u32 = 18; // People Flow service
pub static GRIDEYE_SVC:        u32 = 19; // Grid Eye service
pub static LATENT_DMD_SVC:     u32 = 20; // Latent Demand service
pub static LATENT_DMD_DSP_SVC: u32 = 21; // Latent Demand Display service
pub static ALT_PT_SVC:         u32 = 22; // Alternative Public Transit Service
