import { PlusIcon, FolderPlus } from "lucide-react"
import { Button } from "../ui/button"

type EmptyStateProps = {
    onImport: () => void
}

const EmptyState = ({ onImport }: EmptyStateProps) => {
    return (
        <div className="text-center">
            <FolderPlus className="mx-auto h-12 w-12 text-secondary-500" aria-hidden="true" />
            <h3 className="mt-4 text-sm font-semibold text-secondary-700">No micrographs</h3>
            <p className="mt-2 text-sm text-neutral-200">Get started by importing a new micrograph.</p>
            <div className="mt-12">
                <Button variant="secondary" onClick={onImport}>
                    <PlusIcon className="-ml-1 mr-2 h-5 w-5" aria-hidden="true" />
                    Import micrograph
                </Button>
            </div>
        </div>
    )
}

export default EmptyState