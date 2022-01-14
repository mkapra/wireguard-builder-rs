import React from "react";
import PropTypes from "prop-types";

const SubmitButton = ({ children }) => {
  return (
    <input
      type="submit"
      className="float-right bg-blue-500 items-center justify-center py-2 px-4 rounded-full text-blue-100 hover:bg-orange-400 hover:text-gray-800 transition delay-75"
      value={children}
    />
  );
};

// Props type validation
SubmitButton.propTypes = {
  children: PropTypes.node.isRequired,
};

export default SubmitButton;
