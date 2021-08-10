import { useState, useEffect } from "react";

export default function PersistedState(initialState, key, forceState) {
  useEffect(() => {
    if (forceState) {
      localStorage.setItem(key, JSON.stringify(forceState));
    }
  }, [key, forceState]);

  const rawStateFromStorage = localStorage.getItem(key);
  const stateFromStorage =
    rawStateFromStorage && JSON.parse(rawStateFromStorage);

  const [value, setter] = useState(
    forceState || stateFromStorage || initialState
  );

  return [
    value,
    (data) => {
      localStorage.setItem(key, JSON.stringify(data));
      setter(data);
    },
  ];
}
