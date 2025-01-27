use af_sui_pkg_sdk::sui_pkg_sdk;

sui_pkg_sdk!(package {
    module role {
        /// Should become
        /// ```ignore
        /// #[move_(module = role)]
        /// #[move_(crate = ::af_sui_pkg_sdk::af_move_type)]
        /// #[serde(crate = "::af_sui_pkg_sdk::serde")]
        /// #[serde(bound(deserialize = ""))]
        /// #[tabled(crate = "::af_sui_pkg_sdk::tabled")]
        /// #[allow(non_snake_case)]
        /// pub struct ADMIN(bool);
        ///
        /// #[serde_with(crate = ":: af_sui_pkg_sdk :: af_move_type :: external :: serde_with")]
        /// pub struct ADMINTypeTag {
        ///     pub address: ::af_sui_pkg_sdk::af_move_type::external::Address,
        /// }
        /// ```
        public struct ADMIN() has drop;

        public(package) struct ASSISTANT() has drop;
    }

    module events {
        /// Should become
        /// ```ignore
        /// #[move_(module = events)]
        /// #[move_(crate = ::af_sui_pkg_sdk::af_move_type)]
        /// #[serde(crate = "::af_sui_pkg_sdk::serde")]
        /// #[serde(bound(deserialize = ""))]
        /// #[tabled(crate = "::af_sui_pkg_sdk::tabled")]
        /// #[allow(non_snake_case)]
        /// pub struct Event<VersionedEvent: ::af_sui_pkg_sdk::MoveType>(pub VersionedEvent);
        ///
        /// impl<VersionedEvent: ::af_sui_pkg_sdk::MoveType> Event<VersionedEvent> {
        ///     ///Constructs a new `Event`.
        ///     #[allow(non_snake_case)]
        ///     pub fn new(f0: VersionedEvent) -> Self {
        ///         Event(f0)
        ///     }
        /// }
        ///
        /// #[derive_where(crate = ::af_sui_pkg_sdk::af_move_type::external::derive_where)]
        /// #[serde_with(crate = ":: af_sui_pkg_sdk :: af_move_type :: external :: serde_with")]
        /// #[derive_where(PartialOrd, Ord)]
        /// pub struct EventTypeTag<VersionedEvent: ::af_sui_pkg_sdk::MoveType> {
        ///     pub address: ::af_sui_pkg_sdk::af_move_type::external::Address,
        ///     pub versioned_event: <VersionedEvent as ::af_sui_pkg_sdk::af_move_type::MoveType>::TypeTag,
        /// }
        /// ```
        public struct Event<VersionedEvent: copy + drop>(VersionedEvent) has copy, drop;
    }
});

fn main() {}
