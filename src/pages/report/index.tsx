import { useRouter } from "next/router";
import type { NextPageWithLayout } from "../_app";
import { useQuery } from "@tanstack/react-query";
import { CachedMicrograph } from "../../../src-tauri/bindings/CachedMicrograph";
import { CachedSegment } from "../../../src-tauri/bindings/CachedSegment";
import { invoke } from "@tauri-apps/api/tauri";
import ScatterOrientation from "../../components/reports/ScatterOrientation";

const Page: NextPageWithLayout = () => {
    const { query } = useRouter();
    const { data, isLoading } = useQuery<{ micrograph: CachedMicrograph, segments: CachedSegment[] }>(["micrograph", query.project, query.micrograph], async () => {
        return {
            micrograph: await invoke("get_micrograph", { projectId: query.project, micrographId: query.micrograph }),
            segments: await invoke("get_segments", { projectId: query.project, micrographId: query.micrograph }),
        }
    },
        {
            enabled: !!query.project && !!query.micrograph,
            refetchInterval(data, query) {
                if (data?.micrograph.status == "Done") {
                    return false;
                } else {
                    return 200;
                }
            },
        }
    );

    return (
        <div className="h-full w-full p-4 space-y-8">
            <p className="text-2xl font-bold text-white">Reports</p>
            <p className="text-xl">Scatterplot Orientation</p>
            {data?.segments && <ScatterOrientation segments={data.segments} />}
        </div>
    )
}

Page.getLayout = (page) => {
    return (
        <div className="h-full w-full">
            {page}
        </div>
    )
}

export default Page