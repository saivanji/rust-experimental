import { useContext } from "react";
import { Redirect, Route } from "react-router-dom";
import { context as userContext } from "context/user";
import * as paths from "paths.js";

export default function Page({ path, guard, component }) {
  return (
    <Route exact path={path}>
      <Root guard={guard} component={component} />
    </Route>
  );
}

function Root({ guard, component: Component }) {
  const { isAuthenticated } = useContext(userContext);

  if (!guard(isAuthenticated)) {
    return <Redirect to={redirectTo(isAuthenticated)} />;
  }

  return <Component />;
}

const redirectTo = (isAuthenticated) => {
  if (!isAuthenticated) {
    return paths.signIn;
  }

  return paths.root;
};
