import { Outlet, Navigate } from "react-router-dom";
import React from "react";

import Navbar from "./Navbar";
import { getAccessToken } from "./accessToken";
import { toast } from "react-toastify";

function App() {
  if (!getAccessToken()) {
    toast.error("You must be logged in to view this page.", {
      toastId: "auth-error",
    });
    return <Navigate to="/login" />;
  }
  return (
    <div className="App flex overflow-hidden h-screen">
      <Navbar />
      <div className="p-10 flex-1 overflow-y-auto max-w-7xl mx-auto">
        <Outlet />
      </div>
    </div>
  );
}

export default App;
