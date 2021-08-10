import React from "react";
import ReactDOM from "react-dom";
import { QueryClient, QueryClientProvider } from "react-query";
import Context from "./context";
import Router from "./pages";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: false,
      refetchIntervalInBackground: false,
      refetchOnWindowFocus: false,
    },
  },
});

ReactDOM.render(
  <React.StrictMode>
    <Context>
      <QueryClientProvider client={queryClient}>
        <Router />
      </QueryClientProvider>
    </Context>
  </React.StrictMode>,
  document.getElementById("root")
);
