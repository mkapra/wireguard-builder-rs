import React from "react";
import { Link } from "react-router-dom";
import { PropTypes } from "prop-types";

const Table = ({ headings, data, onDelete, onEdit, onView }) => {
  const hasAction = onDelete || onEdit || onView;

  // Loop over data and create a row for each item
  return (
    <table className="border border-collapse w-full">
      <thead>
        <tr>
          {headings.map((heading) => (
            <th className="border border-collapse" key={heading}>
              {heading}
            </th>
          ))}
          {hasAction && <th className="border border-collapse">Actions</th>}
        </tr>
      </thead>
      <tbody>
        {data.map((row) => (
          <tr key={row.id}>
            {/* iterate over object to get the values as table data */}
            {Object.entries(row).map(
              ([key, value], index) =>
                // skip value if value starts with __
                !key.startsWith("_") &&
                (typeof value === "object" && value !== null ? (
                  <td
                    className="border border-collapse px-2 py-1 text-center"
                    key={index}
                  >
                    <Link
                      className="text-blue-500 hover:text-blue-800"
                      to={`/${key}s`} // TODO: Add ID back to url if a single preview of an object works ${value.id}
                    >
                      {value.id}
                    </Link>
                  </td>
                ) : (
                  <td
                    className="border border-collapse px-2 py-1 text-center"
                    key={index}
                  >
                    {value}
                  </td>
                ))
            )}

            {hasAction && (
              <td className="border border-collapse px-2 py-1 text-center space-x-1">
                {onDelete && (
                  <button
                    title="Delete"
                    onClick={() => onDelete(row.id)}
                    className="bg-red-500 hover:bg-red-700 text-white font-bold p-1 rounded"
                  >
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      className="h-5 w-5"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth="2"
                        d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                      />
                    </svg>
                  </button>
                )}
                {onEdit && (
                  <button
                    title="Edit"
                    onClick={() => onEdit(row.id)}
                    className="bg-blue-500 hover:bg-blue-700 text-white font-bold p-1 rounded"
                  >
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      className="h-5 w-5"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        o
                        strokeWidth="2"
                        d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                      />
                    </svg>
                  </button>
                )}
                {onView && (
                  <button
                    title="View Detail"
                    onClick={() => onView(row.id)}
                    className="bg-green-600 hover:bg-green-700 text-white font-bold p-1 rounded"
                  >
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      className="h-5 w-5"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth="2"
                        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                      />
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth="2"
                        d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                      />
                    </svg>
                  </button>
                )}
              </td>
            )}
          </tr>
        ))}
      </tbody>
    </table>
  );
};

// Props validation
Table.propTypes = {
  headings: PropTypes.array.isRequired,
  data: PropTypes.array.isRequired,
  onDelete: PropTypes.func,
  onEdit: PropTypes.func,
  onView: PropTypes.func,
};

export default Table;
