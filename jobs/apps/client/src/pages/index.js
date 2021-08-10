import { BrowserRouter, Route } from "react-router-dom";
import { QueryParamProvider } from "use-query-params";
import { Page } from "components/service";
import * as guards from "guards.js";
import * as paths from "paths.js";

import SignIn from "./SignIn";
import Jobs from "./Jobs";

export default function Router() {
  return (
    <BrowserRouter>
      <QueryParamProvider ReactRouterRoute={Route}>
        <Page
          guard={guards.unauthenticated}
          path={paths.signIn}
          component={SignIn}
        />
        <Page guard={guards.authenticated} path={paths.root} component={Jobs} />
      </QueryParamProvider>
    </BrowserRouter>
  );
}
