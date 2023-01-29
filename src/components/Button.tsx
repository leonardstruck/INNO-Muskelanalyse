import clsx from "clsx";
import Spinner from "./Spinner";

type ButtonProps = {
    children: React.ReactNode;
    onClick?: () => void;
    className?: string;
    disabled?: boolean;
    loading?: boolean;
    theme?: "primary" | "secondary";
}

const Button = (props: ButtonProps) => {
    return (
        <button
            onClick={props.onClick}
            className={clsx(
                props.className,

                "rounded-md border py-2 px-4 text-sm font-medium shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 inline-flex justify-center",

                props.theme == "secondary" && clsx(
                    "border-gray-300 bg-white text-gray-700",
                    !props.disabled && "hover:bg-gray-50 focus:ring-indigo-500"
                ),

                props.theme == "primary" && clsx(
                    "border-transparent bg-indigo-600 text-white",
                    !props.disabled && "hover:bg-indigo-700 focus:ring-indigo-500"
                ),

                props.disabled && "opacity-80 cursor-not-allowed"
            )}
            disabled={props.disabled}
            type="button"
        >
            {props.loading ? <><Spinner className="w-4 h-4 mr-2" />{props.children}</> : props.children}
        </button>
    );
}

export default Button;