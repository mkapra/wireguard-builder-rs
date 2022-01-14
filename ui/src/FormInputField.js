import React from "react";
import PropTypes from "prop-types";

const FormInputField = ({
  labelName,
  value,
  setValue,
  type,
  placeholder,
  autoFocus,
  noLabel,
}) => {
  return (
    <div className="flex flex-col space-y-1">
      {!noLabel && (
        <label htmlFor={labelName} className="text-gray-700">
          {labelName}
        </label>
      )}

      {type === "textarea" ? (
        <textarea
          id={labelName}
          className="shadow appearance-none border rounded w-full py-2
            px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline
            max-h-64"
          placeholder={placeholder}
          value={value}
          onChange={(e) => setValue(e.target.value)}
          autoFocus={autoFocus || false}
        />
      ) : (
        <input
          id={labelName}
          className="shadow appearance-none border rounded w-full py-2
            px-3 text-gray-700 leading-tight focus:outline-none
            focus:shadow-outline"
          placeholder={placeholder}
          type={type}
          value={value}
          onChange={(e) => setValue(e.target.value)}
          autoFocus={autoFocus || false}
        />
      )}
    </div>
  );
};

// Prop types validation
FormInputField.propTypes = {
  autoFocus: PropTypes.bool,
  labelName: PropTypes.string.isRequired,
  placeholder: PropTypes.string.isRequired,
  setValue: PropTypes.func.isRequired,
  type: PropTypes.string.isRequired,
  value: PropTypes.oneOfType([PropTypes.string, PropTypes.number]).isRequired,
  noLabel: PropTypes.bool,
};

export default FormInputField;
