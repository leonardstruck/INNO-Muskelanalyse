import { PlusIcon } from '@heroicons/react/20/solid'
import { BeakerIcon } from '@heroicons/react/24/outline'
import Button from '../Button'

type EmptyCaseListProps = {
    createCase: (isOpen: boolean) => void
}

const EmptyCaseList = ({ createCase }: EmptyCaseListProps) => {
    return (
        <div className="text-center py-6 flex flex-col items-center">
            <BeakerIcon className="w-16" />
            <h3 className="mt-2 font-medium text-gray-900">Keine Fälle</h3>
            <p className="mt-1 text-sm text-gray-500">Beginnen Sie, indem Sie einen neuen Fall anlegen.</p>
            <div className="mt-6">
                <Button theme='primary' onClick={() => createCase(true)}>
                    <PlusIcon className="-ml-1 mr-2 h-5 w-5" aria-hidden="true" />
                    Fall erstellen
                </Button>
            </div>
        </div>
    )
}


export default EmptyCaseList