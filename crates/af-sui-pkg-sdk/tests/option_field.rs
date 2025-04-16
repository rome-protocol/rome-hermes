use af_sui_pkg_sdk::sui_pkg_sdk;

sui_pkg_sdk!(package {
    module dummy {
        struct Dummy {
            option: Option<u64>,
        }
    }
});

fn main() {}
