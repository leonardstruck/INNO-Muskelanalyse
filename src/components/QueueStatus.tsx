import { useQuery } from "@tanstack/react-query"
import { invoke } from "@tauri-apps/api/tauri"
import { QueueLengths } from "../../src-tauri/bindings/QueueLengths"
import { useAutoAnimate } from "@formkit/auto-animate/react";

const QueueStatus = () => {
    const [animationParent] = useAutoAnimate();

    const { data } = useQuery(["queue_status"], fetchStatus, {
        refetchInterval: (data, query) => {
            // refetch every second if there are items in the queue or every 5 seconds if there are no items in the queue
            return data && data.import_queue > 0 ? 1000 : 5000;
        },
    });

    return (
        <div ref={animationParent}>
            {data && data.import_queue > 0 && <div className="text-red-500">Import queue: {data.import_queue}</div>}
        </div>
    )
}

export default QueueStatus

const fetchStatus = async () => {
    return invoke("queue_get_status") as Promise<QueueLengths>
}