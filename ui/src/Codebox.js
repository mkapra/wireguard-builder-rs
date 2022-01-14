import React from "react";
import PropTypes from "prop-types";
import { CopyToClipboard } from "react-copy-to-clipboard";
import { toast } from "react-toastify";

const Codebox = ({ value }) => {
  return (
    <>
      <CopyToClipboard
        text={value}
        onCopy={() => {
          toast.success("Copied to clipboard!", {
            toastId: "copy-to-clipboard",
          });
        }}
      >
        <div className="bg-gray-200 p-4 whitespace-pre-line relative">
          <p className="font-mono">{value}</p>
          <p className="absolute right-0 bottom-0 p-2 bg-gray-300 rounded-tl">
            Click in box to copy the configuration
          </p>
        </div>
      </CopyToClipboard>
    </>
  );
};

// Prop types validation
Codebox.propTypes = {
  value: PropTypes.string.isRequired,
};

export default Codebox;
