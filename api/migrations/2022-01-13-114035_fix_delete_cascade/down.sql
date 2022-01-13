ALTER TABLE clients ALTER COLUMN dns_server_id TYPE SERIAL;
ALTER TABLE clients ALTER COLUMN keypair_id TYPE SERIAL;
ALTER TABLE clients ALTER COLUMN vpn_ip_address_id TYPE SERIAL;

ALTER TABLE servers ALTER COLUMN keypair_id TYPE SERIAL;
ALTER TABLE servers ALTER COLUMN vpn_ip_address_id TYPE SERIAL;

ALTER TABLE vpn_ip_addresses ALTER COLUMN vpn_network_id TYPE SERIAL;