import React, { useState, useEffect } from "react";
import PropTypes from "prop-types";
import { gql, useQuery, useMutation } from "@apollo/client";
import Loader from "react-loader-spinner";
import { toast } from "react-toastify";

import Modal from "./Modal";
import FormInputField from "./FormInputField";
import SubmitButton from "./SubmitButton";
import SelectInputField from "./SelectInputField";
import { GET_SERVERS } from "./ServerList";

const GET_REFERENCES = gql`
  query GetReferences {
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

const CREATE_SERVER = gql`
  mutation CreateServer($newServer: newServerInput!) {
    createServer(newServer: $newServer) {
      id
    }
  }
`;

const NewServer = ({ setIsOpen }) => {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [ipAddress, setIpAddress] = useState("");
  const [forwardInterface, setForwardInterface] = useState("");
  const [vpnNetwork, setVpnNetwork] = useState("");
  const [keypair, setKeypair] = useState("");
  const [externalIpAddress, setExternalIpAddress] = useState("");

  const { data, loading, error } = useQuery(GET_REFERENCES);

  const [createServer] = useMutation(CREATE_SERVER, {
    refetchQueries: [{ query: GET_SERVERS }, { query: GET_REFERENCES }],
  });

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
  if (error) {
    toast.error("Could not fetch data from API: " + error.message, {
      toastId: "query-error",
    });
    return null;
  }

  const handleSubmit = async (e) => {
    e.preventDefault();

    await createServer({
      variables: {
        newServer: {
          name,
          description,
          external_ip_address: externalIpAddress,
          ip_address: ipAddress,
          forward_interface: forwardInterface,
          vpn_network: vpnNetwork,
          keypair,
        },
      },
    })
      .then(() => {
        toast.success("Server created successfully", {
          toastId: "new-server-success",
        });
        setIsOpen(false);
      })
      .catch((err) => {
        toast.error("Could not create server: " + err.message, {
          toastId: "new-server-error",
        });
      });
  };

  useEffect(() => {
    if (data) {
      if (data.vpnNetworks) {
        setVpnNetwork(data.vpnNetworks[0].id);
      }
      if (data.unusedKeypairs) {
        setKeypair(data.unusedKeypairs[0].id);
      }
    }
  }, [data]);

  return (
    <Modal setIsOpen={setIsOpen} heading="Create new Server">
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
          labelName="Forward Interface"
          value={forwardInterface}
          setValue={setForwardInterface}
          type="text"
          placeholder="e.g. eth0"
        />

        <FormInputField
          labelName="External IP-Address or DNS-Name"
          value={externalIpAddress}
          setValue={setExternalIpAddress}
          type="text"
          placeholder="e.g. vpn.example.com, 123.123.123.123"
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
        <SubmitButton>Create Server</SubmitButton>
      </form>
    </Modal>
  );
};

// Prop types validation
NewServer.propTypes = {
  setIsOpen: PropTypes.func.isRequired,
};

export default NewServer;
