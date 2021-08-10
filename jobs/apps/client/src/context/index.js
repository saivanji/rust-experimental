import * as user from "./user";

export default function Context({ children }) {
  return (
      <user.Provider>
        {children}
      </user.Provider>
  );
}
