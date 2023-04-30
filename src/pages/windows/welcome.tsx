import clsx from "clsx"
import { FolderPlusIcon, FolderOpenIcon, ClockIcon } from "@heroicons/react/24/outline"
import packageInfo from '../../../package.json';

const { version, author, displayName, filetypeAssociation } = packageInfo;

const Welcome = () => {
    return (
        <div className={"bg-myotube h-full w-full bg-contain bg-no-repeat bg-right"}>
            <div data-tauri-drag-region className="h-10 w-full bg-gradient-to-r from-dark-blue/40 to-white/5 backdrop-blur-md flex items-center p-4 select-none cursor-default">
                <span data-tauri-drag-region>{displayName}</span>
            </div>
            <div className="grid grid-cols-5 gap-1 py-4">
                <Item icon={<FolderPlusIcon />} onClick={handle_create_project}>Create a new Project</Item>
                <Item icon={<FolderOpenIcon />} onClick={handle_open_project}>Open Project</Item>
                <Item icon={<ClockIcon />} disabled>Open Recent</Item>

            </div>
            <span className="fixed bottom-0 text-[0.54rem] right-0 m-1 text-neutral-200 font-extralight">{displayName} v{version} - {author}</span>
        </div>
    )
}

const Item = ({ children, className, icon, disabled, ...props }: React.ComponentPropsWithoutRef<"div"> & { icon?: React.ReactNode, disabled?: boolean }) => {
    return (
        <div className={clsx("flex gap-4 items-center px-6 text-sm font-medium  col-start-1 col-span-3 select-none h-16", "bg-gradient-to-r from-transparent to-transparent h-12 transition-colors duration-100", !disabled && "hover:from-lime-400/20 hover:text-lime-400 cursor-pointer", disabled && " brightness-50 cursor-not-allowed", className)} {...props}>
            {icon && <div className="w-5 h-5">{icon}</div>}{children}
        </div>
    )
}

const handle_create_project = async () => {
    const dialog = await import("@tauri-apps/api/dialog");

    const result = await dialog.save({
        filters: [{
            name: `${filetypeAssociation.name} Project`,
            extensions: filetypeAssociation.extensions
        }]
    })

    if (result) {
        console.log(result);
    }
}

const handle_open_project = async () => {
    const dialog = await import("@tauri-apps/api/dialog");

    const result = await dialog.open({
        filters: [{
            name: `${filetypeAssociation.name} Project`,
            extensions: filetypeAssociation.extensions
        }]
    })

    if (result) {
        console.log(result);
    }
}

export default Welcome
