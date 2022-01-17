import React from "react";
import PropTypes from "prop-types";

const Error = ({ error }) => {
  return (
    <div className="flex w-full h-full items-center justify-center">
      <div className="w-full md:w-1/2 bg-red-500 p-8">
        <h3 className="text-gray-100 text-2xl text-center font-bold">Error</h3>
        <div className="space-y-4">
          <p className="text-gray-100 text-2xl text-center">
            An error occured while processing the data. If the problem persists
            please contact the administrator.
          </p>
          <div className="text-gray-200 text-sm flex space-x-2">
            <span className="font-extrabold uppercase">Error:</span>{" "}
            <p>{error.message}</p>
          </div>
        </div>
      </div>
    </div>
  );
};

// Props validation
Error.propTypes = {
  error: PropTypes.object.isRequired,
};

export default Error;
