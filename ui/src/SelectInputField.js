import React from "react";
import PropTypes, { object } from "prop-types";

const SelectInputField = ({
  labelName,
  mainField,
  options,
  secondField,
  setValue,
}) => {
  return (
    <div className="flex flex-col space-y-1">
      <label htmlFor={labelName}>{labelName}</label>
      <select
        id={labelName}
        onChange={(e) =>
          setValue(e.target.options[e.target.selectedIndex].value)
        }
        className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
      >
        {options.map((obj, index) => {
          return (
            <option key={index} value={obj.id}>
              {obj[mainField]} {secondField && `(${obj[secondField]})`}
            </option>
          );
        })}
      </select>
    </div>
  );
};

// Prop types validation
SelectInputField.propTypes = {
  labelName: PropTypes.string.isRequired,
  mainField: PropTypes.string.isRequired,
  secondField: PropTypes.string,
  setValue: PropTypes.func.isRequired,
  value: PropTypes.string.isRequired,
  options: PropTypes.arrayOf(object).isRequired,
};

export default SelectInputField;
