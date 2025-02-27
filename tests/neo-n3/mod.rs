// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Neo N3 FaaS platform tests

pub mod neo_source {
    include!("neo-source/neo_source_test.rs");
}

pub mod js_runtime {
    include!("js-runtime/js_runtime_test.rs");
}

pub mod oracle_services {
    include!("oracle-services/oracle_services_test.rs");
}

pub mod tee_services {
    include!("tee-services/tee_services_test.rs");
}

pub mod service_api {
    include!("service-api/service_api_test.rs");
}

pub mod test_utils;
pub mod test_fixtures;
