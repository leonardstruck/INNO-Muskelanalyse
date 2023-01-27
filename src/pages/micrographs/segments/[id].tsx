import { convertFileSrc, invoke } from '@tauri-apps/api/tauri'
import Image from 'next/image'
import { useRouter } from 'next/router'
import useSWR from 'swr'

import type { Segment } from '../../../../src-tauri/bindings/Segment'
import Loading from '../../../components/layout/Loading'

const SegmentPage = () => {
    const router = useRouter()
    const { id } = router.query
    const { data, error } = useSWR(
        `/api/micrographs/segments/${id}`,
        () => fetcher(`${id}`),
    )

    if (error) return <div>Beim Laden ist ein Fehler aufgetreten: {error}</div>
    if (!data) return <Loading />

    return (
        <div className="space-y-4">
            <a onClick={() => router.back()} className="text-gray-600 hover:text-gray-900 cursor-pointer">← Zurück</a>
            <div className="bg-white shadow sm:rounded-lg">
                <div className="px-4 py-5 sm:px-6 space-y-4">
                    <h3 className="text-lg font-medium leading-6 text-gray-900"><>Segment (ID: {data.uuid})</></h3>
                    <div className="grid grid-cols-2 gap-4">
                        <div>
                            <Image src={convertFileSrc(data.path)} width={data.width} height={data.height} alt="" />
                        </div>
                        <div>
                            {data.status === "new" && <div className="text-center">Die Analyse des Segments ist noch nicht abgeschlossen.<Loading /></div>}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}

const fetcher = (id: string) => invoke("get_segment", { segmentId: id })
    .then((res: string) => JSON.parse(res) as Segment);

export default SegmentPage