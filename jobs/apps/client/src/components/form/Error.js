export default function Error({ children, ...props }) {
  return (
    <div className="text-red-500 text-sm mt-2" {...props}>
      {children}
    </div>
  );
}
