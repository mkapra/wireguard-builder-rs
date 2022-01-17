import { NavLink } from "react-router-dom";
import React from "react";
import PropTypes from "prop-types";

const NavbarLink = ({ to, children }) => {
  return (
    <NavLink
      to={to}
      className={({ isActive }) => {
        return (
          "flex space-x-2 py-2 px-4 transition delay-75 rounded-lg hover:bg-orange-400 hover:bg-opacity-80 hover:text-gray-800" +
          (isActive ? " bg-orange-400 text-gray-800" : "")
        );
      }}
    >
      {children}
    </NavLink>
  );
};

// Props validation
NavbarLink.propTypes = {
  to: PropTypes.string.isRequired,
  children: PropTypes.node.isRequired,
};

export default NavbarLink;
