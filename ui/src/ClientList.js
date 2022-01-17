import React, { useState } from "react";
import { useQuery, gql } from "@apollo/client";

import Table from "./components/Table";
import Error from "./components/Error";
import Searchbar from "./components/Searchbar";
import Button from "./components/Button";
import Loading from "./components/Loading";
import NewClient from "./NewClient";
import ClientDetail from "./ClientDetail";

const GET_CLIENTS = gql`
  query Query {
    clients {
      id
      name
      description
      dnsServer {
        id
      }
      vpnNetwork {
        id
      }
      ipAddress
      keepaliveInterval
      keypair {
        id
      }
    }
  }
`;

const ClientList = () => {
  const { loading, error, data } = useQuery(GET_CLIENTS);
  const [search, setSearch] = useState("");
  const [newModalIsOpen, setNewModalIsOpen] = useState(false);
  const [detailModalIsOpen, setDetailModalIsOpen] = useState(false);
  const [selectedId, setSelectedId] = useState(null);

  if (loading) return <Loading />;

  return (
    <>
      <h2 className="text-3xl mb-4">Clients</h2>
      {error && <Error error={error} />}
      {!error && (
        <>
          <div className="float-right mb-4">
            <Button onClick={() => setNewModalIsOpen(true)}>
              <div className="flex space-x-2">
                <span>New Client</span>
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
              placeholder="Search for name, ip address, description or keepalive interval..."
            />
            <Table
              headings={[
                "ID",
                "Name",
                "Description",
                "DNS-Server",
                "VPN Network",
                "IP-Address",
                "Keepalive Interval",
                "Keypair",
              ]}
              data={data.clients.filter((client) => {
                return (
                  client.name.toLowerCase().includes(search.toLowerCase()) ||
                  (client.description &&
                    client.description
                      .toLowerCase()
                      .includes(search.toLowerCase())) ||
                  client.ip_address
                    .toLowerCase()
                    .includes(search.toLowerCase()) ||
                  client.keepalive_interval
                    .toLowerCase()
                    .includes(search.toLowerCase())
                );
              })}
              onView={(id) => {
                setSelectedId(id);
                setDetailModalIsOpen(true);
              }}
            />
          </div>

          {newModalIsOpen && <NewClient setIsOpen={setNewModalIsOpen} />}
          {detailModalIsOpen && (
            <ClientDetail
              setIsOpen={setDetailModalIsOpen}
              clientId={selectedId}
              setClientId={setSelectedId}
            />
          )}
        </>
      )}
    </>
  );
};

export default ClientList;
export { GET_CLIENTS };
