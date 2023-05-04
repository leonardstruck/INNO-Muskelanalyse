import clsx from "clsx"
import { FolderPlusIcon, FolderOpenIcon, ClockIcon } from "@heroicons/react/24/outline"
import packageInfo from '../../package.json';
import { XMarkIcon } from "@heroicons/react/20/solid";
import { invoke } from "@tauri-apps/api/tauri";

const { version, author, displayName, filetypeAssociation, homepage } = packageInfo;

const Welcome = () => {
    return (
        <div className={"bg-myotube h-full w-full bg-contain bg-no-repeat bg-right"}>
            <div data-tauri-drag-region className="h-12 px-6 w-full bg-gradient-to-r from-dark-blue/40 to-white/5 backdrop-blur-md flex items-center select-none cursor-default text-xl text-cyan-500">
                <span data-tauri-drag-region>{displayName}</span>
            </div>
            <div className="grid grid-cols-5 gap-1 py-4">
                <Item icon={<FolderPlusIcon />} onClick={handle_create_project}>Create a new Project</Item>
                <Item icon={<FolderOpenIcon />} onClick={handle_open_project}>Open Project</Item>
                <Item icon={<ClockIcon />} disabled>Open Recent</Item>
                <Item icon={<XMarkIcon />} onClick={handle_close_window}>Close</Item>
            </div>
            <span className="fixed bottom-0 text-[0.54rem] right-0 m-1 text-cyan-500 font-extralight">{displayName} v{version} - <a href={homepage} target="_blank">{author}</a></span>
        </div>
    )
}

const Item = ({ children, className, icon, disabled, ...props }: React.ComponentPropsWithoutRef<"div"> & { icon?: React.ReactNode, disabled?: boolean }) => {
    return (
        <div className={clsx("flex gap-4 items-center px-6 text-sm font-medium  col-start-1 col-span-3 select-none h-16", "h-12 transition-colors duration-100 bg-gradient-to-r from-transparent", !disabled && "hover:backdrop-brightness-150 hover:to-black/30 hover:text-lime-400 cursor-pointer", disabled && " brightness-50 cursor-not-allowed", className)} {...props}>
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
        await invoke("open_project", { path: result }).catch(err => {
            dialog.message(err, { type: "error", title: "Something went wrong" })
        })
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
        await invoke("open_project", { path: result }).catch(err => {
            dialog.message(err, { type: "error", title: "Something went wrong" })
        })
    }
}

const handle_close_window = async () => {
    const window = await import("@tauri-apps/api/window");

    const current = window.getCurrent();

    if (current) {
        current.close();
    }
}

export default Welcome
