use super::*;
use crate::schema::{allowedips, allowedipsclients};

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "allowedipsclients"]
struct AllowedIPsMapping {
    id: i32,
    ip_id: i32,
    client_id: i32,
}

#[derive(InputObject)]
pub struct InputAllowedIpAddress {
    /// The id of the client that should be associated with the address
    pub client_id: i32,
    #[graphql(validator(list, r"(25[0-5]|2[0-4]\d|1\d\d|\d\d|\d).(?1).(?1).(?1)\/?(\d\d)?"))]
    pub ip_address: String,
}

#[derive(Debug, Insertable)]
#[table_name = "allowedipsclients"]
struct NewAllowedIPsMapping {
    ip_id: i32,
    client_id: i32,
}

impl AllowedIPsMapping {
    fn get_by_client_id(
        connection: &DatabaseConnection,
        query_client_id: i32,
    ) -> Result<Vec<Self>> {
        use crate::schema::allowedipsclients::dsl::*;

        allowedipsclients
            .filter(client_id.eq(query_client_id))
            .load(connection)
            .map_err(Error::from)
    }

    fn create(
        connection: &DatabaseConnection,
        new_mapping: &NewAllowedIPsMapping,
    ) -> Result<AllowedIPsMapping> {
        diesel::insert_into(allowedipsclients::table)
            .values(new_mapping)
            .get_result(connection)
            .map_err(Error::from)
    }
}

#[derive(Debug, Queryable)]
pub struct AllowedIP {
    pub id: i32,
    pub ip: String,
    pub subnetmask: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "allowedips"]
pub struct NewAllowedIp<'a> {
    ip: &'a str,
    subnetmask: i32,
}

impl AllowedIP {
    /// Returns the allowed ips that are associated with the given client
    pub fn get_by_client_id(connection: &DatabaseConnection, client: &Client) -> Option<Vec<Self>> {
        AllowedIPsMapping::get_by_client_id(connection, client.id)
            .map(|ips| {
                ips.iter()
                    .map(|ip| {
                        Self::get_by_id(connection, ip.ip_id)
                            .expect(&format!("IP with id {} not found", ip.id))
                    })
                    .collect::<Vec<Self>>()
            })
            .ok()
    }

    /// Returns the allowed ip for the given id
    ///
    /// # Arguments
    /// * `connection` - A connection to the database
    /// * `ip_id` - The id of the allowed ip that should be queried
    pub fn get_by_id(connection: &DatabaseConnection, ip_id: i32) -> Option<Self> {
        use crate::schema::allowedips::dsl::*;

        allowedips
            .filter(id.eq(ip_id))
            .first::<Self>(connection)
            .ok()
    }

    /// Returns the allowed ip for the given ip address
    ///
    /// # Arguments
    /// * `connection` - A connection to the database
    /// * `ip_address` - The ip address in CIDR format of the allowed ip that should be queried. If no subnetmask is
    ///     given, 24 will be used as default
    pub fn get_by_ip(connection: &DatabaseConnection, ip_address: &str) -> Option<Self> {
        use crate::schema::allowedips::dsl::*;

        let splitted_ip = ip_address
            .split('/')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let ip_address = splitted_ip.first()?;
        allowedips
            .filter(
                ip.eq(ip_address).and(
                    subnetmask.eq(splitted_ip
                        .get(1)
                        .unwrap_or(&"24".to_string())
                        .parse()
                        .unwrap_or(24)),
                ),
            )
            .first::<Self>(connection)
            .ok()
    }

    pub fn get_or_create(
        connection: &DatabaseConnection,
        ip: &str,
        client: &QueryableClient,
    ) -> Result<Self> {
        if let Some(ip) = Self::get_by_ip(connection, ip) {
            Self::assign_ip(connection, &ip, client)?;
            return Ok(ip);
        }

        // The ip does not exist yet and thus needs to be created
        let splitted_ip = ip
            .split('/')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let ip = splitted_ip
            .first()
            .ok_or(Error::new("Not a valid ip address"))?;
        let subnetmask = splitted_ip
            .get(1)
            .map(|v| v.parse().unwrap())
            .unwrap_or(24 as u32);
        Self::create(connection, client, ip, subnetmask)
    }

    pub fn create(
        connection: &DatabaseConnection,
        client: &QueryableClient,
        ip: &str,
        subnetmask: u32,
    ) -> Result<AllowedIP> {
        let allowed_ip = NewAllowedIp {
            subnetmask: subnetmask as i32,
            ip,
        };
        let created_ip = diesel::insert_into(allowedips::table)
            .values(&allowed_ip)
            .get_result::<AllowedIP>(connection)
            .map_err(Error::from)?;

        let mapping = NewAllowedIPsMapping {
            ip_id: created_ip.id,
            client_id: client.id,
        };
        AllowedIPsMapping::create(connection, &mapping)?;

        Ok(created_ip)
    }

    fn assign_ip(
        connection: &DatabaseConnection,
        allowed_ip: &Self,
        client: &QueryableClient,
    ) -> Result<()> {
        let mapping = NewAllowedIPsMapping {
            ip_id: allowed_ip.id,
            client_id: client.id,
        };
        AllowedIPsMapping::create(connection, &mapping)?;
        Ok(())
    }
}
