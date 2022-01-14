import React, { useState } from "react";
import { gql, useMutation } from "@apollo/client";
import { toast } from "react-toastify";
import PropTypes from "prop-types";

import { GET_DNS_SERVERS } from "./DnsServerList";
import Modal from "./Modal";
import FormInputField from "./FormInputField";
import SubmitButton from "./SubmitButton";

const CREATE_DNS_SERVER = gql`
  mutation CreateDnsServer($name: String!, $ip: String!, $description: String) {
    createDnsServer(name: $name, ip: $ip, description: $description) {
      id
    }
  }
`;

const NewDnsServer = ({ setIsOpen }) => {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [ipAddress, setIpAddress] = useState("");

  const [createDnsServer] = useMutation(CREATE_DNS_SERVER, {
    refetchQueries: [{ query: GET_DNS_SERVERS }],
  });

  const handleSubmit = async (e) => {
    e.preventDefault();

    await createDnsServer({
      variables: {
        name,
        description,
        ip: ipAddress,
      },
    })
      .catch((err) => {
        toast.error(err.message, { toastId: "new-dns-server-error" });
      })
      .then(() => {
        toast.success("DNS Server created successfully!", {
          toastId: "new-dns-server-success",
        });
        setIsOpen(false);
      });
  };

  return (
    <Modal setIsOpen={setIsOpen} heading="Create new DNS server">
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

        <SubmitButton>Create DNS Server</SubmitButton>
      </form>
    </Modal>
  );
};

// Props validation
NewDnsServer.propTypes = {
  setIsOpen: PropTypes.func.isRequired,
};

export default NewDnsServer;
