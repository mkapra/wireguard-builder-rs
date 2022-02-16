CREATE TABLE IF NOT EXISTS AllowedIps (
  id SERIAL PRIMARY KEY,
  ip TEXT NOT NULL,
  subnetmask INTEGER NOT NULL DEFAULT 32,

  UNIQUE (ip, subnetmask)
);

-- Create associaction between allowed ips and cllients
CREATE TABLE IF NOT EXISTS AllowedIpsClients (
  id SERIAL PRIMARY KEY,
  ip_id INTEGER NOT NULL,
  client_id INTEGER NOT NULL,

  FOREIGN KEY (ip_id) REFERENCES AllowedIps(id),
  FOREIGN KEY (client_id) REFERENCES Clients(id),
  UNIQUE (ip_id, client_id)
);