table! {
    clients (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        keepalive_interval -> Int4,
        dns_server_id -> Int4,
        keypair_id -> Int4,
        vpn_ip_address_id -> Int4,
    }
}

table! {
    dns_servers (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        ip_address -> Text,
    }
}

table! {
    keypairs (id) {
        id -> Int4,
        public_key -> Text,
        private_key -> Text,
    }
}

table! {
    servers (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        forward_interface -> Nullable<Text>,
        external_ip_address -> Text,
        keypair_id -> Int4,
        vpn_ip_address_id -> Int4,
    }
}

table! {
    vpn_ip_addressses (id) {
        id -> Int4,
        vpn_network_id -> Int4,
        ip_address -> Text,
    }
}

table! {
    vpn_networks (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        ip_network -> Text,
        subnetmask -> Int4,
        listen_port -> Int4,
        interface_name -> Int4,
    }
}

joinable!(clients -> dns_servers (dns_server_id));
joinable!(clients -> keypairs (keypair_id));
joinable!(clients -> vpn_ip_addressses (vpn_ip_address_id));
joinable!(servers -> keypairs (keypair_id));
joinable!(servers -> vpn_ip_addressses (vpn_ip_address_id));
joinable!(vpn_ip_addressses -> vpn_networks (vpn_network_id));

allow_tables_to_appear_in_same_query!(
    clients,
    dns_servers,
    keypairs,
    servers,
    vpn_ip_addressses,
    vpn_networks,
);
