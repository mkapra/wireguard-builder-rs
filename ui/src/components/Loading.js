import React from "react";
import { Oval } from "react-loader-spinner";

const Loading = () => {
  return (
    <div className="absolute top-0 left-0 overflow-hidden w-screen h-screen bg-black bg-opacity-40 flex items-center justify-center">
      <Oval color="#00BFFF" height={100} width={100} timeout={3000} />
    </div>
  );
};

export default Loading;
