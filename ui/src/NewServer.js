import React, { useState, useEffect } from "react";
import PropTypes from "prop-types";
import { gql, useQuery, useMutation } from "@apollo/client";
import Loader from "react-loader-spinner";
import { toast } from "react-toastify";

import Modal from "./components/Modal";
import FormInputField from "./components/FormInputField";
import SubmitButton from "./components/SubmitButton";
import SelectInputField from "./components/SelectInputField";
import { GET_SERVERS } from "./ServerList";

const GET_REFERENCES = gql`
  query GetReferences {
    vpnNetworks {
      id
      name
      ipNetwork
    }
    unusedKeypairs {
      id
      publicKey
    }
  }
`;

const CREATE_SERVER = gql`
  mutation CreateServer($newServer: InputServer!) {
    createServer(server: $newServer) {
      id
    }
  }
`;

const NewServer = ({ setIsOpen }) => {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [ipAddress, setIpAddress] = useState("");
  const [forwardInterface, setForwardInterface] = useState("");
  const [vpnNetwork, setVpnNetwork] = useState(null);
  const [keypair, setKeypair] = useState(null);
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

  const handleSubmit = async (e) => {
    e.preventDefault();

    await createServer({
      variables: {
        newServer: {
          name,
          description,
          externalIpAddress: externalIpAddress,
          ipAddress: ipAddress,
          forwardInterface: forwardInterface,
          vpnNetworkId: vpnNetwork,
          keypairId: keypair,
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
  if (error) {
    toast.error("Could not fetch data from API: " + error.message, {
      toastId: "query-error",
    });
    return null;
  }

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
          secondField="ipNetwork"
          value={vpnNetwork}
          setValue={setVpnNetwork}
        />

        <SelectInputField
          labelName="Keypair"
          mainField="publicKey"
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
