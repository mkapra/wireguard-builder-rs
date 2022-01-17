import React, { useState } from "react";
import { useMutation, gql } from "@apollo/client";

import Logo from "./components/Logo";
import FormInputField from "./components/FormInputField";
import SubmitButton from "./components/SubmitButton";
import { useNavigate } from "react-router-dom";
import { setAccessToken } from "./helpers/accessToken";

const LOGIN = gql`
  mutation Mutation($username: String!, $password: String!) {
    login(username: $username, password: $password)
  }
`;

const Login = () => {
  const navigate = useNavigate();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const [login, { loading, error }] = useMutation(LOGIN);

  if (loading) {
    return null;
  }

  return (
    <div className="w-screen h-screen overflow-hidden bg-gradient-to-tl from-blue-700 to-blue-400 flex justify-center items-center">
      <div className="flex flex-col items-center w-1/3 bg-opacity-60 bg-gray-200 rounded-lg shadow-lg p-8 space-y-14">
        <Logo tailwindStyles="text-gray-700" />

        {error && (
          <div className="bg-red-600 bg-opacity-75 py-10 px-5 w-full rounded text-gray-800">
            An error occured while logging in. Maybe your username or password
            is wrong?
          </div>
        )}
        <form
          className="space-y-4 w-full"
          onSubmit={async (e) => {
            e.preventDefault();
            const response = await login({
              variables: {
                username,
                password,
              },
            });
            if (response && response.data) {
              setAccessToken(response.data.login);
            }

            navigate("/");
          }}
        >
          <FormInputField
            labelName="Username"
            value={username}
            setValue={setUsername}
            type="text"
            autoFocus={true}
            placeholder="Username"
            noLabel={true}
          />

          <FormInputField
            labelName="Password"
            value={password}
            setValue={setPassword}
            type="password"
            placeholder="Password"
            noLabel={true}
          />

          <SubmitButton>Login</SubmitButton>
        </form>
      </div>
    </div>
  );
};

export default Login;
