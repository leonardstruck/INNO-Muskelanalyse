import { Menu, Transition } from "@headlessui/react";
import { PortableMicrograph } from "../../../src-tauri/bindings/PortableMicrograph";
import { MoreVertical } from "lucide-react";
import { Fragment } from "react";
import clsx from "clsx";

type ListProps = {
    micrographs: PortableMicrograph[]
}

const List = ({ micrographs }: ListProps) => {
    console.log(micrographs)
    return (
        <div>
            <ul role="list" className="divide-y divide-gray-100">
                {micrographs.map((micrograph) => (
                    <ListItem key={micrograph.uuid} micrograph={micrograph} />
                ))}
            </ul>
        </div>
    )
}

const statuses = {
    Done: 'text-green-700 bg-green-50 ring-green-600/20',
    Pending: 'text-gray-600 bg-gray-50 ring-gray-500/10',
    Segmented: 'text-yellow-800 bg-yellow-50 ring-yellow-600/20',
    Imported: 'text-blue-800 bg-blue-50 ring-blue-600/20',
    Error: 'text-red-800 bg-red-50 ring-red-600/20',
}

type ListItemProps = {
    micrograph: PortableMicrograph
}
const ListItem = ({ micrograph }: ListItemProps) => {
    return (
        <li key={micrograph.uuid} className="flex items-center justify-between gap-x-6 py-5">
            <div className="min-w-0">
                <div className="flex items-start gap-x-3">
                    <p className="text-sm font-semibold leading-6 text-white">{micrograph.name}</p>
                    <p
                        className={clsx(
                            statuses[micrograph.status],
                            'rounded-md whitespace-nowrap mt-0.5 px-1.5 py-0.5 text-xs font-medium ring-1 ring-inset'
                        )}
                    >
                        {micrograph.status}
                    </p>
                </div>
                <div className="mt-1 flex items-center gap-x-2 text-xs leading-5 text-gray-400">
                    <p className="whitespace-nowrap">
                        Imported on <time dateTime={micrograph.created_at}>{micrograph.created_at}</time>
                    </p>
                    <svg viewBox="0 0 2 2" className="h-0.5 w-0.5 fill-current">
                        <circle cx={1} cy={1} r={1} />
                    </svg>
                    <p className="truncate">Created by </p>
                </div>
            </div>
            <div className="flex flex-none items-center gap-x-4">
                <a
                    className="hidden rounded-md bg-white px-2.5 py-1.5 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:block"
                >
                    Open in Viewer<span className="sr-only">, {micrograph.name}</span>
                </a>
                <Menu as="div" className="relative flex-none">
                    <Menu.Button className="-m-2.5 block p-2.5 text-gray-500 hover:text-white">
                        <span className="sr-only">Open options</span>
                        <MoreVertical className="h-5 w-5" aria-hidden="true" />
                    </Menu.Button>
                    <Transition
                        as={Fragment}
                        enter="transition ease-out duration-100"
                        enterFrom="transform opacity-0 scale-95"
                        enterTo="transform opacity-100 scale-100"
                        leave="transition ease-in duration-75"
                        leaveFrom="transform opacity-100 scale-100"
                        leaveTo="transform opacity-0 scale-95"
                    >
                        <Menu.Items className="absolute right-0 z-10 mt-2 w-32 origin-top-right rounded-md bg-white py-2 shadow-lg ring-1 ring-gray-900/5 focus:outline-none">
                            <Menu.Item>
                                {({ active }) => (
                                    <a
                                        href="#"
                                        className={clsx(
                                            active ? 'bg-gray-50' : '',
                                            'block px-3 py-1 text-sm leading-6 text-gray-900'
                                        )}
                                    >
                                        Edit<span className="sr-only">, {micrograph.name}</span>
                                    </a>
                                )}
                            </Menu.Item>
                            <Menu.Item>
                                {({ active }) => (
                                    <a
                                        href="#"
                                        className={clsx(
                                            active ? 'bg-gray-50' : '',
                                            'block px-3 py-1 text-sm leading-6 text-gray-900'
                                        )}
                                    >
                                        Move<span className="sr-only">, {micrograph.name}</span>
                                    </a>
                                )}
                            </Menu.Item>
                            <Menu.Item>
                                {({ active }) => (
                                    <a
                                        href="#"
                                        className={clsx(
                                            active ? 'bg-gray-50' : '',
                                            'block px-3 py-1 text-sm leading-6 text-gray-900'
                                        )}
                                    >
                                        Delete<span className="sr-only">, {micrograph.name}</span>
                                    </a>
                                )}
                            </Menu.Item>
                        </Menu.Items>
                    </Transition>
                </Menu>
            </div>
        </li>
    );
}

export default List;