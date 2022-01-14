import React, { useState, useEffect } from "react";
import PropTypes from "prop-types";
import { gql, useQuery, useMutation } from "@apollo/client";
import Loader from "react-loader-spinner";
import { toast } from "react-toastify";

import { GET_CLIENTS } from "./ClientList";
import Modal from "./Modal";
import FormInputField from "./FormInputField";
import SubmitButton from "./SubmitButton";
import SelectInputField from "./SelectInputField";

import "react-loader-spinner/dist/loader/css/react-spinner-loader.css";

const GET_REFERENCES = gql`
  query GetReferences {
    dnsServers {
      id
      name
      ip_address
    }
    vpnNetworks {
      id
      name
      ip_address
    }
    unusedKeypairs {
      id
      public_key
    }
  }
`;

const CREATE_CLIENT = gql`
  mutation Mutation($newClient: newClientInput!) {
    createClient(newClient: $newClient) {
      id
    }
  }
`;

const NewClient = ({ setIsOpen }) => {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [ipAddress, setIpAddress] = useState("");
  const [keepalive, setKeepalive] = useState(24);
  const [vpnNetwork, setVpnNetwork] = useState("");
  const [keypair, setKeypair] = useState("");
  const [dnsServer, setDnsServer] = useState("");

  // Mutation
  const [createClient] = useMutation(CREATE_CLIENT, {
    refetchQueries: [{ query: GET_CLIENTS }, { query: GET_REFERENCES }],
  });

  const { data, loading, error } = useQuery(GET_REFERENCES);
  if (error) {
    toast.error("Could not fetch data from API: " + error.message, {
      toastId: "query-error",
    });
  }

  useEffect(() => {
    if (data) {
      if (data.dnsServers) {
        setDnsServer(data.dnsServers[0].id);
      }
      if (data.vpnNetworks) {
        setVpnNetwork(data.vpnNetworks[0].id);
      }
      if (data.unusedKeypairs) {
        setKeypair(data.unusedKeypairs[0].id);
      }
    }
  }, [data]);

  const handleSubmit = async (e) => {
    e.preventDefault();
    await createClient({
      variables: {
        newClient: {
          name,
          description,
          ip_address: ipAddress,
          keepalive_interval: keepalive,
          vpn_network: vpnNetwork,
          keypair,
          dns_server: dnsServer,
        },
      },
    })
      .then(() => {
        toast.success("Client created successfully!", {
          toastId: "new-client-success",
        });
        setIsOpen(false);
      })
      .catch((err) => {
        toast.error("Could not create client: " + err.message, {
          toastId: "new-client-error",
        });
      });
  };

  if (loading) {
    return (
      <Loader
        type="Puff"
        color="#00BFFF"
        height={100}
        width={100}
        timeout={3000}
      />
    );
  }

  if (!data) {
    return null;
  }

  return (
    <Modal setIsOpen={setIsOpen} heading="Create new Client">
      <form className="space-y-4" onSubmit={handleSubmit}>
        <FormInputField
          labelName="Name"
          value={name}
          setValue={setName}
          type="text"
          placeholder="Name"
          autoFocus={true}
        />

        <FormInputField
          labelName="Description"
          value={description}
          setValue={setDescription}
          type="text"
          placeholder="Description"
        />

        <FormInputField
          labelName="IP-Address"
          value={ipAddress}
          setValue={setIpAddress}
          type="text"
          placeholder="e.g. 192.168.178.2"
        />

        <FormInputField
          labelName="Keepalive Interval"
          value={keepalive}
          setValue={setKeepalive}
          type="number"
          placeholder="30"
        />

        <SelectInputField
          labelName="DNS Server"
          mainField="name"
          options={data.dnsServers}
          secondField="ip_address"
          value={dnsServer}
          setValue={setDnsServer}
        />

        <SelectInputField
          labelName="VPN Network"
          mainField="name"
          options={data.vpnNetworks}
          secondField="ip_address"
          value={vpnNetwork}
          setValue={setVpnNetwork}
        />

        <SelectInputField
          labelName="Keypair"
          mainField="public_key"
          options={data.unusedKeypairs}
          value={keypair}
          setValue={setKeypair}
        />

        <SubmitButton>Create Client</SubmitButton>
      </form>
    </Modal>
  );
};

// Prop types validation
NewClient.propTypes = {
  setIsOpen: PropTypes.func.isRequired,
};

export default NewClient;
