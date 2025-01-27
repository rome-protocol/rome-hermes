use af_sui_pkg_sdk::sui_pkg_sdk;

sui_pkg_sdk!(package {
    module events {
        public struct Event<VersionedEvent: copy + drop> has copy, drop {
            inner: VersionedEvent
        }

        public(package) struct Instance has copy, drop {}
    }
});

fn main() {}
