import React, { useState } from "react";
import { useQuery, gql } from "@apollo/client";

import Table from "./Table";
import Error from "./Error";
import Searchbar from "./Searchbar";
import Button from "./Button";
import NewServer from "./NewServer";
import ServerDetail from "./ServerDetail";

const GET_SERVERS = gql`
  query Query {
    servers {
      id
      name
      description
      ip_address
      forward_interface
      keypair {
        id
      }
      vpn_network {
        id
      }
    }
  }
`;

const ServerList = () => {
  const { loading, error, data } = useQuery(GET_SERVERS);
  const [search, setSearch] = useState("");
  const [newModalIsOpen, setNewModalIsOpen] = useState(false);
  const [viewModalIsOpen, setViewModalIsOpen] = useState(false);
  const [selectedServerId, setSelectedServerId] = useState(null);

  if (loading) return <p>Loading...</p>;

  return (
    <>
      <h2 className="text-3xl mb-4">Servers</h2>
      {error && <Error error={error} />}
      {!error && (
        <>
          <div className="float-right mb-4">
            <Button onClick={() => setNewModalIsOpen(true)}>
              <div className="flex space-x-2">
                <span>New Server</span>
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
            <Searchbar
              search={search}
              setSearch={setSearch}
              placeholder="Search for name, ip address, description or forward interface..."
            />
            <Table
              headings={[
                "ID",
                "Name",
                "Description",
                "IP-Address",
                "Forward Interface",
                "Keypair",
                "VPN Network",
              ]}
              data={data.servers.filter((server) => {
                return (
                  server.name.toLowerCase().includes(search.toLowerCase()) ||
                  (server.description &&
                    server.description
                      .toLowerCase()
                      .includes(search.toLowerCase())) ||
                  server.ip_address
                    .toLowerCase()
                    .includes(search.toLowerCase()) ||
                  server.forward_interface
                    .toLowerCase()
                    .includes(search.toLowerCase())
                );
              })}
              onView={(id) => {
                setSelectedServerId(id);
                setViewModalIsOpen(true);
              }}
            />
          </div>

          {newModalIsOpen && <NewServer setIsOpen={setNewModalIsOpen} />}
          {viewModalIsOpen && (
            <ServerDetail
              setIsOpen={setViewModalIsOpen}
              serverId={selectedServerId}
            />
          )}
        </>
      )}
    </>
  );
};

export default ServerList;
export { GET_SERVERS };
