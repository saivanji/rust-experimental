import { createContext } from "react";
import { usePersistedState } from "hooks/index";

export const context = createContext({});

export const Provider = ({ children }) => {
  const [isAuthenticated, setAuthenticated] = usePersistedState(
    false,
    "is_authenticated"
  );

  return (
    <context.Provider
      value={{
        isAuthenticated,
        signIn: () => {
          setAuthenticated(true);
        },
        signOut: () => {
          setAuthenticated(false);
        },
      }}
    >
      {children}
    </context.Provider>
  );
};
