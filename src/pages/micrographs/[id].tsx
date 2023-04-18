import { useRouter } from 'next/router'
import ImageViewer from '../../components/ImageViewer';

import useSWR from 'swr'
import { invoke } from '@tauri-apps/api/tauri';
import type { Micrograph } from '../../../src-tauri/bindings/Micrograph';
import type { Segment } from "../../../src-tauri/bindings/Segment";
import Loading from '../../components/layout/Loading';
import { ProgressBar } from '@tremor/react';
import Status from '../../components/Analysis/Status';
import Analysis from '../../components/Analysis';

const MicrographPage = () => {
    const router = useRouter();
    const { id } = router.query;

    const { data, error, isLoading } = useSWR(`/micrographs/${id}`, () => fetcher(String(id)), {
        refreshInterval(latestData) {
            if (latestData?.micrograph.status === "Pending") return 1000;
            if (latestData?.segments.length ?? 0 > (latestData?.processedSegments ?? 0)) return 1000;
            return 5000;
        },
        onError: (error) => {
            console.error(error);
        }
    });

    if (error) return <div>Es ist ein Fehler aufgetreten</div>
    if (isLoading) return <Loading />

    if (!data) return <div>Keine Daten vorhanden</div>

    const { micrograph, segments, processedSegments } = data;

    return (
        <div className="space-y-4">
            <a onClick={() => router.back()} className="text-gray-600 hover:text-gray-900 cursor-pointer">← Zurück</a>

            <div className="space-y-4 2xl:space-y-0 2xl:grid 2xl:grid-cols-6 2xl:gap-4">
                <div className="2xl:col-span-2 space-y-4">
                    <div className="bg-white shadow sm:rounded-lg">
                        <div className="px-4 py-5 sm:px-6">
                            <h3 className="text-lg font-medium leading-6 text-gray-900"><>{micrograph.name} (ID: {micrograph.uuid})</></h3>
                        </div>
                    </div>
                    <Status micrograph={micrograph} segments={segments} analysedSegments={processedSegments} />
                    {micrograph.status === "Done" && (
                        <div className="bg-white shadow sm:rounded-lg">
                            <div className="px-4 py-5 sm:px-6 space-y-4">
                                <h3 className="text-lg font-medium leading-6 text-gray-900">Ergebnisse:</h3>
                                <Analysis micrograph={micrograph} segments={segments} />
                            </div>
                        </div>
                    )}
                </div>

                <div className="bg-white shadow sm:rounded-lg p-4 space-y-4 2xl:col-span-4">
                    <h3 className="text-lg font-medium leading-6 text-gray-900">Mikroskopaufnahme</h3>
                    <ImageViewer micrograph={micrograph} segments={segments} />
                </div>

            </div>
        </div>
    );
}

const fetcher = async (micrographId: string) => {
    const micrograph = await invoke("get_micrograph", { micrographId }).then((res) => JSON.parse(res as string)) as Micrograph;
    const segments = await invoke("get_segments", { micrographId }).then((res) => JSON.parse(res as string)) as Segment[];

    const processedSegments = segments.reduce((acc, segment) => segment.status === "ok" ? acc + 1 : acc, 0);

    return { micrograph, segments, processedSegments };
}

export default MicrographPage;