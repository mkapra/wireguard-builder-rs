import React, { useState } from "react";
import PropTypes from "prop-types";
import { gql, useMutation } from "@apollo/client";
import { toast } from "react-toastify";

import { GET_VPN_NETWORKS } from "./VpnNetworkList";
import FormInputField from "./FormInputField";
import Modal from "./Modal";
import SubmitButton from "./SubmitButton";

const CREATE_VPN_NETWORK = gql`
  mutation Mutation($newVpnNetwork: InputVpnNetwork!) {
    createVpnNetwork(vpnNetwork: $newVpnNetwork) {
      id
    }
  }
`;

const NewVpnNetwork = ({ setIsOpen }) => {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [ipAddress, setIpAddress] = useState("");
  const [subnetmask, setSubnetmask] = useState("");
  const [port, setPort] = useState("");
  const [wgInterface, setWgInterface] = useState("");

  const [createVpnNetwork] = useMutation(CREATE_VPN_NETWORK, {
    refetchQueries: [{ query: GET_VPN_NETWORKS }],
  });

  const handleSubmit = async (e) => {
    e.preventDefault();

    let variables = {
      name,
      description,
      ipNetwork: ipAddress,
      listenPort: parseInt(port),
      interfaceName: wgInterface,
    };
    if (subnetmask !== undefined && subnetmask !== "" && subnetmask !== null) {
      variables.subnetmask = parseInt(subnetmask);
    }

    console.log("SDFSDFSDF", variables);

    await createVpnNetwork({
      variables: {
        newVpnNetwork: {
          ...variables,
        },
      },
    })
      .then(() => {
        toast.success("VPN Network created successfully!", {
          toastId: "new-vpn-network-success",
        });
        setIsOpen(false);
      })
      .catch((err) => {
        toast.error(
          `There was an error while creating the VPN Network: ${err.message}`,
          { toastId: "new-vpn-network-error" }
        );
      });
  };

  return (
    <Modal setIsOpen={setIsOpen} heading="Create new VPN Network">
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
          type="textarea"
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
          labelName="Subnetmask (default: 24)"
          value={subnetmask}
          setValue={setSubnetmask}
          type="number"
          placeholder="24"
        />

        <FormInputField
          labelName="Port"
          value={port}
          setValue={setPort}
          type="number"
          placeholder="e.g. 51820"
        />

        <FormInputField
          labelName="Interface"
          value={wgInterface}
          setValue={setWgInterface}
          type="text"
          placeholder="e.g. wg0"
        />

        <SubmitButton>Create VPN Network</SubmitButton>
      </form>
    </Modal>
  );
};

// Prop types validation
NewVpnNetwork.propTypes = {
  setIsOpen: PropTypes.func.isRequired,
};

export default NewVpnNetwork;
