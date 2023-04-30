import clsx from "clsx"
import { FolderPlusIcon, FolderOpenIcon, PencilSquareIcon } from "@heroicons/react/20/solid"
import { version } from '../../../package.json';

const Welcome = () => {
    return (
        <div className={"bg-myotube h-full w-full bg-contain bg-no-repeat bg-right"}>
            <div data-tauri-drag-region className="h-10 w-full bg-gradient-to-r from-dark-blue/40 to-white/5 backdrop-blur-md flex items-center p-4 select-none cursor-default">
                <span data-tauri-drag-region>MyoVision</span>
            </div>
            <div className="grid grid-cols-4 gap-4 py-6 px-4">
                <Item icon={<FolderPlusIcon />}>Create a new Project</Item>
                <Item icon={<FolderOpenIcon />}>Open Project</Item>
            </div>
            <span className="fixed bottom-0 text-[0.6rem] right-0 m-1 text-neutral-200 font-extralight">MyoVision v{version}</span>
        </div>
    )
}

const Item = ({ children, className, icon, ...props }: React.ComponentPropsWithoutRef<"div"> & { icon?: React.ReactNode }) => {
    return (
        <div className={clsx("flex justify-between items-center px-2 py-2 text-sm font-medium rounded-md col-start-1 col-span-2 cursor-pointer select-none", "bg-white/5 backdrop-blur-md h-12 shadow-md transition-all duration-100", "hover:bg-lime-400/20 hover:shadow-lg", className)} {...props}>
            {children}{icon && <div className="w-6 h-6">{icon}</div>}
        </div>
    )
}

export default Welcome
