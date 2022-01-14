import React from "react";
import PropTypes from "prop-types";

const Searchbar = ({ placeholder, search, setSearch }) => {
  return (
    <div>
      <input
        className="p-2 border w-full rounded"
        type="text"
        placeholder={placeholder}
        value={search}
        onChange={(e) => setSearch(e.target.value)}
      />
    </div>
  );
};

// Default Prop for placeholder
Searchbar.defaultProps = {
  placeholder: "Search ...",
};

// Prop validation
Searchbar.propTypes = {
  placeholder: PropTypes.string,
  search: PropTypes.string.isRequired,
  setSearch: PropTypes.func.isRequired,
};

export default Searchbar;
