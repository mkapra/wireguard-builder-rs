CREATE TABLE IF NOT EXISTS Keypairs (
  id SERIAL PRIMARY KEY,
  public_key TEXT NOT NULL UNIQUE,
  private_key TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS Dns_Servers (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  description TEXT,
  ip_address TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS Vpn_Networks (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  description TEXT,
  ip_network TEXT NOT NULL UNIQUE,
  subnetmask INTEGER NOT NULL DEFAULT 24,
  listen_port SERIAL NOT NULL UNIQUE,
  interface_name SERIAL NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS Vpn_Ip_Addressses (
  id SERIAL PRIMARY KEY,
  vpn_network_id SERIAL NOT NULL,
  ip_address TEXT NOT NULL UNIQUE,

  FOREIGN KEY (vpn_network_id) REFERENCES Vpn_Networks(id)
);

CREATE TABLE IF NOT EXISTS Servers (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  description TEXT,
  forward_interface TEXT,
  external_ip_address TEXT NOT NULL,
  keypair_id SERIAL NOT NULL UNIQUE,
  vpn_ip_address_id SERIAL NOT NULL UNIQUE,

  FOREIGN KEY (keypair_id) REFERENCES Keypairs(id) ON DELETE CASCADE,
  FOREIGN KEY (vpn_ip_address_id) REFERENCES Vpn_Ip_Addressses(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Clients (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  description TEXT,
  keepalive_interval INTEGER NOT NULL DEFAULT 25,
  dns_server_id SERIAL NOT NULL,
  keypair_id SERIAL NOT NULL UNIQUE,
  vpn_ip_address_id SERIAL NOT NULL UNIQUE,

  FOREIGN KEY (dns_server_id) REFERENCES Dns_Servers(id),
  FOREIGN KEY (keypair_id) REFERENCES Keypairs(id) ON DELETE CASCADE,
  FOREIGN KEY (vpn_ip_address_id) REFERENCES Vpn_Ip_Addressses(id) ON DELETE CASCADE
);