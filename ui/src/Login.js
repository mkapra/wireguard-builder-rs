import React, { useState } from "react";
import FormInputField from "./FormInputField";
import SubmitButton from "./SubmitButton";
import Logo from "./Logo";

const Login = () => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  return (
    <div className="w-screen h-screen overflow-hidden bg-gradient-to-tl from-blue-700 to-blue-400 flex justify-center items-center">
      <div className="flex flex-col items-center w-1/3 bg-opacity-60 bg-gray-200 rounded-lg shadow-lg p-8 space-y-14">
        <Logo tailwindStyles="text-gray-700" />
        <form className="space-y-4 w-full">
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
