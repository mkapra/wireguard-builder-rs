ALTER TABLE clients ALTER COLUMN dns_server_id DROP DEFAULT;
ALTER TABLE clients ALTER COLUMN dns_server_id TYPE INT;
DROP SEQUENCE clients_dns_server_id_seq;

ALTER TABLE clients ALTER COLUMN keypair_id DROP DEFAULT;
ALTER TABLE clients ALTER COLUMN keypair_id TYPE INT;
DROP SEQUENCE clients_keypair_id_seq;

ALTER TABLE clients ALTER COLUMN vpn_ip_address_id DROP DEFAULT;
ALTER TABLE clients ALTER COLUMN vpn_ip_address_id TYPE INT;
DROP SEQUENCE clients_vpn_ip_address_id_seq;

ALTER TABLE servers ALTER COLUMN keypair_id DROP DEFAULT;
ALTER TABLE servers ALTER COLUMN keypair_id TYPE INT;
DROP SEQUENCE servers_keypair_id_seq;

ALTER TABLE servers ALTER COLUMN vpn_ip_address_id DROP DEFAULT;
ALTER TABLE servers ALTER COLUMN vpn_ip_address_id TYPE INT;
DROP SEQUENCE servers_vpn_ip_address_id_seq;

ALTER TABLE vpn_ip_addresses ALTER COLUMN vpn_network_id DROP DEFAULT;
ALTER TABLE vpn_ip_addresses ALTER COLUMN vpn_network_id TYPE INT;
DROP SEQUENCE vpn_ip_addressses_vpn_network_id_seq;
