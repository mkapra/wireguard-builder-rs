import React, { useEffect } from "react";
import PropTypes from "prop-types";

import Button from "./Button";

const Modal = ({ setIsOpen, children, heading }) => {
  // close modal if user clicks outside of modal
  const handleClick = (target) => {
    console.log("Closing modal by click is currently disabled", target);
    // if (!target.id && target.type !== "submit") setIsOpen(false);
  };

  // setIsOpen(false) if escape button is pressed
  const handleEscape = (e) => {
    if (e.keyCode === 27) {
      setIsOpen(false);
    }
  };

  useEffect(() => {
    document.addEventListener("keydown", handleEscape);

    return () => {
      document.removeEventListener("keydown", handleEscape);
    };
  }, []);

  return (
    <div
      onClick={(e) => handleClick(e.target)}
      className="absolute top-0 left-0 w-screen h-screen bg-black bg-opacity-40 flex items-center justify-center"
    >
      <div
        id="modal"
        className="w-1/2 bg-gray-100 shadow-lg p-6 space-y-4 rounded-lg"
      >
        <div className="flex justify-between">
          <h2 className="text-2xl font-bold">{heading}</h2>

          <Button onClick={() => setIsOpen(false)}>
            <div className="flex space-x-2">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="h-8 w-8"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth="2"
                  d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
            </div>
          </Button>
        </div>
        {children}
      </div>
    </div>
  );
};

// Prop types validation
Modal.propTypes = {
  setIsOpen: PropTypes.func.isRequired,
  children: PropTypes.node.isRequired,
  // string or object
  heading: PropTypes.oneOfType([PropTypes.string, PropTypes.object]),
};

export default Modal;
