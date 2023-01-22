import { ArrowDownOnSquareIcon, DocumentMagnifyingGlassIcon } from '@heroicons/react/20/solid'
export type EmptyProps = {
    onImport: () => void
}


const Empty = (props: EmptyProps) => {
    return (
        <div className="text-center">
            <h3 className="mt-2 text-sm font-medium text-gray-900">Keine Mikroskopaufnahmen vorhanden</h3>
            <p className="mt-1 text-sm text-gray-500">Importieren Sie neue Aufnahmen, oder suchen Sie in den bereits importierten Aufnahmen</p>
            <div className="mt-6 space-x-4">
                <button
                    type="button"
                    className="inline-flex items-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                    onClick={props.onImport}
                >
                    <ArrowDownOnSquareIcon className="-ml-1 mr-2 h-5 w-5" aria-hidden="true" />
                    Aufnahmen importieren
                </button>
                <button
                    type="button"
                    className="inline-flex items-center rounded-md border border-indigo-600 px-4 py-2 text-sm font-medium text-indigo-600 shadow-sm hover:bg-indigo-200 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                >
                    <DocumentMagnifyingGlassIcon className="-ml-1 mr-2 h-5 w-5" aria-hidden="true" />
                    Aufnahmen suchen
                </button>
            </div>
        </div >
    );
}

export default Empty