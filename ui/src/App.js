import { Outlet } from "react-router-dom";
import React from "react";

import Navbar from "./Navbar";

function App() {
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
