import React, { useState } from "react";
import { useQuery, gql } from "@apollo/client";

import Table from "./Table";
import Searchbar from "./Searchbar";
import Error from "./Error";
import NewVpnNetwork from "./NewVpnNetwork";
import Button from "./Button";

const GET_VPN_NETWORKS = gql`
  query Query {
    vpnNetworks {
      id
      name
      description
      ip_address
      subnetmask
      port
      interface
    }
  }
`;

const VpnNetworkList = () => {
  const { loading, error, data } = useQuery(GET_VPN_NETWORKS);
  const [search, setSearch] = useState("");
  const [isOpen, setIsOpen] = useState(false);

  if (loading) return <p>Loading...</p>;

  const matches = (vpnNetwork) => {
    return (
      vpnNetwork.name.toLowerCase().includes(search.toLowerCase()) ||
      vpnNetwork.description.toLowerCase().includes(search.toLowerCase()) ||
      vpnNetwork.ip_address.toLowerCase().includes(search.toLowerCase()) ||
      vpnNetwork.interface.toLowerCase().includes(search.toLowerCase())
    );
  };

  return (
    <>
      <h2 className="text-3xl mb-4">VPN Networks</h2>
      {error && <Error error={error} />}

      {!error && (
        <>
          <div className="float-right mb-4">
            <Button onClick={() => setIsOpen(true)}>
              <div className="flex space-x-2">
                <span>New VPN Network</span>
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="h-6 w-6"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth="2"
                    d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
              </div>
            </Button>
          </div>

          <div className="space-y-2">
            <Searchbar search={search} setSearch={setSearch} />
            <Table
              headings={[
                "ID",
                "Name",
                "Description",
                "IP-Address",
                "Subnetmask",
                "Port",
                "Interface",
              ]}
              data={data.vpnNetworks.filter((network) =>
                matches(network, search)
              )}
            />
          </div>

          {isOpen && <NewVpnNetwork setIsOpen={setIsOpen} />}
        </>
      )}
    </>
  );
};

export default VpnNetworkList;
export { GET_VPN_NETWORKS };
