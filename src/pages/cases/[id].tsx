import { useEffect, useState } from "react";
import Loading from "../../components/layout/Loading";
import type { Case } from "../../../src-tauri/bindings/Case";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "next/router";
import DeleteCaseModal from "../../components/cases/DeleteCaseModal";
import ImageBrowser from "../../components/ImageBrowser";

const CasePage = () => {
    const [caseObj, setCaseObj] = useState<Case>();
    const [loading, setLoading] = useState(true);

    const [showDeleteModal, setShowDeleteModal] = useState(false);

    const router = useRouter();
    const { id } = router.query;

    const caseId = parseInt(id as string);

    const handleDelete = async (id: Number) => {
        await invoke("delete_case", { caseId });
        setShowDeleteModal(false);
        router.push("/cases");
    }

    useEffect(() => {
        invoke("get_case", { caseId }).then((res: string) => {
            setCaseObj(JSON.parse(res) as Case);
            setLoading(false);
        }).catch((err) => {
            console.error(err);
        })
    }, [caseId])

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
                </div>

                <div className="bg-white shadow sm:rounded-lg">
                    <div className="px-4 py-5 sm:p-6">
                        <h3 className="text-lg font-medium leading-6 text-gray-900 mb-4">Mikroskop-Aufnahmen</h3>
                        <ImageBrowser caseId={caseId} />
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