import { useEffect, useState } from "react"
import { invoke } from "@tauri-apps/api/tauri"
import { Case } from "../../types/Case"
import Loading from "../../components/layout/Loading"
import { useRouter } from "next/router"
import CreateCaseModal from "../../components/cases/CreateCaseModal"
import clsx from "clsx"
import EmptyCaseList from "../../components/cases/EmptyCaseList"


const CasePage = () => {
    const router = useRouter();
    const [cases, setCases] = useState<Case[]>([]);
    const [loading, setLoading] = useState(true);

    const [showModal, setShowModal] = useState(false);

    useEffect(() => {
        invoke("get_cases").then((res: string) => {
            setCases(JSON.parse(res) as Case[]);
            setLoading(false);
        })
    }, [loading, showModal])

    if (loading) return <Loading />

    if (cases.length === 0) return (
        <>
            <h1 className="text-4xl font-display">Fälle</h1>
            <EmptyCaseList createCase={setShowModal} />
            <CreateCaseModal modalOpen={showModal} setModalOpen={setShowModal} />
        </>
    )

    return (
        <div className={clsx(showModal && "blur-sm opacity-30", "transition-all duration-500")}>
            <h1 className="text-4xl font-display">Fälle</h1>
            <div className="flex items-center">
                <div className="flex-auto">
                    <p className="mt-2 text-sm text-gray-700 dark:text-gray-200">
                        Hier können die Fälle verwaltet werden. Um einen neuen Fall zu erstellen, klicken Sie auf den Button "Fall erstellen".
                    </p>
                </div>
                <div className="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
                    <button
                        type="button"
                        onClick={() => setShowModal(true)}
                        className="inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:w-auto"
                    >
                        Fall erstellen
                    </button>
                </div>
            </div>
            <div className="mt-8 flex flex-col">
                <div className="-my-2 -mx-4 overflow-x-auto sm:-mx-6 lg:-mx-8">
                    <div className="inline-block min-w-full py-2 align-middle md:px-6 lg:px-8">
                        <div className="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
                            <table className="min-w-full divide-y divide-gray-300">
                                <thead className="bg-gray-50">
                                    <tr>
                                        <th scope="col" className="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6">
                                            Fall-ID
                                        </th>
                                        <th scope="col" className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
                                            Name
                                        </th>

                                    </tr>
                                </thead>
                                <tbody className="divide-y divide-gray-200 bg-white">
                                    {cases.map((item) => (
                                        <tr key={`case-${item.id}`} className="hover:bg-gray-50 cursor-pointer" onClick={() => router.push(`/cases/${item.id}`)}>
                                            <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">
                                                {item.id.toString()}
                                            </td>
                                            <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{item.name}</td>
                                        </tr>
                                    ))}
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            </div>
            <CreateCaseModal modalOpen={showModal} setModalOpen={setShowModal} />
        </div>
    )

}

export default CasePage