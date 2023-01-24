import useSWR, { useSWRConfig } from "swr"
import Loading from "../layout/Loading"
import Empty from "./Empty"
import ImageCard from "./ImageCard"
import { emit } from "@tauri-apps/api/event"

import { handleImport, fetchMicrographs } from "./lib"
import { ArrowDownOnSquareIcon, DocumentMagnifyingGlassIcon } from "@heroicons/react/20/solid"

export type ImageBrowserProps = {
    caseId?: number
}

const ImageBrowser = ({ caseId }: ImageBrowserProps) => {
    const { data, error, isLoading } = useSWR(`cases/${caseId}/micrographs`, () => fetchMicrographs({ caseId }), {
        refreshInterval: 1000,
    });
    const { mutate } = useSWRConfig();

    const clickHandler = async () => {
        await handleImport({ linkToCase: caseId }).then(() => {
            mutate(`cases/${caseId}/micrographs`);
        });
    }


    if (isLoading) return <Loading />

    if (error) return <div>Beim Abrufen der Aufnahmen ist ein Fehler aufgetreten: {error}</div>
    if (data.length == 0) return <Empty onImport={clickHandler} />

    return (
        <>
            <ul role="list" className="grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8">
                {data.map((image) => (
                    <ImageCard key={image.uuid} image={image} />
                ))}
            </ul>
            <div className="mt-6 space-x-4">
                <button
                    type="button"
                    className="inline-flex items-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                    onClick={clickHandler}
                >
                    <ArrowDownOnSquareIcon className="-ml-1 mr-2 h-5 w-5" aria-hidden="true" />
                    Aufnahmen importieren
                </button>
                <button
                    type="button"
                    className="inline-flex items-center rounded-md border border-indigo-600 px-4 py-2 text-sm font-medium text-indigo-600 shadow-sm hover:bg-indigo-200 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                    onClick={() => {
                        emit("micrograph-import");
                    }}
                >
                    <DocumentMagnifyingGlassIcon className="-ml-1 mr-2 h-5 w-5" aria-hidden="true" />
                    Aufnahmen suchen
                </button>
            </div>
        </>
    )
}

export default ImageBrowser