import React from "react";
import { gql, useQuery } from "@apollo/client";
import PropTypes from "prop-types";
import Loader from "react-loader-spinner";
import { toast } from "react-toastify";

import Modal from "./Modal";
import ConfigurationViewer from "./ConfigurationViewer";

const GET_SERVER = gql`
  query Query($id: ID!) {
    server(serverId: $id) {
      name
      config
    }
  }
`;

const ServerDetail = ({ setIsOpen, serverId }) => {
  const { loading, error, data } = useQuery(GET_SERVER, {
    variables: { id: serverId },
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

  return (
    <Modal
      setIsOpen={setIsOpen}
      heading={
        <span>
          Detail for <span className="text-blue-500">{data.server.name}</span>
        </span>
      }
    >
      <ConfigurationViewer config={data.server.config} server={true} />
    </Modal>
  );
};

// Prop types validation
ServerDetail.propTypes = {
  setIsOpen: PropTypes.func.isRequired,
  serverId: PropTypes.string.isRequired,
};

export default ServerDetail;
