import React from "react";
import { gql, useQuery } from "@apollo/client";
import PropTypes from "prop-types";
import { toast } from "react-toastify";

import Modal from "./components/Modal";
import Loading from "./components/Loading";
import ConfigurationViewer from "./components/ConfigurationViewer";

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

  if (loading) return <Loading />;

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
