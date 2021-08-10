import { useState, useContext } from "react";
import cx from "classnames";
import * as yup from "yup";
import { Form, Formik } from "formik";
import * as api from "api.js";
import { Field } from "components/service";
import { Input } from "components/form";
import { context as userContext } from "context/user";

export default function SignIn() {
  const [error, setError] = useState(null);
  const { signIn } = useContext(userContext);

  return (
    <div className="min-h-screen flex items-center justify-center">
      <div className="max-w-md w-full p-4">
        <h1 className="font-semibold text-xl mb-4 text-gray-700">Sign in</h1>
        {error && (
          <div className="rounded p-3 mb-3 text-xs border border-red-500 bg-red-100 text-red-900">
            {error}
          </div>
        )}
        <Formik
          validationSchema={validationSchema}
          initialValues={{ username: "", password: "" }}
          onSubmit={async (data) => {
            try {
              await api.signIn(data.username, data.password);

              signIn();
            } catch (err) {
              if (err.response?.status === 401) {
                setError("Either username or password is invalid");
              }

              throw err;
            }
          }}
        >
          {({ isSubmitting }) => (
            <Form>
              <Field
                placeholder="Username"
                name="username"
                type="username"
                component={Input}
                className="mb-3"
              />
              <Field
                placeholder="Password"
                name="password"
                type="password"
                component={Input}
              />
              <button
                type="submit"
                disabled={isSubmitting}
                className={cx(
                  "rounded-sm w-full py-2 text-sm mt-5 block outline-none",
                  isSubmitting
                    ? "bg-gray-200 text-gray-800 cursor-not-allowed"
                    : "bg-blue-500 hover:bg-blue-600 text-white"
                )}
              >
                Sign in
              </button>
            </Form>
          )}
        </Formik>
      </div>
    </div>
  );
}

const requiredText = "This field is required";

const validationSchema = yup.object().shape({
  username: yup.string().required(requiredText),
  password: yup.string().required(requiredText),
});
