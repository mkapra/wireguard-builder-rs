import React from "react";
import ReactDOM from "react-dom";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import {
  ApolloClient,
  InMemoryCache,
  ApolloProvider,
  createHttpLink,
} from "@apollo/client";
import { setContext } from "@apollo/client/link/context";

import { ToastContainer } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

import "./index.css";
import App from "./App";
import Home from "./Home";
import KeypairList from "./KeypairList";
import DnsServerList from "./DnsServerList";
import VpnNetworkList from "./VpnNetworkList";
import ServerList from "./ServerList";
import ClientList from "./ClientList";
import Login from "./Login";
import { getAccessToken } from "./accessToken";

const httpLink = createHttpLink({
  uri: process.env.REACT_APP_GRAPHQL_URL || "http://localhost:8000/",
});

const authLink = setContext((_, { headers }) => {
  const token = getAccessToken();
  console.log("TOKEN BY GET:", token);
  return {
    headers: {
      ...headers,
      token: token ? token : "",
    },
  };
});

const client = new ApolloClient({
  cache: new InMemoryCache(),
  link: authLink.concat(httpLink),
});

ReactDOM.render(
  <React.StrictMode>
    <ApolloProvider client={client}>
      <BrowserRouter>
        <Routes>
          <Route path="/login" element={<Login />} />
          <Route path="/" element={<App />}>
            <Route index element={<Home />} />
            <Route path="/keypairs" element={<KeypairList />} />
            <Route path="/dns_servers" element={<DnsServerList />} />
            <Route path="/vpn_networks" element={<VpnNetworkList />} />
            <Route path="/clients" element={<ClientList />} />
            <Route path="/servers" element={<ServerList />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </ApolloProvider>
    <ToastContainer />
  </React.StrictMode>,
  document.getElementById("root")
);
