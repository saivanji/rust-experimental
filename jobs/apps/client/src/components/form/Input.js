import cx from "classnames";
import Error from "./Error";

export default function Input({ error, className, ...props }) {
  return (
    <>
      <div className={cx(className, "relative w-full")}>
        <input
          {...props}
          className={cx(
            "h-12 pr-4 appearance-none relative block w-full border-b text-gray-900 placeholder-gray-500 focus:outline-none focus:ring-green-50 focus:z-10 text-sm leading-5",
            error ? "border-red-400" : "border-gray-300 focus:border-green-400"
          )}
        />
        {error && <Error>{error}</Error>}
      </div>
    </>
  );
}
