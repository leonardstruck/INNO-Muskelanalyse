import { useEffect, useState } from "react";
import Loading from "../../components/layout/Loading";
import type { Case } from "../../types/Case";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "next/router";
import DeleteCaseModal from "../../components/cases/DeleteCaseModal";

const CasePage = () => {
    const [caseObj, setCaseObj] = useState<Case>();
    const [loading, setLoading] = useState(true);

    const [showDeleteModal, setShowDeleteModal] = useState(false);

    const router = useRouter();
    const { id } = router.query;

    const handleDelete = async (id: Number) => {
        await invoke("delete_case", { caseId: id });
        setShowDeleteModal(false);
        router.push("/cases");
    }

    useEffect(() => {
        invoke("get_case", { caseId: Number.parseInt(`${id}`) }).then((res: string) => {
            setCaseObj(JSON.parse(res) as Case);
            setLoading(false);
        }).catch((err) => {
            console.error(err);
        })
    }, [])

    if (loading) return <Loading />

    return (
        <>
            <div className="space-y-4">
                <a onClick={() => router.push("/cases")} className="text-gray-600 hover:text-gray-900 cursor-pointer">← Übersicht</a>
                <div className="bg-white shadow sm:rounded-lg">
                    <div className="px-4 py-5 sm:px-6">
                        <h3 className="text-lg font-medium leading-6 text-gray-900"><>{caseObj.name} (Fall-ID: {caseObj.id})</></h3>
                        <p className="mt-1 max-w-2xl text-sm text-gray-500">{caseObj.description}</p>
                    </div>
                    <div className="border-t border-gray-200 px-4 py-5 sm:px-6">
                        <dl className="grid grid-cols-1 gap-x-4 gap-y-8 sm:grid-cols-2">
                            <div className="sm:col-span-1">
                                <dt className="text-sm font-medium text-gray-500">Full name</dt>
                                <dd className="mt-1 text-sm text-gray-900">Margot Foster</dd>
                            </div>
                            <div className="sm:col-span-1">
                                <dt className="text-sm font-medium text-gray-500">Application for</dt>
                                <dd className="mt-1 text-sm text-gray-900">Backend Developer</dd>
                            </div>
                            <div className="sm:col-span-1">
                                <dt className="text-sm font-medium text-gray-500">Email address</dt>
                                <dd className="mt-1 text-sm text-gray-900">margotfoster@example.com</dd>
                            </div>
                            <div className="sm:col-span-1">
                                <dt className="text-sm font-medium text-gray-500">Salary expectation</dt>
                                <dd className="mt-1 text-sm text-gray-900">$120,000</dd>
                            </div>
                            <div className="sm:col-span-2">
                                <dt className="text-sm font-medium text-gray-500">About</dt>
                                <dd className="mt-1 text-sm text-gray-900">
                                    Fugiat ipsum ipsum deserunt culpa aute sint do nostrud anim incididunt cillum culpa consequat. Excepteur
                                    qui ipsum aliquip consequat sint. Sit id mollit nulla mollit nostrud in ea officia proident. Irure nostrud
                                    pariatur mollit ad adipisicing reprehenderit deserunt qui eu.
                                </dd>
                            </div>
                            <div className="sm:col-span-2">
                                <dt className="text-sm font-medium text-gray-500">Attachments</dt>
                                <dd className="mt-1 text-sm text-gray-900">
                                    <ul role="list" className="divide-y divide-gray-200 rounded-md border border-gray-200">
                                        <li className="flex items-center justify-between py-3 pl-3 pr-4 text-sm">
                                            <div className="flex w-0 flex-1 items-center">
                                                <span className="ml-2 w-0 flex-1 truncate">resume_back_end_developer.pdf</span>
                                            </div>
                                            <div className="ml-4 flex-shrink-0">
                                                <a href="#" className="font-medium text-indigo-600 hover:text-indigo-500">
                                                    Download
                                                </a>
                                            </div>
                                        </li>
                                        <li className="flex items-center justify-between py-3 pl-3 pr-4 text-sm">
                                            <div className="flex w-0 flex-1 items-center">
                                                <span className="ml-2 w-0 flex-1 truncate">coverletter_back_end_developer.pdf</span>
                                            </div>
                                            <div className="ml-4 flex-shrink-0">
                                                <a href="#" className="font-medium text-indigo-600 hover:text-indigo-500">
                                                    Download
                                                </a>
                                            </div>
                                        </li>
                                    </ul>
                                </dd>
                            </div>
                        </dl>
                    </div>
                </div>

                <div className="bg-white shadow sm:rounded-lg">
                    <div className="px-4 py-5 sm:p-6">
                        <h3 className="text-lg font-medium leading-6 text-gray-900">Fall löschen</h3>
                        <div className="mt-2 text-sm text-gray-500">
                            <p>Bitte beachten Sie, dass sowohl die Metadaten, als auch die verknüpften Bilddaten zu diesem Fall gelöscht werden.</p>
                        </div>
                        <div className="mt-5">
                            <button
                                onClick={() => setShowDeleteModal(true)}
                                type="button"
                                className="inline-flex items-center justify-center rounded-md border border-transparent bg-red-100 px-4 py-2 font-medium text-red-700 hover:bg-red-200 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 sm:text-sm"
                            >
                                Alle Daten zu diesem Fall löschen
                            </button>
                        </div>
                    </div>
                </div>

            </div>

            {/* DELETE MODAL */}
            <DeleteCaseModal open={showDeleteModal} setOpen={setShowDeleteModal} handleDelete={() => handleDelete(Number.parseInt(`${id}`))} />
        </>
    )
}

export default CasePage