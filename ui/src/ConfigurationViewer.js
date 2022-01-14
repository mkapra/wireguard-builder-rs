import React from "react";
import QRCode from "react-qr-code";
import parse from "html-react-parser";
import PropTypes from "prop-types";

import Codebox from "./Codebox";

const ConfigurationViewer = ({ config, server }) => {
  return (
    <>
      <h3 className="text-xl">Configuration</h3>
      <Codebox value={parse(config)} />

      {!server && (
        <>
          <p className="font-semibold">
            Or scan the QR code with the wireguard app:
          </p>
          <div className="flex justify-center">
            <QRCode value={parse(config)} />
          </div>
        </>
      )}
    </>
  );
};

// Prop types validation
ConfigurationViewer.propTypes = {
  config: PropTypes.string.isRequired,
  server: PropTypes.bool,
};

export default ConfigurationViewer;
