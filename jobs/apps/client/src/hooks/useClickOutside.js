import { useEffect } from "react";

export default function ClickOutside(...args) {
  const [ref, scopeRef, onClickOutside] =
    args.length === 3 ? args : [args[0], undefined, args[1]];

  const handleClick = (event) => {
    if (ref.current && !ref.current.contains(event.target)) {
      onClickOutside();
    }
  };

  useEffect(() => {
    const scope = scopeRef?.current || document;

    scope.addEventListener("mousedown", handleClick);

    return () => {
      scope.removeEventListener("mousedown", handleClick);
    };
  });
}
