import { Dialog, Transition } from "@headlessui/react";
import { Fragment, useEffect, useState } from "react";

import { XMarkIcon } from '@heroicons/react/24/outline'
import Button from "../Button";

import { invoke } from "@tauri-apps/api/tauri";

import useSWRMutation from "swr/mutation";
import type { Case } from "../../../src-tauri/bindings/Case";

type CreateCaseModalProps = {
    // pass useState setter function to close modal
    setModalOpen: (value: boolean) => void;
    // pass useState value to check if modal is open
    modalOpen: boolean;
}

async function createCase(key: string, { arg }: { arg: Pick<Case, "name" | "description"> }) {
    await invoke("create_case", { case: JSON.stringify(arg) });
};


const CreateCaseModal = ({ setModalOpen, modalOpen }: CreateCaseModalProps) => {
    const [name, setName] = useState("");
    const [description, setDescription] = useState("");

    const handleClose = () => {
        setName("");
        setDescription("");
        setModalOpen(false);
    }

    const { trigger, isMutating, error } = useSWRMutation("/cases", createCase, {
        onSuccess: handleClose
    });

    return (
        <Transition.Root show={modalOpen} as={Fragment}>
            <Dialog as="div" className="relative z-10" onClose={handleClose}>
                <div className="fixed inset-0" />

                <div className="fixed inset-0 overflow-hidden">
                    <div className="absolute inset-0 overflow-hidden">
                        <div className="pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-72">
                            <Transition.Child
                                as={Fragment}
                                enter="transform transition ease-in-out duration-500 sm:duration-700"
                                enterFrom="translate-x-full"
                                enterTo="translate-x-0"
                                leave="transform transition ease-in-out duration-500 sm:duration-700"
                                leaveFrom="translate-x-0"
                                leaveTo="translate-x-full"
                            >
                                <Dialog.Panel className="pointer-events-auto w-screen max-w-4xl">
                                    <form className="flex h-full flex-col overflow-y-scroll bg-white shadow-xl">
                                        <div className="flex-1">
                                            {/* Header */}
                                            <div className="bg-gray-50 px-4 py-6 sm:px-6">
                                                <div className="flex items-start justify-between space-x-3">
                                                    <div className="space-y-1">
                                                        <Dialog.Title className="text-lg font-medium text-gray-900">Neuer Fall</Dialog.Title>
                                                        <p className="text-sm text-gray-500">
                                                            Füllen Sie das Formular aus, um einen neuen Fall zu erstellen.
                                                        </p>
                                                    </div>
                                                    <div className="flex h-7 items-center">
                                                        <button
                                                            type="button"
                                                            className="text-gray-400 hover:text-gray-500"
                                                            onClick={handleClose}
                                                        >
                                                            <span className="sr-only">Schliessen</span>
                                                            <XMarkIcon className="h-6 w-6" aria-hidden="true" />
                                                        </button>
                                                    </div>
                                                </div>
                                            </div>

                                            {/* Divider container */}
                                            <div className="space-y-6 py-6 sm:space-y-0 sm:divide-y sm:divide-gray-200 sm:py-0">
                                                {/* Project name */}
                                                <div className="space-y-1 px-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:space-y-0 sm:px-6 sm:py-5">
                                                    <div>
                                                        <label
                                                            htmlFor="project-name"
                                                            className="block text-sm font-medium text-gray-900 sm:mt-px sm:pt-2"
                                                        >
                                                            Name
                                                        </label>
                                                    </div>
                                                    <div className="sm:col-span-2">
                                                        <input
                                                            disabled={isMutating}
                                                            type="text"
                                                            name="project-name"
                                                            id="project-name"
                                                            value={name}
                                                            onChange={(e) => setName(e.target.value)}
                                                            className="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                                                        />
                                                    </div>
                                                </div>

                                                {/* Project description */}
                                                <div className="space-y-1 px-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:space-y-0 sm:px-6 sm:py-5">
                                                    <div>
                                                        <label
                                                            htmlFor="project-description"
                                                            className="block text-sm font-medium text-gray-900 sm:mt-px sm:pt-2"
                                                        >
                                                            Beschreibung
                                                        </label>
                                                    </div>
                                                    <div className="sm:col-span-2">
                                                        <textarea
                                                            disabled={isMutating}
                                                            id="project-description"
                                                            name="project-description"
                                                            value={description}
                                                            onChange={(e) => setDescription(e.target.value)}
                                                            rows={3}
                                                            className="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                                                            defaultValue={''}
                                                        />
                                                    </div>
                                                </div>
                                            </div>
                                        </div>

                                        {/* Action buttons */}
                                        <div className="flex-shrink-0 border-t border-gray-200 px-4 py-5 sm:px-6">
                                            <div className="flex justify-end space-x-3">
                                                <Button onClick={handleClose} theme="secondary">Abbrechen</Button>
                                                <Button onClick={() => trigger({ name, description })} disabled={isMutating} theme="primary" loading={isMutating}>Fall erstellen</Button>
                                            </div>
                                        </div>
                                    </form>
                                </Dialog.Panel>
                            </Transition.Child>
                        </div>
                    </div>
                </div>
            </Dialog>
        </Transition.Root >
    )
};

export default CreateCaseModal;