import React from "react";
import PropTypes from "prop-types";

const Button = ({ children, onClick }) => {
  return (
    <button
      onClick={onClick}
      className="bg-blue-500 py-2 px-4 rounded-lg text-blue-100 hover:bg-orange-400 hover:text-gray-800 transition delay-75"
    >
      {children}
    </button>
  );
};

// Props validation
Button.propTypes = {
  children: PropTypes.node.isRequired,
  onClick: PropTypes.func.isRequired,
};

export default Button;
