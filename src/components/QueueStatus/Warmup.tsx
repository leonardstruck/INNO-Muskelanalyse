import { useQuery, useQueryClient } from "@tanstack/react-query";
import { QueueInfo } from "../../../src-tauri/bindings/QueueInfo";
import { invoke } from "@tauri-apps/api/tauri";

type WarmupProps = {
    queueInfo: QueueInfo
}

const Warmup = ({ queueInfo }: WarmupProps) => {
    const queryClient = useQueryClient();
    const { } = useQuery(["warmup"], warmup, {
        refetchOnWindowFocus: false,
        refetchOnMount: false,
        refetchOnReconnect: false,
        retry: false,
        onSuccess: () => {
            queryClient.invalidateQueries(["queue_status"])
        }
    })

    return (
        <div className="flex flex-row items-center bg-secondary-950 p-4 rounded-md text-sm">
            <div className="animate-pulse bg-secondary-600 h-2 w-2 rounded-full mr-2"></div>
            <span>Warming up</span>
        </div>
    )
}

const warmup = async () => {
    return invoke("queue_warmup")
}

export default Warmup;