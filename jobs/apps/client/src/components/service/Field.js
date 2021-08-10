import { Field, getIn } from "formik";

export default function CustomField({
  name,
  component: Component,
  map = (x) => x,
  ...props
}) {
  return (
    <>
      <Field name={name}>
        {({ field, form: { errors, touched, submitCount } }) => {
          const error = getIn(errors, field.name);
          const isTouched = !!getIn(touched, field.name);
          const isFormSubmitted = submitCount > 0;

          return (
            <Component
              {...field}
              {...props}
              onChange={(e) => {
                field.onChange({
                  target: { name: e.target.name, value: map(e.target.value) },
                });
              }}
              error={(isTouched || isFormSubmitted) && error}
            />
          );
        }}
      </Field>
    </>
  );
}
