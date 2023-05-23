// ChannelTypeVersion is a common version number for Synerex Providers
const ChannelTypeVersion: &str = "0.1.12"; // string for pbase version

// if you change this number you should update "ChannelTypeVersion"
static ChannelTypeMax: usize = 32; // Default Synerex Server channel size

// Channel Types
static RIDE_SHARE:         u32 = 1;  // Rideshare Service Information
static AD_SERVICE:         u32 = 2;  // Advertisement Service Information
static LIB_SERVICE:        u32 = 3;  // Public Library Service Information
static PT_SERVICE:         u32 = 4;  // Public Transit Information
static ROUTING_SERVICE:    u32 = 5;  // Routing Service
static MARKETING_SERVICE:  u32 = 6;  // Marketing (Ad/Enquate)
static FLUENTD_SERVICE:    u32 = 7;  // Fluentd Service (td-agent/fluetnd)
static MEETING_SERVICE:    u32 = 8;  // RPA Meetinng Service (rpa provider)
static STORAGE_SERVICE:    u32 = 9;  // Storage Service (storage providers)
static RETRIEVAL_SERVICE:  u32 = 10; // Retrieval Service (retrieval providers)
static PEOPLE_COUNTER_SVC: u32 = 11; // People Counter Service (Pflow providers)
static AREA_COUNTER_SVC:   u32 = 12; // Area counter service
static PEOPLE_AGENT_SVC:   u32 = 13; // people agent service
static GEOGRAPHIC_SVC:     u32 = 14; // Geographical mapping service
static JSON_DATA_SVC:      u32 = 15; // Json data service
static MQTT_GATEWAY_SVC:   u32 = 16; // MQTT Gateway service
static WAREHOUSE_SVC:      u32 = 17; // Warehouse Execution/Management service
static PEOPLE_FLOW_SVC:    u32 = 18; // People Flow service
static GRIDEYE_SVC:        u32 = 19; // Grid Eye service
static LATENT_DMD_SVC:     u32 = 20; // Latent Demand service
static LATENT_DMD_DSP_SVC: u32 = 21; // Latent Demand Display service
static ALT_PT_SVC:         u32 = 22; // Alternative Public Transit Service
