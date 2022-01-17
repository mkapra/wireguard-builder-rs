import React, { useState, useEffect } from "react";
import PropTypes from "prop-types";
import { gql, useQuery, useMutation } from "@apollo/client";
import { toast } from "react-toastify";

import { GET_CLIENTS } from "./ClientList";
import Modal from "./components/Modal";
import FormInputField from "./components/FormInputField";
import SubmitButton from "./components/SubmitButton";
import SelectInputField from "./components/SelectInputField";
import Loading from "./components/Loading";

const GET_REFERENCES = gql`
  query GetReferences {
    dnsServers {
      id
      name
      ipAddress
    }
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

const CREATE_CLIENT = gql`
  mutation Mutation($newClient: InputClient!) {
    createClient(client: $newClient) {
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
          ipAddress,
          keepaliveInterval: keepalive,
          keypairId: keypair,
          dnsServerId: dnsServer,
          vpnNetworkId: vpnNetwork,
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

  if (loading) return <Loading />;

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
          secondField="ipAddress"
          value={dnsServer}
          setValue={setDnsServer}
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
