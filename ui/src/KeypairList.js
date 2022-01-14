import { useQuery, useMutation, gql } from "@apollo/client";
import { toast } from "react-toastify";
import React from "react";

import Table from "./Table";
import Error from "./Error";
import Button from "./Button";

const GET_KEYPAIRS = gql`
  query Query {
    keypairs {
      id
      publicKey
    }
  }
`;

const GENERATE_KEYPAIR = gql`
  mutation Mutation {
    generateKeypair {
      id
    }
  }
`;

const DELETE_KEYPAIR = gql`
  mutation Mutation($deleteKeypairId: ID!) {
    deleteKeypair(id: $deleteKeypairId)
  }
`;

const KeypairList = () => {
  const {
    loading: listLoading,
    error: listError,
    data: listData,
  } = useQuery(GET_KEYPAIRS);
  const [
    generateKeypair,
    {
      data: generateData,
      loading: generateLoading,
      error: generateError,
      reset: generateReset,
    },
  ] = useMutation(GENERATE_KEYPAIR, {
    refetchQueries: [{ query: GET_KEYPAIRS }],
  });
  const [deleteKeypair] = useMutation(DELETE_KEYPAIR, {
    variables: { deleteKeypairId: "" },
    refetchQueries: [{ query: GET_KEYPAIRS }],
  });

  if (listLoading) return <p>Loading...</p>;
  if (generateLoading) return <p>Generate Keypair...</p>;

  const successGeneratedKeypair = (id) => {
    toast.success(`Keypair with id ${id} created successfully`, {
      toastId: id,
    });
  };

  const handleDeleteKeypair = (id) => {
    deleteKeypair({
      variables: { deleteKeypairId: id },
      onCompleted: () => {
        toast.success(`Keypair with id ${id} deleted successfully`, {
          toastId: id,
        });
      },
      onError: (error) => {
        toast.error(`Error deleting keypair with id ${id}: ${error}`, {
          toastId: id,
        });
      },
    });
  };

  return (
    <>
      <h2 className="text-3xl mb-4">Keypairs</h2>
      {listError && <Error error={listError} />}
      {generateError && <Error error={generateError} />}
      {!listError && !generateError && (
        <div>
          {generateData &&
            generateReset &&
            successGeneratedKeypair(generateData.generateKeypair.id)}

          <div className="float-right mb-4">
            <Button onClick={() => generateKeypair()} className="float-right">
              <div className="flex space-x-2">
                <span>Generate Keypair</span>
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="h-6 w-6"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth="2"
                    d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
              </div>
            </Button>
          </div>

          <Table
            headings={["ID", "Public Key"]}
            data={listData.keypairs}
            onDelete={handleDeleteKeypair}
          />
        </div>
      )}
    </>
  );
};

export default KeypairList;
